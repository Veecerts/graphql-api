use async_graphql::*;
use entity::entities::file;

#[derive(SimpleObject)]
pub struct FileType {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub file_ipfs_hash: String,
    pub nft_ipfs_hash: String,
    pub creator: String,
    pub public: Option<bool>,
}

impl From<file::Model> for FileType {
    fn from(file: file::Model) -> Self {
        Self {
            id: file.id,
            title: file.title,
            description: file.description,
            file_ipfs_hash: file.file_ipfs_hash,
            nft_ipfs_hash: file.nft_ipfs_hash,
            creator: file.creator,
            public: file.public,
        }
    }
}
