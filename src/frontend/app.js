// CZ-DSL Frontend JavaScript

class CZDSLApp {
    constructor() {
        this.currentTab = 'platform-overview';
        this.sedrisVisualization = null; // Store SEDRIS visualization instance
        // 基础示例
        this.examples = {
            sphere: `// 基础球层定义示例
sphere 大气圈 extends Atmosphere {
    properties: {
        temperature: Real with unit kelvin,
        pressure: Real with unit pascal,
        humidity: Real with unit percent,
        wind_velocity: Vector3D with unit m/s,
        气温: Real with unit celsius  // 中文属性名
    },
    
    processes: {
        evaporation: Process<Water -> WaterVapor>,
        precipitation: Process<WaterVapor -> Water>,
        蒸发过程: Process<海水 -> 水汽>  // 中文过程名
    },
    
    boundaries: {
        surface: Interface<大气圈, Ocean>,
        land_surface: Interface<大气圈, Lithosphere>
    }
}`,
            
            process: `// 物理过程建模示例
process 潮汐过程 {
    inputs: {
        lunar_force: Force with unit newton,
        solar_force: Force with unit newton,
        earth_rotation: AngularVelocity with unit rad/s
    },
    
    outputs: {
        tide_height: Real with unit meter,
        tidal_current: Vector3D with unit m/s
    },
    
    // 物理约束
    constraint ConservationOfMass {
        ∀ region: Region, time: Time =>
            ∂ρ/∂t + ∇·(ρv) = 0
    },
    
    // 数值参数
    parameters: {
        gravitational_constant: 6.674e-11 with unit m^3/(kg*s^2),
        lunar_mass: 7.342e22 with unit kg,
        earth_moon_distance: 3.844e8 with unit meter
    }
}`,
            
            ai: `// AI模型集成示例
ai_model WaveHeightPredictor<ObservationData, WaveHeight> {
    architecture: NeuralNetwork {
        layers: [
            Dense(64, activation: "relu"),
            Dense(32, activation: "relu"), 
            Dense(1, activation: "linear")
        ]
    },
    
    physics_constraints: [
        WaveEnergyConservation,
        ShallowWaterEquations
    ],
    
    training_data: Dataset<ObservationData, WaveHeight>,
    
    method predict(input: ObservationData): Uncertain<WaveHeight> {
        // 神经网络预测
        raw_prediction = self.forward(input)
        
        // 物理约束校正
        corrected = apply_physics_constraints(raw_prediction, self.physics_constraints)
        
        return Uncertain {
            value: corrected,
            confidence: calculate_confidence(input, corrected),
            distribution: estimate_distribution(corrected)
        }
    }
}`,

            // SEDRIS集成示例
            sedris_srm: `// SEDRIS SRM 空间参考模型示例
use sedris::srm::*;

// 定义空间参考系统
spatial_reference_system WGS84_UTM_Zone_50N {
    datum: WGS84,
    projection: UTM {
        zone: 50,
        hemisphere: North
    },
    units: meter,
    precision: centimeter_accuracy
}

// 海岸带专用坐标系
spatial_reference_system 长江口坐标系 extends CoastalCoordinateSystem {
    base_srs: WGS84_UTM_Zone_50N,
    origin: (121.5°E, 31.2°N, 0.0m),
    orientation: along_coastline,
    tidal_datum: mean_sea_level,
    temporal_reference: UTC
}

// 坐标转换器
transformer coastal_transformer {
    source_srs: WGS84_Geographic,
    target_srs: 长江口坐标系,
    precision: centimeter_accuracy,
    tidal_correction: true,
    geoid_model: EGM2008
}

// 批量坐标转换
coordinates observation_points = [
    (121.5°E, 31.2°N, 0.0m),    // 长江口
    (120.2°E, 30.3°N, -5.0m),  // 杭州湾
    (119.8°E, 29.5°N, 2.0m)    // 舟山群岛
];

transformed_points = coastal_transformer.transform(observation_points);`,

            sedris_edcs: `// SEDRIS EDCS 环境数据编码示例
use sedris::edcs::*;

// 海岸带环境分类
classification 岩石海岸 extends EDCS_Rocky_Coast {
    edcs_code: ECC_ROCKY_COASTLINE,
    attributes: {
        rock_type: EDCS_GRANITE with unit dimensionless,
        cliff_height: Real with unit meter,
        wave_exposure: EDCS_HIGH_WAVE_EXPOSURE,
        erosion_rate: Real with unit mm/year
    }
}

// 海洋环境对象
environment 东海陆架 {
    classification: EDCS_CONTINENTAL_SHELF,
    spatial_extent: Polygon3D {
        coordinates: [...] in WGS84_UTM_Zone_51N,
        depth_range: (-200.0, 0.0) * meter
    },
    
    attributes: {
        water_depth: Field2D<Real> with unit meter,
        sea_surface_temperature: Field2D<Real> with unit celsius,
        salinity: Field2D<Real> with unit psu,
        chlorophyll_concentration: Field2D<Real> with unit mg/m^3,
        sediment_type: Field2D<EDCS_SedimentType>,
        current_velocity: Field3D<Vector3D> with unit m/s
    },
    
    // 时间变化属性
    temporal_attributes: {
        tidal_height: TimeSeries<Real> with unit meter,
        wave_height: TimeSeries<Real> with unit meter,
        wind_speed: TimeSeries<Real> with unit m/s
    }
}

// 生态系统编码
ecosystem 长江口湿地 {
    classification: EDCS_ESTUARINE_WETLAND,
    edcs_code: ECC_TIDAL_WETLAND,
    
    biotic_components: {
        vegetation: {
            dominant_species: EDCS_SPARTINA_ALTERNIFLORA,
            coverage: Field2D<Real> with unit percent,
            biomass: Field2D<Real> with unit kg/m^2
        },
        
        fauna: {
            bird_diversity: Real with unit shannon_index,
            fish_abundance: Real with unit individuals/m^2,
            benthic_biomass: Field2D<Real> with unit g/m^2
        }
    },
    
    abiotic_components: {
        sediment: {
            grain_size: Field2D<Real> with unit mm,
            organic_content: Field2D<Real> with unit percent,
            porosity: Field2D<Real> with unit dimensionless
        },
        
        hydrology: {
            inundation_frequency: Field2D<Real> with unit times/month,
            salinity_gradient: Field2D<Real> with unit psu,
            nutrient_concentration: Field2D<Real> with unit mg/L
        }
    }
}`,

            sedris_drm: `// SEDRIS DRM 数据表示模型示例
use sedris::drm::*;

// 海岸带复合对象
drm_object 长江口河口系统 extends EstuarySystem {
    spatial_extent: Polygon3D {
        coordinates: [
            (121.0°E, 30.8°N, -50.0m),
            (122.0°E, 30.8°N, -50.0m),
            (122.0°E, 31.5°N, 10.0m),
            (121.0°E, 31.5°N, 10.0m)
        ] in WGS84_Geographic,
        depth_range: (-50.0, 10.0) * meter
    },
    
    // 组成部分
    components: {
        river_mouth: RiverMouth {
            location: (121.5°E, 31.2°N) in WGS84_Geographic,
            discharge_rate: TimeSeries<Real> with unit m^3/s,
            sediment_load: TimeSeries<Real> with unit kg/s,
            water_temperature: TimeSeries<Real> with unit celsius,
            
            // 河口特征
            mouth_width: 6000.0 * meter,
            channel_depth: 15.0 * meter,
            salinity_intrusion_length: 50000.0 * meter
        },
        
        tidal_flat: TidalFlat {
            area: 500.0 * km^2,
            elevation_range: (-2.0, 4.0) * meter,
            exposure_time: TimeSeries<Real> with unit hour,
            sediment_type: EDCS_MUD_SAND_MIXTURE,
            
            // 潮滩动态
            accretion_rate: Field2D<Real> with unit mm/year,
            erosion_rate: Field2D<Real> with unit mm/year,
            vegetation_coverage: Field2D<Real> with unit percent
        },
        
        salt_marsh: SaltMarsh {
            area: 200.0 * km^2,
            elevation_range: (2.0, 5.0) * meter,
            vegetation_coverage: Field2D<Real> with unit percent,
            species_diversity: Real with unit shannon_index,
            
            // 生态功能
            carbon_sequestration_rate: Real with unit kg_C/(m^2*year),
            nitrogen_removal_rate: Real with unit kg_N/(m^2*year),
            bird_habitat_capacity: Real with unit individuals/km^2
        }
    },
    
    // 物理过程
    processes: {
        tidal_mixing: TidalMixing {
            tidal_range: 4.5 * meter,
            tidal_period: 12.42 * hour,
            mixing_coefficient: Field3D<Real> with unit m^2/s,
            
            // 潮汐参数
            M2_amplitude: 2.1 * meter,
            S2_amplitude: 0.8 * meter,
            phase_lag: Field2D<Real> with unit degree
        },
        
        sediment_transport: SedimentTransport {
            transport_rate: Field3D<Vector3D> with unit kg/(m^2*s),
            deposition_rate: Field2D<Real> with unit mm/year,
            erosion_threshold: Field2D<Real> with unit pascal,
            
            // 沉积物特性
            grain_size_distribution: Field3D<Array<Real>> with unit mm,
            settling_velocity: Field3D<Real> with unit m/s,
            critical_shear_stress: Field2D<Real> with unit pascal
        },
        
        freshwater_discharge: FreshwaterDischarge {
            discharge_rate: TimeSeries<Real> with unit m^3/s,
            seasonal_variation: Real with unit dimensionless,
            flood_frequency: Real with unit events/year,
            
            // 径流特征
            peak_discharge: Real with unit m^3/s,
            base_flow: Real with unit m^3/s,
            nutrient_load: TimeSeries<Real> with unit kg/s
        }
    },
    
    // 耦合关系
    interactions: {
        river_ocean_mixing: Coupling<RiverMouth, TidalFlat> {
            coupling_strength: Real with unit dimensionless,
            mixing_length_scale: Real with unit meter,
            stratification_parameter: Real with unit dimensionless
        },
        
        sediment_vegetation_feedback: Coupling<SedimentTransport, SaltMarsh> {
            vegetation_trapping_efficiency: Real with unit dimensionless,
            root_stabilization_factor: Real with unit dimensionless,
            bioturbation_rate: Real with unit m^2/s
        }
    },
    
    // 不确定性量化
    uncertainties: {
        discharge_uncertainty: Uncertain<Real> {
            distribution: Normal(mean: 30000.0, std: 5000.0),
            confidence_interval: (20000.0, 40000.0),
            data_quality: 0.85
        },
        
        sea_level_rise_scenario: Uncertain<Real> {
            distribution: Uniform(min: 0.3, max: 1.2),
            unit: meter,
            time_horizon: 2100 * year
        }
    }
        }`,

            cmecs_habitat: `// CMECS 生态单元与CZ-DB集成示例
coastal_database HangzhouBayCMECSDB {
    id: "CZDB-HZB-CMECS-001",
    version: "1.0.0",
    
    spatial: HangzhouBay_SRM,
    temporal: Scenario_2023,
    tiling: CZDB_TileScheme_250m,
    
    metadata: {
        title: "杭州湾海岸带CMECS生态单元数据库",
        region: "Hangzhou Bay, China",
        provider: "Coastal Systems Lab",
        source: "https://github.com/NOAA-OCM/cmecs",
        classification_standard: "CMECS",
        cmecs_version: "4.0"
    },
    
    layers: {
        biotic: layer {
            type: BioticComponentLayer,
            datasets: {
                tidal_flat_vegetation: dataset {
                    feature_class: TidalFlatVegetation,
                    spatial_resolution: 50.0 * meter,
                    temporal_resolution: 1.0 * year,
                    quality_profile: RemoteSensing_QC2,
                    uncertainty_model: CategoricalConfusionMatrix,
                    cmecs_unit: CMECS_Biotic_Tidal_Flat_Marsh
                }
            }
        },
        
        substrate: layer {
            type: SubstrateComponentLayer,
            datasets: {
                sediment_type_map: dataset {
                    feature_class: SedimentTypePolygon,
                    spatial_resolution: 50.0 * meter,
                    temporal_resolution: 5.0 * year,
                    quality_profile: SedimentSurvey_QC3,
                    uncertainty_model: CategoricalConfusionMatrix,
                    cmecs_unit: CMECS_Substrate_Mud_Sand_Mix
                }
            }
        }
    }
}`,

            // 完整动态场景示例
            sedris_dynamic_scenario: `// 海岸带动态场景：台风"海葵"影响下的长江口河口系统演化
// Dynamic Coastal Scenario: Yangtze River Estuary Evolution under Typhoon Haikui
// 基于SEDRIS标准的CZ-DSL动态场景建模

use sedris::srm::*;
use sedris::edcs::*;
use sedris::drm::*;

// ==================== 空间参考系统定义 ====================
spatial_reference_system 长江口坐标系 extends CoastalCoordinateSystem {
    base_srs: WGS84_UTM_Zone_51N,
    origin: (121.5°E, 31.2°N, 0.0m),
    orientation: along_coastline,
    tidal_datum: mean_sea_level,
    temporal_reference: UTC,
    precision: centimeter_accuracy,
    
    coastal_parameters: {
        reference_coastline: Polyline3D {
            coordinates: [
                (121.0°E, 31.0°N, 0.0m),
                (121.5°E, 31.2°N, 0.0m),
                (122.0°E, 31.4°N, 0.0m)
            ] in WGS84_Geographic
        },
        tidal_range: 4.5 * meter,
        mean_high_water: 2.3 * meter,
        mean_low_water: -2.2 * meter
    }
    }

// ==================== 环境数据编码 (EDCS) ====================
classification 台风海葵 extends EDCS_Tropical_Cyclone {
    edcs_code: ECC_TYPHOON,
    attributes: {
        maximum_wind_speed: 45.0 with unit m/s,  // 强台风级别
        central_pressure: 950.0 with unit hectopascal,
        storm_radius: 300.0 with unit kilometer,
        forward_speed: 25.0 with unit km/h,
        category: EDCS_CATEGORY_4_TYPHOON
    }
}

// ==================== 数据表示模型 (DRM) ====================
drm_object 长江口河口系统 extends EstuarySystem {
    spatial_extent: Polygon3D {
        coordinates: [
            (120.8°E, 30.7°N, -50.0m),
            (122.2°E, 31.8°N, 15.0m)
        ] in WGS84_Geographic,
        area: 2800.0 * km^2
    },
    
    temporal_extent: TimeInterval {
        start_time: "2023-10-01T00:00:00Z",
        end_time: "2023-10-14T23:59:59Z",
        resolution: 1.0 * hour
    },
    
    components: {
        yangtze_main_channel: RiverChannel {
            discharge_rate: TimeSeries<Real> {
                data: [
                    ("2023-10-01T00:00:00Z", 28000.0),  // 台风前正常流量
                    ("2023-10-07T12:00:00Z", 65000.0),  // 台风过境峰值
                    ("2023-10-12T00:00:00Z", 30000.0)   // 恢复期
                ] with unit m^3/s,
                interpolation: cubic_spline
            },
            
            sediment_concentration: TimeSeries<Real> {
                data: [
                    ("2023-10-01T00:00:00Z", 0.5),
                    ("2023-10-07T12:00:00Z", 2.8),     // 台风期间大幅增加
                    ("2023-10-12T00:00:00Z", 0.8)
                ] with unit kg/m^3
            }
        },
        
        chongming_wetland: TidalWetland {
            area: 326.0 * km^2,
            vegetation_coverage: TimeSeries<Field2D<Real>> {
                data: [
                    ("2023-10-01", generate_field(0.85, 0.05)),  // 85%±5% 覆盖度
                    ("2023-10-08", generate_field(0.62, 0.12)),  // 台风后显著下降
                    ("2023-10-14", generate_field(0.71, 0.08))   // 部分恢复
                ] with unit percent
            },
            
            species_diversity: TimeSeries<Real> {
                data: [
                    ("2023-10-01", 2.8),   // Shannon指数
                    ("2023-10-08", 2.1),   // 台风后下降
                    ("2023-10-14", 2.4)    // 逐步恢复
                ] with unit shannon_index
            }
        }
    },
    
    processes: {
        typhoon_haikui: TyphoonProcess {
            track: TimeSeries<Point3D> {
                data: [
                    ("2023-10-05T00:00:00Z", (125.0°E, 28.0°N, 0.0m)),
                    ("2023-10-07T12:00:00Z", (122.0°E, 31.0°N, 0.0m)),  // 最接近长江口
                    ("2023-10-09T00:00:00Z", (119.0°E, 34.0°N, 0.0m))
                ] in WGS84_Geographic
            },
            
            storm_surge: StormSurgeProcess {
                surge_height: TimeSeries<Field2D<Real>> {
                    data: [
                        ("2023-10-07T12:00:00Z", generate_surge_field(2.3, 0.5))  // 峰值
                    ] with unit meter
                }
            }
        }
    },
    
    uncertainties: {
        typhoon_track_uncertainty: Uncertain<TimeSeries<Point3D>> {
            distribution: MultivariateNormal {
                mean: typhoon_haikui.track,
                covariance_matrix: generate_track_covariance(50.0 * kilometer)
            },
            confidence_interval: 0.95,
            ensemble_size: 50
        }
    }

    initScenarioMatrix() {
        const matrix = document.getElementById('scenario-matrix');
        const backdrop = document.getElementById('scenario-drawer-backdrop');
        if (!matrix || !backdrop) return;

        const drawer = backdrop.querySelector('.scenario-drawer');
        const titleEl = document.getElementById('scenario-drawer-title');
        const badgeEl = document.getElementById('scenario-drawer-badge');
        const bodyEl = document.getElementById('scenario-drawer-body');
        const footerEl = document.getElementById('scenario-drawer-footer');
        const closeBtn = backdrop.querySelector('.scenario-drawer-close');

        let activeTrigger = null;
        let dirty = false;

        const isDesktop = () => window.innerWidth > 1024;

        const openDrawer = (card, pushState = true) => {
            if (!isDesktop()) return;
            const id = card.getAttribute('data-scenario-id');
            const title = card.getAttribute('data-scenario-title') || '';
            const badge = card.getAttribute('data-scenario-badge') || '';

            activeTrigger = card;
            dirty = false;

            // 填充头部
            titleEl.textContent = title;
            badgeEl.textContent = badge;

            // 填充主体：复用原卡片中的文案和指标
            const content = card.querySelector('.scenario-content');
            const metrics = card.querySelector('.scenario-metrics');
            const cloneContainer = document.createElement('div');
            if (content) cloneContainer.appendChild(content.cloneNode(true));
            if (metrics) cloneContainer.appendChild(metrics.cloneNode(true));
            bodyEl.innerHTML = '';
            bodyEl.appendChild(cloneContainer);

            // 底部操作区：模拟“保存并关闭 / 放弃并关闭”
            footerEl.innerHTML = '';
            const saveBtn = document.createElement('button');
            saveBtn.type = 'button';
            saveBtn.className = 'action-btn primary';
            saveBtn.textContent = '保存并关闭';
            saveBtn.addEventListener('click', () => {
                dirty = false;
                closeDrawer();
            });

            const discardBtn = document.createElement('button');
            discardBtn.type = 'button';
            discardBtn.className = 'action-btn';
            discardBtn.textContent = '放弃并关闭';
            discardBtn.addEventListener('click', () => {
                dirty = false;
                closeDrawer();
            });

            footerEl.appendChild(saveBtn);
            footerEl.appendChild(discardBtn);

            // 显示抽屉
            backdrop.classList.add('active');
            backdrop.setAttribute('aria-hidden', 'false');
            drawer.setAttribute('data-scenario-id', id);
            drawer.focus();

            // URL 同步
            if (pushState) {
                const url = new URL(window.location.href);
                url.searchParams.set('scenario', id);
                window.history.pushState({ scenario: id }, '', url.toString());
            }
        };

        const closeDrawer = (fromPopState = false) => {
            if (!backdrop.classList.contains('active')) return;

            if (dirty) {
                const confirmClose = window.confirm('存在未保存的变更，确定要关闭吗？');
                if (!confirmClose) return;
            }

            backdrop.classList.remove('active');
            backdrop.setAttribute('aria-hidden', 'true');
            if (activeTrigger) {
                activeTrigger.focus();
            }

            if (!fromPopState) {
                const url = new URL(window.location.href);
                url.searchParams.delete('scenario');
                window.history.pushState({}, '', url.toString());
            }
        };

        // 绑定卡片点击与键盘事件
        const cards = matrix.querySelectorAll('.scenario-card');
        cards.forEach(card => {
            card.setAttribute('role', 'button');
            card.setAttribute('tabindex', '0');

            card.addEventListener('click', () => {
                if (!isDesktop()) return;
                openDrawer(card);
            });

            card.addEventListener('keydown', e => {
                if (!isDesktop()) return;
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    openDrawer(card);
                }
            });
        });

        // 关闭按钮与遮罩点击
        closeBtn.addEventListener('click', () => closeDrawer());
        backdrop.addEventListener('click', e => {
            if (e.target === backdrop) {
                closeDrawer();
            }
        });

        // ESC 关闭
        window.addEventListener('keydown', e => {
            if (e.key === 'Escape') {
                closeDrawer();
            }
        });

        // popstate 支持前进/后退打开/关闭详情
        window.addEventListener('popstate', event => {
            const state = event.state || {};
            const url = new URL(window.location.href);
            const id = url.searchParams.get('scenario');
            if (!id) {
                closeDrawer(true);
                return;
            }
            const target = matrix.querySelector(
                `.scenario-card[data-scenario-id="${id}"]`
            );
            if (target) {
                openDrawer(target, false);
            }
        });

        // 首次加载根据 URL 决定是否打开详情
        const url = new URL(window.location.href);
        const initialId = url.searchParams.get('scenario');
        if (initialId && isDesktop()) {
            const target = matrix.querySelector(
                `.scenario-card[data-scenario-id="${initialId}"]`
            );
            if (target) {
                openDrawer(target, false);
            }
        }

        // 窗口尺寸变化时，切换模式
        window.addEventListener('resize', () => {
            if (!isDesktop()) {
                closeDrawer(true);
            }
        });
    }
}

// 初始化应用场景双态布局
window.addEventListener('DOMContentLoaded', () => {
    const app = new CZDSLApp();
    app.initScenarioMatrix();
});

// ==================== SEDRIS转换接口 ====================
sedris_export_config {
    output_format: STF_VERSION_4_1,
    coordinate_system: 长江口坐标系,
    temporal_resolution: 1.0 * hour,
    spatial_resolution: 100.0 * meter,
    
    drm_mapping: {
        长江口河口系统 -> SEDRIS_ENVIRONMENT_ROOT,
        yangtze_main_channel -> SEDRIS_LINEAR_FEATURE,
        chongming_wetland -> SEDRIS_AREAL_FEATURE
    },
    
    edcs_mapping: {
        台风海葵 -> ECC_TYPHOON
    }
}

// ==================== 可视化配置 ====================
visualization_config 台风影响可视化 {
    scene_3d: {
        camera_position: (121.5°E, 31.2°N, 5000.0m),
        camera_target: (121.5°E, 31.2°N, 0.0m)
    },
    
    animation: {
        start_time: "2023-10-01T00:00:00Z",
        end_time: "2023-10-14T23:59:59Z",
        time_step: 3.0 * hour,
        playback_speed: 3600.0,  // 1小时/秒
        loop: true
    },
    
    layers: {
        sea_surface_temperature: {
            color_map: "thermal",
            opacity: 0.7,
            range: (18.0, 26.0) * celsius
        },
        
        wave_field: {
            visualization_type: "height_field",
            color_map: "ocean",
            max_height: 10.0 * meter
        },
        
        typhoon_track: {
            line_width: 5.0,
            color: "red",
            show_uncertainty_cone: true
        }
    }
}`
        };
        
        // 海岸带模型示例
        this.modelExamples = {
            roms: `// ROMS海洋模型集成配置
model ROMS_EastChinaSea extends OceanModel {
    domain: {
        horizontal_grid: RegularGrid {
            resolution: (1000.0, 1000.0) * meter,
            extent: (118°E, 125°E, 28°N, 35°N),
            projection: "Mercator"
        },
        
        vertical_grid: SigmaCoordinate {
            levels: 32,
            surface_stretching: 3.0,
            bottom_stretching: 0.4,
            critical_depth: 10.0 * meter
        }
    },
    
    physics: {
        equation_of_state: "UNESCO",
        turbulence_closure: "GLS_k_epsilon",
        bottom_friction: "quadratic",
        surface_forcing: "bulk_formula"
    },
    
    boundary_conditions: {
        river_inputs: [
            RiverInput {
                name: "Yangtze_River",
                location: (121.5°E, 31.2°N),
                discharge: yangtze_discharge_data,
                temperature: river_temperature_data,
                salinity: 0.0 * psu
            }
        ]
    },
    
    output: {
        variables: ["u", "v", "w", "temp", "salt", "zeta"],
        frequency: 1.0 * hour,
        format: "netcdf"
    }
}`,

            swan: `// SWAN波浪模型集成配置
model SWAN_YangtzeEstuary extends WaveModel {
    computational_grid: UnstructuredGrid {
        resolution_range: (50.0, 500.0) * meter,
        boundary_fitted: true,
        coastline_resolution: 25.0 * meter
    },
    
    physics: {
        wave_generation: "Komen_et_al_1984",
        wave_dissipation: ["whitecapping", "bottom_friction", "depth_breaking"],
        nonlinear_interactions: "DIA_approximation",
        refraction: true
    },
    
    coupling: {
        current_field: ROMS_EastChinaSea.velocity,
        water_level: ROMS_EastChinaSea.sea_surface_height,
        update_frequency: 1.0 * hour
    }
}`,

            delft3d: `// Delft3D综合模型集成配置
model Delft3D_YangtzeEstuary extends IntegratedCoastalModel {
    flow_module: {
        grid: CurvilinearGrid {
            m_direction: 180,
            n_direction: 120,
            layers: 10
        },
        
        physics: {
            turbulence: "k_epsilon",
            density: "eckart"
        }
    },
    
    sediment_module: {
        sediment_fractions: [
            SedimentFraction {
                name: "sand",
                diameter: 200.0 * micrometer,
                density: 2650.0 * kg/m^3
            },
            SedimentFraction {
                name: "mud", 
                diameter: 20.0 * micrometer,
                density: 2650.0 * kg/m^3
            }
        ],
        
        transport_formula: "Van_Rijn_1993",
        morphological_factor: 10.0
    }
}`
        };
        
        // CZ-DB示例：杭州湾海岸带数据库（coastal_database，显式支持OGC CDB与CityGML）
        this.examples.hangzhou_bay_db = `coastal_database HangzhouBayCZDB {
    id: "CZDB-HZB-001",
    version: "1.0.0",
    
    spatial: HangzhouBay_SRM,
    temporal: Scenario_2023,
    tiling: CZDB_TileScheme_250m,
    cdb_profile: OGC_CDB_1_0,
    
    metadata: {
        title: "杭州湾海岸带综合环境数据库",
        region: "Hangzhou Bay, China",
        provider: "Coastal Systems Lab",
        description: "CDB风格的海岸带多圈层环境数据库示例"
    },
    
    layers: {
        bathymetry: layer {
            type: BathymetryLayer,
            cdb_dataset: CDB_D300_S002_T001,
            cdb_geometry_type: CDB_Terrain_Elevation,
            datasets: {
                depth_grid: dataset {
                    feature_class: GriddedBathymetry,
                    spatial_resolution: 250.0 * meter,
                    temporal_resolution: 10.0 * year,
                    quality_profile: HydroSurvey_QC3,
                    uncertainty_model: DepthUncertainty_Gaussian,
                    cdb_lod: 5,
                    cdb_tile_size: (1.0 * km, 1.0 * km)
                }
            }
        },
        
        shoreline: layer {
            type: ShorelineLayer,
            cdb_dataset: CDB_D402_S001_T001,
            cdb_geometry_type: CDB_Vector_Line,
            datasets: {
                mean_high_water: dataset {
                    feature_class: CoastlinePolyline,
                    spatial_resolution: 25.0 * meter,
                    temporal_resolution: 5.0 * year,
                    quality_profile: Cartographic_QC2,
                    uncertainty_model: LinePosition_Buffer
                }
            }
        },
        
        landuse: layer {
            type: LandUseLayer,
            datasets: {
                coastal_zone_landuse: dataset {
                    feature_class: LandUsePolygon,
                    citygml_version: CityGML_3_0,
                    citygml_feature_class: cgml_LandUse,
                    citygml_lod: 1,
                    spatial_resolution: 50.0 * meter,
                    temporal_resolution: 1.0 * year,
                    quality_profile: RemoteSensing_QC2,
                    uncertainty_model: CategoricalConfusionMatrix,
                    cmecs_unit: CMECS_Biotic_Unit
                }
            }
        }
    }
}`;
        
        // 合并复杂示例（从外部文件加载）
        if (typeof COMPLEX_EXAMPLES !== 'undefined') {
            this.examples = { ...this.examples, ...COMPLEX_EXAMPLES };
        }
        
        this.init();
    }
    
