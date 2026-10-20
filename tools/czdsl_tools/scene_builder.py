#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CZ-DSL 场景构造器 (Scene Builder)
=====================================
提供 SceneBuilder 类，以编程方式构造 CZ-DSL 场景模型，
并序列化为 .cz 文本、XML（XSD 结构）和 JSON。

CLI 用法:
    python3 scene_builder.py --template hangzhou_slr --out scenario.cz
    python3 scene_builder.py --template yangtze_typhoon --out scenario.xml
    python3 scene_builder.py --template wetland_restoration --out scenario.json

模板:
    hangzhou_slr      — 杭州湾2050海平面上升与生态修复
    yangtze_typhoon   — 长江口台风风暴潮应急
    wetland_restoration — 渤海湾湿地修复与碳循环
"""

import json
import argparse
import sys
import os
from dataclasses import dataclass, field, asdict
from typing import List, Dict, Optional, Any
from xml.sax.saxutils import escape as xml_escape

# ============================================================
# 数据模型：7 类基础分量 + 四级分层分类体系
# ============================================================

# 四级分类体系常量
CLASSIFICATION_LEVELS = ["domain", "subdomain", "component", "atom"]
CLASSIFICATION_PREFIXES = {
    "WD": "Wetland / 湿地",
    "CC": "Coastal City / 海岸城市",
    "CP": "Coastal Process / 海岸过程",
    "BE": "Blue Economy / 蓝色经济",
}


@dataclass
class ClassificationCode:
    """四级分类编码，格式: CZ.<domain>.<subdomain>.<component>"""
    code: str  # e.g. "CZ.WD.SaltMarsh.Vegetation"
    domain: str = ""
    subdomain: str = ""
    component: str = ""

    def __post_init__(self):
        parts = self.code.split(".")
        if len(parts) >= 4:
            self.domain = parts[1]
            self.subdomain = parts[2]
            self.component = parts[3]


@dataclass
class DataSource:
    """基础分量 1: 数据源"""
    name: str
    source_type: str  # observation / simulation / remote_sensing
    coordinate_ref: str = ""
    temporal_ref: str = ""
    resolution: str = ""
    format: str = ""
    url: str = ""
    classification: str = ""


@dataclass
class ModelAdapter:
    """基础分量 2: 模型适配器"""
    name: str
    model_type: str  # hydrodynamics / ecological / economic ...
    engine: str = ""  # FVCOM / SWAN / SLAMM ...
    inputs: List[str] = field(default_factory=list)
    outputs: List[str] = field(default_factory=list)
    parameters: Dict[str, str] = field(default_factory=dict)
    classification: str = ""


@dataclass
class CrossSubjectLink:
    """基础分量 3: 跨主体链接"""
    name: str
    source: str
    target: str
    link_type: str  # data_flow / feedback / constraint
    variables: List[str] = field(default_factory=list)
    coupling_interval: str = ""


@dataclass
class LLMDispatch:
    """基础分量 4: LLM 调度"""
    name: str
    task: str
    model_name: str = ""
    prompt_template: str = ""
    inputs: List[str] = field(default_factory=list)
    outputs: List[str] = field(default_factory=list)


@dataclass
class Granularity:
    """基础分量 5: 粒度控制"""
    name: str
    spatial_resolution: str = ""
    temporal_resolution: str = ""
    level: str = ""  # regional / local / micro
    lod: int = 0  # Level of Detail


@dataclass
class ClassificationLayer:
    """基础分量 6: 分类层"""
    name: str
    standard: str = "CMECS"
    codes: List[ClassificationCode] = field(default_factory=list)
    description: str = ""


@dataclass
class DigitalTwinMapping:
    """基础分量 7: 数字孪生映射"""
    name: str
    physical_entity: str = ""
    digital_entity: str = ""
    sync_mode: str = ""  # real_time / scheduled / on_demand
    sensors: List[str] = field(default_factory=list)
    actuators: List[str] = field(default_factory=list)


@dataclass
class Scenario:
    """CZ-DSL 场景模型"""
    name: str = ""
    description: str = ""
    time_horizon: str = ""
    spatial_ref: str = ""
    data_sources: List[DataSource] = field(default_factory=list)
    model_adapters: List[ModelAdapter] = field(default_factory=list)
    cross_subject_links: List[CrossSubjectLink] = field(default_factory=list)
    llm_dispatchs: List[LLMDispatch] = field(default_factory=list)
    granularities: List[Granularity] = field(default_factory=list)
    classification_layers: List[ClassificationLayer] = field(default_factory=list)
    digital_twin_mappings: List[DigitalTwinMapping] = field(default_factory=list)
    drivers: Dict[str, str] = field(default_factory=dict)
    objectives: List[str] = field(default_factory=list)


# ============================================================
# 模板
# ============================================================

def _template_hangzhou_slr() -> Scenario:
    """杭州湾2050海平面上升与生态修复"""
    s = Scenario(
        name="HangzhouBay_2050_SLR",
        description="杭州湾2050海平面上升与生态修复综合场景",
        time_horizon="2050",
        spatial_ref="WGS84_UTM_Zone_51N",
        drivers={
            "sea_level_rise": "0.5 * meter",
            "tidal_amplification": "1.05",
            "subsidence_rate": "3.0 with unit mm/year",
        },
        objectives=[
            "minimize(Expected_Annual_Loss - 0.3 * Ecosystem_Service_Value)",
            "maximize(Habitat_Connectivity)",
        ],
    )
    s.data_sources = [
        DataSource("HangzhouBay_Bathymetry", "observation", "WGS84_UTM_Zone_51N",
                   "UTC", "250.0 * meter", "NetCDF", "cdb://D300/S002/T001",
                   "CZ.CP.Bathymetry.Grid"),
        DataSource("HangzhouBay_LandUse_2023", "remote_sensing", "WGS84",
                   "2023", "50.0 * meter", "GeoTIFF", "file://landuse_2023.tif",
                   "CZ.CC.LandUse.Industrial"),
        DataSource("HangzhouBay_SaltMarsh", "remote_sensing", "WGS84",
                   "2023", "50.0 * meter", "GeoJSON", "file://saltmarsh.geojson",
                   "CZ.WD.SaltMarsh.Vegetation"),
    ]
    s.model_adapters = [
        ModelAdapter("FVCOM_Hydro", "hydrodynamics", "FVCOM",
                     ["bathymetry", "tide_forcing", "wind_field"],
                     ["zeta", "u", "v", "salinity"],
                     {"horizontal_resolution": "250 * meter",
                      "vertical_layers": "10"},
                     "CZ.CP.Hydrodynamics.FVCOM"),
        ModelAdapter("SLAMM_Marsh", "ecological", "SLAMM",
                     ["dem", "sea_level_rise", "salinity"],
                     ["marsh_migration", "habitat_loss"],
                     {"time_step": "5 * year"},
                     "CZ.WD.SaltMarsh.SLAMM"),
    ]
    s.cross_subject_links = [
        CrossSubjectLink("Hydro2Eco", "FVCOM_Hydro", "SLAMM_Marsh",
                         "data_flow", ["zeta", "salinity"], "1 * day"),
        CrossSubjectLink("Eco2Economy", "SLAMM_Marsh", "EconomicAssessment",
                         "feedback", ["habitat_loss_area"], "1 * year"),
    ]
    s.llm_dispatchs = [
        LLMDispatch("SLR_Inference", "infer_sea_level_rise_scenario",
                     "GPT-4-Coastal",
                     "Given region {region} and year {year}, estimate SLR",
                     ["region", "year"], ["slr_estimate"]),
    ]
    s.granularities = [
        Granularity("Regional", "250 * meter", "1 * day", "regional", 1),
        Granularity("Local", "50 * meter", "1 * hour", "local", 2),
    ]
    s.classification_layers = [
        ClassificationLayer("CMECS_HangzhouBay", "CMECS",
                            [ClassificationCode("CZ.WD.SaltMarsh.Low"),
                             ClassificationCode("CZ.WD.Substrate.Mud"),
                             ClassificationCode("CZ.CP.Tidal.Flood")],
                            "杭州湾CMECS生态分类层"),
    ]
    s.digital_twin_mappings = [
        DigitalTwinMapping("SeawallTwin", "Seawall_Section_A",
                           "DigitalSeawallModel", "real_time",
                           ["strain_gauge", "water_level_sensor"],
                           ["gate_controller"]),
    ]
    return s


def _template_yangtze_typhoon() -> Scenario:
    """长江口台风风暴潮应急"""
    s = Scenario(
        name="YangtzeEstuary_Typhoon_Surge",
        description="长江口台风风暴潮应急响应综合场景",
        time_horizon="2025-09",
        spatial_ref="YangtzeEstuarySRS",
        drivers={
            "sea_level_anomaly": "0.5 with unit meter",
            "wind_field": "from TyphoonHaikui",
            "river_discharge": "35000.0 with unit m^3/s",
        },
        objectives=[
            "minimize(MaxInundationDepth)",
            "minimize(EvacuationTime)",
        ],
    )
    s.data_sources = [
        DataSource("Yangtze_Tide_Gauge", "observation", "YangtzeEstuarySRS",
                   "UTC", "1 * minute", "CSV", "sensor://tide_gauge_network",
                   "CZ.CP.Tide.Gauge"),
        DataSource("TyphoonHaikui_Track", "observation", "WGS84",
                   "UTC", "6 * hour", "GeoJSON", "noaa://best_track",
                   "CZ.CP.Typhoon.Track"),
        DataSource("ChongmingDEM", "remote_sensing", "WGS84_UTM_Zone_51N",
                   "2023", "10 * meter", "GeoTIFF", "file://chongming_dem.tif",
                   "CZ.CP.Topography.DEM"),
    ]
    s.model_adapters = [
        ModelAdapter("SWAN_Wave", "wave", "SWAN",
                     ["wind_field", "bathymetry"],
                     ["Hsig", "Dir", "dissipation"],
                     {"coupling": "two_way"},
                     "CZ.CP.Wave.SWAN"),
        ModelAdapter("ADCIRC_Surge", "storm_surge", "ADCIRC",
                     ["wind_field", "pressure_field", "tide"],
                     ["surge_level", "inundation_extent"],
                     {"unstructured": "true"},
                     "CZ.CP.Surge.ADCIRC"),
    ]
    s.cross_subject_links = [
        CrossSubjectLink("Wave2Surge", "SWAN_Wave", "ADCIRC_Surge",
                         "data_flow", ["Hsig", "Dir"], "600 * second"),
        CrossSubjectLink("Surge2Warning", "ADCIRC_Surge", "EmergencyWarning",
                         "feedback", ["surge_level"], "10 * minute"),
    ]
    s.llm_dispatchs = [
        LLMDispatch("EvacuationPlanner", "generate_evacuation_plan",
                     "GPT-4-Coastal",
                     "Given surge level {surge} and population {pop}, plan evacuation",
                     ["surge_level", "population"], ["evacuation_routes"]),
    ]
    s.granularities = [
        Granularity("Estuary", "500 * meter", "1 * hour", "regional", 1),
        Granularity("Coastline", "50 * meter", "10 * minute", "local", 2),
    ]
    s.classification_layers = [
        ClassificationLayer("TyphoonImpact", "EDCS",
                            [ClassificationCode("CZ.CP.Typhoon.Category4"),
                             ClassificationCode("CZ.CC.Infrastructure.Seawall"),
                             ClassificationCode("CZ.WD.TidalFlat.Wetland")],
                            "台风影响分类层"),
    ]
    s.digital_twin_mappings = [
        DigitalTwinMapping("EstuaryTwin", "YangtzeEstuary",
                           "DigitalEstuaryModel", "real_time",
                           ["tide_gauge", "weather_radar", "satellite"],
                           ["warning_system", "evacuation_alert"]),
    ]
    return s


def _template_wetland_restoration() -> Scenario:
    """渤海湾湿地修复与碳循环"""
    s = Scenario(
        name="Bohai_Wetland_Restoration",
        description="渤海湾湿地修复与蓝碳循环综合场景",
        time_horizon="2030",
        spatial_ref="BohaiBay_SRS",
        drivers={
            "sea_level_rise": "0.3 * meter",
            "sediment_supply": "150.0 with unit kg/s",
            "restoration_area": "50.0 with unit km^2",
        },
        objectives=[
            "maximize(Carbon_Sequestration)",
            "maximize(Biodiversity_Index)",
            "minimize(Restoration_Cost)",
        ],
    )
    s.data_sources = [
        DataSource("Bohai_Sediment", "observation", "BohaiBay_SRS",
                   "UTC", "100 * meter", "NetCDF", "cdb://sediment_survey",
                   "CZ.WD.Substrate.Sediment"),
        DataSource("Bohai_Vegetation", "remote_sensing", "WGS84",
                   "2024", "30 * meter", "GeoTIFF", "file://vegetation_2024.tif",
                   "CZ.WD.SaltMarsh.Suaeda"),
        DataSource("Carbon_Flux", "observation", "BohaiBay_SRS",
                   "UTC", "1 * hour", "CSV", "sensor://eddy_covariance",
                   "CZ.BE.Carbon.Flux"),
    ]
    s.model_adapters = [
        ModelAdapter("Delft3D_Morpho", "morphodynamics", "Delft3D",
                     ["sediment_supply", "tide", "wave"],
                     ["bed_change", "shoreline_position"],
                     {"morpho_factor": "10"},
                     "CZ.CP.Morpho.Delft3D"),
        ModelAdapter("CarbonBudget", "biogeochemical", "CustomCarbonModel",
                     ["biomass", "sediment", "temperature"],
                     ["carbon_sequestration", "methane_flux"],
                     {"calibration": "Bohai_2023"},
                     "CZ.BE.Carbon.Budget"),
    ]
    s.cross_subject_links = [
        CrossSubjectLink("Morpho2Carbon", "Delft3D_Morpho", "CarbonBudget",
                         "data_flow", ["bed_change", "sediment"], "1 * month"),
        CrossSubjectLink("Carbon2Economy", "CarbonBudget", "CarbonCredit",
                         "feedback", ["carbon_sequestration"], "1 * year"),
    ]
    s.llm_dispatchs = [
        LLMDispatch("RestorationAdvisor", "recommend_restoration_strategy",
                     "GPT-4-Coastal",
                     "Given site {site} and constraints {constraints}, recommend restoration",
                     ["site_conditions", "budget"], ["restoration_plan"]),
    ]
    s.granularities = [
        Granularity("Bay", "500 * meter", "1 * day", "regional", 1),
        Granularity("Marsh", "30 * meter", "1 * hour", "local", 2),
        Granularity("Micro", "1 * meter", "1 * minute", "micro", 3),
    ]
    s.classification_layers = [
        ClassificationLayer("BohaiCMECS", "CMECS",
                            [ClassificationCode("CZ.WD.SaltMarsh.Suaeda"),
                             ClassificationCode("CZ.WD.Substrate.Mud"),
                             ClassificationCode("CZ.BE.Carbon.Sequestration"),
                             ClassificationCode("CZ.CP.Tidal.Flood")],
                            "渤海湾CMECS蓝碳分类层"),
    ]
    s.digital_twin_mappings = [
        DigitalTwinMapping("WetlandTwin", "BohaiWetland",
                           "DigitalWetlandModel", "scheduled",
                           ["eddy_covariance", "sediment_sensor", "drone"],
                           ["water_level_control", "inflow_gate"]),
    ]
    return s


TEMPLATES = {
    "hangzhou_slr": _template_hangzhou_slr,
    "yangtze_typhoon": _template_yangtze_typhoon,
    "wetland_restoration": _template_wetland_restoration,
}


# ============================================================
# SceneBuilder 类
# ============================================================

class SceneBuilder:
    """CZ-DSL 场景构造器"""

    def __init__(self, scenario: Optional[Scenario] = None):
        self.scenario = scenario or Scenario()

    @classmethod
    def from_template(cls, template_name: str) -> "SceneBuilder":
        if template_name not in TEMPLATES:
            raise ValueError(
                f"未知模板 '{template_name}'，可用模板: {', '.join(TEMPLATES.keys())}"
            )
        return cls(TEMPLATES[template_name]())

    def set_name(self, name: str):
        self.scenario.name = name

    def set_description(self, desc: str):
        self.scenario.description = desc

    def add_data_source(self, ds: DataSource):
        self.scenario.data_sources.append(ds)

    def add_model_adapter(self, ma: ModelAdapter):
        self.scenario.model_adapters.append(ma)

    def add_cross_subject_link(self, link: CrossSubjectLink):
        self.scenario.cross_subject_links.append(link)

    def add_llm_dispatch(self, llm: LLMDispatch):
        self.scenario.llm_dispatchs.append(llm)

    def add_granularity(self, g: Granularity):
        self.scenario.granularities.append(g)

    def add_classification_layer(self, cl: ClassificationLayer):
        self.scenario.classification_layers.append(cl)

    def add_digital_twin_mapping(self, dtm: DigitalTwinMapping):
        self.scenario.digital_twin_mappings.append(dtm)

    def add_driver(self, key: str, value: str):
        self.scenario.drivers[key] = value

    def add_objective(self, obj: str):
        self.scenario.objectives.append(obj)

    # ---- 序列化：.cz 文本 ----

    def to_cz(self) -> str:
        """序列化为 CZ-DSL 格式文本"""
        lines = []
        s = self.scenario
        lines.append(f"// CZ-DSL 场景: {s.name}")
        lines.append(f"// {s.description}")
        lines.append("")

        # drivers
        if s.drivers:
            lines.append("// 驱动因子")
            for k, v in s.drivers.items():
                lines.append(f"//   {k}: {v}")
            lines.append("")

        # data_source
        for ds in s.data_sources:
            lines.append(f"data_source {ds.name} {{")
            lines.append(f"    source_type: {ds.source_type},")
            if ds.coordinate_ref:
                lines.append(f"    coordinate_ref: {ds.coordinate_ref},")
            if ds.temporal_ref:
                lines.append(f"    temporal_ref: {ds.temporal_ref},")
            if ds.resolution:
                lines.append(f"    resolution: {ds.resolution},")
            if ds.format:
                lines.append(f"    format: {ds.format},")
            if ds.url:
                lines.append(f"    url: \"{ds.url}\",")
            if ds.classification:
                lines.append(f"    classification: {ds.classification},")
            lines.append("}")
            lines.append("")

        # model_adapter
        for ma in s.model_adapters:
            lines.append(f"model_adapter {ma.name} {{")
            lines.append(f"    model_type: {ma.model_type},")
            if ma.engine:
                lines.append(f"    engine: {ma.engine},")
            if ma.inputs:
                lines.append(f"    inputs: [{', '.join(ma.inputs)}],")
            if ma.outputs:
                lines.append(f"    outputs: [{', '.join(ma.outputs)}],")
            if ma.parameters:
                lines.append("    parameters: {")
                for pk, pv in ma.parameters.items():
                    lines.append(f"        {pk}: {pv},")
                lines.append("    },")
            if ma.classification:
                lines.append(f"    classification: {ma.classification},")
            lines.append("}")
            lines.append("")

        # cross_subject_link
        for csl in s.cross_subject_links:
            lines.append(f"cross_subject_link {csl.name} {{")
            lines.append(f"    source: {csl.source},")
            lines.append(f"    target: {csl.target},")
            lines.append(f"    link_type: {csl.link_type},")
            if csl.variables:
                lines.append(f"    variables: [{', '.join(csl.variables)}],")
            if csl.coupling_interval:
                lines.append(f"    coupling_interval: {csl.coupling_interval},")
            lines.append("}")
            lines.append("")

        # llm_dispatch
        for llm in s.llm_dispatchs:
            lines.append(f"llm_dispatch {llm.name} {{")
            lines.append(f"    task: {llm.task},")
            if llm.model_name:
                lines.append(f"    model: \"{llm.model_name}\",")
            if llm.prompt_template:
                lines.append(f"    prompt: \"{llm.prompt_template}\",")
            if llm.inputs:
                lines.append(f"    inputs: [{', '.join(llm.inputs)}],")
            if llm.outputs:
                lines.append(f"    outputs: [{', '.join(llm.outputs)}],")
            lines.append("}")
            lines.append("")

        # granularity
        for g in s.granularities:
            lines.append(f"granularity {g.name} {{")
            if g.spatial_resolution:
                lines.append(f"    spatial: {g.spatial_resolution},")
            if g.temporal_resolution:
                lines.append(f"    temporal: {g.temporal_resolution},")
            if g.level:
                lines.append(f"    level: {g.level},")
            lines.append(f"    lod: {g.lod},")
            lines.append("}")
            lines.append("")

        # classification_layer
        for cl in s.classification_layers:
            lines.append(f"classification_layer {cl.name} {{")
            lines.append(f"    standard: {cl.standard},")
            if cl.description:
                lines.append(f"    description: \"{cl.description}\",")
            if cl.codes:
                lines.append("    codes: [")
                for c in cl.codes:
                    lines.append(f"        {c.code},")
                lines.append("    ],")
            lines.append("}")
            lines.append("")

        # digital_twin_mapping
        for dtm in s.digital_twin_mappings:
            lines.append(f"digital_twin_mapping {dtm.name} {{")
            lines.append(f"    physical_entity: {dtm.physical_entity},")
            lines.append(f"    digital_entity: {dtm.digital_entity},")
            lines.append(f"    sync_mode: {dtm.sync_mode},")
            if dtm.sensors:
                lines.append(f"    sensors: [{', '.join(dtm.sensors)}],")
            if dtm.actuators:
                lines.append(f"    actuators: [{', '.join(dtm.actuators)}],")
            lines.append("}")
            lines.append("")

        # objectives
        if s.objectives:
            lines.append("// 目标函数")
            for obj in s.objectives:
                lines.append(f"//   {obj}")
            lines.append("")

        return "\n".join(lines)

    # ---- 序列化：XML ----

    def to_xml(self) -> str:
        """序列化为 XSD 结构 XML"""
        s = self.scenario
        ind = "  "
        lines = [
            '<?xml version="1.0" encoding="UTF-8"?>',
            '<czdsl:Scenario xmlns:czdsl="http://czdsl.org/ns"',
            '  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"',
            '  xsi:schemaLocation="http://czdsl.org/ns czdsl.xsd">',
        ]

        def _elem(tag, value, indent=2):
            if value:
                lines.append(f"{ind*indent}<{tag}>{xml_escape(str(value))}</{tag}>")

        _elem("Name", s.name)
        _elem("Description", s.description)
        _elem("TimeHorizon", s.time_horizon)
        _elem("SpatialReference", s.spatial_ref)

        # drivers
        if s.drivers:
            lines.append(f"{ind*2}<Drivers>")
            for k, v in s.drivers.items():
                lines.append(f'{ind*3}<Driver name="{k}">{xml_escape(v)}</Driver>')
            lines.append(f"{ind*2}</Drivers>")

        # data sources
        if s.data_sources:
            lines.append(f"{ind*2}<DataSources>")
            for ds in s.data_sources:
                lines.append(f'{ind*3}<DataSource name="{ds.name}">')
                _elem("SourceType", ds.source_type, 4)
                _elem("CoordinateRef", ds.coordinate_ref, 4)
                _elem("TemporalRef", ds.temporal_ref, 4)
                _elem("Resolution", ds.resolution, 4)
                _elem("Format", ds.format, 4)
                _elem("URL", ds.url, 4)
                _elem("Classification", ds.classification, 4)
                lines.append(f"{ind*3}</DataSource>")
            lines.append(f"{ind*2}</DataSources>")

        # model adapters
        if s.model_adapters:
            lines.append(f"{ind*2}<ModelAdapters>")
            for ma in s.model_adapters:
                lines.append(f'{ind*3}<ModelAdapter name="{ma.name}">')
                _elem("ModelType", ma.model_type, 4)
                _elem("Engine", ma.engine, 4)
                if ma.inputs:
                    lines.append(f"{ind*4}<Inputs>")
                    for inp in ma.inputs:
                        lines.append(f"{ind*5}<Input>{inp}</Input>")
                    lines.append(f"{ind*4}</Inputs>")
                if ma.outputs:
                    lines.append(f"{ind*4}<Outputs>")
                    for out in ma.outputs:
                        lines.append(f"{ind*5}<Output>{out}</Output>")
                    lines.append(f"{ind*4}</Outputs>")
                if ma.parameters:
                    lines.append(f"{ind*4}<Parameters>")
                    for pk, pv in ma.parameters.items():
                        lines.append(f'{ind*5}<Parameter name="{pk}">{pv}</Parameter>')
                    lines.append(f"{ind*4}</Parameters>")
                _elem("Classification", ma.classification, 4)
                lines.append(f"{ind*3}</ModelAdapter>")
            lines.append(f"{ind*2}</ModelAdapters>")

        # cross subject links
        if s.cross_subject_links:
            lines.append(f"{ind*2}<CrossSubjectLinks>")
            for csl in s.cross_subject_links:
                lines.append(f'{ind*3}<CrossSubjectLink name="{csl.name}">')
                _elem("Source", csl.source, 4)
                _elem("Target", csl.target, 4)
                _elem("LinkType", csl.link_type, 4)
                if csl.variables:
                    lines.append(f"{ind*4}<Variables>")
                    for var in csl.variables:
                        lines.append(f"{ind*5}<Variable>{var}</Variable>")
                    lines.append(f"{ind*4}</Variables>")
                _elem("CouplingInterval", csl.coupling_interval, 4)
                lines.append(f"{ind*3}</CrossSubjectLink>")
            lines.append(f"{ind*2}</CrossSubjectLinks>")

        # llm dispatch
        if s.llm_dispatchs:
            lines.append(f"{ind*2}<LLMDispatchs>")
            for llm in s.llm_dispatchs:
                lines.append(f'{ind*3}<LLMDispatch name="{llm.name}">')
                _elem("Task", llm.task, 4)
                _elem("Model", llm.model_name, 4)
                _elem("PromptTemplate", llm.prompt_template, 4)
                if llm.inputs:
                    lines.append(f"{ind*4}<Inputs>")
                    for inp in llm.inputs:
                        lines.append(f"{ind*5}<Input>{inp}</Input>")
                    lines.append(f"{ind*4}</Inputs>")
                if llm.outputs:
                    lines.append(f"{ind*4}<Outputs>")
                    for out in llm.outputs:
                        lines.append(f"{ind*5}<Output>{out}</Output>")
                    lines.append(f"{ind*4}</Outputs>")
                lines.append(f"{ind*3}</LLMDispatch>")
            lines.append(f"{ind*2}</LLMDispatchs>")

        # granularity
        if s.granularities:
            lines.append(f"{ind*2}<Granularities>")
            for g in s.granularities:
                lines.append(f'{ind*3}<Granularity name="{g.name}">')
                _elem("SpatialResolution", g.spatial_resolution, 4)
                _elem("TemporalResolution", g.temporal_resolution, 4)
                _elem("Level", g.level, 4)
                _elem("LOD", g.lod, 4)
                lines.append(f"{ind*3}</Granularity>")
            lines.append(f"{ind*2}</Granularities>")

        # classification layers
        if s.classification_layers:
            lines.append(f"{ind*2}<ClassificationLayers>")
            for cl in s.classification_layers:
                lines.append(f'{ind*3}<ClassificationLayer name="{cl.name}">')
                _elem("Standard", cl.standard, 4)
                _elem("Description", cl.description, 4)
                if cl.codes:
                    lines.append(f"{ind*4}<Codes>")
                    for c in cl.codes:
                        lines.append(f'{ind*5}<Code domain="{c.domain}" '
                                     f'subdomain="{c.subdomain}" '
                                     f'component="{c.component}">{c.code}</Code>')
                    lines.append(f"{ind*4}</Codes>")
                lines.append(f"{ind*3}</ClassificationLayer>")
            lines.append(f"{ind*2}</ClassificationLayers>")

        # digital twin mappings
        if s.digital_twin_mappings:
            lines.append(f"{ind*2}<DigitalTwinMappings>")
            for dtm in s.digital_twin_mappings:
                lines.append(f'{ind*3}<DigitalTwinMapping name="{dtm.name}">')
                _elem("PhysicalEntity", dtm.physical_entity, 4)
                _elem("DigitalEntity", dtm.digital_entity, 4)
                _elem("SyncMode", dtm.sync_mode, 4)
                if dtm.sensors:
                    lines.append(f"{ind*4}<Sensors>")
                    for sen in dtm.sensors:
                        lines.append(f"{ind*5}<Sensor>{sen}</Sensor>")
                    lines.append(f"{ind*4}</Sensors>")
                if dtm.actuators:
                    lines.append(f"{ind*4}<Actuators>")
                    for act in dtm.actuators:
                        lines.append(f"{ind*5}<Actuator>{act}</Actuator>")
                    lines.append(f"{ind*4}</Actuators>")
                lines.append(f"{ind*3}</DigitalTwinMapping>")
            lines.append(f"{ind*2}</DigitalTwinMappings>")

        # objectives
        if s.objectives:
            lines.append(f"{ind*2}<Objectives>")
            for obj in s.objectives:
                lines.append(f"{ind*3}<Objective>{xml_escape(obj)}</Objective>")
            lines.append(f"{ind*2}</Objectives>")

        lines.append("</czdsl:Scenario>")
        return "\n".join(lines)

    # ---- 序列化：JSON ----

    def to_json(self) -> str:
        """序列化为 JSON"""
        s = self.scenario
        obj = {
            "name": s.name,
            "description": s.description,
            "time_horizon": s.time_horizon,
            "spatial_ref": s.spatial_ref,
            "drivers": s.drivers,
            "data_sources": [asdict(ds) for ds in s.data_sources],
            "model_adapters": [asdict(ma) for ma in s.model_adapters],
            "cross_subject_links": [asdict(csl) for csl in s.cross_subject_links],
            "llm_dispatchs": [asdict(llm) for llm in s.llm_dispatchs],
            "granularities": [asdict(g) for g in s.granularities],
            "classification_layers": [asdict(cl) for cl in s.classification_layers],
            "digital_twin_mappings": [asdict(dtm) for dtm in s.digital_twin_mappings],
            "objectives": s.objectives,
        }
        return json.dumps(obj, indent=2, ensure_ascii=False)


# ============================================================
# CLI
# ============================================================

def main():
    parser = argparse.ArgumentParser(
        description="CZ-DSL 场景构造器"
    )
    parser.add_argument(
        "--template",
        required=True,
        choices=list(TEMPLATES.keys()),
        help="场景模板名称"
    )
    parser.add_argument(
        "--out",
        required=True,
        help="输出文件路径（根据扩展名决定格式：.cz / .xml / .json）"
    )
    args = parser.parse_args()

    builder = SceneBuilder.from_template(args.template)

    ext = os.path.splitext(args.out)[1].lower()
    if ext == ".cz":
        content = builder.to_cz()
    elif ext == ".xml":
        content = builder.to_xml()
    elif ext == ".json":
        content = builder.to_json()
    else:
        print(f"错误：不支持的输出格式 '{ext}'，请使用 .cz / .xml / .json", file=sys.stderr)
        sys.exit(1)

    parent_dir = os.path.dirname(args.out)
    if parent_dir:
        os.makedirs(parent_dir, exist_ok=True)
    with open(args.out, "w", encoding="utf-8") as f:
        f.write(content)

    print(f"场景已生成: {args.out} ({len(content)} 字节)")


if __name__ == "__main__":
    main()
