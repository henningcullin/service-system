#[macro_export]
macro_rules! update_field {
    ($query_builder:expr, $field:expr, $value:expr) => {
        match $value {
            Field::Str(Some(val)) => {
                $query_builder.push(format!(" {} = ", $field)).push_bind_unseparated(val);
            }
            Field::Int(Some(val)) => {
                $query_builder.push(format!(" {} = ", $field)).push_bind_unseparated(val);
            }
            Field::Bool(Some(val)) => {
                $query_builder.push(format!(" {} = ", $field)).push_bind_unseparated(val);
            }
            _ => {}
        }
    };
}