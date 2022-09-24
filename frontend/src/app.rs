use crate::web_sys::HtmlAnchorElement;
use zoon::button::OnPressFlagNotSet;
use zoon::el::ChildFlagSet;
use zoon::link::ToFlagSet;
use zoon::web_sys::{HtmlDivElement, HtmlElement};
use zoon::*;

mod home;

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
    Column::new().item(page())
}

/***************************************/
/* Constants                          */
/***************************************/

pub const BACKGROUND: [HSLuv; 3] = [
    // #f8f9fa
    hsluv! {235.5, 18.4, 97.9},
    // #f1f3f5
    hsluv! {235.7, 18.3, 95.7},
    // #e9ecef
    hsluv! {235.8, 17.2, 93.3},
];

pub const PAGE_WIDTH: u32 = 1500;
// #edf2ff
pub const ACCENT_BACK: HSLuv = hsluv! {252.3, 100.0, 95.5};
// #5c7cfa
pub const ACCENT_TINT: HSLuv = hsluv! {260.9, 96.1, 55.7};
// #4263eb
pub const ACCENT: HSLuv = hsluv! {261.9, 87.7, 47.1};
// #364fc7
pub const ACCENT_SHADE: HSLuv = hsluv! {262.5, 79.0, 38.7};
// #343a40
pub const TEXT: [HSLuv; 2] = [hsluv! {237.8, 19.6, 24.1}, hsluv! {236.9, 12.3, 58.6}];

pub const SPACING: [u32; 12] = [2, 4, 8, 12, 16, 24, 32, 48, 64, 80, 96, 128];
pub const SIZE: [u32; 15] = [10, 12, 14, 16, 18, 20, 24, 30, 36, 44, 52, 62, 74, 86, 98];

/***************************************/
/* Components                          */
/***************************************/

pub fn section<T: Element>(
    background: HSLuv,
    child: T,
) -> El<ChildFlagSet, RawHtmlEl<HtmlElement>> {
    El::new()
        .s(Background::new().color(background))
        .s(Padding::new().x(SPACING[2]))
        .child(child)
}

pub fn make_button(
    label: &str,
    color: HSLuv,
    background: HSLuv,
    background_active: HSLuv,
) -> Button<button::LabelFlagSet, button::OnPressFlagNotSet, RawHtmlEl<HtmlDivElement>> {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .label(label)
        .s(Padding::new().x(16).y(8))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(move || background_active, move || background)))
        .s(RoundedCorners::all(9))
        .s(Font::new().color(color))
        .on_hovered_change(move |change| hovered.set_neq(change))
}

pub fn make_link(
    label: &str,
    link: &str,
    width: u32,
) -> Link<link::LabelFlagSet, ToFlagSet, RawHtmlEl<HtmlAnchorElement>> {
    Link::new()
        .label(
            Row::new().item(label).item(
                Image::new()
                    .url(public_url("icons/out_box.svg"))
                    .description("icon")
                    .s(Width::exact(width)),
            ),
        )
        .to(link)
}
