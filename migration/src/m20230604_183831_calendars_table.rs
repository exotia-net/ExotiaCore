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
                    .table(Calendars::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Calendars::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Calendars::UserId).integer().not_null().unique_key())
                    .col(ColumnDef::new(Calendars::Step).integer().not_null())
                    .col(ColumnDef::new(Calendars::LastObtained).timestamp().not_null())
                    .col(ColumnDef::new(Calendars::Streak).integer().not_null())
                    .col(ColumnDef::new(Calendars::ObtainedRewards).text().not_null())
                    .col(ColumnDef::new(Calendars::CreatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Calendars::UpdatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))//.extra("ON UPDATE CURRENT_TIMESTAMP".to_string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-id")
                            .from(Calendars::Table, Calendars::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Calendars::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Calendars {
    Table,
    Id,
    UserId,
    Step,
    LastObtained,
    Streak,
    ObtainedRewards,
    CreatedAt,
    UpdatedAt,
}
