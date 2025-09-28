// components/model_viewer.rs
use yew::prelude::*;
use web_sys::{HtmlCanvasElement, console};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// JavaScript bindings for Three.js
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "initThreeJsScene")]
    fn init_threejs_scene(canvas: &HtmlCanvasElement, obj_path: &str);
}

#[derive(Properties, PartialEq)]
pub struct ModelViewerProps {
    #[prop_or_default]
    pub obj_path: String,
    #[prop_or(400)]
    pub width: u32,
    #[prop_or(300)]
    pub height: u32,
}

#[function_component(ModelViewer)]
pub fn model_viewer(props: &ModelViewerProps) -> Html {
    let canvas_ref = use_node_ref();
    let loading = use_state(|| true);
    let obj_path = if props.obj_path.is_empty() {
        "/static/cathedral/cathedral.obj".to_string()
    } else {
        props.obj_path.clone()
    };

    {
        let canvas_ref = canvas_ref.clone();
        let obj_path = obj_path.clone();
        let loading = loading.clone();
        
        use_effect_with((), move |_| {
            let canvas_ref = canvas_ref.clone();
            let obj_path = obj_path.clone();
            let loading = loading.clone();
            
            // Callback from JS -> Rust
            let callback = {
                let loading = loading.clone();
                wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    console::log_1(&"Model loading complete callback triggered!".into());
                    loading.set(false);
                }) as Box<dyn Fn()>)
            };

            let window = web_sys::window().unwrap();
            let js_callback = callback.as_ref().unchecked_ref::<js_sys::Function>();
            js_sys::Reflect::set(&window, &"modelLoadComplete".into(), js_callback).unwrap();

            
            // small delay to ensure canvas is mounted
            let timeout = gloo_timers::callback::Timeout::new(100, move || {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    init_threejs_scene(&canvas, &obj_path);
                } else {
                    console::error_1(&"Canvas element not found".into());
                }
            });
            timeout.forget();

            // keep closure alive for JS to call
            callback.forget();
            || ()
        });
    }

    html! {
        <div class="model-viewer-container relative     ">
            <canvas 
                ref={canvas_ref}
                width={props.width.to_string()}
                height={props.height.to_string()}
                style={format!("width: {}px; height: {}px; background: transparent;", props.width, props.height)}
                id="threejs-canvas"
            />
            if *loading {
                <div class="absolute inset-0 flex items-center justify-center">
                    <div class="text-red-600 text-center">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-red-600 mx-auto mb-2"></div>
                        <div class="text-xl">{"Loading 3D Model..."}</div>
                    </div>
                </div>
            }
        </div>
    }
}