use crate::app::{ACCENT, ACCENT_BACK, ACCENT_SHADE, BACKGROUND, PAGE_WIDTH, SIZE, SPACING, TEXT};
use zoon::named_color::GRAY_4;
use zoon::*;

pub fn page() -> impl Element {
    Column::new()
        .s(Font::new().color(TEXT[0]).size(SIZE[6]))
        .item(introduction())
        .item(projects())
        .item(call_to_action())
}

fn introduction() -> impl Element {
    super::section(
        ACCENT_BACK,
        Row::new()
            .s(Padding::new().y(SPACING[6]))
            .s(Gap::new().x(SPACING[1]))
            .s(Align::center())
            .s(Width::fill().max(PAGE_WIDTH))
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
                        super::make_button("Email me", ACCENT_BACK, ACCENT, ACCENT_SHADE)
                            .s(Align::new().left()),
                    ),
            )
            .item(Spacer::growable())
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

fn projects() -> impl Element {
    super::section(BACKGROUND[0], Column::new()
        .s(Width::growable().max(PAGE_WIDTH))
        .s(Align::new().center_x())
        .item(
            El::new()
                .child("Projects")
                .s(Align::new().left())
                .s(Font::new().color(ACCENT).size(SIZE[10])),
        )
        .item(project("Website", Some("https://github.com/Mach1212/website.git"), Some("https://mpruchn.com"), "The website you are on now. I designed it in figma and created it in MoonZoon, a Rust fullstack web framework. It is currently deployed to netlify, but will be on AWS"))
        .item(project("Snake", Some("snake_url"), Some("https://mpruchn.com/snake"), "A snake game written in Bevy a Rust game engine"))
    )
}

fn project(
    title: &str,
    github: Option<&str>,
    url: Option<&str>,
    description: &str,
) -> impl Element {
    Row::new()
        .item(Column::new().item(title).items({
            let mut vec = vec![];

            if let Some(link) = github {
                vec.push(
                    super::make_link("Github", link, SIZE[5])
                        .new_tab(NewTab::new().follow(true))
                        .s(Font::new().color(TEXT[1])),
                )
            }

            if let Some(link) = url {
                vec.push(
                    super::make_link("Demo", link, SIZE[5])
                        .new_tab(NewTab::new().follow(true))
                        .s(Font::new().color(TEXT[1])),
                )
            }

            vec
        }))
        .item(Spacer::growable())
        .item(description)
}

fn call_to_action() -> impl Element {
    super::section(BACKGROUND[0], El::new().child("Work in progress on the entire site. Built in a Rust framework MoonZoon so development is a bit slow."))
}
