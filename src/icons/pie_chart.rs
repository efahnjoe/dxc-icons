use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M448 68.48v64.832A384.128 384.128 0 0 0 512 896a384.13 384.13 0 0 0 378.688-320h64.768A448.128 448.128 0 0 1 64 512 448.13 448.13 0 0 1 448 68.48"/><path d="M576 97.28V448h350.72A384.064 384.064 0 0 0 576 97.28M512 64V33.152A448 448 0 0 1 990.848 512H512z"/>"#;
        
#[component]
/// `PieChart`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        PieChart {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn PieChart(size: Option<String>, color: Option<String>) -> Element {
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
