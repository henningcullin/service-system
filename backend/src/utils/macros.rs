#[macro_export]
macro_rules! update_field {
    ($query_builder:expr, $field:expr, $value:expr) => {
        match $value {
            Field::Str(Some(val)) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            Field::Int(Some(val)) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            Field::Bool(Some(val)) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! insert_fields {
    ($query_builder:expr, $fields:expr) => {
        let mut field_list = $query_builder.separated(", ");

        let fields: Vec<_> = $fields
            .into_iter()
            .filter(|(_, field)| match field {
                Field::Str(ref value) => value.is_some(),
                Field::Int(ref value) => value.is_some(),
                Field::Bool(ref value) => value.is_some(),
                Field::Uuid(ref value) => value.is_some(),
                Field::DateTime(ref value) => value.is_some(),
            })
            .collect();

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

#[macro_export]
macro_rules! field_vec {
    ($($name:ident => $values:expr),*) => {{

        macro_rules! to_field {
            ($value:expr) => {{
                Field::from($value)
            }};
        }

        vec![
            $(
                (stringify!($name), to_field!($values)),
            )*
        ]
    }};
}

#[macro_export]
macro_rules! user_from_id {
    ($id:expr) => {
        query_as!(
            User,
            r#"
                SELECT
                    u.id,
                    u.first_name,
                    u.last_name,
                    u.email,
                    u.phone,
                    (
                        r.id,
                        r.name,
                        r.level,
                        r.has_password,
                        r.user_view,
                        r.user_create,
                        r.user_edit,
                        r.user_delete,
                        r.machine_view,
                        r.machine_create,
                        r.machine_edit,
                        r.machine_delete,
                        r.task_view,
                        r.task_create,
                        r.task_edit,
                        r.task_delete,
                        r.report_view,
                        r.report_create,
                        r.report_edit,
                        r.report_delete,
                        r.facility_view,
                        r.facility_create,
                        r.facility_edit,
                        r.facility_delete
                    ) AS "role!: Role",
                    u.active,
                    u.last_login,
                    u.occupation,
                    u.image,
                    (
                        f.id,
                        f.name,
                        f.address
                    ) AS "facility?: Facility"
                FROM
                    users u
                INNER JOIN
                    roles r
                ON
                    u.role = r.id
                LEFT JOIN
                    facilities f
                ON
                    u.facility = f.id
                WHERE
                    u.id = $1
            "#,
            $id
        )
    };
}
