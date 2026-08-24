// components/hud_avatar.rs
use yew::prelude::*;
use web_sys::{window, HtmlElement};
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;

use crate::router::Route;
use crate::hooks::use_navigation;

const GRID_COLS: i32 = 5;
const CENTER_COL: i32 = GRID_COLS / 2;
const SPRITE_TOGGLE_INTERVAL_MS: u32 = 500;

// horizontal mouse movement tilts the sprite in 3D (rotateY); vertical spins
// it flat in 2D (rotate), on a separate nested element so the 3D tilt
// flattens before the roll applies — keeps the roll direction consistent.
// exception: in the dead-center column, vertical also uses 3D (rotateX).
const PERSPECTIVE_PX: f64 = 500.0;
const MAX_ROTATE_X_DEG: f64 = 14.0;
const MAX_ROTATE_Y_DEG: f64 = 14.0;
const MAX_ROTATE_Z_DEG: f64 = 14.0;
// vertical rotation is asymmetric: strong at the top, almost flat at the bottom.
const VERTICAL_ROTATE_MIN_DEG: f64 = -1.0;

// mouse at the top (y_norm=0) -> max_deg; mouse at the bottom (y_norm=1) -> just below neutral.
fn vertical_rotate_deg(y_norm: f64, max_deg: f64) -> f64 {
    max_deg + (VERTICAL_ROTATE_MIN_DEG - max_deg) * y_norm
}

// alternates the sprite between its upper/lower pose on a timer instead of vertical mouse position.
#[hook]
fn use_sprite_toggle() -> bool {
    let is_upper = use_state(|| true);

    {
        let is_upper = is_upper.clone();
        use_effect_with((), move |_| {
            // tracks state itself, since reading *is_upper here would be stale
            // (this closure is only ever created once).
            let current = std::rc::Rc::new(std::cell::Cell::new(true));
            let interval = Interval::new(SPRITE_TOGGLE_INTERVAL_MS, move || {
                let next = !current.get();
                current.set(next);
                is_upper.set(next);
            });
            move || drop(interval)
        });
    }

    *is_upper
}

// grid_col drives the sprite swap via Yew state; roll/tilt refs get the
// continuous transform written directly, bypassing re-render.
#[hook]
fn use_avatar_tracking() -> (i32, NodeRef, NodeRef) {
    let col_state = use_state(|| CENTER_COL);
    let roll_ref = use_node_ref();
    let tilt_ref = use_node_ref();

    {
        let col_state = col_state.clone();
        let roll_ref = roll_ref.clone();
        let tilt_ref = tilt_ref.clone();
        use_effect_with((), move |_| {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();

            let move_roll_ref = roll_ref.clone();
            let move_tilt_ref = tilt_ref.clone();
            let move_listener = EventListener::new(&body, "mousemove", move |event| {
                if let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() {
                    let x = mouse_event.client_x() as f64;
                    let y = mouse_event.client_y() as f64;

                    if let (Ok(viewport_width), Ok(viewport_height)) = (
                        window.inner_width(),
                        window.inner_height()
                    ) {
                        let viewport_width = viewport_width.as_f64().unwrap_or(1920.0);
                        let viewport_height = viewport_height.as_f64().unwrap_or(1080.0);

                        let x_norm = (x / viewport_width).clamp(0.0, 1.0);
                        let y_norm = (y / viewport_height).clamp(0.0, 1.0);

                        let col = ((x_norm * GRID_COLS as f64).floor() as i32).clamp(0, GRID_COLS - 1);
                        // set unconditionally: this closure is only created once, so a
                        // "changed" check against *col_state here would read a stale value.
                        col_state.set(col);

                        let rotate_y = (x_norm - 0.5) * 2.0 * MAX_ROTATE_Y_DEG;

                        // dead-center column: vertical drives a 3D tilt (rotateX) instead of
                        // the flat roll, since there's no mirrored left/right art to fix up.
                        let (roll_transform, tilt_transform) = if col == CENTER_COL {
                            let rotate_x = vertical_rotate_deg(y_norm, MAX_ROTATE_X_DEG);
                            (
                                "none".to_string(),
                                format!("perspective({PERSPECTIVE_PX}px) rotateX({rotate_x:.2}deg) rotateY({rotate_y:.2}deg)"),
                            )
                        } else {
                            // sprite art is mirrored left/right, so flip the roll sign on the
                            // right half to keep "mouse up" reading as tilting up.
                            let roll_side_sign = if x_norm < 0.5 { 1.0 } else { -1.0 };
                            let rotate_z = vertical_rotate_deg(y_norm, MAX_ROTATE_Z_DEG) * roll_side_sign;
                            (
                                format!("rotate({rotate_z:.2}deg)"),
                                format!("perspective({PERSPECTIVE_PX}px) rotateY({rotate_y:.2}deg)"),
                            )
                        };

                        if let Some(el) = move_roll_ref.cast::<HtmlElement>() {
                            let _ = el.style().set_property("transform", &roll_transform);
                        }
                        if let Some(el) = move_tilt_ref.cast::<HtmlElement>() {
                            let _ = el.style().set_property("transform", &tilt_transform);
                        }
                    }
                }
            });

            // reset to neutral once the cursor leaves the window
            let leave_roll_ref = roll_ref.clone();
            let leave_tilt_ref = tilt_ref.clone();
            let leave_listener = EventListener::new(&document, "mouseleave", move |_| {
                if let Some(el) = leave_roll_ref.cast::<HtmlElement>() {
                    let _ = el.style().set_property("transform", "none");
                }
                if let Some(el) = leave_tilt_ref.cast::<HtmlElement>() {
                    let _ = el.style().set_property("transform", "none");
                }
            });

            move || {
                drop(move_listener);
                drop(leave_listener);
            }
        });
    }

    (*col_state, roll_ref, tilt_ref)
}