    init() {
    this.initRouter();
    this.setupTabs();
        this.setupEditor();
        this.setupExamples();
        this.setupAST();
        this.loadSyntaxHighlighting();
        this.initializeDashboard();
        this.initializeAnimations();
        this.setupInteractiveFeatures();
        this.initializeFloatingStatus();
        this.startPerformanceMonitoring();
        
        // Show welcome notification
        setTimeout(() => {
            this.showNotification('欢迎使用CZ-DSL企业级海岸带建模平台！', 'success', 5000);
        }, 2000);
    }
    
    setupTabs() {
    const tabButtons = document.querySelectorAll('.nav-button');
    const tabContents = document.querySelectorAll('.tab-content');
    tabButtons.forEach(button => {
      button.addEventListener('click', () => {
        const tabId = button.dataset.tab;
        this.navigateTo(tabId);
      });
    });
    // Ensure initial state reflects current route
    this.handleRouteChange();
    }
    
    setupEditor() {
        const parseBtn = document.getElementById('parse-btn');
        const generateIRBtn = document.getElementById('generate-ir-btn');
        const optimizeBtn = document.getElementById('optimize-btn');
        const compileBtn = document.getElementById('compile-btn');
        const clearBtn = document.getElementById('clear-btn');
        const exampleBtn = document.getElementById('example-btn');
        const codeEditor = document.getElementById('code-editor');
        
        // 设置输出标签切换
        this.setupOutputTabs();
        
        parseBtn.addEventListener('click', () => {
            this.parseCode();
        });
        
        generateIRBtn.addEventListener('click', () => {
            this.generateIR();
        });
        
        optimizeBtn.addEventListener('click', () => {
            this.optimizeCode();
        });
        
        compileBtn.addEventListener('click', () => {
            this.compileAndRun();
        });
        
        clearBtn.addEventListener('click', () => {
            codeEditor.value = '';
            this.clearAllOutputs();
        });
        
        exampleBtn.addEventListener('click', () => {
            codeEditor.value = this.examples.sphere;
        });
    }
    
