pub use sea_orm_migration::prelude::*;

mod m20230415_125808_users_table;
mod m20230415_160925_survival_economy_table;
mod m20230415_163049_cosmetics_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230415_125808_users_table::Migration),
            Box::new(m20230415_160925_survival_economy_table::Migration),
            Box::new(m20230415_163049_cosmetics_table::Migration),
        ]
    }
}
