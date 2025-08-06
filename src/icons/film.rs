use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M160 160v704h704V160zm-32-64h768a32 32 0 0 1 32 32v768a32 32 0 0 1-32 32H128a32 32 0 0 1-32-32V128a32 32 0 0 1 32-32"/><path d="M320 288V128h64v352h256V128h64v160h160v64H704v128h160v64H704v128h160v64H704v160h-64V544H384v352h-64V736H128v-64h192V544H128v-64h192V352H128v-64z"/>"#;
        
#[component]
/// `Film`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        Film {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn Film(size: Option<String>, color: Option<String>) -> Element {
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
