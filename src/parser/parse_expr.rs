use super::lexeme;
use crate::parser::parse_constructors::{bool, int, var};
use crate::types::expr;
use nom::branch::alt;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::alpha1,
    combinator::{map, map_res},
    IResult,
};

// Expr with () for annotations (we'll leave this for later)
type ParseExpr = expr::Expr<()>;

fn is_int_digit(c: char) -> bool {
    c.is_digit(10)
}

fn int_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 12, is_int_digit), from_int)(input)
}

fn from_int(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 10)
}

fn parse_my_int(input: &str) -> IResult<&str, ParseExpr> {
    let (input, int_val) = lexeme::ws(int_primary)(input)?;
    let as_i32 = i32::from(int_val);
    Ok((input, int(as_i32)))
}

#[test]
fn test_parse_my_int() {
    assert_eq!(parse_my_int("1"), Ok(("", int(1))));
    assert_eq!(parse_my_int("11"), Ok(("", int(11))));
    assert_eq!(parse_my_int("11dog"), Ok(("dog", int(11))));
}

fn parse_my_var(input: &str) -> IResult<&str, ParseExpr> {
    let (input, var_val) = lexeme::ws(alpha1)(input)?;
    Ok((input, var(var_val)))
}

fn parse_true(input: &str) -> IResult<&str, ParseExpr> {
    map(tag("True"), |_| bool(true))(input)
}

fn parse_false(input: &str) -> IResult<&str, ParseExpr> {
    map(tag("False"), |_| bool(false))(input)
}

fn parse_my_bool(input: &str) -> IResult<&str, ParseExpr> {
    alt((parse_true, parse_false))(input)
}

#[test]
fn test_parse_my_bool() {
    assert_eq!(parse_my_bool("True"), Ok(("", bool(true))));
    assert_eq!(parse_my_bool("False"), Ok(("", bool(false))));
    assert_eq!(parse_my_bool("True100"), Ok(("100", bool(true))));
}

#[test]
fn test_parse_my_var() {
    assert_eq!(parse_my_var("p"), Ok(("", var("p"))));
    assert_eq!(parse_my_var("poo"), Ok(("", var("poo"))));
    assert_eq!(parse_my_var("poo "), Ok((" ", var("poo"))))
}

pub fn parse_my_expr(input: &str) -> IResult<&str, ParseExpr> {
    alt((parse_my_bool, parse_my_int, parse_my_var))(input)
}
