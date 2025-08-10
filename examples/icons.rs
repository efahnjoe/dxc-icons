use dioxus::prelude::*;
use dxc_icons::*;

pub fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        div { "Total icons: {ICONS_COLLECTION.len()}" }
        div {
            for name in ICONS_COLLECTION.iter() {
                match spawn_icon(name) {
                    Some(icon) => icon,
                    None => rsx!("error loading icon"),
                }
            }
        }
    }
}
