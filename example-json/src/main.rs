use autoparse_general::*;
use autoparse_derive::*;
use autoparse::ParseStream;

#[derive(Clone, Debug, Writable, Parsable, TokenSet)]
#[autoparse_for(char)]
pub enum Tokens {
    BraceOpen(token::BraceOpen),
    BraceClose(token::BraceClose),
    BracketOpen(token::BracketOpen),
    BracketClose(token::BracketClose),
    Comma(token::Comma),
    Colon(token::Colon),
    Signed(Signed),
    Space(Space),
    Literal(Literal)
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char, Tokens)]
pub struct JsonKeyValue {
    pub key: MayNotSpaced<Literal>,
    pub colon: MaySpaced<token::Colon>,
    pub value: JsonValue
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char, Tokens)]
pub struct JsonObject {
    pub inner: token::Braced<Punchuated<JsonKeyValue, token::Comma>>
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char, Tokens)]
pub struct JsonArray {
    pub inner: token::Bracketed<Punchuated<JsonValue, token::Comma>>
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char, Tokens)]
pub enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    Str(Literal),
    Numeric(Signed)
}



use autoparse::*;
use autoparse_general::token::{BracketOpen, Comma, BracketClose};

pub fn main() {
    let json = std::fs::read_to_string("test.json").unwrap();
    let mut iter = json.chars();
    let pos = 0;
    let mut ss = ParseStreamInitial::from(&mut iter);
    let mut stream = ParseStream::from(&mut ss);
    let result = Vec::<Tokens>::try_parse(&mut stream, pos);
    let mut ss2 = ParseStreamInitial::from(&mut result.unwrap().0.into_iter());
    let mut stream2 = ParseStream::from(&mut ss);
    let result = JsonValue::try_parse(&mut stream2, pos);
    println!("{:#?}", result);
}