fn get_avatar_image(col: i32, is_upper: bool, is_hover: bool) -> String {
    if is_hover {
        return "/static/hud/avatar/AVATAR_4.png".to_string();
    }

    if is_upper {
        match col {
            0 => "/static/hud/avatar/AVATAR_TOP_LEFT.png".to_string(),
            1 => "/static/hud/avatar/AVATAR_TOP_CENTER_LEFT.png".to_string(),
            2 => "/static/hud/avatar/AVATAR_TOP_CENTER.png".to_string(),
            3 => "/static/hud/avatar/AVATAR_TOP_CENTER_RIGHT.png".to_string(),
            4 => "/static/hud/avatar/AVATAR_TOP_RIGHT.png".to_string(),
            _ => "/static/hud/avatar/AVATAR_1.png".to_string(),
        }
    } else {
        match col {
            0 => "/static/hud/avatar/AVATAR_BOTTOM_LEFT.png".to_string(),
            1 => "/static/hud/avatar/AVATAR_BOTTOM_CENTER_LEFT.png".to_string(),
            2 => "/static/hud/avatar/AVATAR_BOTTOM_CENTER.png".to_string(),
            3 => "/static/hud/avatar/AVATAR_BOTTOM_CENTER_RIGHT.png".to_string(),
            4 => "/static/hud/avatar/AVATAR_BOTTOM_RIGHT.png".to_string(),
            _ => "/static/hud/avatar/AVATAR_1.png".to_string(),
        }
    }
}

#[function_component(HudAvatar)]
pub fn hud_avatar() -> Html {
    let is_upper = use_sprite_toggle();
    let (col, roll_ref, tilt_ref) = use_avatar_tracking();
    let navigate = use_navigation();

    html! {
        <button
            onclick={navigate.reform(|_| Route::Home)}
            class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
            // sized/positioned against HudSection, not the button (which collapses to
            // 0x0) — the transform has to live here rather than on the button itself.
            <div ref={roll_ref} class="w-4/5 absolute transition-transform duration-150 ease-out will-change-transform">
                // separate element so the 3D tilt flattens before the roll applies
                <div ref={tilt_ref} class="transition-transform duration-150 ease-out will-change-transform">
                    <img
                        src={get_avatar_image(col, is_upper, false)}
                        alt="Avatar"
                        class="w-full block transition-opacity duration-200 ease-in-out group-hover:opacity-0"
                    />
                    <img
                        src={get_avatar_image(col, is_upper, true)}
                        alt="Avatar"
                        class="w-full h-full block absolute inset-0 opacity-0 transition-opacity duration-200 ease-in-out group-hover:opacity-100"
                    />
                </div>
            </div>
        </button>
    }
}
