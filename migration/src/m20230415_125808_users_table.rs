use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Uuid).string().not_null())
                    .col(ColumnDef::new(Users::FirstIp).string().not_null())
                    .col(ColumnDef::new(Users::LastIp).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Users::UpdatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))//.extra("ON UPDATE CURRENT_TIMESTAMP".to_string()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Uuid,
    FirstIp,
    LastIp,
    CreatedAt,
    UpdatedAt,
}
