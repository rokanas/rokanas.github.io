// src/hooks.rs
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[hook]
pub fn use_navigation() -> Callback<Route> {
    let navigator = use_navigator().unwrap();

    Callback::from(move |route: Route| {
        navigator.push(&route);
    })
}

// shared show/hide slide animation state machine for Header and Hud:
// returns (is_visible, should_render). should_render stays true for a beat after
// show becomes false so the slide-out transition can play before unmounting.
#[hook]
pub fn use_slide_visibility(show: bool) -> (bool, bool) {
    let is_visible = use_state(|| false);
    let should_render = use_state(|| show);

    {
        let is_visible = is_visible.clone();
        let should_render = should_render.clone();
        use_effect_with(show, move |show| {
            if *show {
                // show: first render component, then slide it in
                should_render.set(true);
                let is_visible_clone = is_visible.clone();
                gloo_timers::callback::Timeout::new(50, move || {
                    is_visible_clone.set(true);
                }).forget();
            } else {
                // hide: first slide out, then stop rendering
                is_visible.set(false);
                let should_render_clone = should_render.clone();
                gloo_timers::callback::Timeout::new(500, move || { // wait for animation to complete
                    should_render_clone.set(false);
                }).forget();
            }
            || {}
        });
    }

    (*is_visible, *should_render)
}

// shared image-carousel state for card galleries/lightboxes: returns
// (current_index, current_src, prev, next, reset). prev/next wrap around
// and stop event propagation (so clicking them doesn't also close an
// enclosing modal/lightbox); reset jumps back to the first image.
#[hook]
pub fn use_image_carousel(images: Vec<String>) -> (usize, String, Callback<MouseEvent>, Callback<MouseEvent>, Callback<()>) {
    let current_image_index = use_state(|| 0usize);

    let prev_image = {
        let current_image_index = current_image_index.clone();
        let total_images = images.len();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation(); // prevent modal/lightbox from closing
            let current = *current_image_index;
            let new_index = if current == 0 {
                total_images - 1
            } else {
                current - 1
            };
            current_image_index.set(new_index);
        })
    };

    let next_image = {
        let current_image_index = current_image_index.clone();
        let total_images = images.len();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation(); // prevent modal/lightbox from closing
            let current = *current_image_index;
            let new_index = (current + 1) % total_images;
            current_image_index.set(new_index);
        })
    };

    let reset = {
        let current_image_index = current_image_index.clone();
        Callback::from(move |_: ()| {
            current_image_index.set(0);
        })
    };

    let current_image_src = images.get(*current_image_index).cloned().unwrap_or_default();

    (*current_image_index, current_image_src, prev_image, next_image, reset)
}
