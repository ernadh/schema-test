use std::fmt;
use std::sync::Arc;
use value_trait::Value;
use std::any;

use super::schema;
use super::validators;


pub type KeywordPair<V> = (Vec<String>, Box<dyn Keyword>);
pub type KeywordMap = hashbrown::HashMap<String, Arc<KeywordConsumer>>;
pub type KeywordCompilationResult = Result<Option<validators::BoxedValidator>, schema::SchemaError>;

pub trait Keyword: Send + Sync + any::Any {
    fn compile<V>(&self, src: &V, ctx: &schema::WalkContext) -> KeywordCompilationResult;
    fn is_exclusive(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct KeywordConsumer
{
    pub keys: Vec<String>,
    pub keyword: Box<dyn Keyword>,
}

impl KeywordConsumer
{
    pub fn consume(&self, set: &mut hashbrown::HashSet<String>) {
        for key in self.keys.iter() {
            if set.contains(key) {
                set.remove(key);
            }
        }
    }
}

impl fmt::Debug for dyn Keyword {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("<keyword>")
    }
}

macro_rules! keyword_key_exists {
    ($val:expr, $key:expr) => {{
        let maybe_val = $val.get($key);

        if maybe_val.is_none() {
            return Ok(None);
        } else {
            maybe_val.unwrap()
        }
    }};
}

pub mod const_;
//pub mod contains;
//pub mod dependencies;
//pub mod ref_;
//pub mod required;

pub fn default<'scope, V: 'scope>() -> KeywordMap
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + std::cmp::PartialEq,
    <V as Value>::Key: std::borrow::Borrow<str>
        + std::hash::Hash
        + Eq
        + std::convert::AsRef<str>
        + std::fmt::Debug
        + std::string::ToString
        + std::marker::Sync
        + std::marker::Send,
{
    let mut map = hashbrown::HashMap::new();

    decouple_keyword((vec!["const".to_string()], Box::new(const_::Const)), &mut map);
    //decouple_keyword((vec!["dependencies".to_string()], Box::new(dependencies::Dependencies)), &mut map);
    //decouple_keyword((vec!["$ref".to_string()], Box::new(ref_::Ref)), &mut map);
    //decouple_keyword((vec!["required".to_string()], Box::new(required::Required)), &mut map);

    map
}

pub fn decouple_keyword<V>(keyword_pair: KeywordPair<V>, map: &mut hashbrown::HashMap<String, Arc<KeywordConsumer>>)
where
    V: Value,
{
    let (keys, keyword) = keyword_pair;

    let consumer = Arc::new(KeywordConsumer {
        keys: keys.clone(),
        keyword,
    });

    for key in keys.iter() {
        map.insert(key.to_string(), consumer.clone());
    }
}

