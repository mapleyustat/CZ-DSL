// 复杂工业级CZ-DSL示例
const COMPLEX_EXAMPLES = {
    // 长江口-东海综合观测系统
    yangtze_estuary: `// 长江口-东海多分辨率综合观测与模拟系统
// Yangtze Estuary - East China Sea Multi-resolution Integrated System

// ==================== 大气圈 Atmosphere Sphere ====================
sphere 大气圈 extends AtmosphereBase {
    // 气象要素 - Meteorological Variables (25 variables)
    properties: {
        // 基本气象要素
        air_temperature: Real with unit celsius,
        air_pressure: Real with unit hectopascal,
        relative_humidity: Real with unit percent,
        absolute_humidity: Real with unit g/m^3,
        dew_point: Real with unit celsius,
        
        // 风场要素
        wind_speed_10m: Real with unit m/s,
        wind_direction_10m: Real with unit degree,
        wind_speed_100m: Real with unit m/s,
        wind_direction_100m: Real with unit degree,
        wind_gust: Real with unit m/s,
        wind_stress: Vector2D with unit pascal,
        
        // 辐射要素
        solar_radiation: Real with unit W/m^2,
        longwave_radiation: Real with unit W/m^2,
        net_radiation: Real with unit W/m^2,
        photosynthetically_active_radiation: Real with unit μmol/(m^2*s),
        
        // 降水要素
        precipitation_rate: Real with unit mm/h,
        precipitation_accumulation: Real with unit mm,
        precipitation_type: Categorical<Rain, Snow, Sleet>,
        
        // 云和能见度
        cloud_cover: Real with unit percent,
        cloud_base_height: Real with unit meter,
        visibility: Real with unit kilometer,
        
        // 大气化学
        pm25_concentration: Real with unit μg/m^3,
        pm10_concentration: Real with unit μg/m^3,
        ozone_concentration: Real with unit μg/m^3,
        nitrogen_dioxide: Real with unit μg/m^3
    },
    
    // 大气过程
    processes: {
        evaporation: Process<Ocean.surface_water -> Atmosphere.water_vapor>,
        precipitation: Process<Atmosphere.water_vapor -> Ocean.surface_water>,
        wind_stress_transfer: Process<Atmosphere.wind_momentum -> Ocean.surface_stress>,
        heat_exchange: Process<Atmosphere.sensible_heat <-> Ocean.surface_heat>,
        气体交换: Process<Atmosphere.co2 <-> Ocean.dissolved_co2>
    }
}

// ==================== 海洋圈 Ocean Sphere ====================
sphere 海洋圈 extends OceanBase {
    // 海洋物理要素 - Ocean Physical Variables (35 variables)
    properties: {
        // 温度场
        sea_surface_temperature: Real with unit celsius,
        subsurface_temperature: Field3D<Real> with unit celsius,
        temperature_gradient: Vector3D with unit celsius/meter,
        mixed_layer_depth: Real with unit meter,
        thermocline_depth: Real with unit meter,
        
        // 盐度场
        sea_surface_salinity: Real with unit psu,
        subsurface_salinity: Field3D<Real> with unit psu,
        salinity_gradient: Vector3D with unit psu/meter,
        halocline_depth: Real with unit meter,
        
        // 流场
        surface_current_velocity: Vector2D with unit m/s,
        subsurface_current_velocity: Field3D<Vector3D> with unit m/s,
        geostrophic_velocity: Vector2D with unit m/s,
        ekman_velocity: Vector2D with unit m/s,
        tidal_velocity: Vector2D with unit m/s,
        
        // 海面高度和压力
        sea_surface_height: Real with unit meter,
        sea_level_anomaly: Real with unit meter,
        bottom_pressure: Real with unit decibar,
        hydrostatic_pressure: Field3D<Real> with unit decibar,
        
        // 波浪要素
        significant_wave_height: Real with unit meter,
        peak_wave_period: Real with unit second,
        wave_direction: Real with unit degree,
        wave_energy_spectrum: Field2D<Real> with unit m^2*s,
        whitecap_coverage: Real with unit percent,
        
        // 海洋化学
        dissolved_oxygen: Field3D<Real> with unit mg/L,
        ph_value: Field3D<Real> with unit dimensionless,
        dissolved_co2: Field3D<Real> with unit μmol/kg,
        alkalinity: Field3D<Real> with unit μmol/kg,
        nitrate_concentration: Field3D<Real> with unit μmol/L,
        phosphate_concentration: Field3D<Real> with unit μmol/L,
        silicate_concentration: Field3D<Real> with unit μmol/L,
        
        // 海洋光学
        chlorophyll_a: Field3D<Real> with unit mg/m^3,
        turbidity: Field3D<Real> with unit NTU,
        colored_dissolved_organic_matter: Field3D<Real> with unit m^-1,
        suspended_sediment: Field3D<Real> with unit mg/L,
        
        // 海冰（冬季）
        sea_ice_concentration: Real with unit percent,
        sea_ice_thickness: Real with unit meter
    },
    
    processes: {
        tidal_mixing: Process<TidalEnergy -> TurbulentKineticEnergy>,
        upwelling: Process<DeepWater -> SurfaceWater>,
        coastal_upwelling: Process<WindStress -> VerticalVelocity>,
        thermohaline_circulation: Process<DensityGradient -> GeostrophicFlow>,
        wave_breaking: Process<WaveEnergy -> TurbulentMixing>
    }
}

// ==================== 陆地圈 Terrestrial Sphere ====================
sphere 陆地圈 extends TerrestrialBase {
    // 陆地水文要素 - Terrestrial Hydrological Variables (25 variables)
    properties: {
        // 河流要素
        river_discharge: Real with unit m^3/s,
        river_stage: Real with unit meter,
        river_velocity: Real with unit m/s,
        river_temperature: Real with unit celsius,
        river_salinity: Real with unit psu,
        
        // 径流组分
        surface_runoff: Real with unit mm/day,
        subsurface_runoff: Real with unit mm/day,
        baseflow: Real with unit mm/day,
        
        // 土壤水分
        soil_moisture_surface: Real with unit percent,
        soil_moisture_root_zone: Real with unit percent,
        soil_temperature: Field3D<Real> with unit celsius,
        
        // 地下水
        groundwater_level: Real with unit meter,
        groundwater_discharge: Real with unit m^3/s,
        
        // 植被要素
        leaf_area_index: Real with unit dimensionless,
        vegetation_cover: Real with unit percent,
        biomass: Real with unit kg/m^2,
        
        // 陆地化学
        river_nitrate: Real with unit mg/L,
        river_phosphate: Real with unit mg/L,
        river_silicate: Real with unit mg/L,
        suspended_sediment_load: Real with unit kg/s,
        dissolved_organic_carbon: Real with unit mg/L,
        
        // 土地利用
        urban_fraction: Real with unit percent,
        agricultural_fraction: Real with unit percent,
        forest_fraction: Real with unit percent,
        wetland_fraction: Real with unit percent
    },
    
    processes: {
        evapotranspiration: Process<SoilWater + PlantWater -> Atmosphere.water_vapor>,
        infiltration: Process<Precipitation -> SoilWater>,
        nutrient_leaching: Process<SoilNutrients -> GroundWater.nutrients>,
        sediment_transport: Process<SurfaceErosion -> River.suspended_sediment>
    }
}

// ==================== 生物圈 Biosphere Sphere ====================
sphere 生物圈 extends BiosphereBase {
    // 生物生态要素 - Biological Ecological Variables (20 variables)
    properties: {
        // 浮游植物
        phytoplankton_biomass: Field3D<Real> with unit mg_C/m^3,
        primary_productivity: Field3D<Real> with unit mg_C/(m^3*day),
        phytoplankton_growth_rate: Field3D<Real> with unit day^-1,
        
        // 浮游动物
        zooplankton_biomass: Field3D<Real> with unit mg_C/m^3,
        zooplankton_grazing_rate: Field3D<Real> with unit day^-1,
        
        // 鱼类
        fish_abundance: Field3D<Real> with unit individuals/m^3,
        fish_biomass: Field3D<Real> with unit g/m^3,
        
        // 底栖生物
        benthic_biomass: Field2D<Real> with unit g/m^2,
        benthic_diversity: Field2D<Real> with unit shannon_index,
        
        // 微生物
        bacterial_abundance: Field3D<Real> with unit cells/mL,
        bacterial_production: Field3D<Real> with unit mg_C/(m^3*day),
        
        // 生态系统功能
        ecosystem_respiration: Field3D<Real> with unit mg_C/(m^3*day),
        carbon_export: Real with unit mg_C/(m^2*day),
        nitrogen_fixation: Field3D<Real> with unit μmol_N/(L*day),
        denitrification: Field3D<Real> with unit μmol_N/(L*day),
        
        // 生物多样性
        species_richness: Field3D<Real> with unit count,
        shannon_diversity: Field3D<Real> with unit shannon_index,
        evenness_index: Field3D<Real> with unit dimensionless,
        
        // 生物地球化学循环
        carbon_fixation_rate: Field3D<Real> with unit mg_C/(m^3*day),
        nutrient_uptake_rate: Field3D<Real> with unit μmol/(L*day)
    },
    
    processes: {
        photosynthesis: Process<CO2 + Nutrients + Light -> OrganicMatter + O2>,
        respiration: Process<OrganicMatter + O2 -> CO2 + Energy>,
        grazing: Process<Phytoplankton -> Zooplankton>,
        predation: Process<Zooplankton -> Fish>,
        decomposition: Process<OrganicMatter -> Nutrients + CO2>,
        nitrogen_cycling: Process<OrganicNitrogen <-> InorganicNitrogen>
    }
}`,

    // 珠江口-南海综合模拟系统
    pearl_river: `// 珠江口-南海多尺度耦合模拟系统
// Pearl River Estuary - South China Sea Multi-scale Coupled System

// 多尺度网格定义
scale SpatialResolution {
    watershed: 1000..50000 * meter,      // 流域尺度: 1-50km
    estuary: 100..5000 * meter,          // 河口尺度: 100m-5km  
    nearshore: 50..1000 * meter,         // 近岸尺度: 50m-1km
    offshore: 1000..10000 * meter        // 离岸尺度: 1-10km
}

scale TemporalResolution {
    high_frequency: 1..600 * second,     // 高频: 1s-10min
    synoptic: 1..24 * hour,             // 天气尺度: 1-24h
    seasonal: 1..365 * day,             // 季节尺度: 1-365天
    interannual: 1..10 * year           // 年际尺度: 1-10年
}

// 综合数据模型
data IntegratedObservationData {
    // 时空坐标
    location: GeoPoint with coordinate_system WGS84,
    timestamp: DateTime with timezone UTC+8,
    depth: Real with unit meter,
    
    // 多源观测数据
    satellite_data: {
        sea_surface_temperature: Uncertain<Real> with unit celsius,
        chlorophyll_concentration: Uncertain<Real> with unit mg/m^3,
        suspended_sediment: Uncertain<Real> with unit mg/L,
        sea_surface_height: Uncertain<Real> with unit meter,
        wind_speed: Uncertain<Real> with unit m/s,
        precipitation: Uncertain<Real> with unit mm/h
    },
    
    buoy_data: {
        water_temperature: TimeSeries<Real> with unit celsius,
        salinity: TimeSeries<Real> with unit psu,
        current_velocity: TimeSeries<Vector3D> with unit m/s,
        wave_height: TimeSeries<Real> with unit meter,
        wind_velocity: TimeSeries<Vector3D> with unit m/s,
        air_pressure: TimeSeries<Real> with unit hectopascal
    },
    
    ship_survey_data: {
        ctd_profiles: Profile<Temperature, Salinity, Density>,
        adcp_velocity: Profile<Vector3D> with unit m/s,
        water_samples: {
            nutrients: ChemicalAnalysis<Nitrate, Phosphate, Silicate>,
            dissolved_gases: ChemicalAnalysis<Oxygen, CO2>,
            biological_parameters: BiologicalAnalysis<Chlorophyll, Phytoplankton>
        }
    },
    
    coastal_station_data: {
        tide_gauge: TimeSeries<Real> with unit meter,
        weather_station: WeatherObservation,
        water_quality: WaterQualityMeasurement,
        biological_monitoring: BiologicalSurvey
    },
    
    // 数据质量控制
    quality_flags: QualityControlFlags,
    uncertainty_estimates: UncertaintyQuantification,
    calibration_info: CalibrationMetadata
}

// 多物理过程耦合模型
model PearlRiverEstuarySystem {
    // 模型组件
    components: {
        // 大气模式 (WRF-based)
        atmospheric_model: AtmosphericModel {
            resolution: 3000 * meter,
            time_step: 30 * second,
            physics_schemes: {
                microphysics: WSM6,
                cumulus: KainFritsch,
                planetary_boundary_layer: YSU,
                surface_layer: MM5,
                land_surface: Noah
            }
        },
        
        // 海洋模式 (ROMS-based)  
        ocean_model: OceanModel {
            horizontal_resolution: 500 * meter,
            vertical_levels: 40,
            time_step: 10 * second,
            physics_schemes: {
                turbulence_closure: GLS,
                advection_scheme: MPDATA,
                pressure_gradient: Spline_density_Jacobian
            }
        },
        
        // 波浪模式 (SWAN-based)
        wave_model: WaveModel {
            resolution: 200 * meter,
            time_step: 60 * second,
            physics: {
                wind_input: Komen,
                whitecapping: Komen,
                bottom_friction: JONSWAP,
                depth_breaking: Battjes_Janssen
            }
        },
        
        // 水文模式 (VIC-based)
        hydrological_model: HydrologicalModel {
            resolution: 1000 * meter,
            time_step: 3600 * second,
            processes: {
                evapotranspiration: Penman_Monteith,
                runoff_generation: Variable_Infiltration_Capacity,
                routing: Unit_Hydrograph
            }
        },
        
        // 生态模式 (NPZD-based)
        ecosystem_model: EcosystemModel {
            resolution: 500 * meter,
            time_step: 300 * second,
            biological_components: {
                phytoplankton: [Diatoms, Flagellates, Cyanobacteria],
                zooplankton: [Microzooplankton, Mesozooplankton],
                nutrients: [Nitrate, Ammonium, Phosphate, Silicate],
                detritus: [PON, POC, DON, DOC]
            }
        },
        
        // 沉积物输运模式
        sediment_model: SedimentTransportModel {
            resolution: 100 * meter,
            time_step: 60 * second,
            sediment_classes: [Clay, Silt, Sand],
            processes: [Erosion, Deposition, Resuspension, Flocculation]
        }
    },
    
    // 模型耦合
    coupling: {
        // 大气-海洋耦合
        atmosphere_ocean: BidirectionalCoupling {
            exchange_frequency: 300 * second,
            variables: [
                heat_flux, momentum_flux, freshwater_flux,
                sea_surface_temperature, surface_roughness
            ]
        },
        
        // 海洋-波浪耦合  
        ocean_wave: BidirectionalCoupling {
            exchange_frequency: 60 * second,
            variables: [
                surface_stress, wave_induced_mixing,
                current_velocity, water_depth
            ]
        },
        
        // 陆地-海洋耦合
        land_ocean: UnidirectionalCoupling {
            exchange_frequency: 3600 * second,
            variables: [
                river_discharge, nutrient_loading,
                sediment_input, freshwater_temperature
            ]
        },
        
        // 物理-生态耦合
        physics_biology: BidirectionalCoupling {
            exchange_frequency: 300 * second,
            variables: [
                temperature, salinity, light_attenuation,
                nutrient_concentrations, primary_production
            ]
        }
    },
    
    // 数据同化
    data_assimilation: {
        method: 4DVar,
        assimilation_window: 24 * hour,
        observations: [
            satellite_sst, satellite_chlorophyll,
            buoy_temperature, buoy_salinity,
            tide_gauge_data, current_measurements
        ],
        background_error_covariance: ModelClimatology,
        observation_error_covariance: InstrumentSpecifications
    }
}`,

    // 渤海湾综合环境监测系统  
    bohai_bay: `// 渤海湾综合环境监测与预警系统
// Bohai Bay Integrated Environmental Monitoring and Warning System

// 环境监测网络
data EnvironmentalMonitoringNetwork {
    // 监测站点网络
    monitoring_stations: {
        // 海洋监测浮标
        ocean_buoys: Array<OceanBuoy> {
            location: GeoPoint,
            deployment_depth: Real with unit meter,
            sensors: {
                // 物理传感器
                ctd_sensor: CTDSensor {
                    temperature_range: -2..40 * celsius,
                    salinity_range: 0..50 * psu,
                    pressure_range: 0..1000 * decibar,
                    accuracy: {temperature: 0.002 * celsius, salinity: 0.003 * psu}
                },
                
                adcp_sensor: ADCPSensor {
                    velocity_range: -5..5 * m/s,
                    depth_range: 2..300 * meter,
                    bin_size: 1 * meter,
                    accuracy: 0.01 * m/s
                },
                
                wave_sensor: WaveSensor {
                    height_range: 0..20 * meter,
                    period_range: 1..30 * second,
                    direction_range: 0..360 * degree,
                    accuracy: 0.1 * meter
                },
                
                // 化学传感器
                dissolved_oxygen_sensor: DOSensor {
                    range: 0..20 * mg/L,
                    accuracy: 0.1 * mg/L,
                    response_time: 30 * second
                },
                
                ph_sensor: PHSensor {
                    range: 6..9 * dimensionless,
                    accuracy: 0.01 * dimensionless,
                    temperature_compensation: true
                },
                
                turbidity_sensor: TurbiditySensor {
                    range: 0..4000 * NTU,
                    accuracy: 2 * percent,
                    anti_fouling: true
                },
                
                // 生物传感器
                chlorophyll_sensor: ChlorophyllSensor {
                    range: 0..500 * μg/L,
                    accuracy: 0.1 * μg/L,
                    excitation_wavelength: 470 * nanometer,
                    emission_wavelength: 685 * nanometer
                },
                
                // 气象传感器
                weather_station: WeatherStation {
                    wind_sensor: WindSensor {
                        speed_range: 0..60 * m/s,
                        direction_range: 0..360 * degree,
                        accuracy: {speed: 0.1 * m/s, direction: 3 * degree}
                    },
                    
                    barometric_sensor: BarometricSensor {
                        range: 800..1100 * hectopascal,
                        accuracy: 0.1 * hectopascal,
                        temperature_compensation: true
                    },
                    
                    humidity_sensor: HumiditySensor {
                        range: 0..100 * percent,
                        accuracy: 2 * percent,
                        temperature_range: -40..80 * celsius
                    }
                }
            },
            
            data_transmission: {
                communication_method: [Satellite, Cellular, Radio],
                transmission_frequency: 15 * minute,
                data_format: NetCDF,
                quality_control: RealTimeQC
            }
        },
        
        // 岸基监测站
        coastal_stations: Array<CoastalStation> {
            location: GeoPoint,
            station_type: [TideGauge, WeatherStation, WaterQuality],
            
            instrumentation: {
                tide_gauge: TideGauge {
                    measurement_range: -5..5 * meter,
                    accuracy: 0.001 * meter,
                    sampling_frequency: 1 * minute,
                    datum: MeanSeaLevel
                },
                
                water_quality_analyzer: WaterQualityAnalyzer {
                    parameters: [
                        Temperature, Salinity, DissolvedOxygen, pH,
                        Turbidity, Chlorophyll, Nitrate, Phosphate,
                        Ammonia, TotalSuspendedSolids, TotalNitrogen,
                        TotalPhosphorus, ChemicalOxygenDemand, BiochemicalOxygenDemand
                    ],
                    sampling_frequency: 1 * hour,
                    calibration_frequency: 1 * week
                },
                
                meteorological_station: MeteorologicalStation {
                    parameters: [
                        AirTemperature, AirPressure, RelativeHumidity,
                        WindSpeed, WindDirection, Precipitation,
                        SolarRadiation, NetRadiation, Visibility
                    ],
                    sampling_frequency: 1 * minute,
                    data_logger: HighResolutionLogger
                }
            }
        },
        
        // 移动观测平台
        mobile_platforms: {
            research_vessels: Array<ResearchVessel> {
                vessel_specifications: {
                    length: Real with unit meter,
                    draft: Real with unit meter,
                    endurance: Real with unit day,
                    crew_capacity: Integer
                },
                
                scientific_equipment: {
                    ctd_rosette: CTDRosette {
                        bottles: 24,
                        sensors: [Temperature, Salinity, Pressure, Oxygen, Fluorescence],
                        max_depth: 6000 * meter
                    },
                    
                    adcp_system: ShipboardADCP {
                        frequency: [75, 150, 300] * kilohertz,
                        range: [300, 150, 80] * meter,
                        velocity_accuracy: 0.005 * m/s
                    },
                    
                    multibeam_sonar: MultibeamSonar {
                        frequency: 200 * kilohertz,
                        swath_width: 120 * degree,
                        depth_accuracy: 0.1 * meter
                    },
                    
                    water_sampling_system: WaterSampler {
                        sample_volume: 10 * liter,
                        preservation_methods: [Refrigeration, Chemical, Filtration],
                        analysis_parameters: [
                            Nutrients, Phytoplankton, Zooplankton,
                            Bacteria, DissolvedOrganicMatter, Trace_metals
                        ]
                    }
                }
            },
            
            autonomous_vehicles: {
                underwater_gliders: Array<UnderwaterGlider> {
                    endurance: 30 * day,
                    max_depth: 1000 * meter,
                    horizontal_speed: 0.25 * m/s,
                    
                    sensor_payload: {
                        ctd: MiniatureCTD,
                        oxygen_sensor: OpticalOxygenSensor,
                        fluorometer: ChlorophyllFluorometer,
                        turbidity_sensor: OpticalTurbiditySensor,
                        acoustic_current_meter: AcousticCurrentMeter
                    }
                },
                
                surface_drifters: Array<SurfaceDrifter> {
                    lifetime: 180 * day,
                    tracking_accuracy: 10 * meter,
                    
                    sensors: {
                        gps: GPSReceiver,
                        temperature_sensor: SurfaceTemperatureSensor,
                        barometric_sensor: BarometricPressureSensor,
                        wave_sensor: AccelerometerWaveSensor
                    }
                }
            }
        }
    },
    
    // 数据管理系统
    data_management: {
        real_time_processing: {
            quality_control: AutomaticQualityControl {
                range_checks: ParameterRangeValidation,
                spike_detection: StatisticalOutlierDetection,
                gradient_checks: SpatialTemporalConsistency,
                climatology_checks: HistoricalDataComparison
            },
            
            data_fusion: MultiSourceDataFusion {
                interpolation_method: OptimalInterpolation,
                error_estimation: KalmanFiltering,
                bias_correction: SystematicErrorRemoval
            }
        },
        
        data_storage: {
            database_system: TimeSeriesDatabase {
                storage_format: NetCDF_CF,
                compression: GZIP,
                indexing: SpatioTemporalIndex,
                backup_strategy: RedundantStorage
            },
            
            metadata_management: MetadataStandards {
                format: ISO19115,
                vocabulary: CF_StandardNames,
                provenance_tracking: DataLineageRecords
            }
        }
    }
}`
};


