use zoon::{named_color::*, *};
mod app;

fn main() {
    start_app("app", app::view::root);
}
