use interpret::interpreter::interpret_expr;
use typecheck::elaborate::elaborate_expr;
use types::expr::Expr;
pub mod compile;
pub mod interpret;
pub mod typecheck;
pub mod types;

// typecheck an arbitrary thing
fn main() {
    match elaborate_expr(Expr::EInt { ann: (), int: 1 }) {
        Ok(expr) => {
            interpret_expr(expr);
        }
        Err(err) => println!("{:?}", err),
    }
}
