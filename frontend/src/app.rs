use crate::home;
use zoon::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Home,
    Blog,
    Unknown,
}

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

#[static_ref]
fn dark_mode() -> &'static Mutable<bool> {
    Mutable::new(false)
}

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

fn page() -> impl Element {
    El::new().child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Home => home::page().into_raw_element(),
        PageId::Blog => El::new().child("Blog").into_raw_element(),
        PageId::Unknown => El::new().child("404").into_raw_element(),
    }))
}

pub fn root() -> impl Element {
    Column::new()
        .item(page())
}
