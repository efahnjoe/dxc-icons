use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M256 832a128 128 0 1 0 0-256 128 128 0 0 0 0 256m0 64a192 192 0 1 1 0-384 192 192 0 0 1 0 384"/><path d="M288 672h320q32 0 32 32t-32 32H288q-32 0-32-32t32-32"/><path d="M768 832a128 128 0 1 0 0-256 128 128 0 0 0 0 256m0 64a192 192 0 1 1 0-384 192 192 0 0 1 0 384"/><path d="M480 192a32 32 0 0 1 0-64h160a32 32 0 0 1 31.04 24.256l96 384a32 32 0 0 1-62.08 15.488L615.04 192zM96 384a32 32 0 0 1 0-64h128a32 32 0 0 1 30.336 21.888l64 192a32 32 0 1 1-60.672 20.224L200.96 384z"/><path d="m373.376 599.808-42.752-47.616 320-288 42.752 47.616z"/>"#;
        
#[component]
/// `Bicycle`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        Bicycle {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn Bicycle(size: Option<String>, color: Option<String>) -> Element {
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
