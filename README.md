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

**Since 'Box' is a keyword in Rust, please use 'IconBox'.**

**'Document' is a keyword in Dioxus, please use 'IconDocument'.**

For a better experience, it can be used in conjunction with `dxc` - our UI component library.

 - Github: [https://github.com/efahnjoe/dxc](https://github.com/efahnjoe/dxc)

 - Crate: [https://crates.io/crates/dxc](https://crates.io/crates/dxc)

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
    IconDocument {}
  }
}
```

### Custom style example

```rust
rsx! {
    Div {
        Plus {
            size: "16px", // The size of the icon: size * size
            color: "black", // The svg fill color
        }
    }
}
```