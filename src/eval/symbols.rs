#[macro_export]
macro_rules! cond_symbol {
    () => {
        $crate::eval::types::Expression::Symbol("cond")
    };
}

#[macro_export]
macro_rules! begin_symbol {
    () => {
        $crate::eval::types::Expression::Symbol("begin")
    };
}

#[macro_export]
macro_rules! def_symbol {
    () => {
        $crate::eval::types::Expression::Symbol("def")
    };
}
