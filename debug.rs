impl < T : Parsable < char > > autoparse :: Parsable < char > for MaySpaced
{
    fn try_parse(buffer : & mut & [T]) -> Result < (Self, usize), autoparse ::
    ParseError < T >>
    {
        let read = & mut 0 ;
        (||
         {
             let(head : MaySpace, r) autoparse :: Parsable ::
             try_parse(buffer) ? ; read += r ; let(inner : T, r) autoparse ::
             Parsable :: try_parse(buffer) ? ; read += r ;
             let(tail : MaySpace, r) autoparse :: Parsable ::
             try_parse(buffer) ? ; read += r ;
             Ok((Self { head, inner, tail }, * read))
         }) () . map_err(| e | e . advance(* read))
    } fn write(& self, buffer : & mut Vec < T >)
    {
        autoparse :: Parsable :: write(buffer) ; autoparse :: Parsable ::
        write(buffer) ; autoparse :: Parsable :: write(buffer) ;
    }
}