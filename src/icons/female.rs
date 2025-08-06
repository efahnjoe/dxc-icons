use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M512 640a256 256 0 1 0 0-512 256 256 0 0 0 0 512m0 64a320 320 0 1 1 0-640 320 320 0 0 1 0 640"/><path d="M512 640q32 0 32 32v256q0 32-32 32t-32-32V672q0-32 32-32"/><path d="M352 800h320q32 0 32 32t-32 32H352q-32 0-32-32t32-32"/>"#;
        
#[component]
/// `Female`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        Female {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn Female(size: Option<String>, color: Option<String>) -> Element {
    let size = size.unwrap_or("1em".to_string());
    let color = color.unwrap_or("currentColor".to_string());

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}",
            height: "{size}",
            fill: color,
            view_box: "0 0 1024 1024",
            dangerous_inner_html: SVG_DATA
        }
    }
}
