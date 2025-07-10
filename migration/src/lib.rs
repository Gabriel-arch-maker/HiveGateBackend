pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250607_142640_create_users_table;
mod m20250607_143909_update_users_table;mod m20250710_093513_add_details_to_users;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250607_142640_create_users_table::Migration),
            Box::new(m20250607_143909_update_users_table::Migration),
        ]
    }
}
