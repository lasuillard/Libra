use ::entity::{resource, resource::Entity as Post};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_resource_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<resource::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }

    /// If ok, returns (resource models, num pages).
    pub async fn find_resources_in_page(
        db: &DbConn,
        page: u64,
        resources_per_page: u64,
    ) -> Result<(Vec<resource::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Post::find()
            .order_by_asc(resource::Column::Id)
            .paginate(db, resources_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated resources
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
