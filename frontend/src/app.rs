use zoon::*;

pub mod view;

#[static_ref]
fn counter() -> &'static Mutable<u32> {
    Mutable::new(0)
}

fn increment() {
    counter().update(|counter| counter + 1)
}
