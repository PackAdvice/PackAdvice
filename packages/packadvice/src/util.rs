#[macro_export]
macro_rules! pack_path {
    ($namespace:expr, $path:expr) => {
        if $path.contains(':') {
            $path.to_string()
        } else {
            format!("{}:{}", $namespace, $path)
        }
    };
    ($path:expr) => {
        pack_path!("minecraft", $path)
    };
}
