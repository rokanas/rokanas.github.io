// components/social_buttons.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SocialButtonProps {
    pub button_size: u8,
    pub svg_size: u8,
    pub professional: bool,
}

#[function_component(SocialButtons)]
pub fn social_buttons(props: &SocialButtonProps) -> Html {
    // cv download handler
    let download_cv = Callback::from(|_| {
        if let Some(window) = web_sys::window() {
            // direct link to cv file
            let _ = window.open_with_url_and_target("/static/KR_CV.pdf", "_blank");
        }
    });
    
    html! {
        <div class="flex justify-center items-center gap-4 mt-8 w-full">
            // github
            <a 
                href="https://github.com/rokanas" 
                target="_blank" 
                class={format!("w-{} h-{} bg-[#1a1a1a] hover:bg-gray-700 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-gray-500", props.button_size, props.button_size)}
                title="GitHub"
            >
                <svg class={format!("w-{} h-{} fill-current text-gray-300 group-hover:text-white transition-colors duration-300", props.svg_size, props.svg_size)} viewBox="0 0 16 16">
                    <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
                </svg>
            </a>

            if props.professional {

                // linkedin
                <a 
                    href="https://www.linkedin.com/in/konstantinos-rokanas-1ab1a113a/" 
                    target="_blank" 
                    class={format!("w-{} h-{} bg-[#1a1a1a] hover:bg-blue-600 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-blue-500", props.button_size, props.button_size)}
                    title="LinkedIn"
                >
                    <svg class={format!("w-{} h-{} fill-current text-gray-300 group-hover:text-white transition-colors duration-300", props.svg_size, props.svg_size)} viewBox="0 0 24 24">
                        <path d="M20.5 2h-17A1.5 1.5 0 002 3.5v17A1.5 1.5 0 003.5 22h17a1.5 1.5 0 001.5-1.5v-17A1.5 1.5 0 0020.5 2zM8 19H5v-9h3zM6.5 8.25A1.75 1.75 0 118.3 6.5a1.78 1.78 0 01-1.8 1.75zM19 19h-3v-4.74c0-1.42-.6-1.93-1.38-1.93A1.74 1.74 0 0013 14.19a.66.66 0 000 .14V19h-3v-9h2.9v1.3a3.11 3.11 0 012.7-1.4c1.55 0 3.36.86 3.36 3.66z"/>
                    </svg>
                </a>

                // cv
                <button 
                    onclick={download_cv}
                    class={format!("w-{} h-{} bg-[#1a1a1a] hover:bg-red-600 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-red-900 cursor-pointer", props.button_size, props.button_size)}
                    title="CV"
                >
                    <svg
                        class={format!("w-{} h-{} fill-current text-gray-300 group-hover:text-white transition-colors duration-300", props.svg_size, props.svg_size)}
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        // document shape
                        <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z" />

                        // cv text inside doc
                        <text
                            x="8"
                            y="16"
                            font-size="6"
                            font-family="Arial, Helvetica, sans-serif"
                            fill="currentColor"
                            font-weight="bold"
                        >
                            {"CV"}
                        </text>
                    </svg>
                </button>

            } else {

                // discord
                <a 
                    href="https://discord.gg/TODO" 
                    target="_blank" 
                    class={format!("w-{} h-{} bg-[#1a1a1a] hover:bg-[#5865F2] rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-[#5865F2]", props.button_size, props.button_size)}
                    title="Discord"
                >
                    <svg class={format!("w-{} h-{} fill-current text-gray-300 group-hover:text-white transition-colors duration-300", props.svg_size, props.svg_size)} viewBox="0 0 24 24">
                        <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"/>
                    </svg>
                </a>

                // instagram
                <a 
                    href="https://www.instagram.com/charybdis.maw/" 
                    target="_blank" 
                    class={format!("w-{} h-{} bg-[#1a1a1a] hover:bg-gradient-to-br hover:from-purple-600 hover:to-pink-500 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-pink-500", props.button_size, props.button_size)}
                    title="Instagram"
                >
                    <svg class={format!("w-{} h-{} fill-current text-gray-300 group-hover:text-white transition-colors duration-300", props.svg_size, props.svg_size)} viewBox="0 0 1000 1000">
                        <path d="M295.42,6c-53.2,2.51-89.53,11-121.29,23.48-32.87,12.81-60.73,30-88.45,57.82S40.89,143,28.17,175.92c-12.31,31.83-20.65,68.19-23,121.42S2.3,367.68,2.56,503.46,3.42,656.26,6,709.6c2.54,53.19,11,89.51,23.48,121.28,12.83,32.87,30,60.72,57.83,88.45S143,964.09,176,976.83c31.8,12.29,68.17,20.67,121.39,23s70.35,2.87,206.09,2.61,152.83-.86,206.16-3.39S799.1,988,830.88,975.58c32.87-12.86,60.74-30,88.45-57.84S964.1,862,976.81,829.06c12.32-31.8,20.69-68.17,23-121.35,2.33-53.37,2.88-70.41,2.62-206.17s-.87-152.78-3.4-206.1-11-89.53-23.47-121.32c-12.85-32.87-30-60.7-57.82-88.45S862,40.87,829.07,28.19c-31.82-12.31-68.17-20.7-121.39-23S637.33,2.3,501.54,2.56,348.75,3.4,295.42,6m5.84,903.88c-48.75-2.12-75.22-10.22-92.86-17-23.36-9-40-19.88-57.58-37.29s-28.38-34.11-37.5-57.42c-6.85-17.64-15.1-44.08-17.38-92.83-2.48-52.69-3-68.51-3.29-202s.22-149.29,2.53-202c2.08-48.71,10.23-75.21,17-92.84,9-23.39,19.84-40,37.29-57.57s34.1-28.39,57.43-37.51c17.62-6.88,44.06-15.06,92.79-17.38,52.73-2.5,68.53-3,202-3.29s149.31.21,202.06,2.53c48.71,2.12,75.22,10.19,92.83,17,23.37,9,40,19.81,57.57,37.29s28.4,34.07,37.52,57.45c6.89,17.57,15.07,44,17.37,92.76,2.51,52.73,3.08,68.54,3.32,202s-.23,149.31-2.54,202c-2.13,48.75-10.21,75.23-17,92.89-9,23.35-19.85,40-37.31,57.56s-34.09,28.38-57.43,37.5c-17.6,6.87-44.07,15.07-92.76,17.39-52.73,2.48-68.53,3-202.05,3.29s-149.27-.25-202-2.53m407.6-674.61a60,60,0,1,0,59.88-60.1,60,60,0,0,0-59.88,60.1M245.77,503c.28,141.8,115.44,256.49,257.21,256.22S759.52,643.8,759.25,502,643.79,245.48,502,245.76,245.5,361.22,245.77,503m90.06-.18a166.67,166.67,0,1,1,167,166.34,166.65,166.65,0,0,1-167-166.34"/>
                    </svg>
                </a>
            }
        </div>
    }
}