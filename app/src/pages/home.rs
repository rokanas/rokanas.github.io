use yew::prelude::*;
use crate::components::model_viewer::ModelViewer;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="min-h-screen flex flex-col">
            <main class="flex-grow flex flex-col items-center justify-center space-y-3">
                <div class="text-center">
                    // <h2 class="text-white mb-4">{ "3D Model Viewer" }</h2>
                    // 3d model viewer
                    <ModelViewer 
                        obj_path="/static/cathedral/cathedral.obj"
                        width={500}
                        height={600}
                    />
                </div>
            </main>
        </div>
    }
}