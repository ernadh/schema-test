use value_trait::*;
use simd_json::value::owned::Value as OwnedValue;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Const {
    pub item: OwnedValue,
}

impl super::Validator for Const
{
    fn validate<V>(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState 
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
        let mut state = super::ValidationState::new();

        if val.to_string() != self.item.to_string() {
            state.errors.push(Box::new(error::Const {
                path: path.to_string(),
            }))
        }

        state
    }
}
