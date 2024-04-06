# New String Template

Simple Customizable String-Templating Library for Rust.

This Library is inspired by [`string_template`](https://github.com/michaelr524/string_template)

## Usage

Add this to your `Cargo.toml` (or use `cargo-add`):

```toml
[dependencies]
new_string_template = "1.5"
```

Example with 2 data points (with fail enabled):

```rust
use new_string_template::template::Template;
use std::collections::HashMap;

fn main() {
    let templ_str = "Something {data1} be {data2}, and { not here }";
    let templ = Template::new(templ_str);
    let data = {
        let mut map = HashMap::new();
        map.insert("data1", "should");
        map.insert("data2", "here");
        map
    };

    let rendered = templ.render(&data).expect("Expected Result to be Ok");
    assert_eq!("Something should be here, and { not here }", rendered);
}
```

Example with 1 data point (with fail disabled):

```rust
use new_string_template::template::Template;
use std::collections::HashMap;

fn main() {
    let templ_str = "Something {data1} be {data2}, and { not here }";
    let templ = Template::new(templ_str);
    let data = {
        let mut map = HashMap::new();
        map.insert("data1", "should");
        // map.insert("data2", "here");
        map
    };

    let rendered = templ.render_nofail(&data);
    assert_eq!("Something should be {data2}, and { not here }", rendered);
}
```

Example with Custom Regex:

```rust
use new_string_template::template::Template;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    // The following regex requires at least one space between "{{" and "}}" and allows variables with spaces
    let custom_regex = Regex::new(r"(?mi)\{\{\s+([^\}]+)\s+\}\}").unwrap();
    let templ_str = "Something {{ data1 }} be {{ data2 }}, and {{ data 3 }}";
    let templ = Template::new(templ_str).with_regex(&custom_regex);
    let data = {
        let mut map = HashMap::new();
        map.insert("data1", "should");
        map.insert("data2", "here");
        map.insert("data 3", "here too");
        map
    };

    let rendered = templ.render_nofail(&data);
    assert_eq!("Something should be here, and here too", rendered);
}
```

Note: with the default regex, a template-variable can have spaces or none at all.

## Working on this Project

This project requires:
- Rust install with `rustfmt` & `clippy` (nightly version of mentioned components), see [`fmt.sh`](./fmt.sh) and [`clippy.sh`](./clippy.sh)
