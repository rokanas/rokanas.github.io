// src/pages/contact.rs
use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

use crate::components::social_buttons::SocialButtons;

#[derive(Serialize, Deserialize, Clone)]
struct FormData {
    name: String,
    email: String,
    subject: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct FormResponse {
    result: String,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let form_data = use_state(|| FormData {
        name: String::new(),
        email: String::new(),
        subject: String::new(),
        message: String::new(),
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
                is_submitting.set(true);
                submission_status.set(None);
                
                let data = (*form_data).clone();
                let is_submitting_clone = is_submitting.clone();
                let submission_status_clone = submission_status.clone();
                let form_data_clone = form_data.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    let result = submit_form(data).await;
                    is_submitting_clone.set(false);
                    
                    match result {
                        Ok(_) => {
                            submission_status_clone.set(Some("Message sent successfully! I'll get back to you soon.".to_string()));
                            // Reset form
                            form_data_clone.set(FormData {
                                name: String::new(),
                                email: String::new(),
                                subject: String::new(),
                                message: String::new(),
                            });
                        }
                        Err(err) => {
                            submission_status_clone.set(Some(format!("Error sending message: {}", err)));
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class="min-h-screen pt-20 pb-16" >
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
                
                // main contact card
                <div class="bg-gradient-to-br from-gray-900 to-black rounded-lg border-2 border-red-600 p-8 mb-8 relative overflow-hidden">                    
                    <div class="relative z-10">
                        <h1 class="text-4xl lg:text-5xl font-bold text-red-500 mb-6 text-center">
                            {"Get In Touch"}
                        </h1>
                        
                        <p class="text-xl text-gray-300 text-center mb-12 max-w-2xl mx-auto">
                            {"Want to drop a message? Collaborate? Ask a question?"}
                        </p>
                        
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
                                            class="w-full px-4 py-3 bg-gray-800 border border-gray-600 rounded-lg 
                                                   text-white placeholder-gray-400 focus:border-red-500 focus:ring-1 
                                                   focus:ring-red-500 focus:outline-none transition-colors
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
                                            class="w-full px-4 py-3 bg-gray-800 border border-gray-600 rounded-lg 
                                                   text-white placeholder-gray-400 focus:border-red-500 focus:ring-1 
                                                   focus:ring-red-500 focus:outline-none transition-colors
                                                   disabled:opacity-50 disabled:cursor-not-allowed"
                                            placeholder="your.email@example.com"
                                        />
                                    </div>
                                </div>
                                
                                // subject
                                <div>
                                    <label for="subject" class="block text-sm font-medium text-gray-300 mb-2">
                                        {"Subject *"}
                                    </label>
                                    <input
                                        type="text"
                                        id="subject"
                                        name="subject"
                                        value={form_data.subject.clone()}
                                        onchange={on_subject_change}
                                        required=true
                                        disabled={*is_submitting}
                                        class="w-full px-4 py-3 bg-gray-800 border border-gray-600 rounded-lg 
                                               text-white placeholder-gray-400 focus:border-red-500 focus:ring-1 
                                               focus:ring-red-500 focus:outline-none transition-colors
                                               disabled:opacity-50 disabled:cursor-not-allowed"
                                        placeholder="What's this about?"
                                    />
                                </div>
                                
                                // Message
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
                                        class="w-full px-4 py-3 bg-gray-800 border border-gray-600 rounded-lg 
                                               text-white placeholder-gray-400 focus:border-red-500 focus:ring-1 
                                               focus:ring-red-500 focus:outline-none transition-colors resize-vertical
                                               disabled:opacity-50 disabled:cursor-not-allowed"
                                        placeholder="What's on your mind?"
                                    />
                                </div>
                                
                                // Submit button
                                <div class="text-center">
                                    <button
                                        type="submit"
                                        disabled={*is_submitting}
                                        class="px-8 py-3 bg-red-600 hover:bg-red-700 disabled:bg-gray-600 
                                               text-white font-semibold rounded-lg transition-colors duration-200
                                               disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer
                                               focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:ring-offset-gray-900"
                                    >
                                        {if *is_submitting { "Sending..." } else { "Send Message" }}
                                    </button>
                                </div>
                                
                                // Status message
                                {if let Some(ref status) = *submission_status {
                                    let (bg_color, text_color, border_color) = if status.contains("successfully") {
                                        ("bg-green-900/20", "text-green-400", "border-green-500")
                                    } else {
                                        ("bg-red-900/20", "text-red-400", "border-red-500")
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
                            svg_size={8}>
                        </SocialButtons>
                        
                        // additional info
                        <div class="mt-12 text-center">
                            <div class="inline-block px-6 py-3 bg-gray-800 rounded-lg border border-gray-600">
                                <p class="text-gray-300">
                                    {"Response time: "}<span class="text-green-400 font-semibold">{"Usually within 24 hours"}</span>
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

async fn submit_form(form_data: FormData) -> Result<FormResponse, String> {
    // Replace with your actual FormEasy Google Apps Script URL
    let url = "https://script.google.com/macros/s/AKfycbwLckDBcah084esScg4oIG0IvmgCb_KPfsjPS979BxWQj8fFVvP6Ia_AF2gbOUHgWgajw/exec";
    
    // Convert form data to JSON string manually to match FormEasy expectations
    let json_body = serde_json::to_string(&form_data)
        .map_err(|e| format!("Failed to serialize form data: {:?}", e))?;
    
    let response = Request::post(url)
        .header("Content-Type", "text/plain;charset=utf-8")
        .body(json_body)
        .map_err(|e| format!("Failed to create request: {:?}", e))?
        .send()
        .await
        .map_err(|e| {
            // More detailed error handling
            let error_msg = format!("{:?}", e);
            if error_msg.contains("CORS") || error_msg.contains("Failed to fetch") {
                "CORS error: Make sure your Google Apps Script is properly configured and deployed as a web app with 'Anyone' access.".to_string()
            } else {
                format!("Network error: {}", error_msg)
            }
        })?;

    if response.ok() {
        // Try to parse as JSON, but handle cases where it might not be JSON
        match response.json::<FormResponse>().await {
            Ok(form_response) => Ok(form_response),
            Err(_) => {
                // If JSON parsing fails, assume success if we got a 200 response
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