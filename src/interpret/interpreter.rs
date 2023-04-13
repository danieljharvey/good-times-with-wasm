use crate::parser::parse_constructors::{bool, int};
use crate::types::expr::{Expr, Prim};

pub fn interpret_expr<Ann>(expr: Expr<Ann>) -> Expr<Ann> {
    match expr {
        Expr::EPrim { ann, prim } => Expr::EPrim { ann, prim },
        Expr::EIf {
            pred_expr,
            then_expr,
            else_expr,
            ..
        } => {
            let interpreted_pred = interpret_expr(*pred_expr);
            match interpreted_pred {
                Expr::EPrim { prim, .. } => match prim {
                    Prim::PBool { bool: true, .. } => interpret_expr(*then_expr),
                    Prim::PBool { bool: false, .. } => interpret_expr(*else_expr),
                    _ => todo!(),
                },

                _other => todo!(),
            }
        }
        Expr::ELet { rest_expr, .. } => interpret_expr(*rest_expr),
        Expr::EVar { .. } => todo!(),
    }
}

#[test]
fn test_interpret_if() {
    let int_one = int(1);

    let int_two = int(2);

    let if_expr = Expr::EIf {
        ann: (),
        pred_expr: Box::new(bool(true)),
        then_expr: Box::new(int_one.clone()),
        else_expr: Box::new(int_two.clone()),
    };

    assert_eq!(interpret_expr(if_expr), int_one);

    let if_expr_2 = Expr::EIf {
        ann: (),
        pred_expr: Box::new(Expr::EPrim {
            ann: (),
            prim: Prim::PBool { bool: false },
        }),
        then_expr: Box::new(int_one.clone()),
        else_expr: Box::new(int_two.clone()),
    };

    assert_eq!(interpret_expr(if_expr_2), int_two);
}
