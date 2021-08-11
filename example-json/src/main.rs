#![recursion_limit="16"]
use autoparse_general::*;
use autoparse_derive::*;
use autoparse::ParseStream;

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub enum Tokens {
    BraceOpen(token::BraceOpen),
    BraceClose(token::BraceClose),
    BrackedOpen(token::BracketOpen),
    BracketClose(token::BracketClose),
    Comma(token::Comma),
    Colon(token::Colon),
    Signed(Signed),
    Space(Space),
    Literal(Literal)
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub struct JsonKeyValue {
    pub key: MayNotSpaced<Literal>,
    pub colon: MaySpaced<token::Colon>,
    pub value: JsonValue
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub struct JsonObject {
    pub inner: token::Braced<Punchuated<JsonKeyValue, token::Comma>>
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub struct JsonArray {
    pub inner: token::Bracketed<Punchuated<JsonValue, token::Comma>>
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub enum JsonValue {
    Str(Literal),
    Numeric(Signed),
    Object(JsonObject),
    Array(JsonArray)
}



use autoparse::*;
pub fn main() {
    let json = std::fs::read_to_string("test.json").unwrap();
    let mut iter = json.chars();
    let mut stream = ParseStream::from(&mut iter);
    let pos = 0;

    let result = JsonObject::try_parse(&mut stream, pos);
    println!("{:#?}", result);
}
