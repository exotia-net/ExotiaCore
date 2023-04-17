use sea_orm_migration::prelude::*;

use crate::m20230417_210219_wallet_table::Wallet;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WalletActions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WalletActions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WalletActions::WalletId).integer().not_null())
                    .col(ColumnDef::new(WalletActions::Action).string().not_null())
                    .col(ColumnDef::new(WalletActions::Value).string().not_null())
                    .col(ColumnDef::new(WalletActions::CreatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(WalletActions::UpdatedAt).date_time().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))//.extra("ON UPDATE CURRENT_TIMESTAMP".to_string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-wallet-id")
                            .from(WalletActions::Table, WalletActions::WalletId)
                            .to(Wallet::Table, Wallet::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletActions::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum WalletActions {
    Table,
    Id,
    WalletId,
    Action,
    Value,
    CreatedAt,
    UpdatedAt,
}
