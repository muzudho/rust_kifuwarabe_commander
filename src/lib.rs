extern crate regex;

pub mod graph;
pub mod shell;
pub mod node;

/// https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization |More concise HashMap initialization
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
