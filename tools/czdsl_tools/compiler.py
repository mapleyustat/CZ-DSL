#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CZ-DSL 系统编译器 (Compiler)
=================================
将 .cz 场景转换为主流软件可消费的目标格式：
  (a) GeoJSON — 用于 GIS 软件
  (b) CityGML-style XML — 用于 3D 城市/场景软件
  (c) Python dict/JSON pipeline descriptor — 用于模型耦合框架
  (d) plain text human-readable report — 人类可读报告

CLI 用法:
    python3 compiler.py <file.cz> --target geojson [--out OUT]
    python3 compiler.py <file.cz> --target citygml [--out OUT]
    python3 compiler.py <file.cz> --target pipeline [--out OUT]
    python3 compiler.py <file.cz> --target report [--out OUT]
"""

import argparse
import json
import re
import sys
import os
from typing import List, Dict, Optional, Any, Tuple
from xml.sax.saxutils import escape as xml_escape


# ============================================================
# 解析器（复用 validator 的简化版本）
# ============================================================

class CZBlock:
    def __init__(self, keyword: str, name: str, line: int, extends: str = ""):
        self.keyword = keyword
        self.name = name
        self.line = line
        self.extends = extends
        self.content: str = ""
        self.children: List[CZBlock] = []
        self.properties: Dict[str, str] = {}
        self.nested_blocks: List[CZBlock] = []


class CZTextParser:
    """简化版 CZ-DSL 文本解析器，提取块和属性"""

    KNOWN_KEYWORDS = {
        "sphere", "scale", "data", "model", "process", "interface",
        "coupling", "coastal_database", "spatial_reference_system",
        "classification", "scenario", "rule", "infrastructure",
        "layer", "policy_package", "workflow", "ai_model",
        "digital_twin", "world_model", "compute_context", "fn",
        "data_source", "model_adapter", "cross_subject_link",
        "llm_dispatch", "granularity", "classification_layer",
        "digital_twin_mapping",
    }

    def __init__(self):
        self.blocks: List[CZBlock] = []
        self.lines: List[str] = []

    def parse(self, text: str):
        self.lines = text.split("\n")
        i = 0
        while i < len(self.lines):
            stripped = self._strip_comment(self.lines[i]).strip()
            if not stripped:
                i += 1
                continue
            block_match = self._try_block_header(stripped, i + 1)
            if block_match:
                block, end_line = self._capture_block(block_match, i)
                self.blocks.append(block)
                i = end_line + 1
            else:
                i += 1

    def _strip_comment(self, line: str) -> str:
        result = []
        in_string = False
        j = 0
        while j < len(line):
            c = line[j]
            if c == '"' and (j == 0 or line[j-1] != '\\'):
                in_string = not in_string
                result.append(c)
            elif not in_string and c == '/' and j + 1 < len(line) and line[j+1] == '/':
                break
            else:
                result.append(c)
            j += 1
        return "".join(result)

    def _try_block_header(self, line: str, lineno: int) -> Optional[CZBlock]:
        match = re.match(
            r'^(\w+)\s+(\w+(?:<[^>]*>)?)\s*(?:extends\s+(\w+))?\s*\{',
            line
        )
        if not match:
            return None
        return CZBlock(match.group(1), match.group(2), lineno, match.group(3) or "")

    def _capture_block(self, block: CZBlock, start_idx: int) -> Tuple[CZBlock, int]:
        """从 start_idx 开始捕获块体，返回 (block, end_line_index)"""
        depth = 0
        body_lines = []
        i = start_idx
        while i < len(self.lines):
            raw = self.lines[i]
            stripped = self._strip_comment(raw)
            # 计算 brace 变化
            in_string = False
            for col, ch in enumerate(raw):
                if ch == '"' and (col == 0 or raw[col-1] != '\\'):
                    in_string = not in_string
                if in_string:
                    continue
                if ch == '{':
                    depth += 1
                elif ch == '}':
                    depth -= 1
            if depth <= 0 and i > start_idx:
                # 本行包含了闭合大括号
                block.content = "\n".join(body_lines)
                self._extract_properties(block)
                return block, i
            body_lines.append(raw)
            i += 1
        block.content = "\n".join(body_lines)
        self._extract_properties(block)
        return block, i

    def _extract_properties(self, block: CZBlock):
        """从块体中提取 key: value 属性"""
        for line in block.content.split("\n"):
            stripped = self._strip_comment(line).strip()
            if not stripped:
                continue
            # 匹配 key: value 或 key: value,
            m = re.match(r'^(\w+)\s*:\s*(.+?)(?:,\s*)?$', stripped)
            if m and '{' not in m.group(2) and '}' not in m.group(2):
                key = m.group(1)
                val = m.group(2).strip().rstrip(',').strip()
                if key not in ("inputs", "outputs", "measures"):
                    block.properties[key] = val

    def get_blocks_by_keyword(self, keyword: str) -> List[CZBlock]:
        return [b for b in self.blocks if b.keyword == keyword]

    def get_all_names(self) -> Dict[str, str]:
        """返回 name -> keyword 映射"""
        return {b.name: b.keyword for b in self.blocks}


# ============================================================
# 编译器
# ============================================================

class CZCompiler:
    """CZ-DSL 系统编译器"""

    def __init__(self):
        self.parser = CZTextParser()
        self.source_file: str = ""

    @staticmethod
    def _portable_source(path: str) -> str:
        """Express the source path relative to the project root when possible.

        Compiled artefacts record where they came from. An absolute,
        machine-specific path would make byte-identical builds differ between
        two checkouts of the same revision, so the path is stored relative to
        the repository root whenever the file lives inside it.
        """
        if not path:
            return ""
        try:
            root = os.path.dirname(os.path.dirname(os.path.dirname(
                os.path.abspath(__file__))))
            rel = os.path.relpath(os.path.abspath(path), root)
            return path if rel.startswith("..") else rel
        except Exception:
            return path

    def compile_file(self, filepath: str, target: str) -> str:
        """编译文件到目标格式"""
        self.source_file = self._portable_source(filepath)
        with open(filepath, "r", encoding="utf-8") as f:
            text = f.read()
        return self.compile_text(text, target, filepath)

    def compile_text(self, text: str, target: str, source: str = "") -> str:
        """编译文本到目标格式"""
        self.source_file = self._portable_source(source)
        self.parser.parse(text)
        if target == "geojson":
            return self._to_geojson()
        elif target == "citygml":
            return self._to_citygml()
        elif target == "pipeline":
            return self._to_pipeline()
        elif target == "report":
            return self._to_report()
        else:
            raise ValueError(f"未知目标格式: {target}")

    # ---- GeoJSON ----

    def _to_geojson(self) -> str:
        """转换为 GeoJSON FeatureCollection"""
        features = []
        all_names = self.parser.get_all_names()

        for block in self.parser.blocks:
            props = dict(block.properties)
            # 提取坐标信息
            coords = self._extract_coordinates(block)
            geom = None
            if coords:
                geom = {
                    "type": "Point",
                    "coordinates": coords
                }
            elif "spatial" in props or "spatial_ref" in props:
                geom = {
                    "type": "Polygon",
                    "coordinates": [[]]
                }

            feature = {
                "type": "Feature",
                "geometry": geom,
                "properties": {
                    "czdsl_type": block.keyword,
                    "name": block.name,
                    "extends": block.extends,
                    "source_line": block.line,
                    **{k: v for k, v in props.items()
                       if not k.startswith("_") and len(v) < 500}
                }
            }
            features.append(feature)

        fc = {
            "type": "FeatureCollection",
            "metadata": {
                "generator": "CZ-DSL Compiler",
                "source_file": self.source_file,
                "feature_count": len(features),
            },
            "features": features
        }
        return json.dumps(fc, indent=2, ensure_ascii=False)

    def _extract_coordinates(self, block: CZBlock) -> Optional[List[float]]:
        """尝试从块中提取坐标"""
        text = block.content
        # 查找 (lon, lat, ...) 格式
        m = re.search(r'\(([\d.]+)([EW])\s*,\s*([\d.]+)([NS])', text)
        if m:
            lon = float(m.group(1)) * (1 if m.group(2) == 'E' else -1)
            lat = float(m.group(3)) * (1 if m.group(4) == 'N' else -1)
            return [lon, lat]
        # 查找数字坐标对
        m = re.search(r'origin:\s*\(([\d.]+),\s*([\d.]+)', text)
        if m:
            return [float(m.group(1)), float(m.group(2))]
        # 查找经纬度属性
        for pat in [
            r'westBoundLongitude:\s*(-?[\d.]+).*?northBoundLatitude:\s*(-?[\d.]+)',
            r'lon(?:gitude)?:\s*(-?[\d.]+).*?lat(?:itude)?:\s*(-?[\d.]+)',
        ]:
            m = re.search(pat, text, re.DOTALL)
            if m:
                return [float(m.group(1)), float(m.group(2))]
        return None

    # ---- CityGML-style XML ----

    def _to_citygml(self) -> str:
        """转换为 CityGML 风格 XML 存根"""
        ind = "  "
        lines = [
            '<?xml version="1.0" encoding="UTF-8"?>',
            '<gml:FeatureCollection',
            '  xmlns:gml="http://www.opengis.net/gml/3.2"',
            '  xmlns:czdsl="http://czdsl.org/ns/citygml"',
            '  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"',
            '  gml:id="czdsl_citygml_export">',
        ]

        for block in self.parser.blocks:
            lines.append(f'{ind*1}<gml:featureMember>')
            city_type = self._map_to_citygml_type(block.keyword)
            lines.append(f'{ind*2}<{city_type} gml:id="{block.name}">')

            # 名称
            lines.append(f'{ind*3}<gml:name>{xml_escape(block.name)}</gml:name>')
            if block.extends:
                lines.append(f'{ind*3}<czdsl:extends>{xml_escape(block.extends)}</czdsl:extends>')

            # 几何（如果有坐标）
            coords = self._extract_coordinates(block)
            if coords:
                lines.append(f'{ind*3}<gml:boundedBy>')
                lines.append(f'{ind*4}<gml:Envelope srsName="EPSG:4326">')
                lines.append(f'{ind*5}<gml:lowerCorner>{coords[0]} {coords[1]}</gml:lowerCorner>')
                lines.append(f'{ind*5}<gml:upperCorner>{coords[0]} {coords[1]}</gml:upperCorner>')
                lines.append(f'{ind*4}</gml:Envelope>')
                lines.append(f'{ind*3}</gml:boundedBy>')

            # 属性
            for key, val in block.properties.items():
                if len(val) > 500:
                    val = val[:500] + "..."
                lines.append(f'{ind*3}<czdsl:{key}>{xml_escape(val)}</czdsl:{key}>')

            lines.append(f'{ind*2}</{city_type}>')
            lines.append(f'{ind*1}</gml:featureMember>')

        lines.append('</gml:FeatureCollection>')
        return "\n".join(lines)

    def _map_to_citygml_type(self, keyword: str) -> str:
        mapping = {
            "infrastructure": "czdsl:CoastalInfrastructure",
            "layer": "czdsl:Layer",
            "coastal_database": "czdsl:CoastalDatabase",
            "data_source": "czdsl:DataSource",
            "model_adapter": "czdsl:ModelAdapter",
            "digital_twin": "czdsl:DigitalTwin",
            "digital_twin_mapping": "czdsl:DigitalTwinMapping",
            "scenario": "czdsl:Scenario",
            "classification": "czdsl:Classification",
            "classification_layer": "czdsl:ClassificationLayer",
            "spatial_reference_system": "czdsl:SpatialReferenceSystem",
        }
        return mapping.get(keyword, f"czdsl:{keyword.capitalize()}")

    # ---- Pipeline descriptor (JSON) ----

    def _to_pipeline(self) -> str:
        """转换为模型耦合框架的 pipeline 描述符"""
        pipeline = {
            "metadata": {
                "generator": "CZ-DSL Compiler",
                "source_file": self.source_file,
                "format": "pipeline-descriptor/v1",
            },
            "stages": [],
            "connections": [],
            "data_products": [],
        }

        all_names = self.parser.get_all_names()

        # data_source -> stages
        for block in self.parser.get_blocks_by_keyword("data_source"):
            stage = {
                "name": block.name,
                "type": "data_source",
                "inputs": [],
                "outputs": [block.name],
                "properties": block.properties,
            }
            pipeline["stages"].append(stage)
            pipeline["data_products"].append({
                "name": block.name,
                "type": "data",
                "format": block.properties.get("format", "unknown"),
            })

        # model_adapter / model -> stages
        for block in self.parser.get_blocks_by_keyword("model_adapter"):
            inputs = self._parse_list_property(block.properties.get("inputs", ""))
            outputs = self._parse_list_property(block.properties.get("outputs", ""))
            stage = {
                "name": block.name,
                "type": "model_adapter",
                "engine": block.properties.get("engine", ""),
                "inputs": inputs,
                "outputs": outputs,
                "properties": block.properties,
            }
            pipeline["stages"].append(stage)

        for block in self.parser.get_blocks_by_keyword("model"):
            stage = {
                "name": block.name,
                "type": "model",
                "inputs": [],
                "outputs": [block.name],
                "properties": block.properties,
            }
            pipeline["stages"].append(stage)

        # process -> stages
        for block in self.parser.get_blocks_by_keyword("process"):
            stage = {
                "name": block.name,
                "type": "process",
                "inputs": [],
                "outputs": [],
                "properties": block.properties,
            }
            pipeline["stages"].append(stage)

        # cross_subject_link -> connections
        for block in self.parser.get_blocks_by_keyword("cross_subject_link"):
            conn = {
                "name": block.name,
                "source": block.properties.get("source", ""),
                "target": block.properties.get("target", ""),
                "link_type": block.properties.get("link_type", ""),
                "variables": self._parse_list_property(
                    block.properties.get("variables", "")),
            }
            pipeline["connections"].append(conn)

        # workflow -> pipeline stages
        for block in self.parser.get_blocks_by_keyword("workflow"):
            stage = {
                "name": block.name,
                "type": "workflow",
                "inputs": [],
                "outputs": [],
                "properties": block.properties,
            }
            pipeline["stages"].append(stage)

        # scenario -> top-level configuration
        for block in self.parser.get_blocks_by_keyword("scenario"):
            pipeline.setdefault("scenarios", []).append({
                "name": block.name,
                "properties": block.properties,
            })

        # llm_dispatch -> stages
        for block in self.parser.get_blocks_by_keyword("llm_dispatch"):
            stage = {
                "name": block.name,
                "type": "llm_dispatch",
                "task": block.properties.get("task", ""),
                "inputs": self._parse_list_property(
                    block.properties.get("inputs", "")),
                "outputs": self._parse_list_property(
                    block.properties.get("outputs", "")),
            }
            pipeline["stages"].append(stage)

        # digital_twin_mapping -> connections
        for block in self.parser.get_blocks_by_keyword("digital_twin_mapping"):
            conn = {
                "name": block.name,
                "source": block.properties.get("physical_entity", ""),
                "target": block.properties.get("digital_entity", ""),
                "link_type": "digital_twin",
                "sync_mode": block.properties.get("sync_mode", ""),
            }
            pipeline["connections"].append(conn)

        return json.dumps(pipeline, indent=2, ensure_ascii=False)

    def _parse_list_property(self, val: str) -> List[str]:
        """解析 [a, b, c] 格式的列表属性"""
        val = val.strip().strip("[]")
        if not val:
            return []
        return [x.strip().strip('"') for x in val.split(",") if x.strip()]

    # ---- Human-readable report ----

    def _to_report(self) -> str:
        """生成人类可读报告"""
        lines = []
        lines.append("=" * 70)
        lines.append("CZ-DSL scenario compilation report")
        lines.append("=" * 70)
        lines.append(f"Source file: {self.source_file}")
        lines.append(f"Total blocks: {len(self.parser.blocks)}")
        lines.append("")

        # 按关键字分组统计
        keyword_counts: Dict[str, int] = {}
        for b in self.parser.blocks:
            keyword_counts[b.keyword] = keyword_counts.get(b.keyword, 0) + 1
        lines.append("Component counts:")
        for kw, count in sorted(keyword_counts.items()):
            lines.append(f"  {kw:30s}: {count}")
        lines.append("")

        # 详细列出每个块
        lines.append("-" * 70)
        lines.append("Details:")
        lines.append("-" * 70)
        for block in self.parser.blocks:
            lines.append("")
            lines.append(f"[{block.keyword}] {block.name}" +
                         (f" extends {block.extends}" if block.extends else ""))
            lines.append(f"  defined at line {block.line}")
            if block.properties:
                lines.append("  properties:")
                for key, val in sorted(block.properties.items()):
                    display = val if len(val) <= 80 else val[:77] + "..."
                    lines.append(f"    {key}: {display}")
            else:
                lines.append("  (no extracted properties)")

        # 交叉引用
        lines.append("")
        lines.append("-" * 70)
        all_names = self.parser.get_all_names()
        lines.append(f"Defined entities ({len(all_names)}):")
        for name, kw in sorted(all_names.items()):
            lines.append(f"  {name:40s} ({kw})")

        lines.append("")
        lines.append("=" * 70)
        lines.append("End of report")
        lines.append("=" * 70)
        return "\n".join(lines)


# ============================================================
# CLI
# ============================================================

def main():
    parser = argparse.ArgumentParser(
        description="CZ-DSL 系统编译器"
    )
    parser.add_argument("file", help="要编译的 .cz 文件路径")
    parser.add_argument(
        "--target",
        required=True,
        choices=["geojson", "citygml", "pipeline", "report"],
        help="目标格式"
    )
    parser.add_argument(
        "--out",
        default=None,
        help="输出文件路径（不指定则输出到 stdout）"
    )
    args = parser.parse_args()

    compiler = CZCompiler()
    try:
        result = compiler.compile_file(args.file, args.target)
    except FileNotFoundError:
        print(f"错误：文件不存在: {args.file}", file=sys.stderr)
        sys.exit(2)
    except Exception as e:
        print(f"错误：编译过程中发生异常: {e}", file=sys.stderr)
        sys.exit(2)

    if args.out:
        parent_dir = os.path.dirname(args.out)
        if parent_dir:
            os.makedirs(parent_dir, exist_ok=True)
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(result)
        print(f"输出已写入: {args.out} ({len(result)} 字节)")
    else:
        print(result)


if __name__ == "__main__":
    main()
