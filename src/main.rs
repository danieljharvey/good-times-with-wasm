use interpret::interpreter::interpret_expr;
use typecheck::elaborate::infer;
use types::expr::Expr;
pub mod interpret;
pub mod typecheck;
pub mod types;

// typecheck an arbitrary thing
fn main() {
    match infer(Expr::EInt { ann: (), int: 1 }) {
        Ok(expr) => {
            interpret_expr(expr);
        }
        Err(err) => println!("{:?}", err),
    }
}
