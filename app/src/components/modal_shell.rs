// components/modal_shell.rs
use yew::prelude::*;

// shared modal chrome used by ProjectItem and DoomModelItem
// backdrop (click-outside to close), bordered panel, a header row with title + caller-supplied close button.
#[derive(Properties, PartialEq)]
pub struct ModalShellProps {
    pub title: String,
    pub onclick_backdrop: Callback<MouseEvent>,
    pub close_button: Html,
    pub children: Children,
}

#[function_component(ModalShell)]
pub fn modal_shell(props: &ModalShellProps) -> Html {
    html! {
        <div
            class="fixed inset-0 backdrop-blur-lg bg-black/60 flex items-center justify-center p-4 z-50"
            onclick={props.onclick_backdrop.clone()}
        >
            <div
                class="bg-doom-panel-inner border-3 border-doom-red rounded-lg max-w-4xl w-full max-h-[90vh] overflow-y-auto"
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
            >
                // modal header
                <div class="flex justify-between items-center p-6 border-b border-doom-gray-dark/50">
                    <h2 class="text-2xl font-bold text-doom-red font-mono">{&props.title}</h2>
                    { props.close_button.clone() }
                </div>

                // modal content
                <div class="p-6 space-y-6">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
