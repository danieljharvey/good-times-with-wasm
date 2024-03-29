use super::lexeme;
use crate::parser::parse_constructors::{bool, int, mk_if, var};
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
    assert_eq!(parse_my_int(" 1"), Ok(("", int(1))));
    assert_eq!(parse_my_int("1"), Ok(("", int(1))));
    assert_eq!(parse_my_int("11"), Ok(("", int(11))));
    assert_eq!(parse_my_int("11dog"), Ok(("dog", int(11))));
}

// check we aren't using protected words for variables
fn var_is_protected(ident: &str) -> bool {
    vec!["True", "False", "if", "then", "else"].contains(&ident)
}

// jesus
fn parse_my_var(input: &str) -> IResult<&str, ParseExpr> {
    map_res(lexeme::ws(alpha1), |var_val| {
        match var_is_protected(var_val) {
            true => Err(nom::Err::Error {
                0: nom::error::Error {
                    code: nom::error::ErrorKind::Tag,
                    input: input,
                },
            }),
            false => Ok(var(var_val)),
        }
    })(input)
}

#[test]
fn test_parse_my_var() {
    assert_ne!(parse_my_var("False"), Ok(("", var("False"))));
    assert_ne!(parse_my_var("True"), Ok(("", var("True"))));
    assert_ne!(parse_my_var("if"), Ok(("", var("if"))));
    assert_eq!(parse_my_var(" p"), Ok(("", var("p"))));
    assert_eq!(parse_my_var("p"), Ok(("", var("p"))));
    assert_eq!(parse_my_var("poo"), Ok(("", var("poo"))));
    assert_eq!(parse_my_var("poo "), Ok((" ", var("poo"))))
}

fn parse_true(input: &str) -> IResult<&str, ParseExpr> {
    map(lexeme::ws(tag("True")), |_| bool(true))(input)
}

fn parse_false(input: &str) -> IResult<&str, ParseExpr> {
    map(lexeme::ws(tag("False")), |_| bool(false))(input)
}

fn parse_my_bool(input: &str) -> IResult<&str, ParseExpr> {
    alt((parse_true, parse_false))(input)
}

#[test]
fn test_parse_my_bool() {
    assert_eq!(parse_my_bool(" True"), Ok(("", bool(true))));
    assert_eq!(parse_my_bool("False"), Ok(("", bool(false))));
    assert_eq!(parse_my_bool("   True100"), Ok(("100", bool(true))));
}

pub fn parse_my_if(input: &str) -> IResult<&str, ParseExpr> {
    let (input, _) = lexeme::ws(tag("if"))(input)?;
    let (input, pred_expr) = parse_my_expr(input)?;

    let (input, _) = lexeme::ws(tag("then"))(input)?;
    let (input, then_expr) = parse_my_expr(input)?;

    let (input, _) = lexeme::ws(tag("else"))(input)?;
    let (input, else_expr) = parse_my_expr(input)?;

    Ok((input, mk_if(pred_expr, then_expr, else_expr)))
}

#[test]
fn test_parse_my_if() {
    assert_eq!(
        parse_my_if("if 1 then False else True"),
        Ok(("", mk_if(int(1), bool(false), bool(true))))
    );

    assert_eq!(
        parse_my_if("if False then 1 else 2"),
        Ok(("", mk_if(bool(false), int(1), int(2))))
    );
}

#[test]
fn test_parse_my_expr() {
    assert_eq!(parse_my_expr("True"), Ok(("", bool(true))));
    assert_eq!(parse_my_expr("False"), Ok(("", bool(false))));
    assert_eq!(parse_my_expr("p"), Ok(("", var("p"))));
    assert_eq!(parse_my_expr("poo"), Ok(("", var("poo"))));
}

pub fn parse_my_expr(input: &str) -> IResult<&str, ParseExpr> {
    alt((parse_my_bool, parse_my_int, parse_my_var, parse_my_if))(input)
}
