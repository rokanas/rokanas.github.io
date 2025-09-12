// components/experience.rs
use yew::prelude::*;

use crate::pages::about::{ExperienceItem};

#[function_component(Experience)]
pub fn experience() -> Html {
    let selected_job = use_state(|| 0usize);    // track which job is selected

    // experience data
    let experience: Vec<ExperienceItem> = vec![
        ExperienceItem {
            title: "Teaching Assistant".to_string(),
            institution: "University of Gothenburg".to_string(),
            date: "2024 - 2025".to_string(),
            description: Some(vec![
                "TA for Software Architecture, Requirements Engineering and Systems Development.".to_string(),
                "Led TA meetings and workshops with students, provided in-person and remote guidance.".to_string(),
                "Provided support and feedback to professors concerning assignments and course materials.".to_string(),
                "Graded student assignments and exams.".to_string()]),
            icon: "/static/U_GOTH.png".to_string(),
        },
        ExperienceItem {
            title: "Compliance Officer".to_string(),
            institution: "Huawei Technologies S.A.".to_string(),
            date: "2019 - 2022".to_string(),
            description: Some(vec![
                "Legal compliance assessment and risk analysis for all areas of company operations in Athens and Cyprus offices (specialization in Data Protection (GDPR) and Cybersecurity)".to_string(),
                "Legal support to regional offices in Albania, Northern Macedonia and Bulgaria.".to_string(),
                "Led training sessions for all regional offices on sensitive areas (personal data protection, cybersecurity, anti-bribery).".to_string(),]),
            icon: "/static/HUAWEI.png".to_string(),
        },
        ExperienceItem {
            title: "Intern at Academy of European Public Law".to_string(),
            institution: "European Public Law Organization (EPLO)".to_string(),
            date: "2019".to_string(),
            description: Some(vec![
                "Assisted in the design, organization and implementation of European and Public Law university programs and summer exchange programs".to_string(),
                "Assisted in the administration of the Department of Education.".to_string(),
                "Drafted reports on EPLO activities and conducted research for various other departments.".to_string(),
                ]),
            icon: "/static/EPLO.png".to_string(),
        },
        ExperienceItem {
            title: "Intern at Greek National Desk".to_string(),
            institution: "Eurojust".to_string(),
            date: "2019 - 2020".to_string(),
            description: Some(vec![
                "Assisted in administrative functions of the Greek national desk.".to_string(),
                "Composed external and internal communications towards national authorities and national desks.".to_string(),
                "Legal case file and database management.".to_string(),
                    ]),
            icon: "/static/EUROJUST.png".to_string(),
        },
    ];

    html! {
        // education component
        <div class="max-w-6xl mx-auto">
            <div class="flex flex-col lg:flex-row gap-8">
        
                // left sidebar (job titles)
                <div class="lg:w-1/3 space-y-3">
                    { for experience.iter().enumerate().map(|(index, exp)| {
                        let is_selected = *selected_job == index; // first item selected by default
                        let selected_job_clone = selected_job.clone();
                        let onclick = Callback::from(move |_| {
                            selected_job_clone.set(index);
                        });
                        html! {
                            <div class={format!("flex items-center p-4 cursor-pointer transition-all duration-300 {}", 
                                if is_selected { 
                                    "bg-red-600/30 border-3 border-red-600/40" 
                                } else { 
                                    "bg-gray-500/25 border-3 border-gray-900 hover:border-gray-600" 
                                }
                            )}
                            onclick={onclick}>
                                <div class="flex-shrink-0 mr-4">
                                    <div class="w-12 h-12 bg-gray-800 rounded-full flex items-center justify-center">
                                        <img 
                                            src={exp.icon.clone()} 
                                            alt="Institution logo" 
                                            class="w-10 h-10 object-contain rounded-full bg-white"
                                        />
                                    </div>
                                </div>
                                <div class="flex-1">
                                    <h3 class="text-white font-semibold text-lg mb-1">{&exp.title}</h3>
                                    <p class="text-gray-400 text-sm">{&exp.institution}</p>
                                </div>
                            </div>
                        }
                    })}
                </div>

                // right content (job details)
                <div class="lg:w-2/3">
                    <div 
                        class="relative p-8"                                                 
                        style="background-image: url('/static/STBAR7.png'); 
                                background-repeat: no-repeat; 
                                background-size: 100% 100%; 
                                image-rendering: pixelated;
                                min-height: 120px;
                                transparency: 0.9;"
                    >
                        // inner box
                        <div 
                            class="absolute inset-0 m-4 z-5 bg-[#1a1a1a] bg-opacity-60 border-4 border-[#0b0b0a]"
                        ></div>
                        
                        <div class="relative z-10">
                            <div class="flex items-center mb-6">
                                <div class="flex-shrink-0 mr-4">
                                    <div class="w-16 h-16 bg-gray-800 rounded-full flex items-center justify-center">
                                        <img 
                                            src={experience[*selected_job].icon.clone()}
                                            alt="Company logo" 
                                            class="w-13 h-13 object-contain rounded-full bg-white"
                                        />
                                    </div>
                                </div>
                                <div>
                                    <h2 class="text-2xl font-bold text-white mb-2">{&experience[*selected_job].title}</h2>
                                    <p class="text-red-600 text-lg mb-1">{&experience[*selected_job].institution}</p>
                                    <p class="text-gray-400 text-sm">{&experience[*selected_job].date}</p>
                                </div>
                            </div>

                            <div class="space-y-4">                                        
                                if let Some(description) = &experience[*selected_job].description {
                                    { for description.iter().map(|point| {
                                        html! {
                                            <li class="flex items-start">
                                                <div class="flex-shrink-0 w-2 h-2 bg-red-600 rounded-full mt-2 mr-3"></div>
                                                <p class="text-gray-300 leading-relaxed mb-2">{point}</p>
                                            </li>
                                        }
                                    })}
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}