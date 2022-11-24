macro_rules! ordered_systems {
    ($name:path => $($args:tt)*) => {
        $name.pipe(ordered_systems!($($args)*))
    };

    ($name:path) => {
        $name
    };
}
