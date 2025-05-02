#[macro_export]
macro_rules! ico {
    ($name:expr) => {
        format!(
            r#"<div class="w-6 h-6">{}</div>"#,
            iconify::svg!($name, width = "100%")
        )
    };
}

#[macro_export]
macro_rules! ico_mini {
    ($name:expr) => {
        format!(
            r#"<div class="w-4 h-4">{}</div>"#,
            iconify::svg!($name, width = "100%")
        )
    };
}
