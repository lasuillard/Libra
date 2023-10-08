use ::entity::{resource, resource::Entity as Resource};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_resource(
        db: &DbConn,
        form_data: resource::Model,
    ) -> Result<resource::ActiveModel, DbErr> {
        resource::ActiveModel {
            path: Set(form_data.path.to_owned()),
            hash: Set(form_data.hash.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_resource_by_id(
        db: &DbConn,
        id: i32,
        form_data: resource::Model,
    ) -> Result<resource::Model, DbErr> {
        let resource: resource::ActiveModel = Resource::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find resource.".to_owned()))
            .map(Into::into)?;

        resource::ActiveModel {
            id: resource.id,
            path: Set(form_data.path.to_owned()),
            hash: Set(form_data.hash.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_resource(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let resource: resource::ActiveModel = Resource::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find resource.".to_owned()))
            .map(Into::into)?;

        resource.delete(db).await
    }

    pub async fn delete_all_resources(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Resource::delete_many().exec(db).await
    }
}
