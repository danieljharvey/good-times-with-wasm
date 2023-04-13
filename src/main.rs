use interpret::interpreter::interpret_expr;
use typecheck::elaborate::elaborate_expr;
use types::expr::{Expr, Prim};

pub mod compile;
pub mod interpret;
pub mod parser;
pub mod typecheck;
pub mod types;

// typecheck an arbitrary thing
fn main() {
    let raw_expr = Expr::EPrim {
        ann: (),
        prim: Prim::PInt { int: 1 },
    };

    match elaborate_expr(raw_expr) {
        Ok(expr) => {
            interpret_expr(expr);
        }
        Err(err) => println!("{:?}", err),
    }
}
