// data/doom_projects.rs
// hardcoded map/model content for the Doom Projects page

#[derive(Clone, PartialEq)]
pub struct Map {
    pub title: String,
    pub description: String,
    pub image_src: String,
    pub image_alt: Option<String>,
    pub additional_images: Vec<String>,
    pub border: String,
}

#[derive(Clone, PartialEq)]
pub struct Model {
    pub title: String,
    pub description: String,
    pub preview_image: String,
    pub model_name: String,
    pub download_url: Option<String>,
    pub file_size: Option<String>,
    pub credits: Option<String>,
    pub border: String,
}

pub fn all_maps() -> Vec<Map> {
    vec![
        Map {
            title: "Cathedral of Charybdis".to_string(),
            description: "A dark and atmospheric map. All are swallowed by the shadow of the cathedral. Can you resist the evil cult of Charybdis?".to_string(),
            image_src: "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_1.webp".to_string(),
            image_alt: Some("Cathedral of Charybdis".to_string()),
            additional_images: vec![
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_2.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_3.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_4.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_5.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_6.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_7.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_8.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_9.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_10.webp".to_string(),
                "/static/doom_projects/cathedral_of_charybdis/cathedral_of_charybdis_11.webp".to_string(),
            ],
            border: "/static/doom_projects/cathedral_of_charybdis/ADEL_B15_3.webp".to_string(),
        },
        Map {
            title: "Jammy".to_string(),
            description: "A gimmicky challenge map involving a lot of scripted terrain transformation and light slaughter. Inspired by Doom64 MAP19. Push through and don't stand still!".to_string(),
            image_src: "/static/doom_projects/jammy/jammy_1.webp".to_string(),
            image_alt: Some("Jammy".to_string()),
            additional_images: vec![
                "/static/doom_projects/jammy/jammy_2.webp".to_string(),
                "/static/doom_projects/jammy/jammy_3.webp".to_string(),
                "/static/doom_projects/jammy/jammy_4.webp".to_string(),
                "/static/doom_projects/jammy/jammy_5.webp".to_string(),
                "/static/doom_projects/jammy/jammy_6.webp".to_string(),
            ],
            border: "/static/doom_projects/jammy/OB8_0N_4.webp".to_string(),

        },
        Map {
            title: "Whispers of Change".to_string(),
            description: "A short and atmospheric map with story elements and light puzzles. Co-authored by Erik Lindstrand and made in 1 day for Chalmers March GameJam 2024.".to_string(),
            image_src: "/static/doom_projects/whispers_of_change/whispers_of_change_1.webp".to_string(),
            image_alt: Some("Whispers of Change".to_string()),
            additional_images: vec![
                "/static/doom_projects/whispers_of_change/whispers_of_change_2.webp".to_string(),
                "/static/doom_projects/whispers_of_change/whispers_of_change_3.webp".to_string(),
                "/static/doom_projects/whispers_of_change/whispers_of_change_4.webp".to_string(),
                "/static/doom_projects/whispers_of_change/whispers_of_change_5.webp".to_string(),
            ],
            border: "/static/doom_projects/whispers_of_change/ADEL_G02_2.webp".to_string(),
        },
        Map {
            title: "SWEDEN".to_string(),
            description: "An adventure map that has nothing to do with Sweden. Explore the demonic presence aroused in the ruins by human interference.".to_string(),
            image_src: "/static/doom_projects/sweden/sweden_1.webp".to_string(),
            image_alt: Some("SWEDEN".to_string()),
            additional_images: vec![
                "/static/doom_projects/sweden/sweden_2.webp".to_string(),
                "/static/doom_projects/sweden/sweden_3.webp".to_string(),
                "/static/doom_projects/sweden/sweden_4.webp".to_string(),
                "/static/doom_projects/sweden/sweden_5.webp".to_string(),
                "/static/doom_projects/sweden/sweden_6.webp".to_string(),
                "/static/doom_projects/sweden/sweden_7.webp".to_string(),
                "/static/doom_projects/sweden/sweden_8.webp".to_string(),
            ],
            border: "/static/doom_projects/sweden/STONE6_2.webp".to_string(),

        },
        Map {
            title: "ΣΣΑΣ".to_string(),
            description: "A map that is definitely not inspired by a real military base. Discover the hellish secrets buried beneath military inefficiency and bureaucracy!".to_string(),
            image_src: "/static/doom_projects/ssas/ssas_1.webp".to_string(),
            image_alt: Some("ΣΣΑΣ".to_string()),
            additional_images: vec![
                "/static/doom_projects/ssas/ssas_2.webp".to_string(),
                "/static/doom_projects/ssas/ssas_3.webp".to_string(),
                "/static/doom_projects/ssas/ssas_4.webp".to_string(),
                "/static/doom_projects/ssas/ssas_5.webp".to_string(),
                "/static/doom_projects/ssas/ssas_6.webp".to_string(),
                "/static/doom_projects/ssas/ssas_7.webp".to_string(),
                "/static/doom_projects/ssas/ssas_8.webp".to_string(),
                "/static/doom_projects/ssas/ssas_9.webp".to_string(),
                "/static/doom_projects/ssas/ssas_10.webp".to_string(),
                "/static/doom_projects/ssas/ssas_11.webp".to_string(),
                "/static/doom_projects/ssas/ssas_12.webp".to_string(),
                "/static/doom_projects/ssas/ssas_13.webp".to_string(),
                "/static/doom_projects/ssas/ssas_14.webp".to_string(),
                "/static/doom_projects/ssas/ssas_15.webp".to_string(),
                "/static/doom_projects/ssas/ssas_16.webp".to_string(),
            ],
            border: "/static/doom_projects/ssas/SP_HOT1.webp".to_string(),
        },
    ]
}

