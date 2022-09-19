use zoon::{named_color::*, *};
mod app;
mod home;
mod router;

fn main() {
    router::router();
    start_app("app", app::root);
}
