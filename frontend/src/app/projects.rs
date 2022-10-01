use zoon::named_color::TRANSPARENT;
use zoon::*;

use crate::app::{svg_link, ACCENT_BACK, CORNER_RADIUS, SIZE, SPACING, TEXT};

pub fn page() -> impl Element {
    Column::new()
        .s(Font::new().color(TEXT[0]).size(SIZE[6]))
        .item(projects())
}

fn projects() -> impl Element {
    super::section(TRANSPARENT, SPACING[4], Column::new()
        .item(
            super::h2("Public Projects"),
        )
        .item(Column::new().s(Gap::new().y(SPACING[4])).s(Padding::new().top(SPACING[5]))
            .item(project("Website", "The website you are on now. I designed it in Figma and created it in MoonZoon, a Rust fullstack web framework. It is currently deployed to Netlify, but will be on AWS via the CDK", Some("https://github.com/Mach1212/website.git"), None, "mzoon.svg"))
            .item(project("Snake", "A snake game written in Bevy a Rust game engine", Some("snake_url"), Some("https://mpruchn.com/snake"), "mzoon.svg"))
        ))
}

fn project(
    title: &str,
    description: &str,
    github: Option<&str>,
    url: Option<&str>,
    image: &str,
) -> impl Element {
    let (row_width, row_width_signal) = Mutable::new_and_signal(0);
    let (project_panel_height, project_panel_height_signal) = Mutable::new_and_signal(0);
    let multiline = row_width_signal.map(|width| width < 770).dedupe().broadcast();
    Row::new()
        .s(Gap::both(SPACING[5]))
        .s(Padding::new().x(SPACING[7]).y(SPACING[6]))
        .s(Background::new().color(ACCENT_BACK))
        .s(RoundedCorners::all(CORNER_RADIUS))
        .on_viewport_size_change(move |width, _| row_width.set(width))
        .multiline_signal(multiline.signal())
        .item(
            Column::new()
                .s(Width::growable().max(640))
                .s(Gap::both(SPACING[3]))
                .s(Align::new().top().left())
                .on_viewport_size_change(move |_, height| {
                    project_panel_height.set(height);
                })
                .item(Row::new().s(Gap::both(SPACING[4])).items({
                    let mut vec = vec![];

                    if let Some(link) = github {
                        vec.push(svg_link("icons/github.svg", link, "Github link", SIZE[10]))
                    }

                    if let Some(link) = url {
                        vec.push(svg_link("icons/demo.svg", link, "Demo link", SIZE[10]))
                    }

                    vec
                }))
                .item(
                    El::new()
                        .s(Font::new().weight(FontWeight::Bold))
                        .child(title),
                )
                .item(El::new().child(description)),
        )
        .item({
            let ratio = 328. / 123.; // hardcoded for MZ logo; @TODO refactor/pass as args
            let width = map_ref!{
                let panel_height = project_panel_height_signal,
                let multiline = multiline.signal() => move {
                    not(multiline).then(|| Width::growable().max((f64::from(*panel_height) * ratio) as u32))
                }
            };
            Image::new()
                .s(Width::with_signal(width))
                .s(RoundedCorners::all(CORNER_RADIUS))
                .url(public_url(image))
                .description("An intriguing image")
        })
}
