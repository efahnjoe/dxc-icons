use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M512 128 128 447.936V896h255.936V640H640v256h255.936V447.936z"/>"#;
        
#[component]
/// `HomeFilled`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        HomeFilled {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn HomeFilled(size: Option<String>, color: Option<String>) -> Element {
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
