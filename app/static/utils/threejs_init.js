// global func to initialize three.js scene
window.initThreeJsScene = function(canvas, objPath) {
    // scene setup
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x222222);

    // camera setup
    const camera = new THREE.PerspectiveCamera(
        75, 
        canvas.width / canvas.height, 
        0.1, 
        1000
    );
    camera.position.z = 5;

    // renderer setup
    const renderer = new THREE.WebGLRenderer({ 
        canvas: canvas,
        antialias: true 
    });
    renderer.setSize(canvas.width, canvas.height);
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;

    // lighting
    const ambientLight = new THREE.AmbientLight(0x404040, 0.6);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(10, 10, 5);
    directionalLight.castShadow = true;
    scene.add(directionalLight);

    const pointLight = new THREE.PointLight(0xffffff, 0.5);
    pointLight.position.set(-10, -10, -5);
    scene.add(pointLight);

    // model loading
    let model = null;
    const loader = new THREE.OBJLoader();
    
    loader.load(
        objPath,
        function(object) {
            console.log('Model loaded successfully');
            
            // scale and position model
            object.scale.setScalar(1);
            object.position.set(0, 0, 0);
            
            // add materials to model
            object.traverse(function(child) {
                if (child.isMesh) {
                    child.material = new THREE.MeshPhongMaterial({
                        color: 0x00ff88,
                        shininess: 100
                    });
                    child.castShadow = true;
                    child.receiveShadow = true;
                }
            });
            
            model = object;
            scene.add(object);
        },
        function(progress) {
            console.log('Loading progress: ', (progress.loaded / progress.total * 100) + '%');
        },
        function(error) {
            console.error('Error loading model:', error);
            
            // fallback: create simple cube if model fails to load
            const geometry = new THREE.BoxGeometry();
            const material = new THREE.MeshPhongMaterial({ color: 0x00ff88 });
            const cube = new THREE.Mesh(geometry, material);
            cube.castShadow = true;
            model = cube;
            scene.add(cube);
        }
    );

    // animation loop
    function animate() {
        requestAnimationFrame(animate);

        // rotate model if it exists
        if (model) {
            model.rotation.x += 0.005;
            model.rotation.y += 0.01;
        }

        renderer.render(scene, camera);
    }

    // jandle window resize
    function onWindowResize() {
        const rect = canvas.getBoundingClientRect();
        camera.aspect = rect.width / rect.height;
        camera.updateProjectionMatrix();
        renderer.setSize(rect.width, rect.height);
    }

    window.addEventListener('resize', onWindowResize);

    // start animation
    animate();
};