use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M352 159.872V230.4a352 352 0 1 0 320 0v-70.528A416.128 416.128 0 0 1 512 960a416 416 0 0 1-160-800.128"/><path d="M512 64q32 0 32 32v320q0 32-32 32t-32-32V96q0-32 32-32"/>"#;
        
#[component]
/// `SwitchButton`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        SwitchButton {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn SwitchButton(size: Option<String>, color: Option<String>) -> Element {
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
