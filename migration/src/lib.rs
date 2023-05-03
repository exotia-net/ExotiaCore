pub use sea_orm_migration::prelude::*;

mod m20230415_125808_users_table;
mod m20230415_160925_survival_economy_table;
mod m20230415_163049_cosmetics_table;
mod m20230417_210219_wallet_table;
mod m20230417_213028_wallet_actions_table;
mod m20230424_185538_servers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230415_125808_users_table::Migration),
            Box::new(m20230415_160925_survival_economy_table::Migration),
            Box::new(m20230415_163049_cosmetics_table::Migration),
            Box::new(m20230417_210219_wallet_table::Migration),
            Box::new(m20230417_213028_wallet_actions_table::Migration),
            Box::new(m20230424_185538_servers_table::Migration),
        ]
    }
}
