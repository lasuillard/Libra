#![cfg(feature = "mock")]
use ::entity::resource;
use sea_orm::*;

pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [resource::Model {
                id: 1,
                path: "Title A".to_owned(),
                hash: "Text A".to_owned(),
            }],
            [resource::Model {
                id: 5,
                path: "Title C".to_owned(),
                hash: "Text C".to_owned(),
            }],
            [resource::Model {
                id: 6,
                path: "Title D".to_owned(),
                hash: "Text D".to_owned(),
            }],
            [resource::Model {
                id: 1,
                path: "Title A".to_owned(),
                hash: "Text A".to_owned(),
            }],
            [resource::Model {
                id: 1,
                path: "New Title A".to_owned(),
                hash: "New Text A".to_owned(),
            }],
            [resource::Model {
                id: 5,
                path: "Title C".to_owned(),
                hash: "Text C".to_owned(),
            }],
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 5,
            },
        ])
        .into_connection()
}
