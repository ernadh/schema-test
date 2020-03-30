use value_trait::*;
use super::scope;
use super::error;
use std::fmt;

/*
macro_rules! nonstrict_process {
    ($val:expr, $path:ident) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return $crate::json_schema::validators::ValidationState::new();
        }

        maybe_val.unwrap()
    }};
}
*/

pub mod const_;
//pub mod contains;
//pub mod dependencies;
//pub mod ref_;
//pub mod required;

pub use self::const_::Const;
//pub use self::contains::Contains;
//pub use self::dependencies::Dependencies;
//pub use self::ref_::Ref;
//pub use self::required::Required;

pub trait Validator
{
    fn validate<V>(&self, item: &V, _: &str, _: &scope::Scope<V>) -> ValidationState
    where
        V: Value,
        <V as Value>::Key:
            std::borrow::Borrow<str> + std::hash::Hash + Eq + std::convert::AsRef<str>;
}

#[derive(Debug)]
pub struct ValidationState {
    pub errors: super::error::SimdjsonSchemaErrors,
    pub missing: Vec<url::Url>,
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState {
            errors: vec![],
            missing: vec![],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn append(&mut self, second: ValidationState) {
        self.errors.extend(second.errors);
        self.missing.extend(second.missing);
    }
}

impl fmt::Debug for dyn Validator + Send + Sync
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("<validator>")
    }
}

pub type BoxedValidator = Box<dyn Validator + Send + Sync>;
pub type Validators = Vec<BoxedValidator>;
