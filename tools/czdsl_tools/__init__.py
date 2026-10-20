#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CZ-DSL 支撑工具集
=====================
提供场景构造、内容校验、系统编译三大工具。

用法:
    from czdsl_tools import SceneBuilder, CZValidator, CZCompiler
"""

from .scene_builder import SceneBuilder
from .validator import CZValidator, CZParser, Diagnostic
from .compiler import CZCompiler, CZTextParser, CZBlock

__all__ = [
    "SceneBuilder", "CZValidator", "CZCompiler",
    # parsing primitives, needed for query and reorganisation benchmarks
    "CZParser", "CZTextParser", "CZBlock", "Diagnostic",
]
__version__ = "1.0.0"
