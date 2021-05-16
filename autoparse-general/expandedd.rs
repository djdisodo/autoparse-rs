#![feature(prelude_import)]
#![feature(or_patterns)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
#[macro_use]
pub mod token {
    mod bracket {
        use crate::{MayNotSpaced, token};
        use autoparse::Parsable;
        use autoparse_derive::*;
        use dede::*;
        pub struct BracketOpen;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BracketOpen {
            #[inline]
            fn clone(&self) -> BracketOpen {
                match *self {
                    BracketOpen => BracketOpen,
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BracketOpen {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    BracketOpen => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BracketOpen");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BracketOpen {
            #[inline]
            fn default() -> BracketOpen {
                BracketOpen {}
            }
        }
        impl BracketOpen {
            pub const TOKEN: &'static str = "[";
        }
        impl autoparse::Parsable<char> for BracketOpen {
            fn try_parse_no_rewind(
                stream: &mut impl autoparse::ParseStream<char>,
                position: usize,
            ) -> Result<(Self, usize), autoparse::ParseError<char>> {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                let mut check = ::alloc::vec::from_elem(0 as char, token.len());
                autoparse::ParseStream::<char>::read(stream, &mut check);
                if check == token {
                    Ok((Self, Self::TOKEN.len()))
                } else {
                    Err(autoparse::ParseError::new([token].into(), position))
                }
            }
        }
        impl autoparse::Writable<char> for BracketOpen {
            fn write(&self, stream: &mut Vec<char>) {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                stream.extend_from_slice(&token)
            }
        }
        pub struct BracketClose;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BracketClose {
            #[inline]
            fn clone(&self) -> BracketClose {
                match *self {
                    BracketClose => BracketClose,
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BracketClose {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    BracketClose => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BracketClose");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BracketClose {
            #[inline]
            fn default() -> BracketClose {
                BracketClose {}
            }
        }
        impl BracketClose {
            pub const TOKEN: &'static str = "]";
        }
        impl autoparse::Parsable<char> for BracketClose {
            fn try_parse_no_rewind(
                stream: &mut impl autoparse::ParseStream<char>,
                position: usize,
            ) -> Result<(Self, usize), autoparse::ParseError<char>> {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                let mut check = ::alloc::vec::from_elem(0 as char, token.len());
                autoparse::ParseStream::<char>::read(stream, &mut check);
                if check == token {
                    Ok((Self, Self::TOKEN.len()))
                } else {
                    Err(autoparse::ParseError::new([token].into(), position))
                }
            }
        }
        impl autoparse::Writable<char> for BracketClose {
            fn write(&self, stream: &mut Vec<char>) {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                stream.extend_from_slice(&token)
            }
        }
        #[autoparse_for(char)]
        pub struct Bracketed<T: Parsable<char>> {
            pub open: MayNotSpaced<BracketOpen>,
            #[deref]
            pub inner: MayNotSpaced<T>,
            pub close: BracketClose,
        }
        impl<T: Parsable<char>> autoparse::Parsable<char> for Bracketed<T> {
            fn try_parse_no_rewind(
                stream: &mut impl autoparse::ParseStream<char>,
                position: usize,
            ) -> Result<(Self, usize), autoparse::ParseError<char>> {
                let mut read = 0;
                let (open, r): (MayNotSpaced<BracketOpen>, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                let (inner, r): (MayNotSpaced<T>, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                let (close, r): (BracketClose, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                Ok((Self { open, inner, close }, read))
            }
        }
        impl<T: Parsable<char>> autoparse::Writable<char> for Bracketed<T> {
            fn write(&self, buffer: &mut Vec<char>) {
                autoparse::Writable::write(&self.open, buffer);
                autoparse::Writable::write(&self.inner, buffer);
                autoparse::Writable::write(&self.close, buffer);
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::fmt::Debug + Parsable<char>> ::core::fmt::Debug for Bracketed<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Bracketed {
                        open: ref __self_0_0,
                        inner: ref __self_0_1,
                        close: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Bracketed");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "open",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "inner",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "close",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::clone::Clone + Parsable<char>> ::core::clone::Clone for Bracketed<T> {
            #[inline]
            fn clone(&self) -> Bracketed<T> {
                match *self {
                    Bracketed {
                        open: ref __self_0_0,
                        inner: ref __self_0_1,
                        close: ref __self_0_2,
                    } => Bracketed {
                        open: ::core::clone::Clone::clone(&(*__self_0_0)),
                        inner: ::core::clone::Clone::clone(&(*__self_0_1)),
                        close: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        impl<T: Parsable<char>> std::ops::Deref for Bracketed<T> {
            type Target = MayNotSpaced<T>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl<T: Parsable<char>> std::ops::DerefMut for Bracketed<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    }
    mod brace {
        use crate::{MayNotSpaced, token};
        use autoparse::Parsable;
        use autoparse_derive::*;
        use dede::*;
        pub struct BraceOpen;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BraceOpen {
            #[inline]
            fn clone(&self) -> BraceOpen {
                match *self {
                    BraceOpen => BraceOpen,
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BraceOpen {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    BraceOpen => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BraceOpen");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BraceOpen {
            #[inline]
            fn default() -> BraceOpen {
                BraceOpen {}
            }
        }
        impl BraceOpen {
            pub const TOKEN: &'static str = "{";
        }
        impl autoparse::Parsable<char> for BraceOpen {
            fn try_parse_no_rewind(
                stream: &mut impl autoparse::ParseStream<char>,
                position: usize,
            ) -> Result<(Self, usize), autoparse::ParseError<char>> {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                let mut check = ::alloc::vec::from_elem(0 as char, token.len());
                autoparse::ParseStream::<char>::read(stream, &mut check);
                if check == token {
                    Ok((Self, Self::TOKEN.len()))
                } else {
                    Err(autoparse::ParseError::new([token].into(), position))
                }
            }
        }
        impl autoparse::Writable<char> for BraceOpen {
            fn write(&self, stream: &mut Vec<char>) {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                stream.extend_from_slice(&token)
            }
        }
        pub struct BraceClose;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BraceClose {
            #[inline]
            fn clone(&self) -> BraceClose {
                match *self {
                    BraceClose => BraceClose,
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BraceClose {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    BraceClose => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BraceClose");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BraceClose {
            #[inline]
            fn default() -> BraceClose {
                BraceClose {}
            }
        }
        impl BraceClose {
            pub const TOKEN: &'static str = "}";
        }
        impl autoparse::Parsable<char> for BraceClose {
            fn try_parse_no_rewind(
                stream: &mut impl autoparse::ParseStream<char>,
                position: usize,
            ) -> Result<(Self, usize), autoparse::ParseError<char>> {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                let mut check = ::alloc::vec::from_elem(0 as char, token.len());
                autoparse::ParseStream::<char>::read(stream, &mut check);
                if check == token {
                    Ok((Self, Self::TOKEN.len()))
                } else {
                    Err(autoparse::ParseError::new([token].into(), position))
                }
            }
        }
        impl autoparse::Writable<char> for BraceClose {
            fn write(&self, stream: &mut Vec<char>) {
                let token: Vec<char> = Self::TOKEN.chars().collect();
                stream.extend_from_slice(&token)
            }
        }
        #[autoparse_for(char)]
        pub struct Braced<T: Parsable<char>> {
            pub open: MayNotSpaced<BraceOpen>,
            #[deref]
            pub inner: MayNotSpaced<T>,
            pub close: BraceClose,
        }
        impl<T: Parsable<char>> autoparse::Parsable<char> for Braced<T> {
            fn try_parse_no_rewind(
                stream: &mut impl autoparse::ParseStream<char>,
                position: usize,
            ) -> Result<(Self, usize), autoparse::ParseError<char>> {
                let mut read = 0;
                let (open, r): (MayNotSpaced<BraceOpen>, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                let (inner, r): (MayNotSpaced<T>, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                let (close, r): (BraceClose, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                Ok((Self { open, inner, close }, read))
            }
        }
        impl<T: Parsable<char>> autoparse::Writable<char> for Braced<T> {
            fn write(&self, buffer: &mut Vec<char>) {
                autoparse::Writable::write(&self.open, buffer);
                autoparse::Writable::write(&self.inner, buffer);
                autoparse::Writable::write(&self.close, buffer);
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::fmt::Debug + Parsable<char>> ::core::fmt::Debug for Braced<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Braced {
                        open: ref __self_0_0,
                        inner: ref __self_0_1,
                        close: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Braced");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "open",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "inner",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "close",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::clone::Clone + Parsable<char>> ::core::clone::Clone for Braced<T> {
            #[inline]
            fn clone(&self) -> Braced<T> {
                match *self {
                    Braced {
                        open: ref __self_0_0,
                        inner: ref __self_0_1,
                        close: ref __self_0_2,
                    } => Braced {
                        open: ::core::clone::Clone::clone(&(*__self_0_0)),
                        inner: ::core::clone::Clone::clone(&(*__self_0_1)),
                        close: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        impl<T: Parsable<char>> std::ops::Deref for Braced<T> {
            type Target = MayNotSpaced<T>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl<T: Parsable<char>> std::ops::DerefMut for Braced<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    }
    pub use bracket::*;
    pub use brace::*;
    pub struct Bang;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Bang {
        #[inline]
        fn clone(&self) -> Bang {
            match *self {
                Bang => Bang,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Bang {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Bang => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Bang");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Bang {
        #[inline]
        fn default() -> Bang {
            Bang {}
        }
    }
    impl Bang {
        pub const TOKEN: &'static str = "!";
    }
    impl autoparse::Parsable<char> for Bang {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            let mut check = ::alloc::vec::from_elem(0 as char, token.len());
            autoparse::ParseStream::<char>::read(stream, &mut check);
            if check == token {
                Ok((Self, Self::TOKEN.len()))
            } else {
                Err(autoparse::ParseError::new([token].into(), position))
            }
        }
    }
    impl autoparse::Writable<char> for Bang {
        fn write(&self, stream: &mut Vec<char>) {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            stream.extend_from_slice(&token)
        }
    }
    pub struct Colon;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Colon {
        #[inline]
        fn clone(&self) -> Colon {
            match *self {
                Colon => Colon,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Colon {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Colon => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Colon");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Colon {
        #[inline]
        fn default() -> Colon {
            Colon {}
        }
    }
    impl Colon {
        pub const TOKEN: &'static str = ":";
    }
    impl autoparse::Parsable<char> for Colon {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            let mut check = ::alloc::vec::from_elem(0 as char, token.len());
            autoparse::ParseStream::<char>::read(stream, &mut check);
            if check == token {
                Ok((Self, Self::TOKEN.len()))
            } else {
                Err(autoparse::ParseError::new([token].into(), position))
            }
        }
    }
    impl autoparse::Writable<char> for Colon {
        fn write(&self, stream: &mut Vec<char>) {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            stream.extend_from_slice(&token)
        }
    }
    pub struct Comma;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Comma {
        #[inline]
        fn clone(&self) -> Comma {
            match *self {
                Comma => Comma,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Comma {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Comma => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Comma");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Comma {
        #[inline]
        fn default() -> Comma {
            Comma {}
        }
    }
    impl Comma {
        pub const TOKEN: &'static str = ",";
    }
    impl autoparse::Parsable<char> for Comma {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            let mut check = ::alloc::vec::from_elem(0 as char, token.len());
            autoparse::ParseStream::<char>::read(stream, &mut check);
            if check == token {
                Ok((Self, Self::TOKEN.len()))
            } else {
                Err(autoparse::ParseError::new([token].into(), position))
            }
        }
    }
    impl autoparse::Writable<char> for Comma {
        fn write(&self, stream: &mut Vec<char>) {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            stream.extend_from_slice(&token)
        }
    }
    pub struct Dash;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Dash {
        #[inline]
        fn clone(&self) -> Dash {
            match *self {
                Dash => Dash,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Dash {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Dash => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Dash");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Dash {
        #[inline]
        fn default() -> Dash {
            Dash {}
        }
    }
    impl Dash {
        pub const TOKEN: &'static str = "-";
    }
    impl autoparse::Parsable<char> for Dash {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            let mut check = ::alloc::vec::from_elem(0 as char, token.len());
            autoparse::ParseStream::<char>::read(stream, &mut check);
            if check == token {
                Ok((Self, Self::TOKEN.len()))
            } else {
                Err(autoparse::ParseError::new([token].into(), position))
            }
        }
    }
    impl autoparse::Writable<char> for Dash {
        fn write(&self, stream: &mut Vec<char>) {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            stream.extend_from_slice(&token)
        }
    }
    pub struct Dot;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Dot {
        #[inline]
        fn clone(&self) -> Dot {
            match *self {
                Dot => Dot,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Dot {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Dot => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Dot");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Dot {
        #[inline]
        fn default() -> Dot {
            Dot {}
        }
    }
    impl Dot {
        pub const TOKEN: &'static str = ".";
    }
    impl autoparse::Parsable<char> for Dot {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            let mut check = ::alloc::vec::from_elem(0 as char, token.len());
            autoparse::ParseStream::<char>::read(stream, &mut check);
            if check == token {
                Ok((Self, Self::TOKEN.len()))
            } else {
                Err(autoparse::ParseError::new([token].into(), position))
            }
        }
    }
    impl autoparse::Writable<char> for Dot {
        fn write(&self, stream: &mut Vec<char>) {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            stream.extend_from_slice(&token)
        }
    }
    pub struct SnailQuote;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SnailQuote {
        #[inline]
        fn clone(&self) -> SnailQuote {
            match *self {
                SnailQuote => SnailQuote,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SnailQuote {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SnailQuote => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "SnailQuote");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SnailQuote {
        #[inline]
        fn default() -> SnailQuote {
            SnailQuote {}
        }
    }
    impl SnailQuote {
        pub const TOKEN: &'static str = "\"";
    }
    impl autoparse::Parsable<char> for SnailQuote {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            let mut check = ::alloc::vec::from_elem(0 as char, token.len());
            autoparse::ParseStream::<char>::read(stream, &mut check);
            if check == token {
                Ok((Self, Self::TOKEN.len()))
            } else {
                Err(autoparse::ParseError::new([token].into(), position))
            }
        }
    }
    impl autoparse::Writable<char> for SnailQuote {
        fn write(&self, stream: &mut Vec<char>) {
            let token: Vec<char> = Self::TOKEN.chars().collect();
            stream.extend_from_slice(&token)
        }
    }
}
mod literal {
    use autoparse::{Parsable, Writable, ParseError, ParseStream};
    use dede::*;
    use snailquote::*;
    use crate::token;
    pub struct Literal {
        #[deref]
        pub inner: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Literal {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Literal {
                    inner: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Literal");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "inner",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Literal {
        #[inline]
        fn clone(&self) -> Literal {
            match *self {
                Literal {
                    inner: ref __self_0_0,
                } => Literal {
                    inner: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl std::ops::Deref for Literal {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
    impl std::ops::DerefMut for Literal {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
    impl Writable<char> for Literal {
        fn write(&self, stream: &mut Vec<char>) {
            let escaped: String = escape(&self.inner).into();
            for c in escaped.chars() {
                stream.push(c);
            }
        }
    }
    impl Parsable<char> for Literal {
        fn try_parse_no_rewind(
            stream: &mut impl ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), ParseError<char>> {
            let mut buffer = ::alloc::vec::Vec::new();
            let mut reader = ['\0'];
            let mut read = 1;
            if let Err(e) = token::SnailQuote::try_parse_no_rewind(stream, position) {
                return Err(e);
            }
            while {
                read += stream.read(&mut reader);
                reader[0] != '\"'
            } {
                buffer.push(reader[0]);
                if reader[0] == '\\' {
                    read += stream.read(&mut reader);
                    buffer.push(reader[0]);
                }
            }
            let collected: String = buffer.into_iter().collect();
            let unescaped = unescape(&collected).unwrap();
            Ok((Self { inner: unescaped }, read))
        }
    }
}
mod numeric {
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
                UnsignedInteger {
                    literal: ref __self_0_0,
                } => UnsignedInteger {
                    literal: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for UnsignedInteger {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                UnsignedInteger {
                    literal: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "UnsignedInteger");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "literal",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Parsable<char> for UnsignedInteger {
        fn try_parse_no_rewind(
            stream: &mut impl ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), ParseError<char>> {
            let mut literal = ::alloc::vec::Vec::new();
            let mut read = 0;
            let mut reader = ['\0'];
            let mut stream_fork = stream.clone();
            while {
                read += stream.read(&mut reader);
                reader[0].is_numeric()
            } {
                literal.push(reader[0]);
                stream_fork = stream.clone()
            }
            *stream = stream_fork;
            read -= 1;
            if !literal.is_empty() {
                Ok((Self { literal }, read))
            } else {
                Err(ParseError::new(
                    <[_]>::into_vec(box [<[_]>::into_vec(box ['0'])]),
                    position,
                ))
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
                    decimal: ref __self_0_2,
                } => UnsignedFloat {
                    integer: ::core::clone::Clone::clone(&(*__self_0_0)),
                    point: ::core::clone::Clone::clone(&(*__self_0_1)),
                    decimal: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
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
                    decimal: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "UnsignedFloat");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "integer",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "point",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "decimal",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl autoparse::Writable<char> for UnsignedFloat {
        fn write(&self, buffer: &mut Vec<char>) {
            autoparse::Writable::write(&self.integer, buffer);
            autoparse::Writable::write(&self.point, buffer);
            autoparse::Writable::write(&self.decimal, buffer);
        }
    }
    impl autoparse::Parsable<char> for UnsignedFloat {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let mut read = 0;
            let (integer, r): (UnsignedInteger, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            let (point, r): (token::Dot, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            let (decimal, r): (UnsignedInteger, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            Ok((
                Self {
                    integer,
                    point,
                    decimal,
                },
                read,
            ))
        }
    }
    #[autoparse_for(char)]
    pub enum Unsigned {
        Float(UnsignedFloat),
        Integer(UnsignedInteger),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Unsigned {
        #[inline]
        fn clone(&self) -> Unsigned {
            match (&*self,) {
                (&Unsigned::Float(ref __self_0),) => {
                    Unsigned::Float(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Unsigned::Integer(ref __self_0),) => {
                    Unsigned::Integer(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Unsigned {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Unsigned::Float(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Float");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Unsigned::Integer(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl autoparse::Writable<char> for Unsigned {
        fn write(&self, buffer: &mut Vec<char>) {
            match self {
                Unsigned::Float(_0) => {
                    let self_a = (_0,);
                    autoparse::Writable::write(self_a.0, buffer);
                }
                Unsigned::Integer(_0) => {
                    let self_a = (_0,);
                    autoparse::Writable::write(self_a.0, buffer);
                }
            }
        }
    }
    impl autoparse::Parsable<char> for Unsigned {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let mut error: autoparse::ParseError<char> = Default::default();
            let stream_fork;
            match {
                stream_fork = stream.clone();
                let mut read = 0;
                let (_0, r): (UnsignedFloat, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                Ok((Unsigned::Float(_0), read))
            } {
                Ok((parsed, read)) => return Ok((parsed, read)),
                Err(e) => {
                    *stream = stream_fork;
                    (*error).extend(e.expections)
                }
            }
            let stream_fork;
            match {
                stream_fork = stream.clone();
                let mut read = 0;
                let (_0, r): (UnsignedInteger, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                Ok((Unsigned::Integer(_0), read))
            } {
                Ok((parsed, read)) => return Ok((parsed, read)),
                Err(e) => {
                    *stream = stream_fork;
                    (*error).extend(e.expections)
                }
            }
            Err(error)
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
                SignedInteger {
                    sign: ref __self_0_0,
                    integer: ref __self_0_1,
                } => SignedInteger {
                    sign: ::core::clone::Clone::clone(&(*__self_0_0)),
                    integer: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SignedInteger {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SignedInteger {
                    sign: ref __self_0_0,
                    integer: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SignedInteger");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "sign",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "integer",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl autoparse::Writable<char> for SignedInteger {
        fn write(&self, buffer: &mut Vec<char>) {
            autoparse::Writable::write(&self.sign, buffer);
            autoparse::Writable::write(&self.integer, buffer);
        }
    }
    impl autoparse::Parsable<char> for SignedInteger {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let mut read = 0;
            let (sign, r): (Option<token::Dash>, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            let (integer, r): (UnsignedInteger, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            Ok((Self { sign, integer }, read))
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
                SignedFloat {
                    sign: ref __self_0_0,
                    integer: ref __self_0_1,
                } => SignedFloat {
                    sign: ::core::clone::Clone::clone(&(*__self_0_0)),
                    integer: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SignedFloat {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SignedFloat {
                    sign: ref __self_0_0,
                    integer: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SignedFloat");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "sign",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "integer",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl autoparse::Writable<char> for SignedFloat {
        fn write(&self, buffer: &mut Vec<char>) {
            autoparse::Writable::write(&self.sign, buffer);
            autoparse::Writable::write(&self.integer, buffer);
        }
    }
    impl autoparse::Parsable<char> for SignedFloat {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let mut read = 0;
            let (sign, r): (Option<token::Dash>, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            let (integer, r): (UnsignedFloat, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            Ok((Self { sign, integer }, read))
        }
    }
    #[autoparse_for(char)]
    pub enum Signed {
        Float(SignedFloat),
        Integer(SignedInteger),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Signed {
        #[inline]
        fn clone(&self) -> Signed {
            match (&*self,) {
                (&Signed::Float(ref __self_0),) => {
                    Signed::Float(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Signed::Integer(ref __self_0),) => {
                    Signed::Integer(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Signed {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Signed::Float(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Float");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Signed::Integer(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl autoparse::Writable<char> for Signed {
        fn write(&self, buffer: &mut Vec<char>) {
            match self {
                Signed::Float(_0) => {
                    let self_a = (_0,);
                    autoparse::Writable::write(self_a.0, buffer);
                }
                Signed::Integer(_0) => {
                    let self_a = (_0,);
                    autoparse::Writable::write(self_a.0, buffer);
                }
            }
        }
    }
    impl autoparse::Parsable<char> for Signed {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let mut error: autoparse::ParseError<char> = Default::default();
            let stream_fork;
            match {
                stream_fork = stream.clone();
                let mut read = 0;
                let (_0, r): (SignedFloat, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                Ok((Signed::Float(_0), read))
            } {
                Ok((parsed, read)) => return Ok((parsed, read)),
                Err(e) => {
                    *stream = stream_fork;
                    (*error).extend(e.expections)
                }
            }
            let stream_fork;
            match {
                stream_fork = stream.clone();
                let mut read = 0;
                let (_0, r): (SignedInteger, _) =
                    autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
                read += r;
                Ok((Signed::Integer(_0), read))
            } {
                Ok((parsed, read)) => return Ok((parsed, read)),
                Err(e) => {
                    *stream = stream_fork;
                    (*error).extend(e.expections)
                }
            }
            Err(error)
        }
    }
}
mod punchuated {
    use std::marker::PhantomData;
    use autoparse::{Parsable, ParseError, ParseStream, Writable};
    use super::{MaySpaced, MayNotSpaced, MaySpace, MayNotSpace};
    use dede::*;
    use std::fmt::Debug;
    pub struct Punchuated<V: Writable<char>, D: Writable<char> + Default> {
        #[deref]
        elements: Vec<V>,
        delimiter: PhantomData<D>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            V: ::core::fmt::Debug + Writable<char>,
            D: ::core::fmt::Debug + Writable<char> + Default,
        > ::core::fmt::Debug for Punchuated<V, D>
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Punchuated {
                    elements: ref __self_0_0,
                    delimiter: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Punchuated");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "elements",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "delimiter",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            V: ::core::clone::Clone + Writable<char>,
            D: ::core::clone::Clone + Writable<char> + Default,
        > ::core::clone::Clone for Punchuated<V, D>
    {
        #[inline]
        fn clone(&self) -> Punchuated<V, D> {
            match *self {
                Punchuated {
                    elements: ref __self_0_0,
                    delimiter: ref __self_0_1,
                } => Punchuated {
                    elements: ::core::clone::Clone::clone(&(*__self_0_0)),
                    delimiter: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<V: Writable<char>, D: Writable<char> + Default> std::ops::Deref for Punchuated<V, D> {
        type Target = Vec<V>;
        fn deref(&self) -> &Self::Target {
            &self.elements
        }
    }
    impl<V: Writable<char>, D: Writable<char> + Default> std::ops::DerefMut for Punchuated<V, D> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.elements
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            V: ::core::default::Default + Writable<char>,
            D: ::core::default::Default + Writable<char> + Default,
        > ::core::default::Default for Punchuated<V, D>
    {
        #[inline]
        fn default() -> Punchuated<V, D> {
            Punchuated {
                elements: ::core::default::Default::default(),
                delimiter: ::core::default::Default::default(),
            }
        }
    }
    impl<V: Parsable<char> + Debug, D: Parsable<char> + Default + Debug> Parsable<char>
        for Punchuated<V, D>
    {
        fn try_parse_no_rewind(
            stream: &mut impl ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), ParseError<char>> {
            let mut elements: Vec<V> = ::alloc::vec::Vec::new();
            let mut read = 0;
            if let Ok((Some(value), r)) = Option::<MayNotSpaced<V>>::try_parse(stream, position) {
                elements.push(value.inner);
                read += r;
            }
            while let Ok(((_delimiter, value), r)) =
                <(MaySpaced<D>, MayNotSpaced<V>)>::try_parse(stream, position)
            {
                read += r;
                elements.push(value.inner);
            }
            Ok((
                Self {
                    elements,
                    delimiter: Default::default(),
                },
                read,
            ))
        }
    }
    impl<V: Writable<char>, D: Writable<char> + Default> Writable<char> for Punchuated<V, D> {
        fn write(&self, stream: &mut Vec<char>) {
            let mut iter = self.iter();
            iter.next().write(stream);
            MayNotSpace::default().write(stream);
            for item in iter {
                D::default().write(stream);
                MaySpace::default().write(stream);
                MayNotSpace::default().write(stream);
                item.write(stream);
            }
        }
    }
}
mod space {
    //! for parsing spaces(blank)
    //! TODO `MayNotSpace` and `MayNotSpaced`
    use autoparse::{Parsable, ParseError, ParseStream, Writable};
    use autoparse_derive::*;
    use dede::*;
    /// parses space like `' '`, `'\t'`, `'\r'`, `'\n'`
    pub struct Space {
        pub spaces: Vec<char>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Space {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Space {
                    spaces: ref __self_0_0,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Space");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "spaces",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Space {
        #[inline]
        fn clone(&self) -> Space {
            match *self {
                Space {
                    spaces: ref __self_0_0,
                } => Space {
                    spaces: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl Default for Space {
        fn default() -> Self {
            Self {
                spaces: <[_]>::into_vec(box [' ']),
            }
        }
    }
    impl Parsable<char> for Space {
        fn try_parse_no_rewind(
            stream: &mut impl ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), ParseError<char>> {
            let mut spaces = Vec::new();
            let mut stream_fork = stream.clone();
            while match {
                let mut c = ['\0'; 1];
                stream.read(&mut c);
                c[0]
            } {
                c @ (' ' | '\t' | '\r' | '\n') => {
                    stream_fork = stream.clone();
                    spaces.push(c);
                    true
                }
                _ => false,
            } {}
            *stream = stream_fork;
            if spaces.is_empty() {
                Err(ParseError::new(
                    [
                        <[_]>::into_vec(box [' ']),
                        <[_]>::into_vec(box ['\t']),
                        <[_]>::into_vec(box ['\r']),
                        <[_]>::into_vec(box ['\n']),
                    ]
                    .into(),
                    position,
                ))
            } else {
                let read = spaces.len();
                Ok((Self { spaces }, read))
            }
        }
    }
    impl Writable<char> for Space {
        fn write(&self, stream: &mut Vec<char>) {
            stream.extend_from_slice(&self.spaces);
        }
    }
    /// optional space
    pub type MaySpace = Option<Space>;
    pub type MayNotSpace = MaySpace;
    /// tailing space for `T`
    #[autoparse_for(char)]
    pub struct Spaced<T: Parsable<char>> {
        #[deref]
        pub inner: T,
        pub space: Space,
    }
    impl<T: Parsable<char>> autoparse::Parsable<char> for Spaced<T> {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let mut read = 0;
            let (inner, r): (T, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            let (space, r): (Space, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            Ok((Self { inner, space }, read))
        }
    }
    impl<T: Parsable<char>> autoparse::Writable<char> for Spaced<T> {
        fn write(&self, buffer: &mut Vec<char>) {
            autoparse::Writable::write(&self.inner, buffer);
            autoparse::Writable::write(&self.space, buffer);
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Parsable<char>> ::core::fmt::Debug for Spaced<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Spaced {
                    inner: ref __self_0_0,
                    space: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Spaced");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "inner",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "space",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Parsable<char>> ::core::clone::Clone for Spaced<T> {
        #[inline]
        fn clone(&self) -> Spaced<T> {
            match *self {
                Spaced {
                    inner: ref __self_0_0,
                    space: ref __self_0_1,
                } => Spaced {
                    inner: ::core::clone::Clone::clone(&(*__self_0_0)),
                    space: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<T: Parsable<char>> std::ops::Deref for Spaced<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
    impl<T: Parsable<char>> std::ops::DerefMut for Spaced<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
    impl<T: Parsable<char>> From<T> for Spaced<T> {
        fn from(inner: T) -> Self {
            Self {
                inner,
                space: Default::default(),
            }
        }
    }
    /// tailing optional space for `T`
    #[autoparse_for(char)]
    pub struct MaySpaced<T: Parsable<char>> {
        #[deref]
        pub inner: T,
        pub space: MaySpace,
    }
    impl<T: Parsable<char>> autoparse::Parsable<char> for MaySpaced<T> {
        fn try_parse_no_rewind(
            stream: &mut impl autoparse::ParseStream<char>,
            position: usize,
        ) -> Result<(Self, usize), autoparse::ParseError<char>> {
            let mut read = 0;
            let (inner, r): (T, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            let (space, r): (MaySpace, _) =
                autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
            read += r;
            Ok((Self { inner, space }, read))
        }
    }
    impl<T: Parsable<char>> autoparse::Writable<char> for MaySpaced<T> {
        fn write(&self, buffer: &mut Vec<char>) {
            autoparse::Writable::write(&self.inner, buffer);
            autoparse::Writable::write(&self.space, buffer);
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Parsable<char>> ::core::fmt::Debug for MaySpaced<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MaySpaced {
                    inner: ref __self_0_0,
                    space: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "MaySpaced");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "inner",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "space",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Parsable<char>> ::core::clone::Clone for MaySpaced<T> {
        #[inline]
        fn clone(&self) -> MaySpaced<T> {
            match *self {
                MaySpaced {
                    inner: ref __self_0_0,
                    space: ref __self_0_1,
                } => MaySpaced {
                    inner: ::core::clone::Clone::clone(&(*__self_0_0)),
                    space: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<T: Parsable<char>> std::ops::Deref for MaySpaced<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
    impl<T: Parsable<char>> std::ops::DerefMut for MaySpaced<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
    impl<T: Parsable<char>> From<T> for MaySpaced<T> {
        fn from(inner: T) -> Self {
            Self {
                inner,
                space: Default::default(),
            }
        }
    }
    pub type MayNotSpaced<T> = MaySpaced<T>;
}
pub use literal::*;
pub use numeric::*;
pub use punchuated::*;
pub use space::*;
