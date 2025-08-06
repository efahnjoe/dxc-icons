use dioxus::prelude::*;

const SVG_DATA: &str = r#"<path d="M688 512a112 112 0 1 0 0 224h208v160H128V352h768v160zm32 160h-32a48 48 0 0 1 0-96h32a48 48 0 0 1 0 96m-80-544 128 160H384z"/>"#;
        
#[component]
/// `WalletFilled`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///    div {
///        WalletFilled {
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }
///    }
///}
/// ```
pub fn WalletFilled(size: Option<String>, color: Option<String>) -> Element {
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
