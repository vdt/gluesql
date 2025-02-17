#[macro_export]
macro_rules! row {
    ( $( $p:path )* ; $( $v:expr )* ) => (
        gluesql_core::data::Row(vec![$( $p($v) ),*])
    )
}

#[macro_export]
macro_rules! idx {
    () => {
        vec![]
    };
    ($name: path, $op: path, $sql_expr: literal) => {
        vec![gluesql_core::ast::IndexItem::NonClustered {
            name: stringify_label!($name).to_owned(),
            asc: None,
            cmp_expr: Some((
                $op,
                gluesql_core::translate::translate_expr(
                    &gluesql_core::parse_sql::parse_expr($sql_expr).unwrap(),
                )
                .unwrap(),
            )),
        }]
    };
    ($name: path) => {
        vec![gluesql_core::ast::IndexItem::NonClustered {
            name: stringify_label!($name).to_owned(),
            asc: None,
            cmp_expr: None,
        }]
    };
    ($name: path, ASC) => {
        vec![gluesql_core::ast::IndexItem::NonClustered {
            name: stringify_label!($name).to_owned(),
            asc: Some(true),
            cmp_expr: None,
        }]
    };
    ($name: path, DESC) => {
        vec![gluesql_core::ast::IndexItem::NonClustered {
            name: stringify_label!($name).to_owned(),
            asc: Some(false),
            cmp_expr: None,
        }]
    };
}

#[macro_export]
macro_rules! select {
    ( $( $c: tt )|+ $( ; )? $( $t: path )|+ ; $( $v: expr )+ ; $( $( $v2: expr )+ );+) => ({
        let mut rows = vec![
            row!($( $t )+ ; $( $v )+),
        ];

        gluesql_core::executor::Payload::Select {
            labels: vec![$( stringify_label!($c).to_owned()),+],
            rows: concat_with!(rows ; $( $t )+ ; $( $( $v2 )+ );+)
        }
    });
    ( $( $c: tt )|+ $( ; )? $( $t: path )|+ ; $( $v: expr )+ ) => (
        gluesql_core::executor::Payload::Select {
            labels: vec![$( stringify_label!($c).to_owned()),+],
            rows: vec![row!($( $t )+ ; $( $v )+ )],
        }
    );
    ( $( $c: tt )|+ $( ; )?) => (
        gluesql_core::executor::Payload::Select {
            labels: vec![$( stringify_label!($c).to_owned()),+],
            rows: vec![],
        }
    );
}

#[macro_export]
macro_rules! concat_with {
    ( $rows: ident ; $( $t:path )* ; $( $v: expr )* ) => ({
        $rows.push(row!($( $t )* ; $( $v )*));

        $rows
    });
    ( $rows: ident ; $( $t:path )* ; $( $v: expr )* ; $( $( $v2: expr )* );* ) => ({
        $rows.push(row!($( $t )* ; $( $v )*));

        concat_with!($rows ; $( $t )* ; $( $( $v2 )* );* )
    });
}

#[macro_export]
macro_rules! stringify_label {
    ($label: literal) => {
        $label
    };
    ($label: tt) => {
        stringify!($label)
    };
}

#[macro_export]
macro_rules! select_with_null {
    ( $( $c: tt )|* ; $( $v: expr )* ) => (
        gluesql_core::executor::Payload::Select {
            labels: vec![$( stringify_label!($c).to_owned()),+],
            rows: vec![gluesql_core::data::Row(vec![$( $v ),*])],
        }
    );
    ( $( $c: tt )|* ; $( $v: expr )* ; $( $( $v2: expr )* );*) => ({
        let mut rows = vec![
            gluesql_core::data::Row(vec![$( $v ),*])
        ];

        gluesql_core::executor::Payload::Select {
            labels: vec![$( stringify_label!($c).to_owned()),+],
            rows: concat_with_null!(rows ; $( $( $v2 )* );*),
        }
    });
}

#[macro_export]
macro_rules! concat_with_null {
    ( $rows: ident ; $( $v: expr )* ) => ({
        $rows.push(gluesql_core::data::Row(vec![$( $v ),*]));

        $rows
    });
    ( $rows: ident ; $( $v: expr )* ; $( $( $v2: expr )* );* ) => ({
        $rows.push(gluesql_core::data::Row(vec![$( $v ),*]));

        concat_with_null!($rows ; $( $( $v2 )* );* )
    });
}
