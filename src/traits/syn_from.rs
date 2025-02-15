// TODO: Use standard_traits::Provide
pub trait SynFrom<Value> {
    type Output;

    fn syn_from(value: Value) -> Self::Output;
}
