use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccountUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccountUser::Uuid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AccountUser::FirstName).string().not_null())
                    .col(ColumnDef::new(AccountUser::LastName).string().not_null())
                    .col(ColumnDef::new(AccountUser::Email).string().not_null())
                    .col(ColumnDef::new(AccountUser::Password).string().not_null())
                    .col(ColumnDef::new(AccountUser::Role).string().not_null())
                    .col(ColumnDef::new(AccountUser::IsActive).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AccountUser {
    Table,
    Uuid,
    FirstName,
    LastName,
    Email,
    Password,
    Role,
    IsActive,
}
