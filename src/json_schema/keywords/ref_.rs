use url::Url;
use value_trait::*;
use super::schema;
use super::validators;

pub struct Ref;

impl<V> super::Keyword<V> for Ref
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + std::cmp::PartialEq,
    <V as Value>::Key: std::borrow::Borrow<str>
        + std::convert::AsRef<str>
        + std::fmt::Debug
        + std::string::ToString
        + std::marker::Sync
        + std::marker::Send,
{
    fn compile(&self, src: &V, ctx: &schema::WalkContext) -> super::KeywordResult<V> {
        let ref_ = keyword_key_exists!(src, "$ref");

        if ref_.is_str() {
            let url = Url::options()
                .base_url(Some(ctx.url))
                .parse(ref_.as_str().unwrap());
            match url {
                Ok(url) => Ok(Some(Box::new(validators::Ref { url }))),
                Err(_) => Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "The value of $ref must be an URI-encoded JSON Pointer".to_string(),
                }),
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of multipleOf must be a string".to_string(),
            })
        }
    }
}
