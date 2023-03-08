use super::ty::Type;

#[derive(Debug, PartialEq)]
pub enum TypeError<Ann>
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
