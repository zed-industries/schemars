use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use serde_spanned::Spanned;

forward_impl!((<T: JsonSchema> JsonSchema for Spanned<T>) => T);
