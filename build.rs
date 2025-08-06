use regex::Regex;
use std::fs;
use std::env;
use std::path::Path;

const ICONS_DIR: &str = "assets/svg";

fn main() {
    println!("cargo:rerun-if-changed={}", ICONS_DIR);
    println!("cargo:rerun-if-changed=build.rs");

    let root_path = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let icons_path = Path::new(&root_path).join(ICONS_DIR);
    let output_path = Path::new(&out_dir);

    // Create output dir
    fs::create_dir_all(&output_path).unwrap();
    let mut mod_file = String::new();
    mod_file.push_str("// Auto-generated. Do not edit.\n\n");

    for entry in fs::read_dir(&icons_path).expect("Failed to read icons directory") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "svg") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let module_name = to_rust_module_name(stem);
            let output_file = output_path.join(format!("{}.rs", module_name));

            let content = fs::read_to_string(&path).unwrap();

            // Extract SVG tag content (remove the head and tail)
            let inner = extract_svg_content(&content);
            let fn_name = to_pascal_case(stem);

            // Generate Dioxus component
            let component_code = format!(
                r##"use dioxus::prelude::*;

const SVG_DATA: &str = r#"{inner}"#;
        
#[component]
/// `{fn_name}`
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {{
///    div {{
///        {fn_name} {{
///            size: "16px".to_string(), // The size of the icon: size * size
///            color: "black".to_string(), // The svg fill color
///        }}
///    }}
///}}
/// ```
pub fn {fn_name}(size: Option<String>, color: Option<String>) -> Element {{
    let size = size.unwrap_or("1em".to_string());
    let color = color.unwrap_or("currentColor".to_string());

    rsx! {{
        svg {{
            xmlns: "http://www.w3.org/2000/svg",
            width: "{{size}}",
            height: "{{size}}",
            fill: color,
            view_box: "0 0 1024 1024",
            dangerous_inner_html: SVG_DATA
        }}
    }}
}}
"##,
            );

            fs::write(output_file, component_code).unwrap();
            mod_file.push_str(&format!("pub mod {};\n", module_name));
            mod_file.push_str(&format!("pub use {}::{};\n", module_name, fn_name));
        }
    }

    // write to mod.rs
    fs::write(output_path.join("mod.rs"), mod_file).unwrap();
}

/// Check if a string is a reserved Rust keyword
fn is_reserved_keyword(word: &str) -> bool {
    matches!(
        word,
        "box" | "type" | "impl" | "let" | "fn" | "mod" | "pub" | "use" | "struct" | "enum"
            | "trait" | "match" | "if" | "else" | "loop" | "while" | "for" | "in" | "as" | "where"
            | "self" | "super" | "async" | "await" | "dyn" | "move" | "become" | "do"
    )
}

/// Convert a string to a valid Rust module name.
/// 
/// Replaces hyphens with underscores and prefixes reserved keywords with "icon_".
/// 
/// e.g. "alarm-clock" -> "alarm_clock", "type" -> "icon_type"
fn to_rust_module_name(name: &str) -> String {
    let name_lower = name.replace('-', "_").to_lowercase();

    if is_reserved_keyword(&name_lower) {
        format!("icon_{}", name_lower)
    } else {
        name_lower
    }
}

/// Convert a string to PascalCase.
/// 
/// If the input is a reserved keyword, it is prefixed with "icon_".
/// 
/// e.g. "alarm-clock" -> "AlarmClock", "type" -> "IconType"
fn to_pascal_case(name: &str) -> String {
    let processed = if is_reserved_keyword(name) {
        format!("icon_{}", name)
    } else {
        name.to_string()
    };

    to_pascal_case_inner(&processed)
}

/// Helper: Convert a string to PascalCase (no safety checks)
fn to_pascal_case_inner(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_')
        .flat_map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .into_iter()
                .flat_map(|c| c.to_uppercase())
                .chain(chars)
        })
        .collect()
}

/// Clean the SVG file
fn extract_svg_content(svg: &str) -> String {
    let svg = svg
        .replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", "")
        .replace("xmlns=\"http://www.w3.org/2000/svg\"", "")
        .trim()
        .to_string();

    let content = if let Some(start) = svg.find('>') {
        if let Some(end) = svg.rfind("</svg>") {
            &svg[start + 1..end]
        } else {
            &svg[start + 1..]
        }
    } else {
        &svg
    };

    // Regular expression matching the path tag with fill="currentColor"
    let re = Regex::new(r#"<path([^>]*?)\s+fill\s*=\s*["']currentColor["']([^>]*)"#).unwrap();

    // Remove the fill="currentColor" attribute
    let cleaned_content = re.replace_all(content, "<path$1$2").to_string();

    cleaned_content.trim().to_string()
}
