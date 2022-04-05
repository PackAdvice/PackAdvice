#[macro_export]
macro_rules! minecraft_path {
    ($namespace:expr, $path:expr) => {
        if $path.contains(':') {
            $path.to_string()
        } else {
            format!("{}:{}", $namespace, $path)
        }
    };
    ($path:expr) => {
        minecraft_path!("minecraft", $path)
    };
}
