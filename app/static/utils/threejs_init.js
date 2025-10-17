// static/utils/threejs_init.js
       
window.initThreeJsScene = function(canvas, objPath) {
    console.log('Starting Three.js initialization...');

    // scene/camera/renderer setup
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, canvas.width / canvas.height, 0.1, 1000);
    camera.position.set(0, 0, 10);

    const renderer = new THREE.WebGLRenderer({ 
        canvas: canvas, 
        antialias: true,
        alpha: true // keep transparent background
    });
    renderer.setSize(canvas.width, canvas.height);
    renderer.setClearColor(0x000000, 0); // transparent

    // Create OrbitControls for interaction
    let controls = null;
    if (typeof THREE.OrbitControls !== 'undefined') {
        controls = new THREE.OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;
        controls.screenSpacePanning = false;
        controls.minDistance = 5;
        controls.maxDistance = 20;
        controls.maxPolarAngle = Math.PI;
        controls.autoRotate = true;
        controls.autoRotateSpeed = 0.5;
        console.log('OrbitControls initialized');
    } else {
        console.warn('OrbitControls not available - manual control disabled');
    }

    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 5, 5);
    scene.add(directionalLight);

    const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.3);
    directionalLight2.position.set(-5, -2, -5);
    scene.add(directionalLight2);

    let model = null;

    // track texture loading and model addition so we only hide the loader once everything is ready
    let pendingTextures = 0;
    let modelAdded = false;
    let loadingSignaled = false;

    function checkComplete() {
        if (!loadingSignaled && modelAdded && pendingTextures === 0) {
            loadingSignaled = true;
            if (typeof window.modelLoadComplete === 'function') {
                try {
                    window.modelLoadComplete();
                } catch (e) {
                    console.warn('Calling modelLoadComplete failed:', e);
                }
            }
        }
    }

    function createFallbackCube() {
        console.log('Creating fallback cube');
        const geometry = new THREE.BoxGeometry(2, 2, 2);
        const material = new THREE.MeshPhongMaterial({ color: 0x00ff00 });
        const cube = new THREE.Mesh(geometry, material);
        model = cube;
        scene.add(cube);
        modelAdded = true;
        // no textures pending in fallback -> mark complete
        checkComplete();
    }

    // if OBJLoader missing, fallback and signal complete
    if (typeof THREE.OBJLoader === 'undefined') {
        console.error('OBJLoader not available, creating fallback cube');
        createFallbackCube();
        animate();
        return;
    }

    console.log('Loading OBJ...');
    const loader = new THREE.OBJLoader();
    const textureLoader = new THREE.TextureLoader();

    loader.load(
        objPath,
        function(object) {
            console.log('OBJ loaded successfully with', object.children.length, 'meshes');

            if (object.children.length === 0) {
                console.error('No meshes found, using fallback');
                createFallbackCube();
                // ensure animate continues
                animate();
                return;
            }

            // traverse and set materials + load textures when applicable
            object.traverse(function(child) {
                if (child.isMesh) {
                    const meshName = child.name;
                    // console.log('Processing mesh:', meshName);

                    // default base material supporting transparency
                    child.material = new THREE.MeshPhongMaterial({
                        color: 0xffffff,
                        side: THREE.DoubleSide,
                        transparent: true,
                        alphaTest: 0.1
                    });

                    // extract texture name if present
                    let textureName = null;
                    if (meshName && meshName.includes('_')) {
                        const parts = meshName.split('_');
                        if (parts.length >= 3 && parts[0] === 'MAP01' && parts[1] === 'MAP01') {
                            const remainingParts = parts.slice(2);
                            if (remainingParts.length === 2 && remainingParts[0] === remainingParts[1]) {
                                textureName = remainingParts[0];
                            } else {
                                textureName = remainingParts.join('_');
                            }
                        }
                    }

                    if (textureName && textureName !== 'undefined') {
                        // console.log('Attempting to load texture:', textureName);
                        const texturePath = `static/cathedral/textures/${textureName}.png`;

                        // increment pending count for this texture
                        pendingTextures++;

                        textureLoader.load(
                            texturePath,
                            function(texture) {
                                // console.log('Successfully loaded texture:', texturePath);

                                // crisp pixel art settings
                                texture.magFilter = THREE.NearestFilter;
                                texture.minFilter = THREE.NearestFilter;
                                texture.wrapS = THREE.RepeatWrapping;
                                texture.wrapT = THREE.RepeatWrapping;

                                child.material.map = texture;
                                child.material.needsUpdate = true;

                                pendingTextures--;
                                checkComplete();
                            },
                            undefined,
                            function(err) {
                                console.log('Failed to load texture:', texturePath, err);
                                applyFallbackColor(child, textureName);
                                pendingTextures--;
                                checkComplete();
                            }
                        );
                    } else {
                        // no texture name — apply neutral color
                        console.log('No texture name found for mesh:', meshName);
                        child.material.color.setHex(0xcccccc);
                    }
                }
            });

            // scale and center (kept from base)
            const box = new THREE.Box3().setFromObject(object);
            const center = box.getCenter(new THREE.Vector3());
            const size = box.getSize(new THREE.Vector3());
            const maxDim = Math.max(size.x, size.y, size.z);

            if (maxDim > 0) {
                const scale = 10 / maxDim;
                object.scale.setScalar(scale);
                const scaledCenter = center.clone().multiplyScalar(scale);
                object.position.sub(scaledCenter);
                object.position.y += 1.5;
            }

            model = object;
            scene.add(object);
            
            // If controls exist, focus them on the model
            // if (controls) {
            //     controls.target.set(0, 1.5, 0);
            //     controls.update();
            // }
            
            modelAdded = true;
            console.log('Model added successfully');

            // if zero textures pending, trigger immediate completion.
            // otherwise checkComplete() will be called after the last texture's callback.
            checkComplete();

            // start animation loop
            animate();
        },
        function(progress) {
            if (progress.lengthComputable) {
                console.log('Loading:', Math.round(progress.loaded / progress.total * 100) + '%');
            }
        },
        function(error) {
            console.error('Failed to load OBJ:', error);
            createFallbackCube();
            // createFallbackCube will call checkComplete internally
            animate();
        }
    );

    function applyFallbackColor(mesh, textureName) {
        if (!textureName) {
            mesh.material.color.setHex(0xcccccc);
            return;
        }
        const name = textureName.toLowerCase();
        let color = 0xcccccc;

        if (name.includes('fire') && name.includes('blu')) {
            color = 0x4444ff;
        } else if (name.includes('fire') && name.includes('lav')) {
            color = 0xff4400;
        } else if (name.includes('fire')) {
            color = 0xff8800;
        } else if (name.includes('wood')) {
            color = 0x996633;
        } else if (name.includes('metal')) {
            color = 0xaaaacc;
        } else if (name.includes('stone') || name.includes('rock')) {
            color = 0x888866;
        } else if (name.includes('door')) {
            color = 0x663311;
        } else if (name.includes('wall')) {
            color = 0xccbbaa;
        } else if (name.includes('grey') || name.includes('gray')) {
            color = 0x777777;
        }

        mesh.material.color.setHex(color);
        console.log('Applied fallback color', color.toString(16), 'to', textureName);
    }

    function animate() {
        requestAnimationFrame(animate);
        
        // Update controls if they exist
        if (controls) {
            controls.update();
        } else if (model) {
            // Fallback to simple rotation if no controls
            model.rotation.y += 0.0005;
        }
        
        renderer.render(scene, camera);
    }

    // start animation
    animate();
    console.log('Scene initialized');
};