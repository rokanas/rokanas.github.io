// components/hud_avatar.rs
use yew::prelude::*;
use web_sys::{window, HtmlElement};
use gloo_events::EventListener;
use gloo_timers::callback::{Interval, Timeout};
use wasm_bindgen::JsCast;

use crate::router::Route;
use crate::hooks::use_navigation;

const GRID_COLS: i32 = 5;
const CENTER_COL: i32 = GRID_COLS / 2;
const SPRITE_TOGGLE_INTERVAL_MS: u32 = 500;
const HOVER_FRAME_COUNT: u8 = 4;
const HOVER_FRAME_INTERVAL_MS: u32 = 80;
const CLICK_FRAME_COUNT: u8 = 7;
const CLICK_FRAME_INTERVAL_MS: u32 = 120;

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

// alternates the sprite between its two poses (AVATAR_*_1 / AVATAR_*_2) on a timer
#[hook]
fn use_sprite_toggle() -> u8 {
    let frame = use_state(|| 1u8);

    {
        let frame = frame.clone();
        use_effect_with((), move |_| {
            // tracks state itself, since reading *frame here would be stale
            // (this closure is only ever created once).
            let current = std::rc::Rc::new(std::cell::Cell::new(1u8));
            let interval = Interval::new(SPRITE_TOGGLE_INTERVAL_MS, move || {
                let next = if current.get() == 1 { 2 } else { 1 };
                current.set(next);
                frame.set(next);
            });
            move || drop(interval)
        });
    }

    *frame
}

// on hover, rapidly steps through _HOVER sprites and holds on frame 4;
// each new hover restarts the cycle from frame 1.
#[hook]
fn use_hover_animation() -> (u8, Callback<MouseEvent>, Callback<MouseEvent>) {
    let frame = use_state(|| 1u8);
    let pending = use_mut_ref(Vec::<Timeout>::new);

    let onmouseenter = {
        let frame = frame.clone();
        let pending = pending.clone();
        Callback::from(move |_: MouseEvent| {
            pending.borrow_mut().clear(); // cancel leftovers from a rapid re-hover
            frame.set(1);
            for step in 2..=HOVER_FRAME_COUNT {
                let frame = frame.clone();
                let delay = HOVER_FRAME_INTERVAL_MS * (step as u32 - 1);
                pending.borrow_mut().push(Timeout::new(delay, move || frame.set(step)));
            }
        })
    };

    let onmouseleave = {
        let pending = pending.clone();
        Callback::from(move |_: MouseEvent| pending.borrow_mut().clear())
    };

    (*frame, onmouseenter, onmouseleave)
}

