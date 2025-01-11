use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use json_spanned_value::Spanned;

forward_impl!((<T: JsonSchema> JsonSchema for Spanned<T>) => T);
