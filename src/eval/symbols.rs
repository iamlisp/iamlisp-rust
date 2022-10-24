#[macro_export]
macro_rules! cond_symbol {
    () => {
        $crate::eval::types::Expression::Symbol("cond")
    };
}
