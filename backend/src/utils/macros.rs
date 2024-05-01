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
macro_rules! insert_fields {
    ($query_builder:expr, $fields:expr) => {
        let mut field_list = $query_builder.separated(", ");

        let fields: Vec<_> = $fields.into_iter().filter(|(_, field)| {
            match field {
                Field::Str(ref value) => value.is_some(),
                Field::Int(ref value) => value.is_some(),
                Field::Bool(ref value) => value.is_some(),
            }
        }).collect();

        for (field, _) in &fields {
            field_list.push(field);
        }

        $query_builder.push(" ) VALUES ( ");    

        let mut value_list = $query_builder.separated(", ");

        for (_, value) in fields {
            match value {
                Field::Str(Some(val)) => {
                    value_list.push_bind(val);
                }
                Field::Int(Some(val)) => {
                    value_list.push_bind(val);
                }
                Field::Bool(Some(val)) => {
                    value_list.push_bind(val);
                }
                _ => {}
            }
        }

        $query_builder.push(" )");
    };
}