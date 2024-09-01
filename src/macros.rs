/// Stringify a variable.
#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

#[macro_export]
macro_rules! info {
    ($x:expr) => {
        use crate::logging::Logging;
        use crate::s;
        Logging::Info.print(s!($x));
    };
}
