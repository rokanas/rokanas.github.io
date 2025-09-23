use yew::prelude::*;
use crate::components::model_viewer::ModelViewer;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="h-screen flex items-center justify-center overflow-hidden pt-40">
                <ModelViewer 
                    obj_path="/static/cathedral/cathedral.obj"
                    width={600}
                    height={600}
                />
        </div>
    }
}