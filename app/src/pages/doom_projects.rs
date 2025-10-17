// pages/doom_projects.rs
use yew::prelude::*;
use web_sys::window;
use crate::components::doom_map_item::{DoomMapItem};
use crate::components::heading::{Heading};

// struct to hold project data
#[derive(Clone, PartialEq)]
pub struct Map {
    pub title: String,
    pub description: String,
    pub image_src: String,
    pub image_alt: Option<String>,
    pub additional_images: Vec<String>,
}

// TODO: make smaller thumbnails if page loads slowly (involves refactoring image_src to thumbnail_src)
#[function_component(DoomProjects)]
pub fn doom_projects() -> Html {
    use_effect_with((), {
        move |_| {
            // scroll to top when component mounts
            if let Some(window) = window() {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
            || {}
        }
    });

    // map item definitions (temporarily hardcoded, can be moved to db later)
    let maps = vec![
        Map {
            title: "Cathedral of Charybdis".to_string(),
            description: "A dark and atmospheric map. All are swallowed by the shadow of the cathedral. Can you resist the evil cult of Charybdis?".to_string(),
            image_src: "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_1.png".to_string(),
            image_alt: Some("Cathedral of Charybdis".to_string()),
            additional_images: vec![
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_2.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_3.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_4.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_5.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_6.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_7.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_8.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_9.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_10.png".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_11.png".to_string(),
            ],
        },
        Map {
            title: "Jammy".to_string(),
            description: "A gimmicky challenge map involving a lot of scripted terrain transformation and light slaughter. Inspired by Doom64 MAP19. Push through and don't stand still!".to_string(),
            image_src: "/static/doom_projects/jammy/jammy_1.png".to_string(),
            image_alt: Some("Jammy".to_string()),
            additional_images: vec![
                "/static/doom_projects/jammy/jammy_2.png".to_string(),
                "/static/doom_projects/jammy/jammy_3.png".to_string(),
                "/static/doom_projects/jammy/jammy_4.png".to_string(),
                "/static/doom_projects/jammy/jammy_5.png".to_string(),
                "/static/doom_projects/jammy/jammy_6.png".to_string(),
            ],

        },
        Map {
            title: "Whispers of Change".to_string(),
            description: "A short and atmospheric map with story elements and light puzzles. Co-authored by Erik Lindstrand and made in 1 day for Chalmers March GameJam 2024.".to_string(),
            image_src: "/static/doom_projects/whispers_of_change/whispers_of_change_1.png".to_string(),
            image_alt: Some("Whispers of Change".to_string()),
            additional_images: vec![
                "/static/doom_projects/whispers_of_change/whispers_of_change_2.png".to_string(),
                "/static/doom_projects/whispers_of_change/whispers_of_change_3.png".to_string(),
                "/static/doom_projects/whispers_of_change/whispers_of_change_4.png".to_string(),
                "/static/doom_projects/whispers_of_change/whispers_of_change_5.png".to_string(),
            ],
        },
        Map {
            title: "SWEDEN".to_string(),
            description: "An adventure map that has nothing to do with Sweden. Explore the demonic presence aroused in the ruins by human interference.".to_string(),
            image_src: "/static/doom_projects/sweden/sweden_1.png".to_string(),
            image_alt: Some("SWEDEN".to_string()),
            additional_images: vec![
                "/static/doom_projects/sweden/sweden_2.png".to_string(),
                "/static/doom_projects/sweden/sweden_3.png".to_string(),
                "/static/doom_projects/sweden/sweden_4.png".to_string(),
                "/static/doom_projects/sweden/sweden_5.png".to_string(),
                "/static/doom_projects/sweden/sweden_6.png".to_string(),
                "/static/doom_projects/sweden/sweden_7.png".to_string(),
                "/static/doom_projects/sweden/sweden_8.png".to_string(),
            ],
            
        },
        Map {
            title: "ΣΣΑΣ".to_string(),
            description: "A map that is definitely not inspired by a real military base. Discover the hellish secrets buried beneath military inefficiency and bureaucracy!".to_string(),
            image_src: "/static/doom_projects/ssas/ssas_1.png".to_string(),
            image_alt: Some("ΣΣΑΣ".to_string()),
            additional_images: vec![
                "/static/doom_projects/ssas/ssas_2.png".to_string(),
                "/static/doom_projects/ssas/ssas_3.png".to_string(),
                "/static/doom_projects/ssas/ssas_4.png".to_string(),
                "/static/doom_projects/ssas/ssas_5.png".to_string(),
                "/static/doom_projects/ssas/ssas_6.png".to_string(),
                "/static/doom_projects/ssas/ssas_7.png".to_string(),
                "/static/doom_projects/ssas/ssas_8.png".to_string(),
                "/static/doom_projects/ssas/ssas_9.png".to_string(),
                "/static/doom_projects/ssas/ssas_10.png".to_string(),
                "/static/doom_projects/ssas/ssas_11.png".to_string(),
                "/static/doom_projects/ssas/ssas_12.png".to_string(),
                "/static/doom_projects/ssas/ssas_13.png".to_string(),
                "/static/doom_projects/ssas/ssas_14.png".to_string(),
                "/static/doom_projects/ssas/ssas_15.png".to_string(),
                "/static/doom_projects/ssas/ssas_16.png".to_string(),
            ],
        },
    ];

    html! {
        <main class="min-h-screen text-white pt-8 pb-10">
            <div class="container mx-auto pt-4 px-4 max-w-7xl">
                
                // page heading
                <Heading 
                    src="/static/doom_projects/DOOM_PROJECTS_1.png" 
                    alt="Doom Projects"
                    sub_heading="All maps made for limit-removing source ports and tested in GZDoom."
                />

                // projects grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 justify-items-center">
                    { for maps.iter().map(|map| html! {
                        <DoomMapItem
                            title={map.title.clone()}
                            description={map.description.clone()}
                            image_src={map.image_src.clone()}
                            image_alt={map.image_alt.clone()}
                            additional_images={map.additional_images.clone()}
                        />
                    })}
                </div>

                // footer note
                <div class="text-center mt-10 text-gray-500">
                    <p class="font-mono text-sm">
                        {"More projects available on my "} 
                        <a 
                            href="https://github.com/rokanas" 
                            target="_blank" 
                            class="text-red-600 hover:text-red-300 underline transition-colors duration-200"
                        >
                            {"GitHub profile"}
                        </a>
                    </p>
                </div>
            </div>
        </main>
    }
}