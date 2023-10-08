mod prepare;

use entity::resource;
use prepare::prepare_mock_db;
use service::{Mutation, Query};

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let resource = Query::find_resource_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(resource.id, 1);
    }

    {
        let resource = Query::find_resource_by_id(db, 5).await.unwrap().unwrap();

        assert_eq!(resource.id, 5);
    }

    {
        let resource = Mutation::create_resource(
            db,
            resource::Model {
                id: 0,
                path: "Title D".to_owned(),
                hash: "Text D".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            resource,
            resource::ActiveModel {
                id: sea_orm::ActiveValue::Unchanged(6),
                path: sea_orm::ActiveValue::Unchanged("Title D".to_owned()),
                hash: sea_orm::ActiveValue::Unchanged("Text D".to_owned())
            }
        );
    }

    {
        let resource = Mutation::update_resource_by_id(
            db,
            1,
            resource::Model {
                id: 1,
                path: "New Title A".to_owned(),
                hash: "New Text A".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            resource,
            resource::Model {
                id: 1,
                path: "New Title A".to_owned(),
                hash: "New Text A".to_owned(),
            }
        );
    }

    {
        let result = Mutation::delete_resource(db, 5).await.unwrap();

        assert_eq!(result.rows_affected, 1);
    }

    {
        let result = Mutation::delete_all_resources(db).await.unwrap();

        assert_eq!(result.rows_affected, 5);
    }
}
