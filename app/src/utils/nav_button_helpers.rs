// utils/nav_button_helpers.rs
// shared logic for HeaderButton and HudButton: both are three-state
// (disabled/active/normal) route buttons that differ only in markup/classes.
use crate::router::Route;

pub fn is_route_active(current: &Option<Route>, target: &Route) -> bool {
    match current {
        Some(current) => current == target,
        None => false,
    }
}

// builds the normal/active image paths from a button's base filepath
pub fn nav_image_paths(src: &str) -> (String, String) {
    (format!("{}_W.png", src), format!("{}_R.png", src))
}
