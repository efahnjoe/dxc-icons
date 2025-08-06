use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M32 128h960v64H32z"/><path d="M192 192v512h640V192zm-64-64h768v608a32 32 0 0 1-32 32H160a32 32 0 0 1-32-32z"/><path d="M322.176 960H248.32l144.64-250.56 55.424 32zm453.888 0h-73.856L576 741.44l55.424-32z"/>"#;
        
#[component]
/// `DataBoard`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        DataBoard {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn DataBoard(size: Option<String>, color: Option<String>) -> Element {
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
