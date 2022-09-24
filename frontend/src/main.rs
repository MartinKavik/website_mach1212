use zoon::{*, start_app};
mod app;
mod router;

fn main() {
    router::router();
    start_app("app", app::root);
}
