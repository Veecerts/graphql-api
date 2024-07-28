use sea_orm_migration::prelude::*;

use crate::m20240728_105913_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(File::Title).string().not_null())
                    .col(ColumnDef::new(File::Description).string().not_null())
                    .col(
                        ColumnDef::new(File::FileIpfsHash)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(File::NftIpfsHash)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(File::Creator).string().not_null())
                    .col(
                        ColumnDef::new(File::Public)
                            .boolean()
                            .default(Value::Bool(Some(true))),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_file_creator")
                            .from(File::Table, File::Creator)
                            .to(User::Table, User::Address)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
    Title,
    Public,
    Creator,
    Description,
    FileIpfsHash,
    NftIpfsHash,
}
