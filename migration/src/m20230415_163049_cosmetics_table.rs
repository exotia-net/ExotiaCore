use sea_orm_migration::prelude::*;

use crate::m20230415_125808_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cosmetics::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cosmetics::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cosmetics::UserId).integer().not_null().unique_key())
                    .col(ColumnDef::new(Cosmetics::CosmeticId).string().not_null())
                    .col(ColumnDef::new(Cosmetics::CreatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Cosmetics::UpdatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))//.extra("ON UPDATE CURRENT_TIMESTAMP".to_string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-id")
                            .from(Cosmetics::Table, Cosmetics::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cosmetics::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Cosmetics {
    Table,
    Id,
    UserId,
    CosmeticId,
    CreatedAt,
    UpdatedAt,
}
