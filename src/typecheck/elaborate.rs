use crate::types::expr::{get_expr_annotation, map_expr, Expr};
use crate::types::ty::{map_type, remove_type_annotation, Type};
use crate::types::typeerror::TypeError;

use std::collections::HashMap;

// entry point, here we create an empty type checking environment
// and then start the internal bits
pub fn elaborate_expr<Ann>(expr: Expr<Ann>) -> Result<Expr<Type<Ann>>, TypeError<Ann>>
where
    Ann: Clone + Copy,
{
    let mut env = HashMap::new();

    infer(&mut env, expr)
}

fn infer<Ann>(
    env: &mut HashMap<String, Type<Ann>>,
    expr: Expr<Ann>,
) -> Result<Expr<Type<Ann>>, TypeError<Ann>>
where
    Ann: Clone + Copy,
{
    match expr {
        Expr::EInt { ann, int } => Result::Ok(Expr::EInt {
            ann: Type::TInt { ann },
            int: int,
        }),
        Expr::EBool { ann, bool } => Result::Ok(Expr::EBool {
            ann: Type::TBool { ann },
            bool: bool,
        }),
        Expr::EIf {
            ann,
            pred_expr,
            then_expr,
            else_expr,
        } => infer_if(env, ann, *pred_expr, *then_expr, *else_expr),
        Expr::ELet {
            identifier,
            bound_expr,
            rest_expr,
            ..
        } => {
            let bound_a = infer(env, *bound_expr)?;
            env.insert(identifier, get_expr_annotation(bound_a));
            infer(env, *rest_expr)
        }
        Expr::EVar { identifier, ann } => match env.get(&identifier).copied() {
            Option::Some(ty) => {
                let type_with_ann = map_type(ty, |_| ann);
                Result::Ok(Expr::EVar {
                    ann: type_with_ann,
                    identifier,
                })
            }
            Option::None => panic!("no"),
        },
    }
}

fn infer_if<Ann>(
    env: &mut HashMap<String, Type<Ann>>,
    ann: Ann,
    pred_expr: Expr<Ann>,
    then_expr: Expr<Ann>,
    else_expr: Expr<Ann>,
) -> Result<Expr<Type<Ann>>, TypeError<Ann>>
where
    Ann: Copy,
{
    Result::map_err(
        check(env, pred_expr, Type::TBool { ann }),
        |err| match err {
            TypeError::TypeMismatch { type_b, .. } => TypeError::PredicateShouldBeBool {
                ann: ann,
                found: type_b,
            },
            other => other,
        },
    )?;

    let then_a = infer(env, then_expr)?;

    Result::map_err(
        check(env, else_expr, get_expr_annotation(then_a)),
        |err| match err {
            TypeError::TypeMismatch { type_a, type_b } => TypeError::MismatchedIfBranches {
                ann,
                then_found: type_a,
                else_found: type_b,
            },
            other => other,
        },
    )
}

fn check<Ann>(
    env: &mut HashMap<String, Type<Ann>>,
    expr: Expr<Ann>,
    expected_type: Type<Ann>,
) -> Result<Expr<Type<Ann>>, TypeError<Ann>>
where
    Ann: Clone + Copy,
{
    let expr_a = infer(env, expr)?;
    let found_type = get_expr_annotation(expr_a.clone());
    let combined_type = subtype(expected_type, found_type)?;
    // when we're doing real subtyping we should probably munge `combined_type`
    Result::Ok(map_expr(expr_a, |_| combined_type))
}

fn subtype<Ann>(type_a: Type<Ann>, type_b: Type<Ann>) -> Result<Type<Ann>, TypeError<Ann>>
where
    Ann: Clone + Copy,
{
    if remove_type_annotation(type_a.clone()) == remove_type_annotation(type_b.clone()) {
        Result::Ok(type_a)
    } else {
        Result::Err(TypeError::TypeMismatch { type_a, type_b })
    }
}

#[test]
fn test_basic_prim_values() {
    let int_expr = Expr::EInt { ann: (), int: 1 };

    assert_eq!(
        Result::map(elaborate_expr(int_expr.clone()), get_expr_annotation),
        Result::Ok(Type::TInt { ann: () })
    );

    let bool_expr = Expr::EBool {
        ann: (),
        bool: true,
    };

    assert_eq!(
        Result::map(elaborate_expr(bool_expr.clone()), get_expr_annotation),
        Result::Ok(Type::TBool { ann: () })
    );

    let if_with_wrong_pred_type = Expr::EIf {
        ann: (),
        pred_expr: Box::new(int_expr.clone()),
        then_expr: Box::new(int_expr.clone()),
        else_expr: Box::new(int_expr.clone()),
    };

    assert_eq!(
        elaborate_expr(if_with_wrong_pred_type),
        Result::Err(TypeError::PredicateShouldBeBool {
            ann: (),
            found: Type::TInt { ann: () }
        })
    );

    let if_with_mismatched_branch_types = Expr::EIf {
        ann: (),
        pred_expr: Box::new(bool_expr.clone()),
        then_expr: Box::new(bool_expr.clone()),
        else_expr: Box::new(int_expr.clone()),
    };

    assert_eq!(
        elaborate_expr(if_with_mismatched_branch_types),
        Result::Err(TypeError::MismatchedIfBranches {
            ann: (),
            then_found: Type::TBool { ann: () },
            else_found: Type::TInt { ann: () }
        })
    );

    let if_that_returns_ints = Expr::EIf {
        ann: (),
        pred_expr: Box::new(bool_expr.clone()),
        then_expr: Box::new(int_expr.clone()),
        else_expr: Box::new(int_expr.clone()),
    };

    assert_eq!(
        Result::map(elaborate_expr(if_that_returns_ints), get_expr_annotation),
        Result::Ok(Type::TInt { ann: () })
    )
}

#[test]
fn test_let_and_var() {
    let int_expr = Expr::EInt { ann: (), int: 1 };

    let let_and_fetch = Expr::ELet {
        ann: (),
        identifier: "a".to_string(),
        bound_expr: Box::new(int_expr.clone()),
        rest_expr: Box::new(Expr::EVar {
            ann: (),
            identifier: "a".to_string(),
        }),
    };

    assert_eq!(
        Result::map(elaborate_expr(let_and_fetch.clone()), get_expr_annotation),
        Result::Ok(Type::TInt { ann: () })
    );
}
