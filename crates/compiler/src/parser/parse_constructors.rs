use crate::types::expr::{Expr, Prim};

// construct int
pub fn int(int_val: i32) -> Expr<()> {
    Expr::EPrim {
        ann: (),
        prim: Prim::PInt { int: int_val },
    }
}

// construct bool
pub fn bool(bool_val: bool) -> Expr<()> {
    Expr::EPrim {
        ann: (),
        prim: Prim::PBool { bool: bool_val },
    }
}

// construct var
pub fn var(identifier: &str) -> Expr<()> {
    Expr::EVar {
        ann: (),
        identifier: identifier.to_string(),
    }
}

// construct if
pub fn mk_if(pred_expr: Expr<()>, then_expr: Expr<()>, else_expr: Expr<()>) -> Expr<()> {
    Expr::EIf {
        ann: (),
        pred_expr: Box::new(pred_expr),
        then_expr: Box::new(then_expr),
        else_expr: Box::new(else_expr),
    }
}
