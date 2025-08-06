use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M512 448v128h239.68l16.064-128zm-64 0H256.256l16.064 128H448zm64-255.36V384h247.744A256.13 256.13 0 0 0 512 192.64m-64 8.064A256.45 256.45 0 0 0 264.256 384H448zm64-72.064A320.13 320.13 0 0 1 825.472 384H896a32 32 0 1 1 0 64h-64v1.92l-56.96 454.016A64 64 0 0 1 711.552 960H312.448a64 64 0 0 1-63.488-56.064L192 449.92V448h-64a32 32 0 0 1 0-64h70.528A320.38 320.38 0 0 1 448 135.04V96a96 96 0 0 1 96-96h128a32 32 0 1 1 0 64H544a32 32 0 0 0-32 32zM743.68 640H280.32l32.128 256h399.104z"/>"#;
        
#[component]
/// `IceDrink`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        IceDrink {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn IceDrink(size: Option<String>, color: Option<String>) -> Element {
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
