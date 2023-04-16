use sea_orm_migration::prelude::*;
use super::m20230415_125808_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SurvivalEconomy::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SurvivalEconomy::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SurvivalEconomy::UserId).integer().not_null())
                    .col(ColumnDef::new(SurvivalEconomy::Balance).integer().not_null())
                    .col(ColumnDef::new(SurvivalEconomy::CreatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(SurvivalEconomy::UpdatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))//.extra("ON UPDATE CURRENT_TIMESTAMP".to_string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_id")
                            .from(SurvivalEconomy::Table, SurvivalEconomy::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SurvivalEconomy::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum SurvivalEconomy {
    Table,
    Id,
    UserId,
    Balance,
    CreatedAt,
    UpdatedAt,
}
