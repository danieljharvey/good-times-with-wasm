use nom::{character::complete::multispace0, error::ParseError, sequence::preceded, IResult};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
  inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  preceded(multispace0, inner)
}

