// components/heading.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeadingProps {
    pub src: String,
    pub alt: String, 
    pub sub_heading: Option<String>,
}

#[function_component(Heading)]
pub fn heading(props: &HeadingProps) -> Html {
    html! {
        <div class="text-center mb-12">
            <img 
                src={props.src.clone()}
                alt={props.alt.clone()}
                class="w-auto h-auto mx-auto"
            />
            <img 
                src="/static/common/DIVIDER_1.png" 
                alt="Divider"
                class="w-auto h-auto mx-auto mb-4 -mt-6"
            />
            <p class="text-gray-300 text-lg max-w-2xl mx-auto leading-relaxed">
                {Some(props.sub_heading.clone())}
            </p>
        </div>
    }
}
                    