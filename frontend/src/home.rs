use crate::web_sys::{HtmlDivElement, HtmlElement};
use zoon::button::{LabelFlagSet, OnPressFlagNotSet};
use zoon::el::ChildFlagSet;
use zoon::named_color::{GRAY_3, GRAY_4, GRAY_5, GRAY_6, GRAY_7, GRAY_9};
use zoon::routing::back;
use zoon::*;

const BACKGROUND: [HSLuv; 3] = [
    // #f8f9fa
    hsluv! {235.5, 18.4, 97.9},
    // #f1f3f5
    hsluv! {235.7, 18.3, 95.7},
    // #e9ecef
    hsluv! {235.8, 17.2, 93.3},
];
// #edf2ff
const ACCENT_BACK: HSLuv = hsluv! {252.3, 100.0, 95.5};
// #5c7cfa
const ACCENT_TINT: HSLuv = hsluv! {260.9, 96.1, 55.7};
// #4263eb
const ACCENT: HSLuv = hsluv! {261.9, 87.7, 47.1};
// #364fc7
const ACCENT_SHADE: HSLuv = hsluv! {262.5, 79.0, 38.7};
// #343a40
const TEXT: HSLuv = hsluv! {237.8, 19.6, 24.1};

const SPACING: [u32; 12] = [2, 4, 8, 12, 16, 24, 32, 48, 64, 80, 96, 128];
const SIZE: [u32; 15] = [10, 12, 14, 16, 18, 20, 24, 30, 36, 44, 52, 62, 74, 86, 98];

pub fn page() -> impl Element {
    Column::new()
        .s(Font::new().color(TEXT).size(SIZE[6]))
        .s(Background::new().color(BACKGROUND[0]))
        .item(introduction())
        .item(about_z_section())
        .item(call_to_action())
}

fn section<T: Element>(background: HSLuv, child: T) -> El<ChildFlagSet, RawHtmlEl<HtmlElement>> {
    El::new()
        .s(Background::new().color(background))
        .child(child)
}

fn make_button(
    label: &str,
    color: HSLuv,
    background: HSLuv,
    background_active: HSLuv,
) -> Button<LabelFlagSet, OnPressFlagNotSet, RawHtmlEl<HtmlDivElement>> {
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

fn introduction() -> impl Element {
    section(
        ACCENT_BACK,
        Row::new()
            .s(Padding::new().y(SPACING[6]))
            .s(Align::center())
            .s(Gap::both(SPACING[9]))
            .item(
                El::new().child(
                    Column::new()
                        .s(Gap::both(SPACING[5]))
                        .item(
                            El::new()
                                .child("Hi, I am Maciej Pruchnik, a software engineer")
                                .s(Width::exact(800))
                                .s(Font::new().size(SIZE[12]).line_height(SPACING[9])),
                        )
                        .item(
                            El::new()
                                .child("Solving your problems with elegant solutions")
                                .s(Font::new().size(SIZE[5]).italic()),
                        )
                        .item(
                            make_button("Email me", ACCENT_BACK, ACCENT, ACCENT_SHADE)
                                .s(Align::new().left()),
                        ),
                ),
            )
            .item(
                Image::new()
                    .url(public_url("self_website.jpg"))
                    .description("An image of myself")
                    .s(Width::growable().max(500))
                    .s(RoundedCorners::all(9))
                    .s(Shadows::new([Shadow::new().color(GRAY_4).blur(SPACING[7])])),
            ),
    )
}

fn about_z_section() -> impl Element {
    Column::new()
        .s(Padding::new().x(48).y(64))
        .s(Gap::new().y(24))
        .item(El::new().s(Font::new().size(36).color(ACCENT)).child("About"))
        .item(
            Row::new()
                .item(El::new().s(Width::percent(50)).child(El::new().s(Align::center()).s(Font::new().size(24)).child("College")))
                .item("An NCSU Computer Science student who likes campus life and his classes. School has increased my knowledge and comfort coding in Java and C. Designing effective programs before any code is written is a major theme at the University."),
        )
        .item(Row::new()
            .item(Column::new().s(Width::percent(50)).item("In high school I was buying Udemy courses to teach myself Java. After I took the AP Computer Science exam and easily got the AP credit, I looked to expand my knowledge into web development via Udemy. In my senior year, I got an internship at Tizbi as a web developer.").item("Currently I am expanding my knowledge of Rust in the hopes of getting a fintech job and because it is a great language which helps me understand many concepts"))
            .item(El::new().s(Align::center()).s(Font::new().size(24)).child("Coder")))
}

fn call_to_action() -> impl Element {
    El::new().child("Work in progress on the entire site. Built in a Rust Framework MoonZoon so development is a bit slow.")
}
