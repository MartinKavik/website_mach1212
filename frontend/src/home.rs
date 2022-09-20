use zoon::named_color::{BLUE_3, BLUE_7, BLUE_8};
use zoon::*;

const ACCENT: HSLuv = hsluv! {254.7, 89.8, 30.1};
const TEXT: HSLuv = hsluv! {237.8, 19.6, 24.1};

pub fn page() -> impl Element {
    Column::new()
        .s(Font::new().color(TEXT))
        .item(introduction())
        .item(about_z_section())
        .item(call_to_action())
}

fn introduction() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::exact(200))
        .s(Background::new().color(ACCENT))
        .child(
            El::new()
                .child(
                    Column::new()
                        .item(
                            El::new()
                                .child("Hello, I am")
                                .s(Align::center())
                                .s(Font::new().italic()),
                        )
                        .item(
                            El::new()
                                .child("Maciej")
                                .s(Align::center())
                                .s(Font::new().size(62)),
                        ),
                )
                .s(Align::center())
                .s(Font::new().color(hsluv! {235.7, 18.3, 95.7}).size(20)),
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
    El::new().child("Work in progress on the entire site")
}
