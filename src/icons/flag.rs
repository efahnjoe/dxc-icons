use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M288 128h608L736 384l160 256H288v320h-96V64h96z"/>"#;
        
#[component]
/// `Flag`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        Flag {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn Flag(size: Option<String>, color: Option<String>) -> Element {
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
