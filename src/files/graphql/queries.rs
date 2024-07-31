use async_graphql::*;
use entity::entities::file;
use sea_orm::{DbConn, EntityTrait};

use super::types::outputs::FileType;

#[derive(Default)]
pub struct FileQueries;

#[Object]
impl FileQueries {
    pub async fn files(&self, ctx: &Context<'_>) -> Result<Vec<FileType>> {
        let db = ctx.data::<DbConn>()?;
        let files = file::Entity::find().all(db).await?;
        Ok(files.into_iter().map(FileType::from).collect())
    }

    pub async fn file(&self, ctx: &Context<'_>, id: i32) -> Result<FileType> {
        let db = ctx.data::<DbConn>()?;
        let file = file::Entity::find_by_id(id).one(db).await?;
        Ok(FileType::from(file.unwrap()))
    }
}
