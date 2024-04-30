#[macro_export]
macro_rules! update_field {
    ($query:expr, $field:expr, $value:expr) => {
        if let Some(val) = $value {
            $query.push(concat!(" ", $field, " = ")).push_bind(val);
        }
    };
}