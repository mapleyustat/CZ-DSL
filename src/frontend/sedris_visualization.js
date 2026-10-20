// SEDRIS动态场景可视化组件
// Dynamic Coastal Scenario Visualization with SEDRIS Integration

class SedrisVisualization {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.scene = null;
        this.camera = null;
        this.renderer = null;
        this.controls = null;
        this.animationId = null;
        
        // 时间控制
        this.currentTime = new Date('2023-10-01T00:00:00Z');
        this.startTime = new Date('2023-10-01T00:00:00Z');
        this.endTime = new Date('2023-10-14T23:59:59Z');
        this.timeStep = 3 * 60 * 60 * 1000; // 3小时
        this.isPlaying = false;
        
        // 数据层
        this.layers = {
            bathymetry: null,
            seaSurfaceTemperature: null,
            waveField: null,
            windVectors: null,
            typhoonTrack: null,
            vegetation: null,
            sedimentPlume: null
        };
        
        // 场景数据
        this.sceneData = null;
        
        this.init();
    }
    
    init() {
        this.setupThreeJS();
        this.setupControls();
        this.setupLighting();
        this.loadSceneData();
        this.setupUI();
        this.animate();
    }
    
    setupThreeJS() {
        // 创建场景
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0x87CEEB); // 天蓝色
        
        // 创建相机
        this.camera = new THREE.PerspectiveCamera(
            60, 
            this.container.clientWidth / this.container.clientHeight, 
            1, 
            100000
        );
        this.camera.position.set(121.5, 31.2, 5000);
        this.camera.lookAt(121.5, 31.2, 0);
        
        // 创建渲染器
        this.renderer = new THREE.WebGLRenderer({ antialias: true });
        this.renderer.setSize(this.container.clientWidth, this.container.clientHeight);
        this.renderer.shadowMap.enabled = true;
        this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
        this.container.appendChild(this.renderer.domElement);
        
        // 响应式调整
        window.addEventListener('resize', () => this.onWindowResize());
    }
    
    setupControls() {
        // 轨道控制器
        this.controls = new THREE.OrbitControls(this.camera, this.renderer.domElement);
        this.controls.enableDamping = true;
        this.controls.dampingFactor = 0.05;
        this.controls.minDistance = 100;
        this.controls.maxDistance = 50000;
    }
    
    setupLighting() {
        // 环境光
        const ambientLight = new THREE.AmbientLight(0x404040, 0.4);
        this.scene.add(ambientLight);
        
        // 太阳光
        const sunLight = new THREE.DirectionalLight(0xffffff, 0.8);
        sunLight.position.set(1000, 2000, 1000);
        sunLight.castShadow = true;
        sunLight.shadow.mapSize.width = 2048;
        sunLight.shadow.mapSize.height = 2048;
        this.scene.add(sunLight);
        
        // 海洋反射光
        const oceanLight = new THREE.HemisphereLight(0x87CEEB, 0x006994, 0.3);
        this.scene.add(oceanLight);
    }
    
    async loadSceneData() {
        try {
            // 模拟加载SEDRIS转换后的场景数据
            this.sceneData = await this.loadSedrisData();
            this.createLayers();
        } catch (error) {
            console.error('Failed to load scene data:', error);
        }
    }
    
    async loadSedrisData() {
        // 模拟SEDRIS数据加载
        return {
            spatialExtent: {
                minX: 120.8, maxX: 122.2,
                minY: 30.7, maxY: 31.8,
                minZ: -50.0, maxZ: 15.0
            },
            
            // 地形数据
            bathymetry: this.generateBathymetryData(),
            
            // 海表温度时间序列
            seaSurfaceTemperature: this.generateSSTData(),
            
            // 波浪场数据
            waveField: this.generateWaveData(),
            
            // 风场数据
            windField: this.generateWindData(),
            
            // 台风路径
            typhoonTrack: this.generateTyphoonTrack(),
            
            // 植被覆盖度
            vegetation: this.generateVegetationData(),
            
            // 沉积物浓度
            sedimentConcentration: this.generateSedimentData()
        };
    }
    
    generateBathymetryData() {
        const data = [];
        const resolution = 0.01; // 0.01度分辨率
        
        for (let x = 120.8; x <= 122.2; x += resolution) {
            for (let y = 30.7; y <= 31.8; y += resolution) {
                // 模拟长江口地形
                const distanceFromMouth = Math.sqrt(
                    Math.pow((x - 121.5) * 111000, 2) + 
                    Math.pow((y - 31.2) * 111000, 2)
                );
                
                let depth = -5 - distanceFromMouth / 2000; // 向海逐渐加深
                
                // 添加河道
                if (Math.abs(y - 31.2) < 0.05 && x < 121.5) {
                    depth = Math.min(depth, -15);
                }
                
                // 添加浅滩
                if (x > 121.8 && y > 31.5) {
                    depth = Math.max(depth, -2);
                }
                
                data.push({ x, y, z: depth });
            }
        }
        
        return data;
    }
    
    generateSSTData() {
        const timeSteps = [];
        const currentTime = new Date(this.startTime);
        
        while (currentTime <= this.endTime) {
            const timeData = {
                time: new Date(currentTime),
                data: []
            };
            
            // 计算台风影响
            const daysSinceStart = (currentTime - this.startTime) / (1000 * 60 * 60 * 24);
            const typhoonEffect = this.calculateTyphoonEffect(daysSinceStart);
            
            for (let x = 120.8; x <= 122.2; x += 0.02) {
                for (let y = 30.7; y <= 31.8; y += 0.02) {
                    // 基础温度 + 台风影响
                    let temperature = 24.5 - typhoonEffect * 3.0;
                    
                    // 添加空间变化
                    temperature += Math.sin((x - 121) * 5) * 0.5;
                    temperature += Math.cos((y - 31) * 5) * 0.3;
                    
                    timeData.data.push({ x, y, value: temperature });
                }
            }
            
            timeSteps.push(timeData);
            currentTime.setHours(currentTime.getHours() + 3);
        }
        
        return timeSteps;
    }
    
    generateWaveData() {
        const timeSteps = [];
        const currentTime = new Date(this.startTime);
        
        while (currentTime <= this.endTime) {
            const timeData = {
                time: new Date(currentTime),
                data: []
            };
            
            const daysSinceStart = (currentTime - this.startTime) / (1000 * 60 * 60 * 24);
            const typhoonEffect = this.calculateTyphoonEffect(daysSinceStart);
            
            for (let x = 120.8; x <= 122.2; x += 0.02) {
                for (let y = 30.7; y <= 31.8; y += 0.02) {
                    // 基础波高 + 台风影响
                    let waveHeight = 1.2 + typhoonEffect * 7.0;
                    
                    // 近岸衰减
                    const distanceFromShore = this.calculateDistanceFromShore(x, y);
                    waveHeight *= Math.min(1.0, distanceFromShore / 10000);
                    
                    timeData.data.push({ x, y, value: waveHeight });
                }
            }
            
            timeSteps.push(timeData);
            currentTime.setHours(currentTime.getHours() + 3);
        }
        
        return timeSteps;
    }
    
    generateWindData() {
        const timeSteps = [];
        const currentTime = new Date(this.startTime);
        
        while (currentTime <= this.endTime) {
            const timeData = {
                time: new Date(currentTime),
                data: []
            };
            
            const daysSinceStart = (currentTime - this.startTime) / (1000 * 60 * 60 * 24);
            const typhoonCenter = this.getTyphoonPosition(daysSinceStart);
            const typhoonIntensity = this.calculateTyphoonEffect(daysSinceStart);
            
            for (let x = 120.8; x <= 122.2; x += 0.05) {
                for (let y = 30.7; y <= 31.8; y += 0.05) {
                    let windU = 5.0; // 基础东风
                    let windV = 2.0; // 基础北风
                    
                    if (typhoonIntensity > 0.1) {
                        // 台风风场模型
                        const dx = (x - typhoonCenter.x) * 111000;
                        const dy = (y - typhoonCenter.y) * 111000;
                        const distance = Math.sqrt(dx * dx + dy * dy);
                        
                        if (distance < 300000) { // 300km范围内
                            const angle = Math.atan2(dy, dx);
                            const windSpeed = 45 * typhoonIntensity * Math.exp(-distance / 100000);
                            
                            // 逆时针旋转风场
                            windU = -windSpeed * Math.sin(angle);
                            windV = windSpeed * Math.cos(angle);
                        }
                    }
                    
                    timeData.data.push({ x, y, u: windU, v: windV });
                }
            }
            
            timeSteps.push(timeData);
            currentTime.setHours(currentTime.getHours() + 3);
        }
        
        return timeSteps;
    }
    
    generateTyphoonTrack() {
        return [
            { time: '2023-10-05T00:00:00Z', x: 125.0, y: 28.0, intensity: 0.6 },
            { time: '2023-10-06T12:00:00Z', x: 123.5, y: 29.5, intensity: 0.8 },
            { time: '2023-10-07T12:00:00Z', x: 122.0, y: 31.0, intensity: 1.0 },
            { time: '2023-10-08T12:00:00Z', x: 120.5, y: 32.5, intensity: 0.7 },
            { time: '2023-10-09T00:00:00Z', x: 119.0, y: 34.0, intensity: 0.4 }
        ];
    }
    
    generateVegetationData() {
        const timeSteps = [];
        const currentTime = new Date(this.startTime);
        
        while (currentTime <= this.endTime) {
            const timeData = {
                time: new Date(currentTime),
                data: []
            };
            
            const daysSinceStart = (currentTime - this.startTime) / (1000 * 60 * 60 * 24);
            const typhoonDamage = this.calculateVegetationDamage(daysSinceStart);
            
            // 崇明东滩湿地区域
            for (let x = 121.8; x <= 122.1; x += 0.01) {
                for (let y = 31.5; y <= 31.7; y += 0.01) {
                    let coverage = 0.85; // 基础覆盖度85%
                    
                    // 台风损害
                    coverage *= (1 - typhoonDamage);
                    
                    // 恢复过程
                    if (daysSinceStart > 7) {
                        const recoveryDays = daysSinceStart - 7;
                        const recoveryRate = 0.02; // 每天2%恢复
                        coverage = Math.min(0.85, coverage + recoveryRate * recoveryDays);
                    }
                    
                    timeData.data.push({ x, y, value: coverage });
                }
            }
            
            timeSteps.push(timeData);
            currentTime.setDate(currentTime.getDate() + 1);
        }
        
        return timeSteps;
    }
    
    generateSedimentData() {
        const timeSteps = [];
        const currentTime = new Date(this.startTime);
        
        while (currentTime <= this.endTime) {
            const timeData = {
                time: new Date(currentTime),
                data: []
            };
            
            const daysSinceStart = (currentTime - this.startTime) / (1000 * 60 * 60 * 24);
            const typhoonEffect = this.calculateTyphoonEffect(daysSinceStart);
            
            for (let x = 120.8; x <= 122.2; x += 0.02) {
                for (let y = 30.7; y <= 31.8; y += 0.02) {
                    let concentration = 0.3; // 基础浓度
                    
                    // 台风期间大幅增加
                    concentration += typhoonEffect * 2.2;
                    
                    // 河口附近浓度更高
                    const distanceFromMouth = Math.sqrt(
                        Math.pow((x - 121.5) * 111000, 2) + 
                        Math.pow((y - 31.2) * 111000, 2)
                    );
                    concentration *= Math.exp(-distanceFromMouth / 20000);
                    
                    timeData.data.push({ x, y, value: concentration });
                }
            }
            
            timeSteps.push(timeData);
            currentTime.setHours(currentTime.getHours() + 6);
        }
        
        return timeSteps;
    }
    
    createLayers() {
        this.createBathymetryLayer();
        this.createSeaSurfaceTemperatureLayer();
        this.createWaveFieldLayer();
        this.createWindVectorLayer();
        this.createTyphoonTrackLayer();
        this.createVegetationLayer();
        this.createSedimentPlumeLayer();
    }
    
    createBathymetryLayer() {
        const geometry = new THREE.PlaneGeometry(
            (this.sceneData.spatialExtent.maxX - this.sceneData.spatialExtent.minX) * 111000,
            (this.sceneData.spatialExtent.maxY - this.sceneData.spatialExtent.minY) * 111000,
            100, 100
        );
        
        // 应用地形高度
        const vertices = geometry.attributes.position.array;
        for (let i = 0; i < vertices.length; i += 3) {
            const x = vertices[i] / 111000 + 121.5;
            const y = vertices[i + 1] / 111000 + 31.25;
            
            // 查找对应的深度值
            const bathymetryPoint = this.sceneData.bathymetry.find(p => 
                Math.abs(p.x - x) < 0.01 && Math.abs(p.y - y) < 0.01
            );
            
            if (bathymetryPoint) {
                vertices[i + 2] = bathymetryPoint.z * 100; // 放大Z轴
            }
        }
        
        geometry.attributes.position.needsUpdate = true;
        geometry.computeVertexNormals();
        
        const material = new THREE.MeshLambertMaterial({
            color: 0x8B7355,
            wireframe: false
        });
        
        this.layers.bathymetry = new THREE.Mesh(geometry, material);
        this.layers.bathymetry.position.set(0, 0, 0);
        this.scene.add(this.layers.bathymetry);
    }
    
    createSeaSurfaceTemperatureLayer() {
        // 创建海表温度可视化层
        const canvas = document.createElement('canvas');
        canvas.width = 128;
        canvas.height = 128;
        const context = canvas.getContext('2d');
        
        const texture = new THREE.CanvasTexture(canvas);
        
        const geometry = new THREE.PlaneGeometry(
            (this.sceneData.spatialExtent.maxX - this.sceneData.spatialExtent.minX) * 111000,
            (this.sceneData.spatialExtent.maxY - this.sceneData.spatialExtent.minY) * 111000
        );
        
        const material = new THREE.MeshBasicMaterial({
            map: texture,
            transparent: true,
            opacity: 0.7
        });
        
        this.layers.seaSurfaceTemperature = new THREE.Mesh(geometry, material);
        this.layers.seaSurfaceTemperature.position.set(0, 0, 50);
        this.scene.add(this.layers.seaSurfaceTemperature);
        
        // 存储canvas和context用于更新
        this.layers.seaSurfaceTemperature.userData = { canvas, context, texture };
    }
    
    createWaveFieldLayer() {
        // 创建波浪场可视化
        const waveGeometry = new THREE.PlaneGeometry(
            (this.sceneData.spatialExtent.maxX - this.sceneData.spatialExtent.minX) * 111000,
            (this.sceneData.spatialExtent.maxY - this.sceneData.spatialExtent.minY) * 111000,
            64, 64
        );
        
        const waveMaterial = new THREE.MeshPhongMaterial({
            color: 0x006994,
            transparent: true,
            opacity: 0.6,
            shininess: 100
        });
        
        this.layers.waveField = new THREE.Mesh(waveGeometry, waveMaterial);
        this.layers.waveField.position.set(0, 0, 100);
        this.scene.add(this.layers.waveField);
    }
    
    createWindVectorLayer() {
        // 创建风场矢量可视化
        this.layers.windVectors = new THREE.Group();
        
        const arrowGeometry = new THREE.ConeGeometry(500, 2000, 8);
        const arrowMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });
        
        // 创建风矢量箭头
        for (let x = 121.0; x <= 122.0; x += 0.1) {
            for (let y = 30.8; y <= 31.6; y += 0.1) {
                const arrow = new THREE.Mesh(arrowGeometry, arrowMaterial);
                arrow.position.set(
                    (x - 121.5) * 111000,
                    (y - 31.25) * 111000,
                    1000
                );
                arrow.userData = { x, y };
                this.layers.windVectors.add(arrow);
            }
        }
        
        this.scene.add(this.layers.windVectors);
    }
    
    createTyphoonTrackLayer() {
        // 创建台风路径可视化
        const trackGeometry = new THREE.BufferGeometry();
        const trackPoints = [];
        
        this.sceneData.typhoonTrack.forEach(point => {
            trackPoints.push(
                (point.x - 121.5) * 111000,
                (point.y - 31.25) * 111000,
                2000
            );
        });
        
        trackGeometry.setAttribute('position', new THREE.Float32BufferAttribute(trackPoints, 3));
        
        const trackMaterial = new THREE.LineBasicMaterial({
            color: 0xff0000,
            linewidth: 5
        });
        
        this.layers.typhoonTrack = new THREE.Line(trackGeometry, trackMaterial);
        this.scene.add(this.layers.typhoonTrack);
        
        // 添加台风眼标记
        const eyeGeometry = new THREE.SphereGeometry(2000, 16, 16);
        const eyeMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });
        this.layers.typhoonEye = new THREE.Mesh(eyeGeometry, eyeMaterial);
        this.scene.add(this.layers.typhoonEye);
    }
    
    createVegetationLayer() {
        // 创建植被覆盖度可视化
        const canvas = document.createElement('canvas');
        canvas.width = 64;
        canvas.height = 64;
        const context = canvas.getContext('2d');
        
        const texture = new THREE.CanvasTexture(canvas);
        
        const geometry = new THREE.PlaneGeometry(30000, 20000); // 崇明东滩区域
        const material = new THREE.MeshBasicMaterial({
            map: texture,
            transparent: true,
            opacity: 0.8
        });
        
        this.layers.vegetation = new THREE.Mesh(geometry, material);
        this.layers.vegetation.position.set(
            (121.95 - 121.5) * 111000,
            (31.6 - 31.25) * 111000,
            10
        );
        this.scene.add(this.layers.vegetation);
        
        this.layers.vegetation.userData = { canvas, context, texture };
    }
    
    createSedimentPlumeLayer() {
        // 创建沉积物羽流可视化
        const canvas = document.createElement('canvas');
        canvas.width = 128;
        canvas.height = 128;
        const context = canvas.getContext('2d');
        
        const texture = new THREE.CanvasTexture(canvas);
        
        const geometry = new THREE.PlaneGeometry(
            (this.sceneData.spatialExtent.maxX - this.sceneData.spatialExtent.minX) * 111000,
            (this.sceneData.spatialExtent.maxY - this.sceneData.spatialExtent.minY) * 111000
        );
        
        const material = new THREE.MeshBasicMaterial({
            map: texture,
            transparent: true,
            opacity: 0.4
        });
        
        this.layers.sedimentPlume = new THREE.Mesh(geometry, material);
        this.layers.sedimentPlume.position.set(0, 0, 25);
        this.scene.add(this.layers.sedimentPlume);
        
        this.layers.sedimentPlume.userData = { canvas, context, texture };
    }
    
    setupUI() {
        // 创建控制面板
        const controlPanel = document.createElement('div');
        controlPanel.className = 'sedris-control-panel';
        controlPanel.innerHTML = `
            <div class="time-controls">
                <h3>🕐 时间控制</h3>
                <div class="time-display">
                    <span id="current-time">${this.formatTime(this.currentTime)}</span>
                </div>
                <div class="time-slider-container">
                    <input type="range" id="time-slider" min="0" max="100" value="0" class="time-slider">
                </div>
                <div class="playback-controls">
                    <button id="play-pause-btn" class="control-btn">▶️ 播放</button>
                    <button id="reset-btn" class="control-btn">⏮️ 重置</button>
                    <select id="speed-select" class="speed-select">
                        <option value="1">1x</option>
                        <option value="2">2x</option>
                        <option value="4" selected>4x</option>
                        <option value="8">8x</option>
                    </select>
                </div>
            </div>
            
            <div class="layer-controls">
                <h3>🗂️ 图层控制</h3>
                <div class="layer-toggles">
                    <label><input type="checkbox" id="bathymetry-toggle" checked> 🗺️ 地形</label>
                    <label><input type="checkbox" id="sst-toggle" checked> 🌡️ 海表温度</label>
                    <label><input type="checkbox" id="wave-toggle" checked> 🌊 波浪场</label>
                    <label><input type="checkbox" id="wind-toggle" checked> 💨 风场</label>
                    <label><input type="checkbox" id="typhoon-toggle" checked> 🌀 台风路径</label>
                    <label><input type="checkbox" id="vegetation-toggle" checked> 🌱 植被</label>
                    <label><input type="checkbox" id="sediment-toggle" checked> 🏔️ 沉积物</label>
                </div>
            </div>
            
            <div class="info-panel">
                <h3>📊 实时信息</h3>
                <div id="info-content">
                    <div>台风强度: <span id="typhoon-intensity">-</span></div>
                    <div>最大波高: <span id="max-wave-height">-</span></div>
                    <div>平均海温: <span id="avg-sst">-</span></div>
                    <div>植被覆盖: <span id="vegetation-coverage">-</span></div>
                </div>
            </div>
            
            <div class="analysis-tools">
                <h3>🔍 分析工具</h3>
                <button id="cross-section-btn" class="tool-btn">📏 剖面分析</button>
                <button id="time-series-btn" class="tool-btn">📈 时间序列</button>
                <button id="statistics-btn" class="tool-btn">📊 统计分析</button>
                <button id="export-btn" class="tool-btn">💾 导出数据</button>
            </div>
        `;
        
        this.container.appendChild(controlPanel);
        
        // 绑定事件
        this.bindUIEvents();
    }
    
    bindUIEvents() {
        // 播放/暂停按钮
        document.getElementById('play-pause-btn').addEventListener('click', () => {
            this.togglePlayback();
        });
        
        // 重置按钮
        document.getElementById('reset-btn').addEventListener('click', () => {
            this.resetAnimation();
        });
        
        // 时间滑块
        document.getElementById('time-slider').addEventListener('input', (e) => {
            const progress = parseFloat(e.target.value) / 100;
            this.setTimeProgress(progress);
        });
        
        // 图层切换
        const layerToggles = {
            'bathymetry-toggle': 'bathymetry',
            'sst-toggle': 'seaSurfaceTemperature',
            'wave-toggle': 'waveField',
            'wind-toggle': 'windVectors',
            'typhoon-toggle': 'typhoonTrack',
            'vegetation-toggle': 'vegetation',
            'sediment-toggle': 'sedimentPlume'
        };
        
        Object.entries(layerToggles).forEach(([toggleId, layerName]) => {
            document.getElementById(toggleId).addEventListener('change', (e) => {
                this.toggleLayer(layerName, e.target.checked);
            });
        });
        
        // 分析工具
        document.getElementById('cross-section-btn').addEventListener('click', () => {
            this.showCrossSection();
        });
        
        document.getElementById('time-series-btn').addEventListener('click', () => {
            this.showTimeSeries();
        });
        
        document.getElementById('statistics-btn').addEventListener('click', () => {
            this.showStatistics();
        });
        
        document.getElementById('export-btn').addEventListener('click', () => {
            this.exportData();
        });
    }
    
    animate() {
        this.animationId = requestAnimationFrame(() => this.animate());
        
        // 更新控制器
        this.controls.update();
        
        // 更新时间动画
        if (this.isPlaying) {
            this.updateTimeAnimation();
        }
        
        // 更新图层
        this.updateLayers();
        
        // 渲染场景
        this.renderer.render(this.scene, this.camera);
    }
    
    updateTimeAnimation() {
        const speed = parseInt(document.getElementById('speed-select').value);
        this.currentTime = new Date(this.currentTime.getTime() + this.timeStep * speed);
        
        if (this.currentTime > this.endTime) {
            this.currentTime = new Date(this.endTime);
            this.isPlaying = false;
            document.getElementById('play-pause-btn').textContent = '▶️ 播放';
        }
        
        this.updateTimeDisplay();
        this.updateTimeSlider();
    }
    
    updateLayers() {
        this.updateSeaSurfaceTemperatureLayer();
        this.updateWaveFieldLayer();
        this.updateWindVectorLayer();
        this.updateTyphoonPosition();
        this.updateVegetationLayer();
        this.updateSedimentPlumeLayer();
        this.updateInfoPanel();
    }
    
    updateSeaSurfaceTemperatureLayer() {
        if (!this.layers.seaSurfaceTemperature || !this.layers.seaSurfaceTemperature.visible) return;
        
        const timeData = this.getDataAtTime(this.sceneData.seaSurfaceTemperature, this.currentTime);
        if (!timeData) return;
        
        const { canvas, context, texture } = this.layers.seaSurfaceTemperature.userData;
        
        // 清空画布
        context.clearRect(0, 0, canvas.width, canvas.height);
        
        // 绘制温度场
        const imageData = context.createImageData(canvas.width, canvas.height);
        
        timeData.data.forEach(point => {
            const x = Math.floor((point.x - 120.8) / (122.2 - 120.8) * canvas.width);
            const y = Math.floor((point.y - 30.7) / (31.8 - 30.7) * canvas.height);
            
            if (x >= 0 && x < canvas.width && y >= 0 && y < canvas.height) {
                const index = (y * canvas.width + x) * 4;
                const color = this.temperatureToColor(point.value);
                
                imageData.data[index] = color.r;
                imageData.data[index + 1] = color.g;
                imageData.data[index + 2] = color.b;
                imageData.data[index + 3] = 180; // 透明度
            }
        });
        
        context.putImageData(imageData, 0, 0);
        texture.needsUpdate = true;
    }
    
    updateWaveFieldLayer() {
        if (!this.layers.waveField || !this.layers.waveField.visible) return;
        
        const timeData = this.getDataAtTime(this.sceneData.waveField, this.currentTime);
        if (!timeData) return;
        
        // 更新波浪场几何
        const geometry = this.layers.waveField.geometry;
        const vertices = geometry.attributes.position.array;
        
        // 应用波浪高度
        for (let i = 0; i < vertices.length; i += 3) {
            const x = vertices[i] / 111000 + 121.5;
            const y = vertices[i + 1] / 111000 + 31.25;
            
            // 查找对应的波高值
            const wavePoint = timeData.data.find(p => 
                Math.abs(p.x - x) < 0.02 && Math.abs(p.y - y) < 0.02
            );
            
            if (wavePoint) {
                vertices[i + 2] = wavePoint.value * 50; // 放大波高显示
            }
        }
        
        geometry.attributes.position.needsUpdate = true;
        geometry.computeVertexNormals();
    }
    
    updateTyphoonPosition() {
        if (!this.layers.typhoonEye || !this.layers.typhoonEye.visible) return;
        
        const daysSinceStart = (this.currentTime - this.startTime) / (1000 * 60 * 60 * 24);
        const typhoonPos = this.getTyphoonPosition(daysSinceStart);
        
        if (typhoonPos) {
            this.layers.typhoonEye.position.set(
                (typhoonPos.x - 121.5) * 111000,
                (typhoonPos.y - 31.25) * 111000,
                2000
            );
        }
    }
    
    updateVegetationLayer() {
        if (!this.layers.vegetation || !this.layers.vegetation.visible) return;
        
        const timeData = this.getDataAtTime(this.sceneData.vegetation, this.currentTime);
        if (!timeData) return;
        
        const { canvas, context, texture } = this.layers.vegetation.userData;
        
        // 清空画布
        context.clearRect(0, 0, canvas.width, canvas.height);
        
        // 绘制植被覆盖度
        const imageData = context.createImageData(canvas.width, canvas.height);
        
        timeData.data.forEach(point => {
            const x = Math.floor((point.x - 121.8) / (122.1 - 121.8) * canvas.width);
            const y = Math.floor((point.y - 31.5) / (31.7 - 31.5) * canvas.height);
            
            if (x >= 0 && x < canvas.width && y >= 0 && y < canvas.height) {
                const index = (y * canvas.width + x) * 4;
                const green = Math.floor(point.value * 255);
                
                imageData.data[index] = Math.floor(green * 0.3);     // R
                imageData.data[index + 1] = green;                   // G
                imageData.data[index + 2] = Math.floor(green * 0.2); // B
                imageData.data[index + 3] = 200;                     // A
            }
        });
        
        context.putImageData(imageData, 0, 0);
        texture.needsUpdate = true;
    }
    
    updateSedimentPlumeLayer() {
        if (!this.layers.sedimentPlume || !this.layers.sedimentPlume.visible) return;
        
        const timeData = this.getDataAtTime(this.sceneData.sedimentConcentration, this.currentTime);
        if (!timeData) return;
        
        const { canvas, context, texture } = this.layers.sedimentPlume.userData;
        
        // 清空画布
        context.clearRect(0, 0, canvas.width, canvas.height);
        
        // 绘制沉积物浓度
        const imageData = context.createImageData(canvas.width, canvas.height);
        
        timeData.data.forEach(point => {
            const x = Math.floor((point.x - 120.8) / (122.2 - 120.8) * canvas.width);
            const y = Math.floor((point.y - 30.7) / (31.8 - 30.7) * canvas.height);
            
            if (x >= 0 && x < canvas.width && y >= 0 && y < canvas.height) {
                const index = (y * canvas.width + x) * 4;
                const intensity = Math.min(point.value / 3.0, 1.0); // 归一化到0-1
                const brown = Math.floor(intensity * 255);
                
                imageData.data[index] = Math.floor(brown * 0.8);     // R
                imageData.data[index + 1] = Math.floor(brown * 0.6); // G
                imageData.data[index + 2] = Math.floor(brown * 0.3); // B
                imageData.data[index + 3] = Math.floor(intensity * 150); // A
            }
        });
        
        context.putImageData(imageData, 0, 0);
        texture.needsUpdate = true;
    }
    
    updateInfoPanel() {
        // 更新实时信息面板
        const daysSinceStart = (this.currentTime - this.startTime) / (1000 * 60 * 60 * 24);
        const typhoonIntensity = this.calculateTyphoonEffect(daysSinceStart);
        
        // 台风强度
        const intensityElement = document.getElementById('typhoon-intensity');
        if (intensityElement) {
            if (typhoonIntensity > 0.1) {
                intensityElement.textContent = `${Math.floor(typhoonIntensity * 100)}%`;
                intensityElement.style.color = typhoonIntensity > 0.8 ? '#ef4444' : '#f59e0b';
            } else {
                intensityElement.textContent = '无影响';
                intensityElement.style.color = '#10b981';
            }
        }
        
        // 最大波高
        const waveData = this.getDataAtTime(this.sceneData.waveField, this.currentTime);
        const maxWaveElement = document.getElementById('max-wave-height');
        if (maxWaveElement && waveData) {
            const maxWave = Math.max(...waveData.data.map(p => p.value));
            maxWaveElement.textContent = `${maxWave.toFixed(1)}m`;
            maxWaveElement.style.color = maxWave > 5 ? '#ef4444' : '#10b981';
        }
        
        // 平均海温
        const sstData = this.getDataAtTime(this.sceneData.seaSurfaceTemperature, this.currentTime);
        const avgSstElement = document.getElementById('avg-sst');
        if (avgSstElement && sstData) {
            const avgSST = sstData.data.reduce((sum, p) => sum + p.value, 0) / sstData.data.length;
            avgSstElement.textContent = `${avgSST.toFixed(1)}°C`;
        }
        
        // 植被覆盖
        const vegData = this.getDataAtTime(this.sceneData.vegetation, this.currentTime);
        const vegElement = document.getElementById('vegetation-coverage');
        if (vegElement && vegData) {
            const avgCoverage = vegData.data.reduce((sum, p) => sum + p.value, 0) / vegData.data.length;
            vegElement.textContent = `${Math.floor(avgCoverage * 100)}%`;
            vegElement.style.color = avgCoverage > 0.7 ? '#10b981' : '#f59e0b';
        }
    }
    
    // 辅助计算函数
    calculateDistanceFromShore(x, y) {
        // 简化的距岸距离计算
        const coastlinePoints = [
            { x: 121.0, y: 31.0 },
            { x: 121.5, y: 31.2 },
            { x: 122.0, y: 31.4 }
        ];
        
        let minDistance = Infinity;
        coastlinePoints.forEach(point => {
            const distance = Math.sqrt(
                Math.pow((x - point.x) * 111000, 2) + 
                Math.pow((y - point.y) * 111000, 2)
            );
            minDistance = Math.min(minDistance, distance);
        });
        
        return minDistance;
    }
    
    calculateVegetationDamage(daysSinceStart) {
        // 台风对植被的损害计算
        if (daysSinceStart < 6) return 0;
        if (daysSinceStart < 8) {
            const typhoonEffect = this.calculateTyphoonEffect(daysSinceStart);
            return typhoonEffect * 0.4; // 最大40%损害
        }
        return 0;
    }
    
    // 辅助函数
    calculateTyphoonEffect(daysSinceStart) {
        // 台风影响强度随时间变化
        if (daysSinceStart < 4) return 0;
        if (daysSinceStart < 6) return (daysSinceStart - 4) / 2;
        if (daysSinceStart < 8) return 1 - (daysSinceStart - 6) / 2;
        return Math.max(0, 0.5 - (daysSinceStart - 8) / 10);
    }
    
    getTyphoonPosition(daysSinceStart) {
        const track = this.sceneData.typhoonTrack;
        
        for (let i = 0; i < track.length - 1; i++) {
            const startTime = new Date(track[i].time);
            const endTime = new Date(track[i + 1].time);
            const startDay = (startTime - this.startTime) / (1000 * 60 * 60 * 24);
            const endDay = (endTime - this.startTime) / (1000 * 60 * 60 * 24);
            
            if (daysSinceStart >= startDay && daysSinceStart <= endDay) {
                const progress = (daysSinceStart - startDay) / (endDay - startDay);
                return {
                    x: track[i].x + (track[i + 1].x - track[i].x) * progress,
                    y: track[i].y + (track[i + 1].y - track[i].y) * progress
                };
            }
        }
        
        return track[0];
    }
    
    temperatureToColor(temperature) {
        // 温度到颜色的映射 (18-26°C)
        const normalized = (temperature - 18) / 8;
        const hue = (1 - normalized) * 0.7; // 从红到蓝
        
        const c = 1;
        const x = c * (1 - Math.abs((hue * 6) % 2 - 1));
        const m = 0;
        
        let r, g, b;
        if (hue < 1/6) { r = c; g = x; b = 0; }
        else if (hue < 2/6) { r = x; g = c; b = 0; }
        else if (hue < 3/6) { r = 0; g = c; b = x; }
        else if (hue < 4/6) { r = 0; g = x; b = c; }
        else if (hue < 5/6) { r = x; g = 0; b = c; }
        else { r = c; g = 0; b = x; }
        
        return {
            r: Math.round((r + m) * 255),
            g: Math.round((g + m) * 255),
            b: Math.round((b + m) * 255)
        };
    }
    
    getDataAtTime(timeSeriesData, targetTime) {
        // 在时间序列数据中查找最接近的时间点
        let closestData = null;
        let minTimeDiff = Infinity;
        
        timeSeriesData.forEach(data => {
            const timeDiff = Math.abs(new Date(data.time) - targetTime);
            if (timeDiff < minTimeDiff) {
                minTimeDiff = timeDiff;
                closestData = data;
            }
        });
        
        return closestData;
    }
    
    formatTime(date) {
        return date.toISOString().replace('T', ' ').substring(0, 16);
    }
    
    togglePlayback() {
        this.isPlaying = !this.isPlaying;
        document.getElementById('play-pause-btn').textContent = this.isPlaying ? '⏸️ 暂停' : '▶️ 播放';
    }
    
    resetAnimation() {
        this.currentTime = new Date(this.startTime);
        this.isPlaying = false;
        document.getElementById('play-pause-btn').textContent = '▶️ 播放';
        this.updateTimeDisplay();
        this.updateTimeSlider();
    }
    
    setTimeProgress(progress) {
        const totalTime = this.endTime - this.startTime;
        this.currentTime = new Date(this.startTime.getTime() + totalTime * progress);
        this.updateTimeDisplay();
    }
    
    updateTimeDisplay() {
        document.getElementById('current-time').textContent = this.formatTime(this.currentTime);
    }
    
    updateTimeSlider() {
        const progress = (this.currentTime - this.startTime) / (this.endTime - this.startTime);
        document.getElementById('time-slider').value = progress * 100;
    }
    
    toggleLayer(layerName, visible) {
        if (this.layers[layerName]) {
            this.layers[layerName].visible = visible;
        }
    }
    
    onWindowResize() {
        this.camera.aspect = this.container.clientWidth / this.container.clientHeight;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(this.container.clientWidth, this.container.clientHeight);
    }
    
    exportData() {
        if (!this.sceneData) {
            alert('场景数据尚未加载完成，稍后再试。');
            return;
        }
        
        const daysSinceStart = (this.currentTime - this.startTime) / (1000 * 60 * 60 * 24);
        const typhoonIntensity = this.calculateTyphoonEffect(daysSinceStart);
        
        const waveData = this.getDataAtTime(this.sceneData.waveField, this.currentTime);
        const maxWave = waveData ? Math.max(...waveData.data.map(p => p.value)) : null;
        
        const sstData = this.getDataAtTime(this.sceneData.seaSurfaceTemperature, this.currentTime);
        const avgSST = sstData
            ? sstData.data.reduce((sum, p) => sum + p.value, 0) / sstData.data.length
            : null;
        
        const vegData = this.getDataAtTime(this.sceneData.vegetation, this.currentTime);
        const avgCoverage = vegData
            ? vegData.data.reduce((sum, p) => sum + p.value, 0) / vegData.data.length
            : null;
        
        const payload = {
            scenario: 'YangtzeEstuary_Typhoon_Haikui_Dynamic',
            current_time: this.currentTime.toISOString(),
            typhoon_intensity: typhoonIntensity,
            max_wave_height: maxWave,
            average_sea_surface_temperature: avgSST,
            average_vegetation_coverage: avgCoverage,
            temporal_extent: {
                start: this.startTime.toISOString(),
                end: this.endTime.toISOString()
            }
        };
        
        const blob = new Blob([JSON.stringify(payload, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        const timestamp = this.currentTime.toISOString().slice(0, 19).replace(/[:T]/g, '-');
        link.href = url;
        link.download = 'sedris_scene_snapshot_' + timestamp + '.json';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
        
        this.showAnalysisModal(
            'SEDRIS 场景导出',
            `
            <h4>导出完成</h4>
            <p>已生成当前时间步的SEDRIS场景摘要数据文件：</p>
            <ul>
                <li>时间: ${this.formatTime(this.currentTime)}</li>
                <li>台风强度: ${typhoonIntensity > 0.1 ? Math.floor(typhoonIntensity * 100) + '%' : '无明显影响'}</li>
                <li>最大波高: ${maxWave !== null ? maxWave.toFixed(1) + ' m' : '-'}</li>
                <li>平均海温: ${avgSST !== null ? avgSST.toFixed(1) + ' °C' : '-'}</li>
                <li>植被覆盖: ${avgCoverage !== null ? Math.floor(avgCoverage * 100) + '%' : '-'}</li>
            </ul>
            <p>可将该JSON文件作为SEDRIS STF前处理或报告附件的一部分。</p>
            `
        );
    }
    
    showCrossSection() {
        if (!this.sceneData || !this.sceneData.bathymetry) {
            this.showAnalysisModal(
                '剖面分析',
                '<p>场景数据尚未加载完成，无法进行剖面分析。</p>'
            );
            return;
        }
        
        const targetLat = 31.2;
        const section = this.sceneData.bathymetry
            .filter(p => Math.abs(p.y - targetLat) < 0.01)
            .sort((a, b) => a.x - b.x);
        
        if (!section.length) {
            this.showAnalysisModal(
                '剖面分析',
                '<p>未能在当前场景中提取到有效的剖面数据。</p>'
            );
            return;
        }
        
        const minDepth = section.reduce((min, p) => Math.min(min, p.z), section[0].z);
        const maxDepth = section.reduce((max, p) => Math.max(max, p.z), section[0].z);
        
        const sampleCount = 12;
        const step = Math.max(1, Math.floor(section.length / sampleCount));
        const samples = [];
        for (let i = 0; i < section.length; i += step) {
            samples.push(section[i]);
            if (samples.length >= sampleCount) break;
        }
        
        const itemsHtml = samples
            .map(p => {
                const distance = (p.x - section[0].x) * 111000;
                return (
                    '<li>距离河口 ' +
                    distance.toFixed(0) +
                    ' m: 水深 ' +
                    p.z.toFixed(1) +
                    ' m</li>'
                );
            })
            .join('');
        
        const bodyHtml =
            '<h4>长江口典型剖面 (约 ' +
            targetLat.toFixed(2) +
            '°N)</h4>' +
            '<p>展示从河口向外海方向的地形变化，用于快速评估台风期间可能的波浪变形与能量耗散。</p>' +
            '<p>剖面最深处约 ' +
            minDepth.toFixed(1) +
            ' m，最浅处约 ' +
            maxDepth.toFixed(1) +
            ' m。</p>' +
            '<ul>' +
            itemsHtml +
            '</ul>';
        
        this.showAnalysisModal('📏 剖面分析结果', bodyHtml);
    }
    
    showTimeSeries() {
        if (!this.sceneData || !this.sceneData.waveField || !this.sceneData.seaSurfaceTemperature) {
            this.showAnalysisModal(
                '时间序列分析',
                '<p>场景数据尚未加载完成，无法进行时间序列分析。</p>'
            );
            return;
        }
        
        const targetX = 121.5;
        const targetY = 31.2;
        
        const waveSeries = this.sceneData.waveField.map(entry => {
            const point = entry.data.find(
                p => Math.abs(p.x - targetX) < 0.02 && Math.abs(p.y - targetY) < 0.02
            );
            return {
                time: entry.time,
                value: point ? point.value : null
            };
        });
        
        const sstSeries = this.sceneData.seaSurfaceTemperature.map(entry => {
            const point = entry.data.find(
                p => Math.abs(p.x - targetX) < 0.02 && Math.abs(p.y - targetY) < 0.02
            );
            return {
                time: entry.time,
                value: point ? point.value : null
            };
        });
        
        const validWaves = waveSeries.filter(p => p.value !== null);
        const validSST = sstSeries.filter(p => p.value !== null);
        
        if (!validWaves.length || !validSST.length) {
            this.showAnalysisModal(
                '时间序列分析',
                '<p>在长江口典型控制点附近未找到完整的时间序列数据。</p>'
            );
            return;
        }
        
        const maxWave = Math.max(...validWaves.map(p => p.value));
        const minWave = Math.min(...validWaves.map(p => p.value));
        const avgWave =
            validWaves.reduce((sum, p) => sum + p.value, 0) / validWaves.length;
        
        const maxSST = Math.max(...validSST.map(p => p.value));
        const minSST = Math.min(...validSST.map(p => p.value));
        const avgSST =
            validSST.reduce((sum, p) => sum + p.value, 0) / validSST.length;
        
        const keyTimes = [validWaves[0], validWaves[Math.floor(validWaves.length / 2)], validWaves[validWaves.length - 1]];
        const timeItems = keyTimes
            .map(p => {
                const date = new Date(p.time);
                return (
                    '<li>' +
                    this.formatTime(date) +
                    '：波高 ' +
                    p.value.toFixed(1) +
                    ' m</li>'
                );
            })
            .join('');
        
        const bodyHtml =
            '<h4>长江口控制点时间序列</h4>' +
            '<p>控制点位置约为经度 ' +
            targetX.toFixed(2) +
            '°E、纬度 ' +
            targetY.toFixed(2) +
            '°N。</p>' +
            '<h4>波高统计</h4>' +
            '<ul>' +
            '<li>最小波高: ' +
            minWave.toFixed(1) +
            ' m</li>' +
            '<li>最大波高: ' +
            maxWave.toFixed(1) +
            ' m</li>' +
            '<li>平均波高: ' +
            avgWave.toFixed(1) +
            ' m</li>' +
            '</ul>' +
            '<h4>海表温度统计</h4>' +
            '<ul>' +
            '<li>最低海温: ' +
            minSST.toFixed(1) +
            ' °C</li>' +
            '<li>最高海温: ' +
            maxSST.toFixed(1) +
            ' °C</li>' +
            '<li>平均海温: ' +
            avgSST.toFixed(1) +
            ' °C</li>' +
            '</ul>' +
            '<h4>关键时刻</h4>' +
            '<ul>' +
            timeItems +
            '</ul>';
        
        this.showAnalysisModal('📈 时间序列分析结果', bodyHtml);
    }
    
    showStatistics() {
        if (!this.sceneData) {
            this.showAnalysisModal(
                '统计分析',
                '<p>场景数据尚未加载完成，无法进行统计分析。</p>'
            );
            return;
        }
        
        const waveData = this.getDataAtTime(this.sceneData.waveField, this.currentTime);
        const sstData = this.getDataAtTime(this.sceneData.seaSurfaceTemperature, this.currentTime);
        const vegData = this.getDataAtTime(this.sceneData.vegetation, this.currentTime);
        const sedData = this.getDataAtTime(this.sceneData.sedimentConcentration, this.currentTime);
        
        const waveMax = waveData ? Math.max(...waveData.data.map(p => p.value)) : null;
        const waveAvg = waveData
            ? waveData.data.reduce((sum, p) => sum + p.value, 0) / waveData.data.length
            : null;
        
        const sstAvg = sstData
            ? sstData.data.reduce((sum, p) => sum + p.value, 0) / sstData.data.length
            : null;
        
        const vegAvg = vegData
            ? vegData.data.reduce((sum, p) => sum + p.value, 0) / vegData.data.length
            : null;
        
        const sedMax = sedData ? Math.max(...sedData.data.map(p => p.value)) : null;
        const sedAvg = sedData
            ? sedData.data.reduce((sum, p) => sum + p.value, 0) / sedData.data.length
            : null;
        
        const bodyHtml =
            '<h4>当前时间步综合统计</h4>' +
            '<p>时间: ' +
            this.formatTime(this.currentTime) +
            '</p>' +
            '<ul>' +
            '<li>域内最大波高: ' +
            (waveMax !== null ? waveMax.toFixed(1) + ' m' : '-') +
            '</li>' +
            '<li>域内平均波高: ' +
            (waveAvg !== null ? waveAvg.toFixed(1) + ' m' : '-') +
            '</li>' +
            '<li>域内平均海表温度: ' +
            (sstAvg !== null ? sstAvg.toFixed(1) + ' °C' : '-') +
            '</li>' +
            '<li>崇明东滩平均植被覆盖度: ' +
            (vegAvg !== null ? Math.floor(vegAvg * 100) + '%' : '-') +
            '</li>' +
            '<li>域内最大悬浮泥沙浓度: ' +
            (sedMax !== null ? sedMax.toFixed(2) : '-') +
            '</li>' +
            '<li>域内平均悬浮泥沙浓度: ' +
            (sedAvg !== null ? sedAvg.toFixed(2) : '-') +
            '</li>' +
            '</ul>' +
            '<p>这些量可以直接与数值模式输出或观测数据进行对比，用于SEDRIS场景质量控制和情景评估。</p>';
        
        this.showAnalysisModal('📊 统计分析', bodyHtml);
    }
    
    showAnalysisModal(title, bodyHtml) {
        const modal = document.createElement('div');
        modal.className = 'model-doc-modal';
        modal.innerHTML =
            '<div class="modal-content">' +
            '<div class="modal-header">' +
            '<h3>' +
            title +
            '</h3>' +
            '<button class="modal-close">&times;</button>' +
            '</div>' +
            '<div class="modal-body">' +
            bodyHtml +
            '</div>' +
            '</div>';
        
        document.body.appendChild(modal);
        
        const close = function () {
            if (document.body.contains(modal)) {
                document.body.removeChild(modal);
            }
        };
        
        modal.querySelector('.modal-close').addEventListener('click', close);
        modal.addEventListener('click', function (e) {
            if (e.target === modal) {
                close();
            }
        });
    }
}
