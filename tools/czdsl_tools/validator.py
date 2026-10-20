#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CZ-DSL 内容校验器 (Validator)
=================================
解析 .cz 文件（行级 + 大括号匹配的实用解析器，不依赖 Rust parser），
进行语法、语义、数据适配三类检查。

CLI 用法:
    python3 validator.py <file.cz> [--format text|json]
"""

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Set, Tuple


# ============================================================
# 已知顶层关键字（复合构造 + 原子构造）
# ============================================================

KNOWN_TOP_LEVEL_KEYWORDS = {
    # v1 关键字
    "sphere", "scale", "data", "model", "process", "interface",
    "coupling", "coastal_database", "spatial_reference_system",
    "classification", "scenario", "rule", "infrastructure",
    "layer", "policy_package", "workflow", "ai_model",
    "digital_twin", "world_model", "compute_context", "fn",
    # 原子构造关键字
    "data_source", "model_adapter", "cross_subject_link",
    "llm_dispatch", "granularity", "classification_layer",
    "digital_twin_mapping",
}

# 已知单位集合（基本检查）
KNOWN_UNITS = {
    "meter", "kilometer", "centimeter", "millimeter", "km", "m", "cm", "mm",
    "second", "minute", "hour", "day", "year", "s", "min", "h",
    "kelvin", "celsius", "fahrenheit",
    "pascal", "hectopascal", "bar", "atm", "Pa", "hPa",
    "joule", "watt", "J", "W",
    "degree", "radian", "deg", "rad",
    "percent", "psu", "ppt",
    "newton", "N",
    "kg", "g", "ton",
    "m/s", "m^3/s", "kg/m^3", "g/m^2", "watt/m^2", "joule/m^3",
    "million_CNY", "billion_CNY", "billion_CNY_per_year",
    "mm/year", "kg/s", "km^2",
    "m^3/(kg*s^2)", "kg/(m^2*s)",
    "points",
}

# 分类编码正则: CZ.(WD|CC|CP|BE).[A-Z0-9-]+
CLASSIFICATION_PATTERN = re.compile(r'CZ\.(WD|CC|CP|BE)\.[A-Z0-9-]+')


# ============================================================
# 诊断消息
# ============================================================

@dataclass
class Diagnostic:
    severity: str  # error / warning / info
    line: int
    column: int
    code: str
    message: str

    def __str__(self):
        return f"[{self.severity.upper():7s}] {self.line}:{self.column} {self.code}: {self.message}"


# ============================================================
# 解析器：行级 + 大括号深度追踪
# ============================================================

@dataclass
class Block:
    keyword: str
    name: str
    line: int
    extends: str = ""
    body_start: int = 0  # line index of opening {
    body_end: int = 0    # line index of closing }
    content: str = ""
    children: List["Block"] = field(default_factory=list)


class CZParser:
    """行级 CZ-DSL 解析器，支持大括号匹配和注释剥离"""

    def __init__(self):
        self.lines: List[str] = []
        self.blocks: List[Block] = []
        self.diagnostics: List[Diagnostic] = []
        # 收集所有定义的标识符
        self.defined_ids: Dict[str, int] = {}  # name -> line
        # 收集所有引用
        self.references: Dict[str, List[int]] = {}  # referenced_name -> [lines]
        # 收集 data_source 块
        self.data_source_blocks: List[Block] = []
        # 收集 classification 编码
        self.classification_codes: List[Tuple[str, int]] = []

    def parse(self, text: str) -> List[Diagnostic]:
        self.lines = text.split("\n")
        self.diagnostics = []
        self._scan_blocks()
        self._collect_symbols()
        return self.diagnostics

    def _strip_comment(self, line: str) -> str:
        """移除行内注释（// ...），但不破坏字符串内的 //"""
        result = []
        in_string = False
        i = 0
        while i < len(line):
            c = line[i]
            if c == '"' and (i == 0 or line[i-1] != '\\'):
                in_string = not in_string
                result.append(c)
            elif not in_string and c == '/' and i + 1 < len(line) and line[i+1] == '/':
                break
            else:
                result.append(c)
            i += 1
        return "".join(result)

    def _scan_blocks(self):
        """扫描顶层块定义和大括号匹配"""
        depth = 0
        brace_stack: List[Tuple[int, int]] = []  # (line, col)
        current_top: Optional[Block] = None
        top_body_lines: List[str] = []

        for lineno, raw_line in enumerate(self.lines, 1):
            stripped = self._strip_comment(raw_line).strip()
            if not stripped:
                continue

            # 检查顶层块定义：keyword Name [extends Parent] {
            if depth == 0:
                block = self._try_parse_block_header(stripped, lineno)
                if block:
                    current_top = block
                    self.blocks.append(block)
                    if block.keyword == "data_source":
                        self.data_source_blocks.append(block)
                    # 不 continue，需要处理本行的 { 

            # 大括号匹配
            in_string = False
            for col, ch in enumerate(raw_line, 1):
                if ch == '"' and (col == 1 or raw_line[col-2] != '\\'):
                    in_string = not in_string
                if in_string:
                    continue
                if ch == '{':
                    depth += 1
                    if depth == 1 and current_top:
                        current_top.body_start = lineno
                        top_body_lines = []
                elif ch == '}':
                    depth -= 1
                    if depth < 0:
                        self.diagnostics.append(Diagnostic(
                            "error", lineno, col, "SYNTAX001",
                            "多余的闭合大括号 '}'"
                        ))
                        depth = 0
                    elif depth == 0 and current_top:
                        current_top.body_end = lineno
                        current_top.content = "\n".join(top_body_lines)
                        current_top = None

            if current_top and depth >= 1:
                top_body_lines.append(raw_line)

        if depth > 0:
            self.diagnostics.append(Diagnostic(
                "error", len(self.lines), 1, "SYNTAX002",
                f"未闭合的大括号，深度={depth}（文件末尾）"
            ))

    def _try_parse_block_header(self, line: str, lineno: int) -> Optional[Block]:
        """尝试解析顶层块头: keyword Name [extends Parent] {"""
        # 匹配: word word [extends word] {
        # 或者: word word {
        match = re.match(
            r'^(\w+)\s+(\w+(?:<[^>]*>)?)\s*(?:extends\s+(\w+))?\s*\{',
            line
        )
        if not match:
            return None

        keyword = match.group(1)
        name = match.group(2)
        extends = match.group(3) or ""

        if keyword not in KNOWN_TOP_LEVEL_KEYWORDS:
            self.diagnostics.append(Diagnostic(
                "error", lineno, 1, "SYNTAX003",
                f"未知的顶层关键字 '{keyword}'"
            ))

        block = Block(keyword=keyword, name=name, line=lineno, extends=extends)

        # 检查重复 ID
        if name in self.defined_ids:
            self.diagnostics.append(Diagnostic(
                "error", lineno, 1, "SEMANTIC001",
                f"重复的标识符 '{name}'（首次定义于第 {self.defined_ids[name]} 行）"
            ))
        else:
            self.defined_ids[name] = lineno

        # 记录 extends 引用
        if extends:
            self.references.setdefault(extends, []).append(lineno)

        return block

    def _collect_symbols(self):
        """收集引用和分类编码"""
        for lineno, raw_line in enumerate(self.lines, 1):
            stripped = self._strip_comment(raw_line)

            # 查找分类编码
            for m in CLASSIFICATION_PATTERN.finditer(stripped):
                self.classification_codes.append((m.group(0), lineno))

            # 查找引用：from X, reference: X, query_database("...").filter(...)
            for pat in [
                r'\bfrom\s+(\w+)',
                r'\breference:\s*(\w+)',
                r'\bphysical:\s*(\w+)',
                r'\bhabitat:\s*(\w+)',
                r'\bsuitability_rule:\s*(\w+)',
                r'\bland_use:\s*(\w+)',
                r'\badaptation_plan:\s*(\w+)',
                r'\bnested_within:\s*(\w+)',
                r'\bdomain:\s*(\w+)',
            ]:
                for m in re.finditer(pat, stripped):
                    ref_name = m.group(1)
                    # 过滤掉类型名和关键字
                    if ref_name not in KNOWN_TOP_LEVEL_KEYWORDS:
                        self.references.setdefault(ref_name, []).append(lineno)

            # 检查单位声明
            unit_match = re.findall(r'with\s+unit\s+(\S+?)[,\s\]]', stripped)
            for u in unit_match:
                # 基本单位检查
                base_u = u.rstrip(',;')
                if base_u and base_u not in KNOWN_UNITS:
                    # 可能是复合单位，只对简单标识符做检查
                    if re.match(r'^[a-zA-Z_]+$', base_u) and '/' not in base_u:
                        self.diagnostics.append(Diagnostic(
                            "warning", lineno, 1, "UNIT001",
                            f"可能未知的单位 '{base_u}'"
                        ))


