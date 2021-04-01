use autoparse::general;
use autoparse_derive::AutoParse;

#[derive(AutoParse)]
pub struct JsonObjectKeyValue {
    pub key: general::Ident,
    pub colon: general::token::Colon,
    pub value: Box<JsonValue>
}

pub struct JsonObject {
    map: 
}
pub type JsonObject = general::CurlyBrakets<
    (
        Option<ObjectKeyValue>,
        Vec<(general::token::Commar, JsonObjectKeyValue)>
    )
>;

pub type JsonArray = general::SquaredBrakets<
    (
        Option<JsonValue>,
        Vec<(general::toekn::Commar, JsonValue)>
    )
>;

#[derive(AutoParse)]
pub enum JsonValue {
    JsonObject(JsonObject),
    JsonArray(JsonArray),
    JsonString(String),
    JsonInteger(i64),
    JsonFloat(f64)
}
