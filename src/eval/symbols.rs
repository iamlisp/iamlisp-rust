#[macro_export]
macro_rules! symbol {
    ($name:literal) => {
        $crate::eval::types::Expression::Symbol($name)
    };
}

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

#[macro_export]
macro_rules! quote_symbol {
    () => {
        $crate::eval::types::Expression::Symbol("quote")
    };
}

#[macro_export]
macro_rules! loop_symbol {
    () => {
        $crate::eval::types::Expression::Symbol("loop")
    };
}
