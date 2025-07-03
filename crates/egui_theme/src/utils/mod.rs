pub mod math;
pub mod random;
pub trait FromRef<T> {
    fn from_ref(value: &T) -> Self;
}
