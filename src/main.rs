use interpret::interpreter::interpret_expr;
use types::expr::{get_expr_annotation, map_expr, Expr};
use types::ty::{remove_type_annotation, Type};
use types::typeerror::TypeError;

pub mod interpret;
pub mod types;

fn infer<Ann>(expr: Expr<Ann>) -> Result<Expr<Type<Ann>>, TypeError<Ann>>
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
        } => {
            Result::map_err(check(*pred_expr, Type::TBool { ann }), |err| match err {
                TypeError::TypeMismatch { type_b, .. } => TypeError::PredicateShouldBeBool {
                    ann: ann,
                    found: type_b,
                },
                other => other,
            })?;

            let then_a = infer(*then_expr)?;

            Result::map_err(
                check(*else_expr, get_expr_annotation(then_a)),
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
    }
}

fn check<Ann>(expr: Expr<Ann>, expected_type: Type<Ann>) -> Result<Expr<Type<Ann>>, TypeError<Ann>>
where
    Ann: Clone + Copy,
{
    let expr_a = infer(expr)?;
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
fn basic_prim_values() {
    let int_expr = Expr::EInt { ann: (), int: 1 };

    assert_eq!(
        Result::map(infer(int_expr.clone()), get_expr_annotation),
        Result::Ok(Type::TInt { ann: () })
    );

    let bool_expr = Expr::EBool {
        ann: (),
        bool: true,
    };

    assert_eq!(
        Result::map(infer(bool_expr.clone()), get_expr_annotation),
        Result::Ok(Type::TBool { ann: () })
    );

    let if_with_wrong_pred_type = Expr::EIf {
        ann: (),
        pred_expr: Box::new(int_expr.clone()),
        then_expr: Box::new(int_expr.clone()),
        else_expr: Box::new(int_expr.clone()),
    };

    assert_eq!(
        infer(if_with_wrong_pred_type),
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
        infer(if_with_mismatched_branch_types),
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
        Result::map(infer(if_that_returns_ints), get_expr_annotation),
        Result::Ok(Type::TInt { ann: () })
    )
}

// typecheck an arbitrary thing
fn main() {
    match infer(Expr::EInt { ann: (), int: 1 }) {
        Ok(expr) => {
            interpret_expr(expr);
        }
        Err(err) => println!("{:?}", err),
    }
}
