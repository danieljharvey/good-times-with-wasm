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
