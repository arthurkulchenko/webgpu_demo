pub struct TypeIs<T>(pub T);

impl<T> std::fmt::Debug for TypeIs<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n~~~~~~~~~~~~TypeIs: {}\n", std::any::type_name::<T>())
    }
}
