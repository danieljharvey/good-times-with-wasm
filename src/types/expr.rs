#[derive(Debug, PartialEq, Clone)]

pub enum Expr<Ann> {
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
    ELet {
        ann: Ann,
        identifier: String,
        bound_expr: Box<Self>,
        rest_expr: Box<Self>,
    },
}

pub fn map_expr<F, A, B>(expr: Expr<A>, f: F) -> Expr<B>
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
        Expr::ELet {
            ann,
            identifier,
            bound_expr,
            rest_expr,
        } => Expr::ELet {
            ann: f(ann),
            identifier,
            bound_expr: Box::new(map_expr(*bound_expr, f)),
            rest_expr: Box::new(map_expr(*rest_expr, f)),
        },
    }
}

pub fn get_expr_annotation<Ann>(expr: Expr<Ann>) -> Ann {
    match expr {
        Expr::EInt { ann, .. } => ann,
        Expr::EBool { ann, .. } => ann,
        Expr::EIf { ann, .. } => ann,
        Expr::ELet { ann, .. } => ann,
    }
}
