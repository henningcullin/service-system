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

#[macro_export]
macro_rules! insert_param {
    ($query_builder:expr, $value:expr) => {
        match $value {
            Field::Str(Some(val)) => {
                $query_builder.push_bind(val);
            }
            Field::Int(Some(val)) => {
                $query_builder.push_bind(val);
            }
            Field::Bool(Some(val)) => {
                $query_builder.push_bind(val);
            }
            _ => {}
        }
    };
}