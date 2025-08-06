use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M896 192H128v128h768zm0 256H384v128h512zm0 256H128v128h768zM320 384 128 512l192 128z"/>"#;
        
#[component]
/// `Fold`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        Fold {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn Fold(size: Option<String>, color: Option<String>) -> Element {
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
