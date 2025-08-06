use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M926.784 480H701.312A192.51 192.51 0 0 0 544 322.688V97.216A416.064 416.064 0 0 1 926.784 480m0 64A416.064 416.064 0 0 1 544 926.784V701.312A192.51 192.51 0 0 0 701.312 544zM97.28 544h225.472A192.51 192.51 0 0 0 480 701.312v225.472A416.064 416.064 0 0 1 97.216 544zm0-64A416.064 416.064 0 0 1 480 97.216v225.472A192.51 192.51 0 0 0 322.688 480H97.216z"/>"#;
        
#[component]
/// `HelpFilled`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        HelpFilled {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn HelpFilled(size: Option<String>, color: Option<String>) -> Element {
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
