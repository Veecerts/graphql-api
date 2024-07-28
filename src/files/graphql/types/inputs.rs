use async_graphql::*;

#[derive(InputObject)]
pub struct FileInput {
    pub creator_address: String,
    pub file: Upload,
    pub nft_image: Upload,
    pub title: String,
    pub description: String,
    pub public: bool,
}
