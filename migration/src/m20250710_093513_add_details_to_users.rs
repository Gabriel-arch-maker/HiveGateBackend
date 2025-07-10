use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter().table(Users::Table)
                .add_column(ColumnDef::new(Users::PhoneNumber).string().null())
                .add_column(ColumnDef::new(Users::IsVerified).boolean().not_null().default(false))
                .add_column(ColumnDef::new(Users::Gender).string().null())
                .add_column(ColumnDef::new(Users::DateOfBirth).string().null())
                .add_column(ColumnDef::new(Users::ProfilePictureUrl).text().null())
                .add_column(ColumnDef::new(Users::ResidencyStatus).string().null())
                .add_column(ColumnDef::new(Users::Role).string().null())
                .add_column(ColumnDef::new(Users::Title).string().null())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter().table(Users::Table) .drop_column(Users::PhoneNumber)
                             .drop_column(Users::IsVerified)
                             .drop_column(Users::Gender)
                             .drop_column(Users::DateOfBirth)
                             .drop_column(Users::ProfilePictureUrl)
                             .drop_column(Users::ResidencyStatus)
                             .drop_column(Users::Role)
                             .drop_column(Users::Title).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    PhoneNumber,
    IsVerified,
    Gender,
    DateOfBirth,
    ProfilePictureUrl,
    ResidencyStatus,
    Role,
    Title,
}
