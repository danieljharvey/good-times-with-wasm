
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type<Ann>
where
    Ann: Clone + Copy,
{
    TInt { ann: Ann },
    TBool { ann: Ann },
}

pub fn map_type<F, A, B>(a: Type<A>, f: F) -> Type<B>
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

pub fn remove_type_annotation<Ann>(ty: Type<Ann>) -> Type<()>
where
    Ann: Clone + Copy,
{
    map_type(ty, |_| ())
}


