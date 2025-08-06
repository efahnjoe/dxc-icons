# Dioxus Components Icon

**Thanks for [Element Plus](https://github.com/element-plus/element-plus-icons) for providing the icons.**

![Element Plus](https://avatars.githubusercontent.com/u/68583457?s=48&v=4)
![Dioxus](https://avatars.githubusercontent.com/u/79236386?s=48&v=4)

## Preview

- [Documentation](https://element-plus.org/en-US/component/icon.html)
- [Icônes](https://icones.js.org/collection/ep)
- [Iconify](https://icon-sets.iconify.design/ep/)

## Install

Make sure your `Cargo.toml` includes the `Dioxus` dependency:

```toml
[dependencies]
dioxus = "0.6"
```

```bash
cargo add dxc-icons
```

## Usage

`Note: Since 'Box' is a keyword in Rust, please use 'IconBox'.`

### API

| Name | Type | Description | Default |
| --- | --- | --- | --- |
| size | `Option<String>` | Icon size | `"1em"` |
| color | `Option<String>` | Svg fill color | `"currentColor"` |

### Basic usage examples

```rust
use dioxus::prelude::*;
use dxc_icons::{Plus, IconBox};

#[component]
fn App() -> Element {
  Div {
    Plus {}
    IconBox {}
  }
}
```

### Custom style example

```rust
rsx! {
    Div {
        Plus {
            size: "16px".to_string(), // The size of the icon: size * size
            color: "black".to_string(), // The svg fill color
        }
    }
}
```