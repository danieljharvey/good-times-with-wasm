use crate::types::expr::Expr;

pub fn interpret_expr<Ann>(expr: Expr<Ann>) -> Expr<Ann> {
    match expr {
        Expr::EInt { ann, int } => Expr::EInt { ann, int },
        Expr::EBool { ann, bool } => Expr::EBool { ann, bool },
        Expr::EIf {
            pred_expr,
            then_expr,
            else_expr,
            ..
        } => {
            let interpreted_pred = interpret_expr(*pred_expr);
            match interpreted_pred {
                Expr::EBool { bool: true, .. } => interpret_expr(*then_expr),
                Expr::EBool { bool: false, .. } => interpret_expr(*else_expr),
                _other => todo!(),
            }
        }
        Expr::ELet { rest_expr, .. } => interpret_expr(*rest_expr),
    }
}

#[test]
fn test_interpret_if() {
    let int_one = Expr::EInt { ann: (), int: 1 };
    let int_two = Expr::EInt { ann: (), int: 2 };
    let if_expr = Expr::EIf {
        ann: (),
        pred_expr: Box::new(Expr::EBool {
            ann: (),
            bool: true,
        }),
        then_expr: Box::new(int_one.clone()),
        else_expr: Box::new(int_two.clone()),
    };

    assert_eq!(interpret_expr(if_expr), int_one);

    let if_expr_2 = Expr::EIf {
        ann: (),
        pred_expr: Box::new(Expr::EBool {
            ann: (),
            bool: false,
        }),
        then_expr: Box::new(int_one.clone()),
        else_expr: Box::new(int_two.clone()),
    };

    assert_eq!(interpret_expr(if_expr_2), int_two);
}
