use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M480 512h64v320h-64z"/><path d="M192 896h640a64 64 0 0 0-64-64H256a64 64 0 0 0-64 64m64-128h512a128 128 0 0 1 128 128v64H128v-64a128 128 0 0 1 128-128m256-256a192 192 0 1 0 0-384 192 192 0 0 0 0 384m0 64a256 256 0 1 1 0-512 256 256 0 0 1 0 512"/>"#;
        
#[component]
/// `Coordinate`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        Coordinate {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn Coordinate(size: Option<String>, color: Option<String>) -> Element {
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
