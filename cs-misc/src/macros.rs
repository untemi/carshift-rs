#[macro_export]
macro_rules! fancy_validator {
    ($name:ident, $pattern:expr) => {
        use fancy_regex::Regex as FancyRegex;
        use std::sync::LazyLock;
        use validator::ValidationError;

        paste::paste! {
            pub static [<REGEX_ $name:upper>]: LazyLock<FancyRegex> =
                LazyLock::new(|| FancyRegex::new($pattern).unwrap());

            pub fn [<validate_ $name>](s: &str) -> Result<(), ValidationError> {
                match [<REGEX_ $name:upper>].is_match(s) {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(ValidationError::new("invalid_string")),
                    Err(_) => Err(ValidationError::new("regex_error")),
                }
            }
        }
    };
}

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
