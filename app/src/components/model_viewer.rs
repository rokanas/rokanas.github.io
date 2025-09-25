// components/model_viewer.rs
use yew::prelude::*;
use web_sys::{HtmlCanvasElement, console};
use wasm_bindgen::prelude::*;

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
    let obj_path = if props.obj_path.is_empty() {
        "/static/cathedral/cathedral.obj".to_string()
    } else {
        props.obj_path.clone()
    };

    {
        let canvas_ref = canvas_ref.clone();
        let obj_path = obj_path.clone();
        
        use_effect_with((), move |_| {
            let canvas_ref = canvas_ref.clone();
            let obj_path = obj_path.clone();
            
            console::log_1(&format!("Setting up effect with obj_path: {}", obj_path).into());
            
            // Small delay to ensure canvas is mounted
            let timeout = gloo_timers::callback::Timeout::new(100, move || {
                console::log_1(&"Timeout callback executing".into());
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    console::log_1(&format!("Canvas found, initializing with path: {}", obj_path).into());
                    init_threejs_scene(&canvas, &obj_path);
                } else {
                    console::error_1(&"Canvas element not found".into());
                }
            });
            timeout.forget();

            || ()
        });
    }

    html! {
        <div class="model-viewer-container">
            <canvas 
                ref={canvas_ref}
                width={props.width.to_string()}
                height={props.height.to_string()}
                style={format!("width: {}px; height: {}px; background: transparent;", props.width, props.height)}
                id="threejs-canvas"
            />
            // <p style="color: white; margin-top: 10px;">
            //     {format!("Loading model: {}", obj_path)}
            // </p>
        </div>
    }
}