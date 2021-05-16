#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
use autoparse::{Parsable, Writable, ParseError, ParseStream};
use crate::token;
use autoparse_derive::*;

pub struct UnsignedInteger {
    pub literal: Vec<char>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for UnsignedInteger {
    #[inline]
    fn clone(&self) -> UnsignedInteger {
        match *self {
            UnsignedInteger { literal: ref __self_0_0 } =>
            UnsignedInteger{literal:
                                ::core::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for UnsignedInteger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            UnsignedInteger { literal: ref __self_0_0 } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f,
                                                              "UnsignedInteger");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "literal",
                                                    &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}

impl Parsable<char> for UnsignedInteger {
    fn try_parse_no_rewind(stream: &mut impl ParseStream<char>,
                           position: usize)
     -> Result<(Self, usize), ParseError<char>> {
        let mut literal =
            //TODO expected numeric






            ::alloc::vec::Vec::new();
        let mut read = 0;
        let mut reader = ['\0'];
        let mut stream_fork = stream.clone();
        while { read += stream.read(&mut reader); reader[0].is_numeric() } {
            literal.push(reader[0]);
            stream_fork = stream.clone()
        }
        *stream = stream_fork;
        read -= 1;
        if !literal.is_empty() {
            Ok((Self{literal,}, read))
        } else {
            Err(ParseError::new(<[_]>::into_vec(box
                                                    [<[_]>::into_vec(box
                                                                         ['0'])]),
                                position))
        }
    }
}
impl Writable<char> for UnsignedInteger {
    fn write(&self, stream: &mut Vec<char>) {
        stream.extend_from_slice(&self.literal)
    }
}
#[autoparse_for(char)]
pub struct UnsignedFloat {
    pub integer: UnsignedInteger,
    pub point: token::Dot,
    pub decimal: UnsignedInteger,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for UnsignedFloat {
    #[inline]
    fn clone(&self) -> UnsignedFloat {
        match *self {
            UnsignedFloat {
            integer: ref __self_0_0,
            point: ref __self_0_1,
            decimal: ref __self_0_2 } =>
            UnsignedFloat{integer:
                              ::core::clone::Clone::clone(&(*__self_0_0)),
                          point: ::core::clone::Clone::clone(&(*__self_0_1)),
                          decimal:
                              ::core::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for UnsignedFloat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            UnsignedFloat {
            integer: ref __self_0_0,
            point: ref __self_0_1,
            decimal: ref __self_0_2 } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f,
                                                              "UnsignedFloat");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "integer",
                                                    &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "point", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "decimal",
                                                    &&(*__self_0_2));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[autoparse_for(char)]
pub enum Unsigned { Float(UnsignedFloat), Integer(UnsignedInteger), }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Unsigned {
    #[inline]
    fn clone(&self) -> Unsigned {
        match (&*self,) {
            (&Unsigned::Float(ref __self_0),) =>
            Unsigned::Float(::core::clone::Clone::clone(&(*__self_0))),
            (&Unsigned::Integer(ref __self_0),) =>
            Unsigned::Integer(::core::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Unsigned {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Unsigned::Float(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "Float");
                let _ =
                    ::core::fmt::DebugTuple::field(debug_trait_builder,
                                                   &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Unsigned::Integer(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                let _ =
                    ::core::fmt::DebugTuple::field(debug_trait_builder,
                                                   &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[autoparse_for(char)]
pub struct SignedInteger {
    pub sign: Option<token::Dash>,
    pub integer: UnsignedInteger,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for SignedInteger {
    #[inline]
    fn clone(&self) -> SignedInteger {
        match *self {
            SignedInteger { sign: ref __self_0_0, integer: ref __self_0_1 } =>
            SignedInteger{sign: ::core::clone::Clone::clone(&(*__self_0_0)),
                          integer:
                              ::core::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for SignedInteger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            SignedInteger { sign: ref __self_0_0, integer: ref __self_0_1 } =>
            {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f,
                                                              "SignedInteger");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "sign", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "integer",
                                                    &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[autoparse_for(char)]
pub struct SignedFloat {
    pub sign: Option<token::Dash>,
    pub integer: UnsignedFloat,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for SignedFloat {
    #[inline]
    fn clone(&self) -> SignedFloat {
        match *self {
            SignedFloat { sign: ref __self_0_0, integer: ref __self_0_1 } =>
            SignedFloat{sign: ::core::clone::Clone::clone(&(*__self_0_0)),
                        integer:
                            ::core::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for SignedFloat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            SignedFloat { sign: ref __self_0_0, integer: ref __self_0_1 } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f,
                                                              "SignedFloat");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "sign", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "integer",
                                                    &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[autoparse_for(char)]
pub enum Signed { Float(SignedFloat), Integer(SignedInteger), }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Signed {
    #[inline]
    fn clone(&self) -> Signed {
        match (&*self,) {
            (&Signed::Float(ref __self_0),) =>
            Signed::Float(::core::clone::Clone::clone(&(*__self_0))),
            (&Signed::Integer(ref __self_0),) =>
            Signed::Integer(::core::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Signed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Signed::Float(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "Float");
                let _ =
                    ::core::fmt::DebugTuple::field(debug_trait_builder,
                                                   &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Signed::Integer(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                let _ =
                    ::core::fmt::DebugTuple::field(debug_trait_builder,
                                                   &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
