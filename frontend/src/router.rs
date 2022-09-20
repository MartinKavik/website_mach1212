use crate::app;
use crate::app::PageId;
use std::collections::VecDeque;
use zoon::println;
use zoon::*;

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

#[route]
#[derive(Clone)]
pub enum Route {
    #[route()]
    Root,
    #[route("home")]
    Home,
    #[route("blog")]
    Blog,
}

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| async {
        println!("{}", routing::url());

        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => {
                return app::set_page_id(PageId::Unknown);
            }
        };

        match route {
            Route::Root => router().go("home"),
            Route::Home => app::set_page_id(PageId::Home),
            Route::Blog => app::set_page_id(PageId::Blog),
        }
    })
}
