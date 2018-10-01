/// 参考:
/// https://github.com/serde-rs/json
/// https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file |How do I parse a JSON File?
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate regex;

/// https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization |More concise HashMap initialization
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub mod models;
pub mod diagram;
pub mod shell;
