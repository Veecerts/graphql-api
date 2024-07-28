use async_graphql::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DbConn};

use crate::services::ipfs::pin_file_to_ipfs;
use entity::entities::file;

use super::types::{inputs::FileInput, outputs::FileType};

#[derive(Default)]
pub struct FileMutations;

#[Object]
impl FileMutations {
    pub async fn upload_file(&self, ctx: &Context<'_>, input: FileInput) -> Result<FileType> {
        let db = ctx.data::<DbConn>()?;

        let file_cid = pin_file_to_ipfs(input.file, ctx).await?;
        let nft_cid = pin_file_to_ipfs(input.nft_image, ctx).await?;
        let new_file = file::ActiveModel {
            id: ActiveValue::NotSet,
            title: ActiveValue::Set(input.title),
            description: ActiveValue::Set(input.description),
            file_ipfs_hash: ActiveValue::Set(file_cid),
            nft_ipfs_hash: ActiveValue::Set(nft_cid),
            creator: ActiveValue::Set(input.creator_address),
            public: ActiveValue::Set(Some(input.public)),
        };
        let file = new_file.insert(db).await?;
        return Ok(file.into());
    }
}
