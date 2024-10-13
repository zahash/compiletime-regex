use std::sync::LazyLock;

use compiletime_regex::regex;
use regex::Regex;

const _: LazyLock<Regex> = LazyLock::new(|| regex!(r#"[A-Z]{2,30}"#));

fn main() {
    let _a = regex!(r#"[A-Z]{2,30}"#);
}
