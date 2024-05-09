#[macro_export]
macro_rules! update_field {
    ($query_builder:expr, $field:expr, $value:expr) => {
        match $value {
            Field::Str(val) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            Field::Int(val) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            Field::Bool(val) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            Field::Uuid(val) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            Field::DateTime(val) => {
                $query_builder
                    .push(format!(" {} = ", $field))
                    .push_bind_unseparated(val);
            }
            Field::Ignore => unreachable!(),
        }
    };
}

#[macro_export]
macro_rules! insert_fields {
    ($query_builder:expr, $fields:expr) => {
        let mut field_list = $query_builder.separated(", ");

        for (field, _) in $fields {
            field_list.push(field);
        }

        $query_builder.push(" ) VALUES ( ");

        let mut value_list = $query_builder.separated(", ");

        for (_, value) in $fields {
            match value {
                Field::Str(val) => {
                    value_list.push_bind(val);
                }
                Field::Int(val) => {
                    value_list.push_bind(val);
                }
                Field::Bool(val) => {
                    value_list.push_bind(val);
                }
                Field::Uuid(val) => {
                    value_list.push_bind(val);
                }
                Field::DateTime(val) => {
                    value_list.push_bind(val);
                }
                Field::Ignore => unreachable!(),
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
                $value.into_field()
            }};
        }

        let mut vec = Vec::new();

        $(
            let field = to_field!($values);
            if field != Field::Ignore {
                vec.push((stringify!($name), field));
            }
        )*

        vec
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