// 添加COAWST和ISO元数据案例
COMPLEX_EXAMPLES.coawst_workflow = `// COAWST耦合建模系统工作流
// COAWST Coupled Modeling System Workflow

// 网格嵌套系统
model GridNestingSystem {
    coarse_grid: NetCDFGrid with {
        resolution: 100 * meter,
        domain: "Barnegat Bay"
    },
    
    refined_grid: NetCDFGrid with {
        resolution: 20 * meter,
        domain: "Reedy Creek",
        nested_within: coarse_grid
    }
}

// 边界条件生成
workflow BoundaryConditionGeneration {
    pipeline: {
        stage1_prepare: {
            F_3d = create_interpolant_3d(),
            F_4d = create_interpolant_4d()
        },
        stage2_north_boundary: {
            boundary_north = interpolate_and_extract()
        }
    }
}`;

COMPLEX_EXAMPLES.ocean_iso_metadata = `// 海洋ISO元数据标准
// Ocean ISO Metadata Standards

data ISO19115_Metadata {
    fileIdentifier: String,
    contact: Array<ResponsibleParty>,
    identificationInfo: DataIdentification
}

workflow MetadataPublishing {
    pipeline: {
        stage1_aggregate: { ncml_aggregation },
        stage2_enhance_acdd: { enhanced_attrs },
        stage3_generate_iso: { iso_xml },
        stage4_assess_quality: { quality_score }
    }
}`;