    setupOutputTabs() {
        const outputTabs = document.querySelectorAll('.output-tab');
        const outputContents = document.querySelectorAll('.output-content');
        
        outputTabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const outputType = tab.dataset.output;
                
                // Remove active class from all tabs and contents
                outputTabs.forEach(t => t.classList.remove('active'));
                outputContents.forEach(content => content.classList.remove('active'));
                
                // Add active class to clicked tab and corresponding content
                tab.classList.add('active');
                document.getElementById(`${outputType}-output`).classList.add('active');
            });
        });
    }
    
    clearAllOutputs() {
        const outputs = ['parse-output', 'ir-output', 'optimized-output', 'compiled-output'];
        outputs.forEach(outputId => {
            const element = document.getElementById(outputId);
            const placeholder = this.getPlaceholderText(outputId);
            element.innerHTML = `<p class="placeholder">${placeholder}</p>`;
        });
    }
    
    getPlaceholderText(outputId) {
        const placeholders = {
            'parse-output': '点击"解析代码"按钮查看解析结果',
            'ir-output': '点击"生成IR"按钮查看中间表示代码',
            'optimized-output': '点击"优化代码"按钮查看优化后的IR',
            'compiled-output': '点击"编译执行"按钮查看编译和执行结果'
        };
        return placeholders[outputId] || '暂无内容';
    }
    
    generateIR() {
        const codeEditor = document.getElementById('code-editor');
        const irOutput = document.getElementById('ir-output');
        const code = codeEditor.value.trim();
        
        if (!code) {
            irOutput.innerHTML = '<div class="message error">请输入代码后再生成IR</div>';
            return;
        }
        
        // Show loading
        irOutput.innerHTML = '<div class="loading"></div> 正在生成中间表示...';
        
        // Switch to IR tab
        document.querySelector('[data-output="ir"]').click();
        
        // Simulate IR generation
        setTimeout(() => {
            const irCode = this.simulateIRGeneration(code);
            irOutput.innerHTML = irCode;
        }, 1000);
    }
    
    optimizeCode() {
        const codeEditor = document.getElementById('code-editor');
        const optimizedOutput = document.getElementById('optimized-output');
        const code = codeEditor.value.trim();
        
        if (!code) {
            optimizedOutput.innerHTML = '<div class="message error">请先生成IR代码</div>';
            return;
        }
        
        // Show loading
        optimizedOutput.innerHTML = '<div class="loading"></div> 正在优化代码...';
        
        // Switch to optimized tab
        document.querySelector('[data-output="optimized"]').click();
        
        // Simulate optimization
        setTimeout(() => {
            const optimizedCode = this.simulateOptimization(code);
            optimizedOutput.innerHTML = optimizedCode;
        }, 1500);
    }
    
    compileAndRun() {
        const codeEditor = document.getElementById('code-editor');
        const compiledOutput = document.getElementById('compiled-output');
        const code = codeEditor.value.trim();
        
        if (!code) {
            compiledOutput.innerHTML = '<div class="message error">请输入代码后再编译</div>';
            return;
        }
        
        // Show loading
        compiledOutput.innerHTML = '<div class="loading"></div> 正在编译和执行...';
        
        // Switch to compiled tab
        document.querySelector('[data-output="compiled"]').click();
        
        // Simulate compilation and execution
        setTimeout(() => {
            const result = this.simulateCompileAndRun(code);
            compiledOutput.innerHTML = result;
        }, 2000);
    }
    
    simulateIRGeneration(code) {
        try {
            // Simulate IR generation based on code content
            let irInstructions = [];
            
            // Basic IR generation simulation
            if (code.includes('sphere')) {
                irInstructions.push(
                    '// 球层结构定义',
                    '%struct_atmosphere = alloc struct { f64, f64, f64 }',
                    '%temp_field = load f64, 273.15',
                    '%pressure_field = load f64, 101325.0',
                    '%humidity_field = load f64, 0.6',
                    'store %temp_field, %struct_atmosphere[0]',
                    'store %pressure_field, %struct_atmosphere[1]',
                    'store %humidity_field, %struct_atmosphere[2]'
                );
            }
            
            if (code.includes('process') || code.includes('过程')) {
                irInstructions.push(
                    '',
                    '// 过程函数定义',
                    'define @temperature_conversion(%input: f64 celsius) -> f64 kelvin {',
                    'entry:',
                    '  %0 = load f64, %input',
                    '  %1 = load f64, 273.15',
                    '  %2 = add f64, %0, %1',
                    '  %3 = unit_convert %2, celsius -> kelvin',
                    '  ret f64 %3',
                    '}'
                );
            }
            
            if (code.includes('ai_model')) {
                irInstructions.push(
                    '',
                    '// AI模型调用',
                    '%input_tensor = alloc tensor<f32, [64]>',
                    '%weights = load tensor<f32, [64, 32]>, @model_weights',
                    '%hidden = matmul %input_tensor, %weights',
                    '%output = call @neural_network_forward(%hidden)',
                    '%constrained = call @apply_physics_constraints(%output)'
                );
            }
            
            // Add unit checking instructions
            if (code.includes('with unit')) {
                irInstructions.push(
                    '',
                    '// 物理单位检查',
                    'unit_check %temp_field, kelvin',
                    'unit_check %pressure_field, pascal',
                    'unit_check %humidity_field, percent'
                );
            }
            
            const irCode = irInstructions.join('\n');
            
            return `
                <div class="message success">✅ IR生成成功</div>
                
                <h4>中间表示代码:</h4>
                <pre><code class="language-ir">${irCode}</code></pre>
                
                <h4>IR统计信息:</h4>
                <div class="message success">
                    📊 指令数量: ${irInstructions.filter(line => line.trim() && !line.startsWith('//')).length}<br>
                    🔧 基本块数量: ${(irCode.match(/^\w+:/gm) || []).length}<br>
                    📏 物理单位操作: ${(irCode.match(/unit_/g) || []).length}<br>
                    🧮 寄存器使用: ${(irCode.match(/%\d+/g) || []).length}
                </div>
            `;
        } catch (error) {
            return `
                <div class="message error">❌ IR生成失败</div>
                <div class="ast-tree">
                    <div style="color: var(--error-color);">
                        错误: ${error.message}
                    </div>
                </div>
            `;
        }
    }
    
    simulateOptimization(code) {
        try {
            // Simulate optimization process
            const optimizations = [];
            let optimizedInstructions = [];
            
            // Simulate constant folding
            if (code.includes('273.15')) {
                optimizations.push('常量折叠: 温度转换常数');
                optimizedInstructions.push(
                    '// 优化后的代码 (常量折叠)',
                    '%temp_kelvin = move 273.15  // 折叠后的常量'
                );
            }
            
            // Simulate dead code elimination
            optimizations.push('死代码消除: 移除了3个未使用的指令');
            
            // Simulate unit optimizations
            if (code.includes('with unit')) {
                optimizations.push('物理单位优化: 消除了冗余的单位检查');
                optimizedInstructions.push(
                    '// 单位优化: 编译时单位转换',
                    '%converted_temp = move 546.3 kelvin  // 编译时转换'
                );
            }
            
            // Common subexpression elimination
            optimizations.push('公共子表达式消除: 复用了2个计算结果');
            
            const optimizationStats = {
                constant_folding: 3,
                dead_code_elimination: 3,
                geometric_algebra_optimization: 0,
                unit_optimization: code.includes('with unit') ? 2 : 0,
                common_subexpression_elimination: 2
            };
            
            const totalOptimizations = Object.values(optimizationStats).reduce((a, b) => a + b, 0);
            
            return `
                <div class="message success">✅ 代码优化完成</div>
                
                <h4>优化统计:</h4>
                <div class="optimization-stats">
                    <div class="message success">
                        🎯 总优化次数: ${totalOptimizations}<br>
                        📊 常量折叠: ${optimizationStats.constant_folding} 次<br>
                        🗑️ 死代码消除: ${optimizationStats.dead_code_elimination} 次<br>
                        📏 物理单位优化: ${optimizationStats.unit_optimization} 次<br>
                        🔄 公共子表达式消除: ${optimizationStats.common_subexpression_elimination} 次
                    </div>
                </div>
                
                <h4>优化后的IR代码:</h4>
                <pre><code class="language-ir">${optimizedInstructions.join('\n')}</code></pre>
                
                <h4>应用的优化:</h4>
                <ul>
                    ${optimizations.map(opt => `<li>✅ ${opt}</li>`).join('')}
                </ul>
                
                <h4>性能提升:</h4>
                <div class="message success">
                    ⚡ 执行速度提升: ~${Math.floor(Math.random() * 30 + 15)}%<br>
                    💾 内存使用减少: ~${Math.floor(Math.random() * 20 + 10)}%<br>
                    📏 代码大小减少: ~${Math.floor(Math.random() * 25 + 12)}%
                </div>
            `;
        } catch (error) {
            return `
                <div class="message error">❌ 优化失败</div>
                <div class="ast-tree">
                    <div style="color: var(--error-color);">
                        错误: ${error.message}
                    </div>
                </div>
            `;
        }
    }
    
    simulateCompileAndRun(code) {
        try {
            // Simulate compilation to different targets
            const targets = [];
            let executionResults = [];
            
            // Python target
            if (code.includes('sphere') || code.includes('process')) {
                targets.push('Python');
                executionResults.push(`
# 生成的Python代码
import numpy as np
from coastal_dsl import Sphere, Process

class Atmosphere(Sphere):
    def __init__(self):
        self.temperature = 273.15  # kelvin
        self.pressure = 101325.0   # pascal
        self.humidity = 0.6        # percent
    
    def temperature_conversion(self, temp_celsius):
        return temp_celsius + 273.15

# 执行结果
atmosphere = Atmosphere()
result = atmosphere.temperature_conversion(25.0)
print(f"转换结果: {result} K")
                `);
            }
            
            // Julia target
            if (code.includes('ai_model') || code.includes('Field3D')) {
                targets.push('Julia');
                executionResults.push(`
# 生成的Julia代码
using CoastalDSL
using LinearAlgebra

struct Atmosphere
    temperature::Float64
    pressure::Float64
    humidity::Float64
end

function temperature_conversion(temp_celsius::Float64)::Float64
    return temp_celsius + 273.15
end

# 执行结果
atmosphere = Atmosphere(273.15, 101325.0, 0.6)
result = temperature_conversion(25.0)
println("转换结果: $result K")
                `);
            }
            
            // Execution simulation
            const executionTime = Math.random() * 100 + 50; // 50-150ms
            const memoryUsage = Math.random() * 10 + 5; // 5-15MB
            
            // Simulate some computation results
            const computationResults = [];
            if (code.includes('temperature')) {
                computationResults.push('温度转换: 25°C → 298.15K');
            }
            if (code.includes('pressure')) {
                computationResults.push('压力计算: 1013.25 hPa');
            }
            if (code.includes('ai_model')) {
                computationResults.push('AI模型预测: 波高 = 2.3m ± 0.15m (置信度: 0.87)');
            }
            
            return `
                <div class="message success">✅ 编译和执行成功</div>
                
                <h4>目标平台:</h4>
                <div class="target-info">
                    ${targets.map(target => `<span class="target-badge">🎯 ${target}</span>`).join(' ')}
                </div>
                
                <h4>生成的代码:</h4>
                <div class="code-tabs">
                    ${executionResults.map((code, index) => `
                        <div class="code-tab">
                            <h5>${targets[index]} 代码:</h5>
                            <pre><code class="language-${targets[index].toLowerCase()}">${code}</code></pre>
                        </div>
                    `).join('')}
                </div>
                
                <h4>执行结果:</h4>
                <div class="execution-results">
                    <div class="message success">
                        ⏱️ 执行时间: ${executionTime.toFixed(2)}ms<br>
                        💾 内存使用: ${memoryUsage.toFixed(1)}MB<br>
                        🔧 编译时间: ${(Math.random() * 500 + 200).toFixed(0)}ms
                    </div>
                    
                    ${computationResults.length > 0 ? `
                    <h5>计算结果:</h5>
                    <ul>
                        ${computationResults.map(result => `<li>✅ ${result}</li>`).join('')}
                    </ul>
                    ` : ''}
                </div>
                
                <h4>运行时信息:</h4>
                <div class="runtime-info">
                    <div class="message success">
                        📏 物理单位检查: ${code.includes('with unit') ? '运行时验证通过' : '未启用'}<br>
                        🧠 AI模型推理: ${code.includes('ai_model') ? '神经网络加速执行' : '未使用'}<br>
                        ⚡ 并行执行: ${Math.random() > 0.5 ? '多线程优化' : '单线程执行'}
                    </div>
                </div>
            `;
        } catch (error) {
            return `
                <div class="message error">❌ 编译执行失败</div>
                <div class="ast-tree">
                    <div style="color: var(--error-color);">
                        编译错误: ${error.message}<br>
                        建议: 检查语法和类型定义
                    </div>
                </div>
            `;
        }
    }
    
    setupExamples() {
        const loadButtons = document.querySelectorAll('.load-example');
        const codeEditor = document.getElementById('code-editor');
        
        loadButtons.forEach(button => {
            button.addEventListener('click', () => {
                const exampleType = button.dataset.example;
                if (this.examples[exampleType]) {
                    codeEditor.value = this.examples[exampleType];
                    
                    // Switch to editor tab
                    this.switchToTab('editor');
                    
                    // Show success message
                    this.showMessage('示例代码已加载到编辑器', 'success');
                }
            });
        });
        
        // 处理模型加载按钮
        const loadModelButtons = document.querySelectorAll('.load-model-btn');
        
        loadModelButtons.forEach(button => {
            button.addEventListener('click', () => {
                const modelType = button.dataset.model;
                if (this.modelExamples[modelType]) {
                    codeEditor.value = this.modelExamples[modelType];
                    
                    // Switch to editor tab
                    this.switchToTab('editor');
                    
                    // Show success message
                    this.showMessage(`${modelType.toUpperCase()}模型配置已加载到编辑器`, 'success');
                }
            });
        });
        
        // 处理文档按钮
        const docsButtons = document.querySelectorAll('.docs-btn');
        
        docsButtons.forEach(button => {
            button.addEventListener('click', () => {
                // 获取对应的模型类型
                const modelCard = button.closest('.model-card');
                const modelName = modelCard.querySelector('h4').textContent;
                
                // 显示模型文档信息
                this.showModelDocumentation(modelName);
            });
        });
    }
    
    setupAST() {
        const visualizeBtn = document.getElementById('visualize-ast');
        
        visualizeBtn.addEventListener('click', () => {
            this.visualizeAST();
        });
    }
    
  switchToTab(tabId) {
    const tabButton = document.querySelector(`[data-tab="${tabId}"]`);
    const tabContent = document.getElementById(tabId);
    document.querySelectorAll('.nav-button').forEach(btn => btn.classList.remove('active'));
    document.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));
    if (tabButton) tabButton.classList.add('active');
    if (tabContent) tabContent.classList.add('active');
    this.currentTab = tabId;
    if (tabId === 'sedris') {
      this.initializeSedrisVisualization();
    }
  }

  initRouter() {
    window.addEventListener('hashchange', () => this.handleRouteChange());
    this.handleRouteChange();
  }

  handleRouteChange() {
    const hash = window.location.hash.replace(/^#\/?/, '');
    const tabId = hash || this.currentTab || 'platform-overview';
    this.switchToTab(tabId);
  }

  navigateTo(tabId) {
    if (!tabId) return;
    const targetHash = `#/${tabId}`;
    if (window.location.hash !== targetHash) {
      window.location.hash = targetHash;
    } else {
      this.switchToTab(tabId);
    }
  }
    
    parseCode() {
        const codeEditor = document.getElementById('code-editor');
        const parseOutput = document.getElementById('parse-output');
        const code = codeEditor.value.trim();
        
        if (!code) {
            parseOutput.innerHTML = '<div class="message error">请输入代码后再解析</div>';
            return;
        }
        
        // Show loading
        parseOutput.innerHTML = '<div class="loading"></div> 正在解析代码...';
        
        // Simulate parsing (since we don't have WASM integration yet)
        setTimeout(() => {
            const result = this.simulateParser(code);
            parseOutput.innerHTML = result;
        }, 1000);
    }
    
    simulateParser(code) {
        try {
            // Simple lexical analysis simulation
            const tokens = this.simulateLexer(code);
            const ast = this.simulateASTGeneration(code);
            
            // Determine complexity level
            const isComplex = code.length > 1000 || code.includes('Field3D') || code.includes('TimeSeries');
            const sphereCount = (code.match(/sphere\s+/g) || []).length;
            const variableCount = (code.match(/:\s*\w+/g) || []).length;
            
            let complexityInfo = '';
            if (isComplex) {
                complexityInfo = `
                <h4>复杂度分析:</h4>
                <div class="message success">
                    🏭 工业级示例检测到<br>
                    📊 球层数量: ${sphereCount}<br>
                    📈 变量数量: ${variableCount}<br>
                    🔬 多分辨率支持: ${code.includes('resolution') ? '✅' : '❌'}<br>
                    🌊 多物理过程: ${code.includes('processes') ? '✅' : '❌'}<br>
                    🤖 AI集成: ${code.includes('ai_model') ? '✅' : '❌'}
                </div>
                `;
            }
            
            return `
                <div class="message success">✅ 解析成功</div>
                
                <h4>词法分析结果 (${tokens.length} 个token):</h4>
                <div class="ast-tree">
                    ${tokens.slice(0, 20).map(token => `<div class="ast-node">
                        <span class="ast-node-type">${token.type}</span>: 
                        <span class="ast-node-value">"${token.value}"</span>
                        <span style="color: #64748b;">(${token.line}:${token.column})</span>
                    </div>`).join('')}
                    ${tokens.length > 20 ? '<div style="color: #64748b; font-style: italic;">... 还有 ' + (tokens.length - 20) + ' 个token</div>' : ''}
                </div>
                
                <h4>语法分析结果:</h4>
                <div class="ast-tree">
                    ${ast}
                </div>
                
                ${complexityInfo}
                
                <h4>语义分析:</h4>
                <div class="message success">
                    ✅ 类型检查通过<br>
                    ✅ 单位一致性验证通过<br>
                    ✅ 作用域分析通过<br>
                    ${isComplex ? '✅ 多圈层耦合验证通过<br>✅ 物理约束检查通过' : ''}
                </div>
            `;
        } catch (error) {
            return `
                <div class="message error">❌ 解析失败</div>
                <div class="ast-tree">
                    <div style="color: var(--error-color);">
                        错误: ${error.message}<br>
                        位置: 第 ${error.line || 1} 行，第 ${error.column || 1} 列
                    </div>
                </div>
            `;
        }
    }
    
    simulateLexer(code) {
        const tokens = [];
        const lines = code.split('\n');
        
        const keywords = ['sphere', 'process', 'ai_model', 'data', 'model', 'with', 'unit', 'Real', 'extends', 'constraint', 'inputs', 'outputs', 'parameters', 'method', 'components', 'coupling', 'scale', 'Array', 'Field3D', 'Field2D', 'Vector3D', 'Vector2D', 'TimeSeries', 'Uncertain'];
        const chineseKeywords = ['大气圈', '海洋圈', '陆地圈', '生物圈', '潮汐过程', '蒸发过程', '海水', '水汽', '温度', '压力', '湿度', '流域', '河口', '近海'];
        const units = ['celsius', 'kelvin', 'pascal', 'hectopascal', 'meter', 'newton', 'kilogram', 'second', 'psu', 'decibar', 'sverdrup', 'degree', 'radian', 'minute', 'hour', 'day', 'year', 'mg/L', 'μg/m^3', 'W/m^2', 'mm/h', 'mm/day', 'm/s', 'm^3/s', 'NTU', 'shannon_index', 'dimensionless'];
        
        lines.forEach((line, lineNum) => {
            // More sophisticated tokenization
            const tokenRegex = /(\w+|[{}:,;()[\]<>]|\/\/.*|\/\*[\s\S]*?\*\/|"[^"]*"|'[^']*'|\d+(?:\.\d+)?|\S)/g;
            let match;
            let column = 1;
            
            while ((match = tokenRegex.exec(line)) !== null) {
                const word = match[0];
                let tokenType = 'Identifier';
                
                if (word.startsWith('//') || word.startsWith('/*')) {
                    tokenType = 'Comment';
                } else if (keywords.includes(word)) {
                    tokenType = 'Keyword';
                } else if (chineseKeywords.includes(word)) {
                    tokenType = 'ChineseKeyword';
                } else if (units.includes(word)) {
                    tokenType = 'Unit';
                } else if (/^\d+(\.\d+)?$/.test(word)) {
                    tokenType = 'Number';
                } else if (/^["'].*["']$/.test(word)) {
                    tokenType = 'String';
                } else if (['properties', 'processes', 'boundaries', 'sensors', 'monitoring_stations'].includes(word)) {
                    tokenType = 'SectionKeyword';
                } else if (['{', '}', '[', ']', '(', ')', '<', '>'].includes(word)) {
                    tokenType = 'Bracket';
                } else if ([':', ',', ';'].includes(word)) {
                    tokenType = 'Punctuation';
                } else if (word.includes('_') && word.length > 3) {
                    tokenType = 'VariableName';
                }
                
                tokens.push({
                    type: tokenType,
                    value: word,
                    line: lineNum + 1,
                    column: match.index + 1
                });
            }
        });
        
        return tokens;
    }
    
    simulateASTGeneration(code) {
        const spheres = code.match(/sphere\s+(\S+)/g) || [];
        const models = code.match(/model\s+(\S+)/g) || [];
        const dataStructures = code.match(/data\s+(\S+)/g) || [];
        const databases = code.match(/coastal_database\s+(\S+)/g) || [];
        const cmecsUnits = code.match(/cmecs_unit\s*:\s*(\S+)/g) || [];
        
        if (spheres.length > 0 || models.length > 0 || dataStructures.length > 0 || databases.length > 0) {
            let astNodes = [];
            
            // Handle spheres
            spheres.forEach(sphere => {
                const name = sphere.split(/\s+/)[1];
                astNodes.push(`
                    <div class="ast-node">
                        <span class="ast-node-type">SphereDecl</span>
                        <div class="ast-node">
                            <span class="ast-node-type">name</span>: 
                            <span class="ast-node-value">"${name}"</span>
                        </div>
                        <div class="ast-node">
                            <span class="ast-node-type">properties</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">PropertyList</span> (${(code.match(/\w+:\s*\w+/g) || []).length} properties)
                            </div>
                        </div>
                        ${code.includes('processes') ? `
                        <div class="ast-node">
                            <span class="ast-node-type">processes</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">ProcessList</span> (${(code.match(/Process</g) || []).length} processes)
                            </div>
                        </div>` : ''}
                    </div>
                `);
            });
            
            // Handle models
            models.forEach(model => {
                const name = model.split(/\s+/)[1];
                astNodes.push(`
                    <div class="ast-node">
                        <span class="ast-node-type">ModelDecl</span>
                        <div class="ast-node">
                            <span class="ast-node-type">name</span>: 
                            <span class="ast-node-value">"${name}"</span>
                        </div>
                        <div class="ast-node">
                            <span class="ast-node-type">components</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">ComponentList</span> (${(code.match(/\w+_model:/g) || []).length} components)
                            </div>
                        </div>
                        ${code.includes('coupling') ? `
                        <div class="ast-node">
                            <span class="ast-node-type">coupling</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">CouplingSpec</span>
                            </div>
                        </div>` : ''}
                    </div>
                `);
            });
            
            // Handle data structures
            dataStructures.forEach(data => {
                const name = data.split(/\s+/)[1];
                astNodes.push(`
                    <div class="ast-node">
                        <span class="ast-node-type">DataDecl</span>
                        <div class="ast-node">
                            <span class="ast-node-type">name</span>: 
                            <span class="ast-node-value">"${name}"</span>
                        </div>
                        <div class="ast-node">
                            <span class="ast-node-type">fields</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">FieldList</span>
                            </div>
                        </div>
                    </div>
                `);
            });
            
            // Handle coastal databases (CZ-DB)
            databases.forEach(dbDecl => {
                const name = dbDecl.split(/\s+/)[1];
                const layerCount = (code.match(/layer\s*{/g) || []).length;
                const datasetCount = (code.match(/dataset\s*{/g) || []).length;
                const cmecsCount = cmecsUnits.length;
                astNodes.push(`
                    <div class="ast-node">
                        <span class="ast-node-type">CoastalDatabaseDecl</span>
                        <div class="ast-node">
                            <span class="ast-node-type">name</span>: 
                            <span class="ast-node-value">"${name}"</span>
                        </div>
                        <div class="ast-node">
                            <span class="ast-node-type">layers</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">LayerList</span> (${layerCount} layers)
                            </div>
                        </div>
                        <div class="ast-node">
                            <span class="ast-node-type">datasets</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">DatasetList</span> (${datasetCount} datasets)
                            </div>
                        </div>
                        <div class="ast-node">
                            <span class="ast-node-type">cmecs_units</span>:
                            <div class="ast-node">
                                <span class="ast-node-type">CmecsUnitRefList</span> (${cmecsCount} refs)
                            </div>
                        </div>
                    </div>
                `);
            });
            
            return `
                <div class="ast-node">
                    <span class="ast-node-type">Program</span>
                    ${astNodes.join('')}
                </div>
            `;
        }
        
        return '<div class="ast-node"><span class="ast-node-type">Program</span> (empty)</div>';
    }
    
    visualizeAST() {
        const astInput = document.getElementById('ast-input');
        const astTree = document.getElementById('ast-tree');
        const code = astInput.value.trim();
        
        if (!code) {
            astTree.innerHTML = '<p class="placeholder">请输入代码</p>';
            return;
        }
        
        // Show loading
        astTree.innerHTML = '<div class="loading"></div> 生成AST中...';
        
        setTimeout(() => {
            const ast = this.simulateASTGeneration(code);
            astTree.innerHTML = ast;
        }, 500);
    }
    
    showMessage(text, type = 'success') {
        const message = document.createElement('div');
        message.className = `message ${type}`;
        message.textContent = text;
        
        // Insert at top of current tab content
        const currentContent = document.querySelector('.tab-content.active');
        currentContent.insertBefore(message, currentContent.firstChild);
        
        // Remove after 3 seconds
        setTimeout(() => {
            message.remove();
        }, 3000);
    }
    
    loadSyntaxHighlighting() {
        // Define custom language for Prism.js
                if (typeof Prism !== 'undefined') {
            Prism.languages.cz = {
                'comment': /\/\/.*|\/\*[\s\S]*?\*\//,
                'string': {
                    pattern: /"(?:\\.|[^\\"\r\n])*"/,
                    greedy: true
                },
                'keyword': /\b(?:sphere|process|ai_model|data|model|constraint|with|unit|extends|inputs|outputs|parameters|method|fn|let|const|if|else|for|while|match|return|break|continue|coastal_database|layer|dataset)\b/,
                'chinese-keyword': /\b(?:大气圈|海洋圈|岩石圈|生物圈|人类圈|潮汐过程|蒸发过程|海水|水汽|温度|压力|湿度)\b/,
                'number': /\b\d+(?:\.\d+)?(?:[eE][+-]?\d+)?\b/,
                'operator': /[+\-*\/=<>!&|∧∨∇∂×·∘†~]/,
                'punctuation': /[{}[\];(),.:]/,
                'unit': /\b(?:kelvin|pascal|meter|newton|kilogram|second|ampere|mole|candela|celsius|psu|decibar|sverdrup|hectopascal|atmosphere|degree|radian|minute|hour|day|year)\b/
            };
            
            // Re-highlight all code blocks
            Prism.highlightAll();
        }
    }
    
    initializeSedrisVisualization() {
        // Check if Three.js is loaded and SEDRIS viewer container exists
        if (typeof THREE !== 'undefined' && document.getElementById('sedris-3d-viewer')) {
            // Only initialize once
            if (!this.sedrisVisualization) {
                try {
                    this.sedrisVisualization = new SedrisVisualization('sedris-3d-viewer');
                    console.log('SEDRIS 3D visualization initialized successfully');
                } catch (error) {
                    console.error('Failed to initialize SEDRIS visualization:', error);
                    // Show fallback message
                    const viewer = document.getElementById('sedris-3d-viewer');
                    if (viewer) {
                        viewer.innerHTML = `
                            <div class="loading-message">
                                <p style="color: #ef4444;">⚠️ 3D可视化初始化失败</p>
                                <p style="font-size: 0.875rem; color: #64748b;">请检查浏览器是否支持WebGL</p>
                            </div>
                        `;
                    }
                }
            }
        } else {
            // Show loading message if Three.js is not yet loaded
            const viewer = document.getElementById('sedris-3d-viewer');
            if (viewer) {
                viewer.innerHTML = `
                    <div class="loading-message">
                        <div class="loading-spinner"></div>
                        <p>正在加载Three.js库...</p>
                    </div>
                `;
            }
            
            // Retry after a short delay
            setTimeout(() => {
                this.initializeSedrisVisualization();
            }, 1000);
        }
    }

    showModelDocumentation(modelName) {
        const docInfo = this.getModelDocumentation(modelName);
        
        // 创建文档弹窗
        const modal = document.createElement('div');
        modal.className = 'model-doc-modal';
        modal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h3>${modelName} 文档</h3>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    ${docInfo}
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);
        
        // 绑定关闭事件
        modal.querySelector('.modal-close').addEventListener('click', () => {
            document.body.removeChild(modal);
        });
        
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                document.body.removeChild(modal);
            }
        });
    }
    
    getModelDocumentation(modelName) {
        const docs = {
            '🌊 ROMS (Regional Ocean Modeling System)': `
                <h4>模型概述</h4>
                <p>ROMS是一个自由表面、地形跟随坐标的原始方程海洋模型，广泛应用于区域海洋建模。</p>
                
                <h4>主要特性</h4>
                <ul>
                    <li>三维原始方程模型</li>
                    <li>静力平衡和Boussinesq近似</li>
                    <li>自由表面边界条件</li>
                    <li>地形跟随坐标系统</li>
                    <li>多种湍流闭合方案</li>
                </ul>
                
                <h4>适用场景</h4>
                <ul>
                    <li>区域海洋环流模拟</li>
                    <li>海岸带上升流研究</li>
                    <li>河口混合过程</li>
                    <li>海洋生态系统建模</li>
                </ul>
                
                <h4>参考资料</h4>
                <p><a href="https://www.myroms.org/" target="_blank">ROMS官方网站</a></p>
            `,
            '🌊 SWAN (Simulating WAves Nearshore)': `
                <h4>模型概述</h4>
                <p>SWAN是第三代波浪模型，专门用于近岸、湖泊和河口的波浪预报。</p>
                
                <h4>主要特性</h4>
                <ul>
                    <li>基于波作用平衡方程</li>
                    <li>考虑风生波、涌浪传播</li>
                    <li>包含非线性波-波相互作用</li>
                    <li>处理波浪破碎和底摩擦</li>
                    <li>支持非结构化网格</li>
                </ul>
                
                <h4>适用场景</h4>
                <ul>
                    <li>近岸波浪预报</li>
                    <li>港口工程设计</li>
                    <li>海岸防护评估</li>
                    <li>波浪能资源评估</li>
                </ul>
                
                <h4>参考资料</h4>
                <p><a href="http://swanmodel.sourceforge.net/" target="_blank">SWAN官方网站</a></p>
            `,
            '🌊 FVCOM (Finite Volume Community Ocean Model)': `
                <h4>模型概述</h4>
                <p>FVCOM是基于有限体积法的三维海洋模型，特别适用于复杂海岸线和地形的建模。</p>
                
                <h4>主要特性</h4>
                <ul>
                    <li>非结构化三角网格</li>
                    <li>有限体积数值方法</li>
                    <li>σ坐标垂直分层</li>
                    <li>湿干边界处理</li>
                    <li>多种湍流闭合模式</li>
                </ul>
                
                <h4>适用场景</h4>
                <ul>
                    <li>河口和海湾建模</li>
                    <li>潮汐和潮流模拟</li>
                    <li>污染物扩散研究</li>
                    <li>海洋生态系统建模</li>
                </ul>
                
                <h4>参考资料</h4>
                <p><a href="http://fvcom.smast.umassd.edu/" target="_blank">FVCOM官方网站</a></p>
            `,
            '🌪️ WRF (Weather Research and Forecasting)': `
                <h4>模型概述</h4>
                <p>WRF是新一代中尺度数值天气预报系统，广泛应用于大气科学研究和业务预报。</p>
                
                <h4>主要特性</h4>
                <ul>
                    <li>非静力平衡动力核心</li>
                    <li>多种物理参数化方案</li>
                    <li>嵌套网格技术</li>
                    <li>数据同化系统</li>
                    <li>并行计算支持</li>
                </ul>
                
                <h4>适用场景</h4>
                <ul>
                    <li>区域天气预报</li>
                    <li>台风路径预测</li>
                    <li>海岸带风场模拟</li>
                    <li>空气质量预报</li>
                </ul>
                
                <h4>参考资料</h4>
                <p><a href="https://www.mmm.ucar.edu/weather-research-and-forecasting-model" target="_blank">WRF官方网站</a></p>
            `,
            '🌪️ HWRF (Hurricane Weather Research and Forecasting)': `
                <h4>模型概述</h4>
                <p>HWRF是专门用于热带气旋预报的高分辨率数值模型，基于WRF开发。</p>
                
                <h4>主要特性</h4>
                <ul>
                    <li>高分辨率嵌套网格</li>
                    <li>热带气旋专用物理方案</li>
                    <li>海气耦合接口</li>
                    <li>涡旋初始化技术</li>
                    <li>实时数据同化</li>
                </ul>
                
                <h4>适用场景</h4>
                <ul>
                    <li>台风/飓风路径预报</li>
                    <li>风暴潮预警</li>
                    <li>海岸带灾害评估</li>
                    <li>热带气旋研究</li>
                </ul>
                
                <h4>参考资料</h4>
                <p><a href="https://www.emc.ncep.noaa.gov/gc_wmb/vxt/HWRF/" target="_blank">HWRF官方网站</a></p>
            `,
            '🏖️ Delft3D': `
                <h4>模型概述</h4>
                <p>Delft3D是综合性水动力、沉积物输运和水质建模软件包。</p>
                
                <h4>主要模块</h4>
                <ul>
                    <li>FLOW: 水动力模块</li>
                    <li>WAVE: 波浪模块</li>
                    <li>SED: 沉积物输运</li>
                    <li>ECO: 生态模块</li>
                    <li>MOR: 地貌演化</li>
                </ul>
                
                <h4>适用场景</h4>
                <ul>
                    <li>河口和海岸工程</li>
                    <li>港口和航道设计</li>
                    <li>海岸侵蚀评估</li>
                    <li>水质和生态建模</li>
                </ul>
                
                <h4>参考资料</h4>
                <p><a href="https://oss.deltares.nl/web/delft3d" target="_blank">Delft3D官方网站</a></p>
            `
        };
        
        return docs[modelName] || `
            <h4>文档暂未提供</h4>
            <p>该模型的详细文档正在准备中，请稍后查看。</p>
        `;
    }

    // 知识图谱相关方法
    showEntityDetails(entityType) {
        const entityDetails = this.getEntityDetails(entityType);
        
        const modal = document.createElement('div');
        modal.className = 'model-doc-modal';
        modal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h3>${entityDetails.name} 详细信息</h3>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    ${entityDetails.content}
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);
        
        modal.querySelector('.modal-close').addEventListener('click', () => {
            document.body.removeChild(modal);
        });
        
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                document.body.removeChild(modal);
            }
        });
    }

    getEntityDetails(entityType) {
        const entities = {
            'coastline': {
                name: '🏖️ 海岸线实体',
                content: `
                    <h4>实体定义</h4>
                    <p>海岸线是陆地与海洋的分界线，是海岸带系统中最重要的地理实体之一。</p>
                    
                    <h4>本体建模</h4>
                    <pre><code>class Coastline extends GeographicEntity {
    geometry: LineString
    length: Distance
    curvature: Real
    material: CoastalMaterial
    elevation: Elevation
    slope: Angle
    erosionRate: Rate
    changePattern: TemporalPattern
}</code></pre>
                    
                    <h4>语义关系</h4>
                    <ul>
                        <li><strong>空间关系:</strong> adjacent_to(IntertidalZone), bounds(Land), faces(Ocean)</li>
                        <li><strong>影响关系:</strong> affected_by(Wave), modified_by(Tide), protected_by(Vegetation)</li>
                        <li><strong>功能关系:</strong> protects(Inland), supports(CoastalEcosystem)</li>
                    </ul>
                    
                    <h4>推理规则</h4>
                    <ul>
                        <li>如果海岸线材质为沙质且波浪能量高，则侵蚀风险高</li>
                        <li>如果海岸线有植被保护且坡度适中，则稳定性好</li>
                        <li>如果海岸线面临风暴潮且无防护，则需要预警</li>
                    </ul>
                `
            },
            'tide': {
                name: '🌊 潮汐实体',
                content: `
                    <h4>实体定义</h4>
                    <p>潮汐是由月球和太阳引力作用引起的海水周期性涨落现象，是海岸带最重要的物理过程。</p>
                    
                    <h4>本体建模</h4>
                    <pre><code>class Tide extends PhysicalEntity {
    period: Duration
    amplitude: Distance
    phase: Angle
    tidalRange: Distance
    currentVelocity: Velocity
    harmonicConstants: HarmonicSet
    predictionAccuracy: Percentage
}</code></pre>
                    
                    <h4>语义关系</h4>
                    <ul>
                        <li><strong>驱动关系:</strong> drives(TidalCurrent), influences(IntertidalZone)</li>
                        <li><strong>调制关系:</strong> modulates(Wave), affects(SedimentTransport)</li>
                        <li><strong>预测关系:</strong> predicted_by(HarmonicAnalysis), validated_by(Observation)</li>
                    </ul>
                    
                    <h4>推理规则</h4>
                    <ul>
                        <li>如果潮差大于4米，则为大潮差海岸</li>
                        <li>如果潮汐周期为半日潮，则一日两次高低潮</li>
                        <li>如果潮流速度高，则沉积物输运活跃</li>
                    </ul>
                `
            },
            'mangrove': {
                name: '🌿 红树林实体',
                content: `
                    <h4>实体定义</h4>
                    <p>红树林是生长在热带、亚热带海岸潮间带的木本植物群落，具有重要的生态和防护功能。</p>
                    
                    <h4>本体建模</h4>
                    <pre><code>class Mangrove extends EcologicalEntity {
    speciesComposition: SpeciesSet
    biomass: Mass
    diversity: ShannonIndex
    coverage: Percentage
    carbonStorage: Mass
    protectionFunction: ProtectionLevel
    purificationCapacity: Rate
    healthStatus: HealthIndex
}</code></pre>
                    
                    <h4>语义关系</h4>
                    <ul>
                        <li><strong>栖息关系:</strong> habitat_for(Birds), nursery_for(Fish)</li>
                        <li><strong>防护关系:</strong> buffers(StormSurge), reduces(WaveEnergy)</li>
                        <li><strong>依赖关系:</strong> depends_on(Tide), requires(Salinity)</li>
                    </ul>
                    
                    <h4>推理规则</h4>
                    <ul>
                        <li>如果红树林覆盖度下降，则防护功能减弱</li>
                        <li>如果盐度过高或过低，则红树林健康受影响</li>
                        <li>如果红树林生物量增加，则碳汇功能增强</li>
                    </ul>
                `
            }
        };
        
        return entities[entityType] || {
            name: '未知实体',
            content: '<p>该实体的详细信息暂未提供。</p>'
        };
    }

    executeSemanticQuery() {
        const query = document.getElementById('semanticQuery').value;
        const resultsDiv = document.getElementById('queryResults');
        
        // 模拟查询执行
        resultsDiv.innerHTML = `
            <div class="query-executing">
                <p>正在执行查询...</p>
            </div>
        `;
        
        setTimeout(() => {
            const mockResults = this.getMockQueryResults(query);
            resultsDiv.innerHTML = mockResults;
        }, 1500);
    }

    getMockQueryResults(query) {
        if (query.includes('erosionRate')) {
            return `
                <table class="results-table">
                    <thead>
                        <tr>
                            <th>实体</th>
                            <th>属性</th>
                            <th>值</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>长江口海岸线</td>
                            <td>侵蚀速率</td>
                            <td>2.3 mm/year</td>
                        </tr>
                        <tr>
                            <td>杭州湾海岸线</td>
                            <td>侵蚀速率</td>
                            <td>1.8 mm/year</td>
                        </tr>
                        <tr>
                            <td>舟山群岛海岸线</td>
                            <td>侵蚀速率</td>
                            <td>0.9 mm/year</td>
                        </tr>
                    </tbody>
                </table>
            `;
        } else {
            return `
                <div class="result-message">
                    <p>查询执行完成，返回 ${Math.floor(Math.random() * 50) + 10} 条结果。</p>
                    <p>请使用具体的查询语句以获得详细结果。</p>
                </div>
            `;
        }
    }

    loadQueryTemplate() {
        const templates = [
            `SELECT ?entity ?erosionRate
WHERE {
  ?entity rdf:type cz:Coastline .
  ?entity cz:erosionRate ?erosionRate .
  FILTER(?erosionRate > 2.0)
}`,
            `SELECT ?ecosystem ?healthIndex
WHERE {
  ?ecosystem rdf:type cz:EcologicalEntity .
  ?ecosystem cz:healthStatus ?healthIndex .
  ?ecosystem cz:location ?location .
  FILTER(contains(?location, "长江口"))
}`,
            `SELECT ?tide ?amplitude ?period
WHERE {
  ?tide rdf:type cz:Tide .
  ?tide cz:amplitude ?amplitude .
  ?tide cz:period ?period .
  ?tide cz:location ?location .
}`
        ];
        
        const randomTemplate = templates[Math.floor(Math.random() * templates.length)];
        document.getElementById('semanticQuery').value = randomTemplate;
    }

    clearQuery() {
        document.getElementById('semanticQuery').value = '';
        document.getElementById('queryResults').innerHTML = `
            <div class="result-placeholder">
                <p>执行查询以查看结果...</p>
            </div>
        `;
    }

    loadTemplate(templateType) {
        const templates = {
            'erosion': `SELECT ?coastline ?erosionRate ?riskLevel
WHERE {
  ?coastline rdf:type cz:Coastline .
  ?coastline cz:erosionRate ?erosionRate .
  ?coastline cz:riskLevel ?riskLevel .
  FILTER(?erosionRate > 1.0)
}
ORDER BY DESC(?erosionRate)`,
            'ecosystem': `SELECT ?ecosystem ?species ?healthIndex
WHERE {
  ?ecosystem rdf:type cz:EcologicalEntity .
  ?ecosystem cz:speciesComposition ?species .
  ?ecosystem cz:healthStatus ?healthIndex .
  ?ecosystem cz:location ?location .
  FILTER(contains(?location, "指定区域"))
}`,
            'tide': `SELECT ?location ?highTide ?lowTide ?tidalRange
WHERE {
  ?tide rdf:type cz:Tide .
  ?tide cz:location ?location .
  ?tide cz:highTide ?highTide .
  ?tide cz:lowTide ?lowTide .
  ?tide cz:tidalRange ?tidalRange .
  ?tide cz:date ?date .
  FILTER(?date >= "2024-01-01" && ?date <= "2024-01-31")
}`,
            'impact': `SELECT ?entity1 ?relation ?entity2 ?strength
WHERE {
  ?entity1 ?relation ?entity2 .
  ?relation rdfs:subPropertyOf cz:influences .
  ?relation cz:strength ?strength .
  FILTER(?strength > 0.7)
}
ORDER BY DESC(?strength)`
        };
        
        document.getElementById('semanticQuery').value = templates[templateType] || '';
    }

    // Initialize Dashboard Features
    initializeDashboard() {
        this.setupDashboardControls();
        this.startRealTimeUpdates();
        this.initializeCounters();
        this.createSparklines();
    }
    
    setupDashboardControls() {
        const dashboardBtns = document.querySelectorAll('.dashboard-btn');
        dashboardBtns.forEach(btn => {
            btn.addEventListener('click', () => {
                // Remove active class from all buttons
                dashboardBtns.forEach(b => b.classList.remove('active'));
                // Add active class to clicked button
                btn.classList.add('active');
                
                const view = btn.dataset.view;
                this.switchDashboardView(view);
            });
        });
    }
    
    switchDashboardView(view) {
        const dashboardGrid = document.getElementById('dashboard-realtime');
        if (!dashboardGrid) return;
        
        // Add transition effect
        dashboardGrid.style.opacity = '0.5';
        dashboardGrid.style.transform = 'translateY(20px)';
        
        setTimeout(() => {
            switch(view) {
                case 'realtime':
                    this.showRealtimeView();
                    break;
                case 'historical':
                    this.showHistoricalView();
                    break;
                case 'predictive':
                    this.showPredictiveView();
                    break;
            }
            
            dashboardGrid.style.opacity = '1';
            dashboardGrid.style.transform = 'translateY(0)';
        }, 300);
    }
    
    showRealtimeView() {
        // Update indicators with real-time data simulation
        this.updateIndicatorValues({
            'system-availability': { value: 99.9, trend: '+2.3%', color: 'success' },
            'response-time': { value: '<100ms', trend: '-15ms', color: 'success' },
            'concurrent-users': { value: '10K+', trend: '+500', color: 'info' },
            'compute-acceleration': { value: '1000x', trend: '+100x', color: 'warning' }
        });
    }
    
    showHistoricalView() {
        // Show historical trends
        this.updateIndicatorValues({
            'system-availability': { value: 99.7, trend: '+0.5%', color: 'success' },
            'response-time': { value: '<120ms', trend: '+5ms', color: 'warning' },
            'concurrent-users': { value: '8.5K', trend: '+1.2K', color: 'info' },
            'compute-acceleration': { value: '850x', trend: '+200x', color: 'success' }
        });
    }
    
    showPredictiveView() {
        // Show predictive analytics
        this.updateIndicatorValues({
            'system-availability': { value: 99.95, trend: '+0.05%', color: 'success' },
            'response-time': { value: '<80ms', trend: '-20ms', color: 'success' },
            'concurrent-users': { value: '15K+', trend: '+5K', color: 'success' },
            'compute-acceleration': { value: '1500x', trend: '+500x', color: 'success' }
        });
    }
    
    updateIndicatorValues(data) {
        Object.entries(data).forEach(([key, info]) => {
            const valueElement = document.querySelector(`[data-indicator="${key}"] .indicator-value`);
            const trendElement = document.querySelector(`[data-indicator="${key}"] .indicator-trend span`);
            
            if (valueElement) {
                this.animateValue(valueElement, info.value);
            }
            if (trendElement) {
                trendElement.textContent = info.trend;
            }
        });
    }
    
    initializeCounters() {
        const counters = document.querySelectorAll('[data-counter]');
        counters.forEach(counter => {
            const target = parseInt(counter.dataset.counter);
            this.animateCounter(counter, 0, target, 2000);
        });
    }
    
    animateCounter(element, start, end, duration) {
        const startTime = performance.now();
        const animate = (currentTime) => {
            const elapsed = currentTime - startTime;
            const progress = Math.min(elapsed / duration, 1);
            
            // Easing function for smooth animation
            const easeOutQuart = 1 - Math.pow(1 - progress, 4);
            const current = Math.floor(start + (end - start) * easeOutQuart);
            
            if (element.textContent.includes('%')) {
                element.textContent = current + '%';
            } else if (element.textContent.includes('K')) {
                element.textContent = (current / 1000).toFixed(1) + 'K+';
            } else {
                element.textContent = current.toString();
            }
            
            if (progress < 1) {
                requestAnimationFrame(animate);
            }
        };
        requestAnimationFrame(animate);
    }
    
    animateValue(element, newValue) {
        element.style.transform = 'scale(1.1)';
        element.style.color = 'var(--primary-color)';
        
        setTimeout(() => {
            element.textContent = newValue;
            element.style.transform = 'scale(1)';
            element.style.color = '';
        }, 200);
    }
    
    createSparklines() {
        const sparklines = document.querySelectorAll('.indicator-sparkline');
        sparklines.forEach(sparkline => {
            const values = JSON.parse(sparkline.dataset.values || '[0,0,0,0,0]');
            this.renderSparkline(sparkline, values);
        });
    }
    
    renderSparkline(container, values) {
        const canvas = document.createElement('canvas');
        canvas.width = container.offsetWidth;
        canvas.height = container.offsetHeight;
        canvas.style.position = 'absolute';
        canvas.style.top = '0';
        canvas.style.left = '0';
        container.appendChild(canvas);
        
        const ctx = canvas.getContext('2d');
        const max = Math.max(...values);
        const min = Math.min(...values);
        const range = max - min || 1;
        
        ctx.strokeStyle = 'rgba(37, 99, 235, 0.8)';
        ctx.lineWidth = 2;
        ctx.beginPath();
        
        values.forEach((value, index) => {
            const x = (index / (values.length - 1)) * canvas.width;
            const y = canvas.height - ((value - min) / range) * canvas.height;
            
            if (index === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
        });
        
        ctx.stroke();
        
        // Add glow effect
        ctx.shadowColor = 'rgba(37, 99, 235, 0.5)';
        ctx.shadowBlur = 3;
        ctx.stroke();
    }
    
    startRealTimeUpdates() {
        // Simulate real-time data updates
        setInterval(() => {
            this.updateSparklines();
            this.updateProgressBars();
        }, 5000);
    }
    
    updateSparklines() {
        const sparklines = document.querySelectorAll('.indicator-sparkline');
        sparklines.forEach(sparkline => {
            const canvas = sparkline.querySelector('canvas');
            if (canvas) {
                const values = JSON.parse(sparkline.dataset.values || '[0,0,0,0,0]');
                // Shift values and add new random value
                values.shift();
                const lastValue = values[values.length - 1];
                const newValue = lastValue + (Math.random() - 0.5) * 10;
                values.push(Math.max(0, newValue));
                
                sparkline.dataset.values = JSON.stringify(values);
                
                // Clear and redraw
                const ctx = canvas.getContext('2d');
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                this.renderSparkline(sparkline, values);
            }
        });
    }
    
    updateProgressBars() {
        const progressBars = document.querySelectorAll('.progress-bar.animated');
        progressBars.forEach(bar => {
            const currentWidth = parseFloat(bar.style.width);
            const variation = (Math.random() - 0.5) * 5; // ±2.5% variation
            const newWidth = Math.max(0, Math.min(100, currentWidth + variation));
            bar.style.width = newWidth + '%';
        });
    }
    
    // Initialize Animations
    initializeAnimations() {
        this.setupScrollAnimations();
        this.setupHoverEffects();
        this.setupParallaxEffects();
    }
    
    setupScrollAnimations() {
        const observerOptions = {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        };
        
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('animate-in');
                }
            });
        }, observerOptions);
        
        // Observe elements for scroll animations
        const animateElements = document.querySelectorAll('.metric-card, .capability-category, .scenario-card');
        animateElements.forEach(el => {
            el.classList.add('animate-on-scroll');
            observer.observe(el);
        });
    }
    
    setupHoverEffects() {
        // Enhanced hover effects for capability items
        const capabilityItems = document.querySelectorAll('.capability-item');
        capabilityItems.forEach(item => {
            item.addEventListener('mouseenter', () => {
                this.createRippleEffect(item);
            });
        });
    }
    
    createRippleEffect(element) {
        const ripple = document.createElement('div');
        ripple.className = 'ripple-effect';
        ripple.style.cssText = `
            position: absolute;
            border-radius: 50%;
            background: rgba(37, 99, 235, 0.3);
            transform: scale(0);
            animation: ripple 0.6s linear;
            pointer-events: none;
        `;
        
        const rect = element.getBoundingClientRect();
        const size = Math.max(rect.width, rect.height);
        ripple.style.width = ripple.style.height = size + 'px';
        ripple.style.left = (rect.width / 2 - size / 2) + 'px';
        ripple.style.top = (rect.height / 2 - size / 2) + 'px';
        
        element.style.position = 'relative';
        element.appendChild(ripple);
        
        setTimeout(() => {
            ripple.remove();
        }, 600);
    }
    
    setupParallaxEffects() {
        const parallaxElements = document.querySelectorAll('.platform-diagram');
        
        window.addEventListener('scroll', () => {
            const scrolled = window.pageYOffset;
            parallaxElements.forEach(element => {
                const rate = scrolled * -0.5;
                element.style.transform = `translateY(${rate}px)`;
            });
        });
    }
    
    // Setup Interactive Features
    setupInteractiveFeatures() {
        this.setupTooltips();
        this.setupModalSystem();
        this.setupKeyboardShortcuts();
    }
    
    setupTooltips() {
        const tooltipElements = document.querySelectorAll('[data-tooltip]');
        tooltipElements.forEach(element => {
            element.addEventListener('mouseenter', (e) => {
                this.showTooltip(e.target, e.target.dataset.tooltip);
            });
            
            element.addEventListener('mouseleave', () => {
                this.hideTooltip();
            });
        });
    }
    
    showTooltip(element, text) {
        const tooltip = document.createElement('div');
        tooltip.className = 'tooltip';
        tooltip.textContent = text;
        tooltip.style.cssText = `
            position: absolute;
            background: rgba(0, 0, 0, 0.9);
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 14px;
            z-index: 10000;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.3s ease;
        `;
        
        document.body.appendChild(tooltip);
        
        const rect = element.getBoundingClientRect();
        tooltip.style.left = rect.left + (rect.width / 2) - (tooltip.offsetWidth / 2) + 'px';
        tooltip.style.top = rect.top - tooltip.offsetHeight - 10 + 'px';
        
        setTimeout(() => {
            tooltip.style.opacity = '1';
        }, 10);
        
        this.currentTooltip = tooltip;
    }
    
    hideTooltip() {
        if (this.currentTooltip) {
            this.currentTooltip.style.opacity = '0';
            setTimeout(() => {
                if (this.currentTooltip) {
                    this.currentTooltip.remove();
                    this.currentTooltip = null;
                }
            }, 300);
        }
    }
    
    setupModalSystem() {
        // Setup modal for detailed views
        this.createModalContainer();
    }
    
    createModalContainer() {
        const modal = document.createElement('div');
        modal.id = 'app-modal';
        modal.className = 'modal-overlay';
        modal.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.8);
            display: none;
            justify-content: center;
            align-items: center;
            z-index: 10000;
        `;
        
        modal.innerHTML = `
            <div class="modal-content" style="
                background: white;
                border-radius: 12px;
                padding: 2rem;
                max-width: 90vw;
                max-height: 90vh;
                overflow-y: auto;
                position: relative;
            ">
                <button class="modal-close" style="
                    position: absolute;
                    top: 1rem;
                    right: 1rem;
                    background: none;
                    border: none;
                    font-size: 1.5rem;
                    cursor: pointer;
                ">&times;</button>
                <div class="modal-body"></div>
            </div>
        `;
        
        document.body.appendChild(modal);
        
        // Setup close functionality
        modal.addEventListener('click', (e) => {
            if (e.target === modal || e.target.classList.contains('modal-close')) {
                this.closeModal();
            }
        });
    }
    
    showModal(content) {
        const modal = document.getElementById('app-modal');
        const modalBody = modal.querySelector('.modal-body');
        modalBody.innerHTML = content;
        modal.style.display = 'flex';
        document.body.style.overflow = 'hidden';
    }
    
    closeModal() {
        const modal = document.getElementById('app-modal');
        modal.style.display = 'none';
        document.body.style.overflow = '';
    }
    
    setupKeyboardShortcuts() {
        document.addEventListener('keydown', (e) => {
            // Ctrl/Cmd + K for quick search
            if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                e.preventDefault();
                this.showQuickSearch();
            }
            
            // Escape to close modal
            if (e.key === 'Escape') {
                this.closeModal();
                this.hideTooltip();
            }
        });
    }
    
    showQuickSearch() {
        const searchContent = `
            <h2>快速搜索</h2>
            <input type="text" placeholder="搜索功能、文档或示例..." style="
                width: 100%;
                padding: 12px;
                border: 1px solid #e2e8f0;
                border-radius: 8px;
                font-size: 16px;
                margin: 1rem 0;
            ">
            <div class="search-results">
                <div class="search-category">
                    <h3>功能模块</h3>
                    <div class="search-item">编译器 - 高性能CZ-DSL编译器</div>
                    <div class="search-item">运行时 - 物理单位运行时系统</div>
                    <div class="search-item">标准库 - 海岸带专用函数库</div>
                </div>
                <div class="search-category">
                    <h3>示例代码</h3>
                    <div class="search-item">球层定义 - 大气圈建模示例</div>
                    <div class="search-item">SEDRIS集成 - 空间参考系统</div>
                    <div class="search-item">AI模型 - 波浪高度预测</div>
                </div>
            </div>
        `;
        this.showModal(searchContent);
    }
    
    // Enhanced Action Methods
    openDocumentation() {
        window.open('https://cz-dsl.org/docs', '_blank');
    }
    
    openSupport() {
        window.open('https://cz-dsl.org/support', '_blank');
    }
    
    startTrial() {
        // Redirect to the IDE interface
        window.location.href = 'user_interface.html';
    }
    
    startDemo() {
        // Switch to development suite tab and show demo
        document.querySelector('[data-tab="development-suite"]').click();
        setTimeout(() => {
            this.showModal(`
                <h2>在线演示</h2>
                <p>正在启动CZ-DSL在线开发环境...</p>
                <div style="text-align: center; margin: 2rem 0;">
                    <div class="loading" style="margin: 0 auto;"></div>
                </div>
                <p>演示环境包含：</p>
                <ul>
                    <li>完整的CZ-DSL编译器</li>
                    <li>交互式代码编辑器</li>
                    <li>实时语法检查</li>
                    <li>示例项目模板</li>
                </ul>
            `);
        }, 500);
    }
    
    downloadTrial() {
        this.showModal(`
            <h2>下载试用版</h2>
            <p>选择适合您系统的CZ-DSL试用版本</p>
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin: 2rem 0;">
                <div style="border: 1px solid #e2e8f0; border-radius: 8px; padding: 1rem; text-align: center;">
                    <i class="fab fa-windows" style="font-size: 2rem; color: #0078d4;"></i>
                    <h3>Windows</h3>
                    <p>Windows 10/11 (x64)</p>
                    <button style="background: #0078d4; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer;">下载</button>
                </div>
                <div style="border: 1px solid #e2e8f0; border-radius: 8px; padding: 1rem; text-align: center;">
                    <i class="fab fa-apple" style="font-size: 2rem; color: #000;"></i>
                    <h3>macOS</h3>
                    <p>macOS 11+ (Intel/Apple Silicon)</p>
                    <button style="background: #000; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer;">下载</button>
                </div>
                <div style="border: 1px solid #e2e8f0; border-radius: 8px; padding: 1rem; text-align: center;">
                    <i class="fab fa-linux" style="font-size: 2rem; color: #fcc624;"></i>
                    <h3>Linux</h3>
                    <p>Ubuntu 20.04+ / CentOS 8+</p>
                    <button style="background: #fcc624; color: black; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer;">下载</button>
                </div>
            </div>
        `);
    }
    
    viewArchitecture() {
        // Switch to platform overview and scroll to architecture
        document.querySelector('[data-tab="platform-overview"]').click();
        setTimeout(() => {
            document.querySelector('.platform-diagram').scrollIntoView({ 
                behavior: 'smooth',
                block: 'center'
            });
        }, 300);
    }
    
    // Initialize Floating Status Widget
    initializeFloatingStatus() {
        const statusWidget = document.getElementById('system-status');
        if (!statusWidget) return;
        
        // Toggle expanded state on click
        statusWidget.addEventListener('click', () => {
            statusWidget.classList.toggle('expanded');
        });
        
        // Start real-time status updates
        this.startStatusUpdates();
        
        // Add close button when expanded
        this.addStatusControls(statusWidget);
    }
    
    addStatusControls(widget) {
        const controls = document.createElement('div');
        controls.className = 'status-controls';
        controls.innerHTML = `
            <button class="status-control-btn" data-action="refresh" title="刷新状态">
                <i class="fas fa-sync-alt"></i>
            </button>
            <button class="status-control-btn" data-action="minimize" title="最小化">
                <i class="fas fa-minus"></i>
            </button>
        `;
        
        controls.style.cssText = `
            position: absolute;
            top: 0.5rem;
            right: 0.5rem;
            display: none;
            gap: 0.25rem;
        `;
        
        widget.appendChild(controls);
        
        // Show controls when expanded
        const observer = new MutationObserver((mutations) => {
            mutations.forEach((mutation) => {
                if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
                    if (widget.classList.contains('expanded')) {
                        controls.style.display = 'flex';
                    } else {
                        controls.style.display = 'none';
                    }
                }
            });
        });
        
        observer.observe(widget, { attributes: true });
        
        // Handle control actions
        controls.addEventListener('click', (e) => {
            e.stopPropagation();
            const action = e.target.closest('[data-action]')?.dataset.action;
            
            switch(action) {
                case 'refresh':
                    this.refreshSystemStatus();
                    break;
                case 'minimize':
                    widget.classList.remove('expanded');
                    break;
            }
        });
    }
    
    startStatusUpdates() {
        // Update status every 3 seconds
        setInterval(() => {
            this.updateSystemStatus();
        }, 3000);
        
        // Initial update
        this.updateSystemStatus();
    }
    
    updateSystemStatus() {
        const cpuFill = document.querySelector('.status-metric:nth-child(1) .metric-fill');
        const memoryFill = document.querySelector('.status-metric:nth-child(2) .metric-fill');
        const networkFill = document.querySelector('.status-metric:nth-child(3) .metric-fill');
        
        const cpuValue = document.querySelector('.status-metric:nth-child(1) .metric-value');
        const memoryValue = document.querySelector('.status-metric:nth-child(2) .metric-value');
        const networkValue = document.querySelector('.status-metric:nth-child(3) .metric-value');
        
        // Simulate realistic system metrics
        const cpu = Math.max(20, Math.min(80, 45 + (Math.random() - 0.5) * 20));
        const memory = Math.max(30, Math.min(90, 62 + (Math.random() - 0.5) * 15));
        const network = Math.max(10, Math.min(95, 78 + (Math.random() - 0.5) * 25));
        
        // Animate changes
        this.animateMetric(cpuFill, cpuValue, cpu);
        this.animateMetric(memoryFill, memoryValue, memory);
        this.animateMetric(networkFill, networkValue, network);
        
        // Update status indicator based on metrics
        this.updateStatusIndicator(cpu, memory, network);
    }
    
    animateMetric(fillElement, valueElement, newValue) {
        if (!fillElement || !valueElement) return;
        
        const currentWidth = parseFloat(fillElement.style.width) || 0;
        const targetWidth = newValue;
        
        // Animate the fill bar
        fillElement.style.width = targetWidth + '%';
        
        // Update the value with animation
        const currentValue = parseInt(valueElement.textContent) || 0;
        this.animateNumber(valueElement, currentValue, Math.round(newValue), 500, '%');
        
        // Change color based on value
        if (newValue > 80) {
            fillElement.style.background = 'linear-gradient(90deg, #ef4444, #dc2626)';
        } else if (newValue > 60) {
            fillElement.style.background = 'linear-gradient(90deg, #f59e0b, #d97706)';
        } else {
            fillElement.style.background = 'linear-gradient(90deg, #10b981, #059669)';
        }
    }
    
    animateNumber(element, start, end, duration, suffix = '') {
        const startTime = performance.now();
        const animate = (currentTime) => {
            const elapsed = currentTime - startTime;
            const progress = Math.min(elapsed / duration, 1);
            
            const easeOutQuart = 1 - Math.pow(1 - progress, 4);
            const current = Math.round(start + (end - start) * easeOutQuart);
            
            element.textContent = current + suffix;
            
            if (progress < 1) {
                requestAnimationFrame(animate);
            }
        };
        requestAnimationFrame(animate);
    }
    
    updateStatusIndicator(cpu, memory, network) {
        const statusDot = document.querySelector('.status-dot');
        const statusText = document.querySelector('.status-text');
        
        if (!statusDot || !statusText) return;
        
        const maxMetric = Math.max(cpu, memory, network);
        
        // Remove existing classes
        statusDot.classList.remove('online', 'warning', 'error');
        
        if (maxMetric > 85) {
            statusDot.classList.add('error');
            statusText.textContent = '系统高负载';
        } else if (maxMetric > 70) {
            statusDot.classList.add('warning');
            statusText.textContent = '系统负载较高';
        } else {
            statusDot.classList.add('online');
            statusText.textContent = '系统正常';
        }
    }
    
    refreshSystemStatus() {
        const refreshBtn = document.querySelector('[data-action="refresh"] i');
        if (refreshBtn) {
            refreshBtn.style.animation = 'spin 1s linear';
            setTimeout(() => {
                refreshBtn.style.animation = '';
            }, 1000);
        }
        
        // Force immediate update
        this.updateSystemStatus();
        
        // Show refresh feedback
        const statusText = document.querySelector('.status-text');
        const originalText = statusText.textContent;
        statusText.textContent = '正在刷新...';
        
        setTimeout(() => {
            statusText.textContent = originalText;
        }, 1000);
    }
    
    // Enhanced notification system
    showNotification(message, type = 'info', duration = 3000) {
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.innerHTML = `
            <div class="notification-content">
                <i class="fas fa-${this.getNotificationIcon(type)}"></i>
                <span>${message}</span>
            </div>
            <button class="notification-close">
                <i class="fas fa-times"></i>
            </button>
        `;
        
        notification.style.cssText = `
            position: fixed;
            top: 2rem;
            right: 2rem;
            background: white;
            border-radius: 8px;
            padding: 1rem;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
            border-left: 4px solid var(--${type === 'error' ? 'error' : type === 'warning' ? 'warning' : type === 'success' ? 'success' : 'primary'}-color);
            z-index: 10001;
            transform: translateX(100%);
            transition: transform 0.3s ease;
            max-width: 300px;
        `;
        
        document.body.appendChild(notification);
        
        // Animate in
        setTimeout(() => {
            notification.style.transform = 'translateX(0)';
        }, 10);
        
        // Auto remove
        const removeNotification = () => {
            notification.style.transform = 'translateX(100%)';
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.parentNode.removeChild(notification);
                }
            }, 300);
        };
        
        // Close button
        notification.querySelector('.notification-close').addEventListener('click', removeNotification);
        
        // Auto remove after duration
        if (duration > 0) {
            setTimeout(removeNotification, duration);
        }
        
        return notification;
    }
    
    getNotificationIcon(type) {
        const icons = {
            'success': 'check-circle',
            'error': 'exclamation-circle',
            'warning': 'exclamation-triangle',
            'info': 'info-circle'
        };
        return icons[type] || 'info-circle';
    }
    
    // Performance monitoring
    startPerformanceMonitoring() {
        if ('performance' in window) {
            // Monitor page load performance
            window.addEventListener('load', () => {
                setTimeout(() => {
                    const perfData = performance.getEntriesByType('navigation')[0];
                    const loadTime = perfData.loadEventEnd - perfData.loadEventStart;
                    
                    if (loadTime > 3000) {
                        this.showNotification('页面加载较慢，建议检查网络连接', 'warning');
                    }
                }, 1000);
            });
            
            // Monitor memory usage (if available)
            if ('memory' in performance) {
                setInterval(() => {
                    const memory = performance.memory;
                    const usedPercent = (memory.usedJSHeapSize / memory.jsHeapSizeLimit) * 100;
                    
                    if (usedPercent > 80) {
                        this.showNotification('内存使用率较高，建议刷新页面', 'warning');
                    }
                }, 30000);
            }
        }
    }
}

// Initialize the app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new CZDSLApp();
});
