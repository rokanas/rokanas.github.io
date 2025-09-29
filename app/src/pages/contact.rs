// src/pages/contact.rs
use yew::prelude::*;
use web_sys::window;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use wasm_bindgen::closure::Closure;

use crate::components::social_buttons::SocialButtons;
use crate::components::heading::Heading;

#[derive(Serialize, Deserialize, Clone)]
struct FormData {
    name: String,
    email: String,
    subject: String,
    message: String,
    #[serde(rename = "g-recaptcha-response")]
    recaptcha_response: String,
}

#[derive(Serialize, Deserialize)]
struct FormResponse {
    result: String,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    use_effect_with((), |_| {
        if let Some(window) = window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }

        // define render function globally so reCAPTCHA can call it
        let render_recaptcha = Closure::wrap(Box::new(move || {
            if let Some(window) = web_sys::window() {
                if let Ok(grecaptcha) = js_sys::Reflect::get(&window, &"grecaptcha".into()) {
                    if let Ok(render) = js_sys::Reflect::get(&grecaptcha, &"render".into()) {
                        if let Ok(render) = render.dyn_into::<js_sys::Function>() {
                            let params = js_sys::Object::new();
                            js_sys::Reflect::set(&params, &"sitekey".into(), 
                                &"6LfHdcsrAAAAAA4ndXu6pT_KvO1sdOxdPIRX3q12".into()).unwrap();
                            js_sys::Reflect::set(&params, &"theme".into(), 
                                &"dark".into()).unwrap();
                            
                            let _ = render.call2(&grecaptcha, &"recaptcha-container".into(), &params);
                        }
                    }
                }
            }
        }) as Box<dyn Fn()>);

        if let Some(window) = web_sys::window() {
            js_sys::Reflect::set(&window, &"renderRecaptcha".into(), 
                render_recaptcha.as_ref().unchecked_ref()).unwrap();
        }
        render_recaptcha.forget();

        // load script if not already present
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if document.query_selector("script[src*='recaptcha/api.js']").unwrap().is_none() {
                let script = document.create_element("script").unwrap();
                script.set_attribute("src", 
                    "https://www.google.com/recaptcha/api.js?onload=renderRecaptcha&render=explicit").unwrap();
                script.set_attribute("async", "").unwrap();
                script.set_attribute("defer", "").unwrap();
                
                if let Some(head) = document.head() {
                    head.append_child(&script).unwrap();
                }
            } else {
                // script already loaded, render immediately
                if let Some(window) = web_sys::window() {
                    if let Ok(render_fn) = js_sys::Reflect::get(&window, &"renderRecaptcha".into()) {
                        if let Ok(render_fn) = render_fn.dyn_into::<js_sys::Function>() {
                            let _ = render_fn.call0(&js_sys::Object::new());
                        }
                    }
                }
            }
        }
        || {}
    });

    let form_data = use_state(|| FormData {
        name: String::new(),
        email: String::new(),
        subject: String::new(),
        message: String::new(),
        recaptcha_response: String::new(),
    });
    
    let is_submitting = use_state(|| false);
    let submission_status = use_state(|| None::<String>);

    let on_name_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut data = (*form_data).clone();
            data.name = input.value();
            form_data.set(data);
        })
    };

    let on_email_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut data = (*form_data).clone();
            data.email = input.value();
            form_data.set(data);
        })
    };

    let on_subject_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut data = (*form_data).clone();
            data.subject = input.value();
            form_data.set(data);
        })
    };

    let on_message_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut data = (*form_data).clone();
            data.message = input.value();
            form_data.set(data);
        })
    };

    let on_submit = {
        let form_data = form_data.clone();
        let is_submitting = is_submitting.clone();
        let submission_status = submission_status.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if !*is_submitting {
                // get reCAPTCHA response using DOM query
                let recaptcha_response = if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Ok(Some(textarea)) = document.query_selector("textarea[name='g-recaptcha-response']") {
                            let textarea: web_sys::HtmlTextAreaElement = textarea.dyn_into().unwrap();
                            textarea.value()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                
                if recaptcha_response.is_empty() {
                    submission_status.set(Some("Please complete the reCAPTCHA verification.".to_string()));
                    return;
                }
                
                is_submitting.set(true);
                submission_status.set(None);
                
                let mut data = (*form_data).clone();
                data.recaptcha_response = recaptcha_response;
                
                let is_submitting_clone = is_submitting.clone();
                let submission_status_clone = submission_status.clone();
                let form_data_clone = form_data.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    let result = submit_form(data).await;
                    is_submitting_clone.set(false);
                    
                    match result {
                        Ok(_) => {
                            submission_status_clone.set(Some("Message submitted successfully. I'll get back to you soon.".to_string()));
                            // reset form
                            form_data_clone.set(FormData {
                                name: String::new(),
                                email: String::new(),
                                subject: String::new(),
                                message: String::new(),
                                recaptcha_response: String::new(),
                            });
                            
                            // reset reCAPTCHA by calling the global reset function if available
                            // reset on error too
                            if let Some(window) = web_sys::window() {
                                if let Ok(grecaptcha) = js_sys::Reflect::get(&window, &"grecaptcha".into()) {
                                    if let Ok(reset_fn) = js_sys::Reflect::get(&grecaptcha, &"reset".into()) {
                                        if let Ok(reset_fn) = reset_fn.dyn_into::<js_sys::Function>() {
                                            let _ = reset_fn.call0(&js_sys::Object::new());
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            submission_status_clone.set(Some(format!("Error sending message: {}", err)));
                            
                            // reset reCAPTCHA on error too
                            if let Some(window) = web_sys::window() {
                                if let Ok(grecaptcha) = js_sys::Reflect::get(&window, &"grecaptcha".into()) {
                                    if let Ok(reset_fn) = js_sys::Reflect::get(&grecaptcha, &"reset".into()) {
                                        if let Ok(reset_fn) = reset_fn.dyn_into::<js_sys::Function>() {
                                            let _ = reset_fn.call0(&js_sys::Object::new());
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class="min-h-screen pt-15 pb-10" >
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
                
                // main contact card
                <div class="rounded-lg border-3 border-red-600 p-8 relative overflow-hidden"
                    // style="background-image: url('/static/contact/ADEL_V99.png'); background-repeat: repeat; background-size: 310px; image-rendering: pixelated;">                    
                    style="background:linear-gradient(135deg,#1a1a1a 0%,#2a2a2a 50%,#1a1a1a 100%);"
                >
                    <div class="relative z-10">
                        <Heading 
                            src = "/static/contact/CONTACT_1.png"
                            alt = "Contact"
                            sub_heading = "Want to drop a message? Collaborate? Ask a question?"
                        ></Heading>
                        
                        // contact form
                        <div class="max-w-2xl mx-auto mb-12">
                            <form onsubmit={on_submit} class="space-y-6">
                                // name and email row
                                <div class="grid md:grid-cols-2 gap-6">
                                    <div>
                                        <label for="name" class="block text-sm font-medium text-gray-300 mb-2">
                                            {"Name *"}
                                        </label>
                                        <input
                                            type="text"
                                            id="name"
                                            name="name"
                                            value={form_data.name.clone()}
                                            onchange={on_name_change}
                                            required=true
                                            disabled={*is_submitting}
                                            class="w-full px-4 py-3 bg-[#1a1a1a] border border-gray-600 rounded-lg 
                                                   text-white placeholder-gray-400 focus:border-red-600 focus:ring-1 
                                                   focus:ring-red-600 focus:outline-none transition-colors
                                                   disabled:opacity-50 disabled:cursor-not-allowed"
                                            placeholder="Your name"
                                        />
                                    </div>
                                    <div>
                                        <label for="email" class="block text-sm font-medium text-gray-300 mb-2">
                                            {"Email *"}
                                        </label>
                                        <input
                                            type="email"
                                            id="email"
                                            name="email"
                                            value={form_data.email.clone()}
                                            onchange={on_email_change}
                                            required=true
                                            disabled={*is_submitting}
                                            class="w-full px-4 py-3 bg-[#1a1a1a] border border-gray-600 rounded-lg 
                                                   text-white placeholder-gray-400 focus:border-red-600 focus:ring-1 
                                                   focus:ring-red-600 focus:outline-none transition-colors
                                                   disabled:opacity-50 disabled:cursor-not-allowed"
                                            placeholder="your.email@example.com"
                                        />
                                    </div>
                                </div>
                                
                                // subject
                                <div>
                                    <label for="subject" class="block text-sm font-medium text-gray-300 mb-2">
                                        {"Subject"}
                                    </label>
                                    <input
                                        type="text"
                                        id="subject"
                                        name="subject"
                                        value={form_data.subject.clone()}
                                        onchange={on_subject_change}
                                        required=false
                                        disabled={*is_submitting}
                                        class="w-full px-4 py-3 bg-[#1a1a1a] border border-gray-600 rounded-lg 
                                               text-white placeholder-gray-400 focus:border-red-600 focus:ring-1 
                                               focus:ring-red-600 focus:outline-none transition-colors
                                               disabled:opacity-50 disabled:cursor-not-allowed"
                                        placeholder="What's this about?"
                                    />
                                </div>
                                
                                // message
                                <div>
                                    <label for="message" class="block text-sm font-medium text-gray-300 mb-2">
                                        {"Message *"}
                                    </label>
                                    <textarea
                                        id="message"
                                        name="message"
                                        rows="6"
                                        value={form_data.message.clone()}
                                        onchange={on_message_change}
                                        required=true
                                        disabled={*is_submitting}
                                        class="w-full px-4 py-3 bg-[#1a1a1a] border border-gray-600 rounded-lg 
                                               text-white placeholder-gray-400 focus:border-red-600 focus:ring-1 
                                               focus:ring-red-600 focus:outline-none transition-colors resize-vertical
                                               disabled:opacity-50 disabled:cursor-not-allowed"
                                        placeholder="What's on your mind?"
                                    />
                                </div>

                                // submit button + recaptcha row
                                <div class="flex flex-col sm:flex-row items-center justify-center gap-15">
                                    // recaptcha container
                                        <div 
                                            id="recaptcha-container"
                                            class="g-recaptcha transform scale-90 sm:scale-100"
                                            data-sitekey="6LfHdcsrAAAAAA4ndXu6pT_KvO1sdOxdPIRX3q12"
                                            data-theme="dark"
                                        ></div>
                                    
                                    // submit button
                                    <button
                                        type="submit"
                                        disabled={*is_submitting}
                                        class="px-8 py-3 bg-red-600 hover:bg-red-700 disabled:bg-gray-600 
                                               text-white font-semibold rounded-lg transition-colors duration-200
                                               disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer
                                               focus:outline-none focus:ring-2 focus:ring-red-600 focus:ring-offset-2 focus:ring-offset-gray-900"
                                    >
                                        {if *is_submitting { "Submitting..." } else { "Submit Message" }}
                                    </button>
                                </div>
                                
                                // status message
                                {if let Some(ref status) = *submission_status {
                                    let (bg_color, text_color, border_color) = if status.contains("successfully") {
                                        ("bg-green-900/20", "text-green-400", "border-green-500")
                                    } else {
                                        ("bg-red-900/20", "text-red-400", "border-red-600")
                                    };
                                    
                                    html! {
                                        <div class={format!("mt-4 p-4 rounded-lg border {} {} {}", bg_color, border_color, text_color)}>
                                            {status}
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </form>
                        </div>

                        <SocialButtons
                            button_size={12}
                            svg_size={8}
                            professional=false
                        > </SocialButtons>
                    </div>
                </div>
            </div>
        </div>
    }
}

async fn submit_form(form_data: FormData) -> Result<FormResponse, String> {
    let url = "https://script.google.com/macros/s/AKfycbwLckDBcah084esScg4oIG0IvmgCb_KPfsjPS979BxWQj8fFVvP6Ia_AF2gbOUHgWgajw/exec";
    
    // convert form data to JSON string manually to match FormEasy expectations
    let json_body = serde_json::to_string(&form_data)
        .map_err(|e| format!("Failed to serialize form data: {:?}", e))?;
    
    let response = Request::post(url)
        .header("Content-Type", "text/plain;charset=utf-8")
        .body(json_body)
        .map_err(|e| format!("Failed to create request: {:?}", e))?
        .send()
        .await
        .map_err(|e| {
            // more detailed error handling
            let error_msg = format!("{:?}", e);
            if error_msg.contains("CORS") || error_msg.contains("Failed to fetch") {
                "CORS error: Make sure Google Apps Script is properly configured and deployed as a web app with 'Anyone' access.".to_string()
            } else {
                format!("Network error: {}", error_msg)
            }
        })?;

    if response.ok() {
        // try parse as JSON, but handle cases where it might not be JSON
        match response.json::<FormResponse>().await {
            Ok(form_response) => Ok(form_response),
            Err(_) => {
                // if JSON parsing fails, assume success if we got 200 response
                Ok(FormResponse {
                    result: "success".to_string()
                })
            }
        }
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("HTTP {} error: {}", status, error_text))
    }
}