use zoon::named_color::GRAY_4;
use zoon::*;

use crate::app::{
    svg_link, ACCENT, ACCENT_BACK, ACCENT_SHADE, BACKGROUND, CORNER_RADIUS, SIZE, SPACING, TEXT,
};

pub fn page() -> impl Element {
    Column::new()
        .s(Font::new().color(TEXT[0]).size(SIZE[6]))
        .item(introduction())
        .item(call_to_action())
}

fn introduction() -> impl Element {
    super::section(
        ACCENT_BACK,
        0,
        Row::new()
            .s(Padding::new().bottom(SPACING[5]))
            .s(Gap::new().x(SPACING[1]))
            .s(Align::center())
            .item(
                Column::new()
                    .s(Gap::both(SPACING[5]))
                    .item(
                        El::new()
                            .s(Width::fill().max(800))
                            .child("Hi, I am Maciej Pruchnik, a software engineer")
                            .s(Font::new().size(SIZE[12]).line_height(SPACING[9])),
                    )
                    .item(
                        El::new()
                            .child("Solving your problems with elegant solutions")
                            .s(Font::new().size(SIZE[5]).italic()),
                    )
                    .item(
                        Link::new()
                            .label(super::make_button(
                                "Email me",
                                ACCENT_BACK,
                                ACCENT,
                                ACCENT_SHADE,
                                || {},
                            ))
                            .to("mailto:mpruchn@ncsu.edu")
                            .new_tab(NewTab::new().follow(true))
                            .s(Align::new().left()),
                    ),
            )
            .item(Spacer::growable())
            .item(
                Image::new()
                    .url(public_url("self_website.jpg"))
                    .description("An image of myself")
                    .s(Width::growable().max(500))
                    .s(RoundedCorners::all(CORNER_RADIUS))
                    .s(Shadows::new([Shadow::new().color(GRAY_4).blur(SPACING[7])])),
            ),
    )
}

fn call_to_action() -> impl Element {
    super::section(BACKGROUND[0], 0, El::new().child("Work in progress on the entire site. Built in a Rust framework MoonZoon so development is a bit slow."))
}