// on click, rapidly steps through AVATAR_CLICK_1..CLICK_6 — always to completion,
// even if the mouse leaves mid-animation. Once the last frame is reached, it
// lingers there while still hovered, and only reverts to None (back to the
// normal hover/grid sprite) once the mouse actually leaves — immediately if
// that happens after the animation already finished, or right as the last
// frame lands if the mouse had already left earlier.
#[hook]
fn use_click_animation() -> (Option<u8>, bool, Callback<MouseEvent>, Callback<MouseEvent>, Callback<MouseEvent>) {
    let click_frame = use_state(|| None::<u8>);
    let pending = use_mut_ref(Vec::<Timeout>::new);
    let is_animating = use_mut_ref(|| false);
    // two trackers for hover, deliberately: the RefCell is read from inside
    // Timeout callbacks (which are only ever created once, so a UseStateHandle
    // read there would be a stale snapshot — see the col_state comment above);
    // the UseStateHandle exists purely so the component can react to hover
    // changes when deciding whether to keep the overlay visible.
    let is_hovering_cell = use_mut_ref(|| false);
    let is_hovering_state = use_state(|| false);

    let onclick = {
        let click_frame = click_frame.clone();
        let pending = pending.clone();
        let is_hovering_cell = is_hovering_cell.clone();
        let is_animating = is_animating.clone();
        Callback::from(move |_: MouseEvent| {
            pending.borrow_mut().clear(); // cancel leftovers from a rapid re-click
            *is_animating.borrow_mut() = true;
            click_frame.set(Some(1));
            for step in 2..CLICK_FRAME_COUNT {
                let click_frame = click_frame.clone();
                let delay = CLICK_FRAME_INTERVAL_MS * (step as u32 - 1);
                pending.borrow_mut().push(Timeout::new(delay, move || click_frame.set(Some(step))));
            }
            // last frame decides whether to linger (still hovering) or revert (already left)
            let click_frame = click_frame.clone();
            let is_hovering_cell = is_hovering_cell.clone();
            let is_animating = is_animating.clone();
            let last_delay = CLICK_FRAME_INTERVAL_MS * (CLICK_FRAME_COUNT as u32 - 1);
            pending.borrow_mut().push(Timeout::new(last_delay, move || {
                *is_animating.borrow_mut() = false;
                if *is_hovering_cell.borrow() {
                    click_frame.set(Some(CLICK_FRAME_COUNT));
                } else {
                    click_frame.set(None);
                }
            }));
        })
    };

    let onmouseenter = {
        let is_hovering_cell = is_hovering_cell.clone();
        let is_hovering_state = is_hovering_state.clone();
        Callback::from(move |_: MouseEvent| {
            *is_hovering_cell.borrow_mut() = true;
            is_hovering_state.set(true);
        })
    };

    let onmouseleave = {
        let click_frame = click_frame.clone();
        let is_hovering_state = is_hovering_state.clone();
        Callback::from(move |_: MouseEvent| {
            *is_hovering_cell.borrow_mut() = false;
            is_hovering_state.set(false);
            // only reset right away if the animation isn't still in flight —
            // otherwise its own last-frame step will pick up on is_hovering itself
            if !*is_animating.borrow() {
                click_frame.set(None);
            }
        })
    };

    (*click_frame, *is_hovering_state, onclick, onmouseenter, onmouseleave)
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

fn get_avatar_image(col: i32, frame: u8) -> String {
    let column_name = match col {
        0 => "LEFT",
        1 => "LEFT_CENTER",
        2 => "CENTER",
        3 => "RIGHT_CENTER",
        4 => "RIGHT",
        _ => return "/static/hud/avatar/AVATAR_HOVER_1.png".to_string(),
    };

    format!("/static/hud/avatar/AVATAR_{column_name}_{frame}.png")
}

fn get_hover_image(hover_frame: u8) -> String {
    format!("/static/hud/avatar/AVATAR_HOVER_{hover_frame}.png")
}

fn get_click_image(click_frame: u8) -> String {
    format!("/static/hud/avatar/AVATAR_CLICK_{click_frame}.png")
}

#[function_component(HudAvatar)]
pub fn hud_avatar() -> Html {
    let frame = use_sprite_toggle();
    let (col, roll_ref, tilt_ref) = use_avatar_tracking();
    let (hover_frame, hover_onmouseenter, hover_onmouseleave) = use_hover_animation();
    let (click_frame, click_is_hovering, animate_click, click_onmouseenter, click_onmouseleave) = use_click_animation();
    let navigate = use_navigation();

    // a click can only happen while already hovering, so the click animation has to
    // override the hover-overlay layer, not the base layer it already sits on top of.
    let overlay_image = match click_frame {
        Some(step) => get_click_image(step),
        None => get_hover_image(hover_frame),
    };

    // the overlay must stay visible for the rest of a click animation even after
    // the mouse leaves, so its opacity can't be pure-CSS :hover driven like the
    // rest of the crossfade — it needs to also account for click_frame directly.
    let overlay_active = click_is_hovering || click_frame.is_some();
    let base_class = if overlay_active {
        "w-full block transition-opacity duration-200 ease-in-out opacity-0"
    } else {
        "w-full block transition-opacity duration-200 ease-in-out opacity-100"
    };
    let overlay_class = if overlay_active {
        "w-full h-full block absolute inset-0 transition-opacity duration-200 ease-in-out opacity-100"
    } else {
        "w-full h-full block absolute inset-0 transition-opacity duration-200 ease-in-out opacity-0"
    };

    let onclick = Callback::from(move |e: MouseEvent| {
        animate_click.emit(e);
        navigate.emit(Route::Home);
    });

    let onmouseenter = Callback::from(move |e: MouseEvent| {
        hover_onmouseenter.emit(e.clone());
        click_onmouseenter.emit(e);
    });
    let onmouseleave = Callback::from(move |e: MouseEvent| {
        hover_onmouseleave.emit(e.clone());
        click_onmouseleave.emit(e);
    });

    html! {
        <button
            onclick={onclick}
            onmouseenter={onmouseenter}
            onmouseleave={onmouseleave}
            class="w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
            // sized/positioned against HudSection, not the button (which collapses to
            // 0x0) — the transform has to live here rather than on the button itself.
            <div ref={roll_ref} class="w-4/5 absolute transition-transform duration-150 ease-out will-change-transform">
                // separate element so the 3D tilt flattens before the roll applies
                <div ref={tilt_ref} class="transition-transform duration-150 ease-out will-change-transform">
                    <img
                        src={get_avatar_image(col, frame)}
                        alt="Avatar"
                        class={base_class}
                    />
                    <img
                        src={overlay_image}
                        alt="Avatar"
                        class={overlay_class}
                    />
                </div>
            </div>
        </button>
    }
}