# ============================================================
# 校验器
# ============================================================

class CZValidator:
    """CZ-DSL 内容校验器"""

    def __init__(self):
        self.parser = CZParser()
        self.diagnostics: List[Diagnostic] = []

    def validate(self, text: str) -> List[Diagnostic]:
        """校验文本，返回诊断列表"""
        self.diagnostics = self.parser.parse(text)
        self._check_semantic()
        self._check_data_adaptation()
        self._check_classification_format()
        return self.diagnostics

    def validate_file(self, filepath: str) -> List[Diagnostic]:
        """校验文件"""
        with open(filepath, "r", encoding="utf-8") as f:
            text = f.read()
        return self.validate(text)

    def _check_semantic(self):
        """语义检查：未定义引用"""
        defined = set(self.parser.defined_ids.keys())
        # 同时 extends 的父类可能是外部标准（如 EDCS_Tropical_Cyclone），不报错
        for ref_name, lines in self.parser.references.items():
            if ref_name not in defined:
                # 过滤掉已知类型/标准前缀
                if ref_name.startswith("EDCS_") or ref_name.startswith("CMECS_"):
                    continue
                if ref_name in ("CoastalCoordinateSystem", "Atmosphere",
                                "Ocean", "Lithosphere", "Biosphere",
                                "Anthroposphere", "String", "Real", "Int32",
                                "Float64", "DateTime", "Boolean"):
                    continue
                for lineno in lines[:1]:  # 只报告第一次
                    self.diagnostics.append(Diagnostic(
                        "warning", lineno, 1, "SEMANTIC002",
                        f"引用了未定义的标识符 '{ref_name}'"
                    ))

    def _check_data_adaptation(self):
        """数据适配检查：data_source 缺少坐标/时间引用"""
        for block in self.parser.data_source_blocks:
            content = block.content
            has_coord = bool(re.search(r'coordinate_ref|spatial_ref|crs|epsg|WGS84|UTM|SRS', content))
            has_temporal = bool(re.search(r'temporal_ref|temporal|time|UTC|year|date', content))

            if not has_coord:
                self.diagnostics.append(Diagnostic(
                    "warning", block.line, 1, "DATA001",
                    f"data_source '{block.name}' 缺少坐标参考系声明"
                ))
            if not has_temporal:
                self.diagnostics.append(Diagnostic(
                    "warning", block.line, 1, "DATA002",
                    f"data_source '{block.name}' 缺少时间参考声明"
                ))

        # 对 coastal_database 也做检查
        for block in self.parser.blocks:
            if block.keyword == "coastal_database":
                content = block.content
                has_spatial = bool(re.search(r'spatial|SRM|SRS|WGS84|UTM', content))
                has_temporal = bool(re.search(r'temporal|time|UTC|year|date', content))
                if not has_spatial:
                    self.diagnostics.append(Diagnostic(
                        "warning", block.line, 1, "DATA001",
                        f"coastal_database '{block.name}' 缺少空间参考声明"
                    ))
                if not has_temporal:
                    self.diagnostics.append(Diagnostic(
                        "warning", block.line, 1, "DATA002",
                        f"coastal_database '{block.name}' 缺少时间参考声明"
                    ))

        # 对 scenario 也做基本检查
        for block in self.parser.blocks:
            if block.keyword == "scenario":
                content = block.content
                has_spatial = bool(re.search(r'spatial_ref|spatial|SRM|SRS|WGS84|UTM', content))
                if not has_spatial:
                    self.diagnostics.append(Diagnostic(
                        "info", block.line, 1, "DATA003",
                        f"scenario '{block.name}' 未声明空间参考"
                    ))

    def _check_classification_format(self):
        """检查分类编码格式：CZ.(WD|CC|CP|BE).[A-Z0-9-]+"""
        # 检查文本中所有 CZ. 开头的编码
        for lineno, raw_line in enumerate(self.parser.lines, 1):
            stripped = self.parser._strip_comment(raw_line)
            # 查找所有 CZ. 开头的模式
            for m in re.finditer(r'CZ\.[A-Za-z0-9._-]+', stripped):
                code = m.group(0)
                if not CLASSIFICATION_PATTERN.match(code):
                    self.diagnostics.append(Diagnostic(
                        "warning", lineno, m.start() + 1, "CLASS001",
                        f"分类编码格式不规范: '{code}'，"
                        f"应为 CZ.(WD|CC|CP|BE).[A-Z0-9-]+"
                    ))

    # ---- 报告格式化 ----

    def format_text_report(self, filepath: str = "") -> str:
        """格式化文本报告"""
        lines = []
        lines.append("=" * 70)
        lines.append("CZ-DSL 内容校验报告")
        lines.append("=" * 70)
        if filepath:
            lines.append(f"文件: {filepath}")
        lines.append("")

        errors = [d for d in self.diagnostics if d.severity == "error"]
        warnings = [d for d in self.diagnostics if d.severity == "warning"]
        infos = [d for d in self.diagnostics if d.severity == "info"]

        lines.append(f"总计: {len(self.diagnostics)} 条诊断")
        lines.append(f"  错误(error): {len(errors)}")
        lines.append(f"  警告(warning): {len(warnings)}")
        lines.append(f"  信息(info): {len(infos)}")
        lines.append("")

        if self.diagnostics:
            lines.append("-" * 70)
            for d in self.diagnostics:
                lines.append(str(d))
            lines.append("-" * 70)
        else:
            lines.append("未发现问题。")

        lines.append("")
        lines.append(f"校验结果: {'失败' if errors else '通过'}")
        return "\n".join(lines)

    def format_json_report(self, filepath: str = "") -> str:
        """格式化 JSON 报告"""
        errors = [d for d in self.diagnostics if d.severity == "error"]
        report = {
            "file": filepath,
            "summary": {
                "total": len(self.diagnostics),
                "errors": len(errors),
                "warnings": len(warnings) if False else len([d for d in self.diagnostics if d.severity == "warning"]),
                "infos": len([d for d in self.diagnostics if d.severity == "info"]),
            },
            "diagnostics": [
                {
                    "severity": d.severity,
                    "line": d.line,
                    "column": d.column,
                    "code": d.code,
                    "message": d.message,
                }
                for d in self.diagnostics
            ],
            "passed": len(errors) == 0,
        }
        return json.dumps(report, indent=2, ensure_ascii=False)


# ============================================================
# CLI
# ============================================================

def main():
    parser = argparse.ArgumentParser(
        description="CZ-DSL 内容校验器"
    )
    parser.add_argument("file", help="要校验的 .cz 文件路径")
    parser.add_argument(
        "--format",
        choices=["text", "json"],
        default="text",
        help="报告格式 (默认: text)"
    )
    args = parser.parse_args()

    validator = CZValidator()
    try:
        validator.validate_file(args.file)
    except FileNotFoundError:
        print(f"错误：文件不存在: {args.file}", file=sys.stderr)
        sys.exit(2)
    except Exception as e:
        print(f"错误：校验过程中发生异常: {e}", file=sys.stderr)
        sys.exit(2)

    if args.format == "json":
        print(validator.format_json_report(args.file))
    else:
        print(validator.format_text_report(args.file))

    errors = [d for d in validator.diagnostics if d.severity == "error"]
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
