use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Servers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Servers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Servers::Name).string().not_null())
                    .col(ColumnDef::new(Servers::MaxRecordedPlayers).integer().not_null())
                    .col(ColumnDef::new(Servers::Version).string().not_null())
                    .col(ColumnDef::new(Servers::StartedAt).timestamp().not_null())
                    .col(ColumnDef::new(Servers::CreatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Servers::UpdatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))//.extra("ON UPDATE CURRENT_TIMESTAMP".to_string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Servers::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Servers {
    Table,
    Id,
    Name,
    MaxRecordedPlayers,
    Version,
    StartedAt,
    CreatedAt,
    UpdatedAt,
}
