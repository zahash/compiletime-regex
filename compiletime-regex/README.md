<div align="center">

<pre>
 ██████╗ ██████╗ ███╗   ███╗██████╗ ██╗██╗     ███████╗████████╗██╗███╗   ███╗███████╗    ██████╗ ███████╗ ██████╗ ███████╗██╗  ██╗
██╔════╝██╔═══██╗████╗ ████║██╔══██╗██║██║     ██╔════╝╚══██╔══╝██║████╗ ████║██╔════╝    ██╔══██╗██╔════╝██╔════╝ ██╔════╝╚██╗██╔╝
██║     ██║   ██║██╔████╔██║██████╔╝██║██║     █████╗     ██║   ██║██╔████╔██║█████╗█████╗██████╔╝█████╗  ██║  ███╗█████╗   ╚███╔╝ 
██║     ██║   ██║██║╚██╔╝██║██╔═══╝ ██║██║     ██╔══╝     ██║   ██║██║╚██╔╝██║██╔══╝╚════╝██╔══██╗██╔══╝  ██║   ██║██╔══╝   ██╔██╗ 
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║     ██║███████╗███████╗   ██║   ██║██║ ╚═╝ ██║███████╗    ██║  ██║███████╗╚██████╔╝███████╗██╔╝ ██╗
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝   ╚═╝   ╚═╝╚═╝     ╚═╝╚══════╝    ╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝
-----------------------------------------------------------------------------------------------------------------------------------
simple proc macro for compile time regex
</pre>

[![Crates.io](https://img.shields.io/crates/v/compiletime-regex.svg)](https://crates.io/crates/compiletime-regex)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

## 🚀 Installation

include it in your `Cargo.toml` under `[dependencies]`

```toml
compiletime-regex = "*"
```

## 🧑‍💻 Usage examples

### Basic Usage

```rust
use compiletime_regex::regex;
use regex::Regex;

fn main() {
    let hex_color_regex = regex!(r#"^#[a-fA-F0-9]{6}$"#);
    assert!(hex_color_regex.is_match("#00C853"));

    let invalid_regex: Regex = regex!(r#"^#[a-fA-F0-9]{6"#); // compile time error
}
```

### with `LazyCell` and `LazyLock`

```rust
use std::{cell::LazyCell, sync::LazyLock};
use compiletime_regex::regex;
use regex::Regex;

const USERNAME_REGEX: LazyCell<Regex> = LazyCell::new(|| regex!(r#"^[A-Za-z0-9_]{2,30}$"#));
const DATE_REGEX: LazyLock<Regex> = LazyLock::new(|| regex!(r#"^\d{4}-\d{2}-\d{2}$"#));

const INVALID_REGEX: LazyCell<Regex> = LazyCell::new(|| regex!(r#"^[A-Za-z0-9_]{2,30"#)); // compile time error
const INVALID_REGEX: LazyLock<Regex> = LazyLock::new(|| regex!(r#"^\d{4}-\d{2}-\d{2"#)); // compile time error

fn main() {
    assert!(USERNAME_REGEX.is_match("enigma"));
    assert!(DATE_REGEX.is_match("1912-06-23"));
}
```

## 🌟 Connect with Us

zahash – zahash.z@gmail.com

Distributed under the MIT license. See `LICENSE` for more information.

[https://github.com/zahash/](https://github.com/zahash/)

## 🤝 Contribute to `compiletime-regex`!

1. Fork it (<https://github.com/zahash/compiletime-regex/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## ❤️ Show Some Love!

If you find `compiletime-regex` helpful and enjoy using it, consider giving it a [⭐ on GitHub!](https://github.com/zahash/compiletime-regex/stargazers) Your star is a gesture of appreciation and encouragement for the continuous improvement of `compiletime-regex`.
