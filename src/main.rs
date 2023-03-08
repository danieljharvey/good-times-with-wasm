#[derive(Debug, PartialEq, Clone)]
enum Expr<Ann> {
    EInt {
        ann: Ann,
        int: i32,
    },
    EBool {
        ann: Ann,
        bool: bool,
    },
    EIf {
        ann: Ann,
        pred_expr: Box<Self>,
        then_expr: Box<Self>,
        else_expr: Box<Self>,
    },
}

fn map_expr<F, A, B>(expr: Expr<A>, f: F) -> Expr<B>
where
    F: FnOnce(A) -> B + Copy,
    A: Clone,
    B: Clone,
{
    match expr {
        Expr::EInt { ann, int } => Expr::EInt { ann: f(ann), int },
        Expr::EBool { ann, bool } => Expr::EBool { ann: f(ann), bool },
        Expr::EIf {
            ann,
            pred_expr,
            then_expr,
            else_expr,
        } => Expr::EIf {
            ann: f(ann),
            pred_expr: Box::new(map_expr(*pred_expr, f)),
            then_expr: Box::new(map_expr(*then_expr, f)),
            else_expr: Box::new(map_expr(*else_expr, f)),
        },
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Type<Ann>
where
    Ann: Clone + Copy,
{
    TInt { ann: Ann },
    TBool { ann: Ann },
}

fn map_type<F, A, B>(a: Type<A>, f: F) -> Type<B>
where
    F: FnOnce(A) -> B,
    A: Clone + Copy,
    B: Clone + Copy,
{
    match a {
        Type::TInt { ann } => Type::TInt { ann: f(ann) },
        Type::TBool { ann } => Type::TBool { ann: f(ann) },
    }
}

fn get_expr_annotation<Ann>(expr: Expr<Ann>) -> Ann {
    match expr {
        Expr::EInt { ann, .. } => ann,
        Expr::EBool { ann, .. } => ann,
        Expr::EIf { ann, .. } => ann,
    }
}
#[derive(Debug, PartialEq)]
enum TypeError<Ann>
where
    Ann: Clone + Copy,
{
    PredicateShouldBeBool {
        ann: Ann,
        found: Type<Ann>,
    },
    MismatchedIfBranches {
        ann: Ann,
        then_found: Type<Ann>,
        else_found: Type<Ann>,
    },
    TypeMismatch {
        type_a: Type<Ann>,
        type_b: Type<Ann>,
    },
}

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

fn remove_type_annotation<Ann>(ty: Type<Ann>) -> Type<()>
where
    Ann: Clone + Copy,
{
    map_type(ty, |_| ())
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

fn main() {
    let _oh = infer(Expr::EInt { ann: (), int: 1 });
    ()
}
