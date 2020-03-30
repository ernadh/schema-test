use std::fmt;
use std::sync::Arc;
use value_trait::Value;
use std::any;

use super::schema;
use super::validators;


pub type KeywordPair<V: Value> = (Vec<String>, Box<dyn Keyword<V>>);
pub type KeywordMap<V: Value> = hashbrown::HashMap<String, Arc<KeywordConsumer<V>>>;
pub type KeywordResult<V: Value> = Result<Option<validators::BoxedValidator<V>>, schema::SchemaError>;

pub trait Keyword<V>: Send + Sync + any::Any {
    fn compile(&self, src: &V, ctx: &schema::WalkContext) -> KeywordResult<V>;
    fn is_exclusive(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct KeywordConsumer<V>
where
    V: Value,
{
    pub keys: Vec<String>,
    pub keyword: Box<dyn Keyword<V>>,
}

impl<V> KeywordConsumer<V>
where
    V: Value,
{
    pub fn consume(&self, set: &mut hashbrown::HashSet<String>) {
        for key in self.keys.iter() {
            if set.contains(key) {
                set.remove(key);
            }
        }
    }
}

impl<V> fmt::Debug for dyn Keyword<V> {
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

pub fn default<V>() -> KeywordMap<V>
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

pub struct KeywordM<V> 
where
    V: Value + std::fmt::Debug
{
    items: KeywordMap<V>
}

impl<V> KeywordM<V>
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::fmt::Display
        + std::marker::Sync
        + std::fmt::Debug
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
    pub fn new(&self) -> KeywordM<V> {
        //let items = self.default(); 
        let mut items = hashbrown::HashMap::new();
        let map = KeywordM {
            items
        };

        map
    }

    fn default() -> KeywordMap<V> {
        let mut map = hashbrown::HashMap::new();

        decouple_keyword((vec!["const".to_string()], Box::new(const_::Const)), &mut map);
        //decouple_keyword((vec!["dependencies".to_string()], Box::new(dependencies::Dependencies)), &mut map);
        //decouple_keyword((vec!["$ref".to_string()], Box::new(ref_::Ref)), &mut map);
        //decouple_keyword((vec!["required".to_string()], Box::new(required::Required)), &mut map);

        map
    }
}

pub fn decouple_keyword<V>(keyword_pair: KeywordPair<V>, map: &mut hashbrown::HashMap<String, Arc<KeywordConsumer<V>>>)
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