pub fn all_models() -> Vec<Model> {
    vec![
        Model {
            title: "Unholy Cathedral".to_string(),
            description: "An evil cathedral inspired by the Kölner Dom, originally made for the Cathedral of Charybdis map.".to_string(),
            preview_image: "/static/models/unholy_cathedral/unholy_cathedral_preview.webp".to_string(),
            model_name: "unholy_cathedral".to_string(),
            download_url: None, // set to Some("/static/downloads/cathedral.zip".to_string()) when ready
            file_size: Some("11.6 MB".to_string()),
            credits: Some(
                "Model made by Bifteki using Ultimate Doom Builder & Blender \n
                All textures from GothicTX by Adelusion et al. and from Doom II by id Software \n
                Inspired by the Kölner Dom in Cologne, Germany."
                .to_string()
            ),
            border: "/static/doom_projects/cathedral_of_charybdis/ADEL_B15_3.webp".to_string(),
        },
        Model {
            title: "Scylla".to_string(),
            description: "A car inspired by the classic Ford Mustang design, originally made for the Cathedral of Charybdis map.".to_string(),
            preview_image: "/static/models/scylla/scylla_preview.webp".to_string(),
            model_name: "scylla".to_string(),
            download_url: None,
            file_size: Some("169 KB".to_string()),
            credits: Some(
                "Model made by Bifteki using Ultimate Doom Builder & Blender \n
                Head lights, steering wheel, license plate and car name textures by Bifteki. \n
                Grille & tire textures from CarPack by AuroraTheKitsune
                (https://www.doomworld.com/forum/topic/138609-carpack-car-truck-textures-for-doom/) \n
                Tail lights and interior textures from GothicTX by Adelusion et al. \n
                All other textures from Doom II by id Software \n
                Inspired by the classic Ford Mustang."
                .to_string()
            ),
            border: "/static/models/scylla/SHAWN4.webp".to_string(),
        },
        Model {
            title: "ΚΑΝΑΔΕΖΑ".to_string(),
            description: "A doomcute vehicle inspired by the trucks used by the Hellenic military, originally made for the ΣΣΑΣ map.".to_string(),
            preview_image: "/static/models/kanadeza/kanadeza_preview.webp".to_string(),
            model_name: "kanadeza".to_string(),
            download_url: None,
            file_size: Some("146 KB".to_string()),
            credits: Some(
                "Model made by Bifteki using Ultimate Doom Builder & Blender \n
                Grille & tire textures from CarPack by AuroraTheKitsune
                (https://www.doomworld.com/forum/topic/138609-carpack-car-truck-textures-for-doom/) \n
                All other textures from Doom II by id Software \n
                Inspired by the Καναδέζα trucks used by the Hellenic military."
                .to_string()
            ),
            border: "/static/models/kanadeza/BROWNGRN_2.webp".to_string(),
        },
    ]
}
