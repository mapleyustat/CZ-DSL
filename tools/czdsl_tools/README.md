# CZ-DSL 支撑工具集

本目录包含 CZ-DSL 的三个核心支撑工具，全部基于 Python 标准库实现，无外部依赖。

## 工具概览

| 工具 | 文件 | 功能 |
|------|------|------|
| 场景构造器 | `scene_builder.py` | 以编程方式构造 CZ-DSL 场景，输出 .cz / XML / JSON |
| 内容校验器 | `validator.py` | 解析 .cz 文件并进行语法、语义、数据适配检查 |
| 系统编译器 | `compiler.py` | 将 .cz 场景编译为 GeoJSON / CityGML / Pipeline / Report |

## 1. 场景构造器 (scene_builder.py)

### CLI 用法

```bash
# 生成 .cz 文件
python3 scene_builder.py --template hangzhou_slr --out scenario.cz

# 生成 XML
python3 scene_builder.py --template yangtze_typhoon --out scenario.xml

# 生成 JSON
python3 scene_builder.py --template wetland_restoration --out scenario.json
```

### 可用模板

- `hangzhou_slr` — 杭州湾2050海平面上升与生态修复
- `yangtze_typhoon` — 长江口台风风暴潮应急
- `wetland_restoration` — 渤海湾湿地修复与碳循环

### 编程接口

```python
from czdsl_tools import SceneBuilder

# 从模板创建
builder = SceneBuilder.from_template("hangzhou_slr")

# 序列化
cz_text = builder.to_cz()
xml_text = builder.to_xml()
json_text = builder.to_json()

# 自定义构建
builder.set_name("MyScenario")
builder.add_driver("sea_level_rise", "0.5 * meter")
```

### 原子构造关键字

生成的 .cz 文本使用以下原子构造关键字：
`data_source`, `model_adapter`, `cross_subject_link`, `llm_dispatch`,
`granularity`, `classification_layer`, `digital_twin_mapping`。

## 2. 内容校验器 (validator.py)

### CLI 用法

```bash
# 文本报告
python3 validator.py path/to/file.cz

# JSON 报告
python3 validator.py path/to/file.cz --format json
```

退出码：0 = 通过（无 error），1 = 失败（有 error），2 = 运行异常。

### 检查项

| 类别 | 检查内容 |
|------|----------|
| 语法 | 大括号匹配、未知顶层关键字 |
| 语义 | 重复标识符、未定义引用、单位基本检查 |
| 分类 | 分类编码格式 `CZ.(WD\|CC\|CP\|BE).[A-Z0-9-]+` |
| 数据适配 | data_source 缺少坐标/时间引用 |

### 编程接口

```python
from czdsl_tools import CZValidator

validator = CZValidator()
diagnostics = validator.validate_file("path/to/file.cz")
# diagnostics 是 Diagnostic 对象列表

# 格式化报告
print(validator.format_text_report("path/to/file.cz"))
print(validator.format_json_report("path/to/file.cz"))
```

## 3. 系统编译器 (compiler.py)

### CLI 用法

```bash
# GeoJSON（用于 GIS 软件）
python3 compiler.py file.cz --target geojson --out output.geojson

# CityGML 风格 XML（用于 3D 场景软件）
python3 compiler.py file.cz --target citygml --out output.gml

# Pipeline 描述符 JSON（用于模型耦合框架）
python3 compiler.py file.cz --target pipeline --out output.json

# 人类可读报告
python3 compiler.py file.cz --target report --out output.txt
```

### 编程接口

```python
from czdsl_tools import CZCompiler

compiler = CZCompiler()
geojson = compiler.compile_file("file.cz", "geojson")
report = compiler.compile_file("file.cz", "report")
```

### 目标格式说明

| 目标 | 格式 | 适用场景 |
|------|------|----------|
| geojson | GeoJSON FeatureCollection | QGIS / ArcGIS / web 地图 |
| citygml | CityGML 风格 XML | 3D 城市建模 / 数字孪生可视化 |
| pipeline | JSON pipeline descriptor | 模型耦合框架 / 工作流引擎 |
| report | 纯文本 | 人工审查 / 文档归档 |

## 环境要求

- Python 3.8+
- 仅依赖标准库（json, argparse, re, dataclasses, xml.sax.saxutils）

## 目录结构

```
tools/czdsl_tools/
├── __init__.py       # 包入口，导出三个类
├── scene_builder.py  # 场景构造器
├── validator.py      # 内容校验器
├── compiler.py       # 系统编译器
└── README.md         # 本文件
```
