use async_graphql::*;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbConn, EntityTrait, QueryFilter};

use crate::services::ipfs::pin_file_to_ipfs;
use entity::entities::{file, user};

use super::types::{inputs::FileInput, outputs::FileType};

#[derive(Default)]
pub struct FileMutations;

#[Object]
impl FileMutations {
    pub async fn upload_file(&self, ctx: &Context<'_>, input: FileInput) -> Result<FileType> {
        let db = ctx.data::<DbConn>()?;

        let user = user::Entity::find()
            .filter(user::Column::Address.eq(&input.creator_address))
            .one(db)
            .await?;

        match user {
            Some(_) => {}
            None => {
                let new_user = user::ActiveModel {
                    id: ActiveValue::NotSet,
                    email: ActiveValue::NotSet,
                    password_hash: ActiveValue::NotSet,
                    address: ActiveValue::Set(input.creator_address.clone()),
                };
                new_user.insert(db).await?;
            }
        }

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
