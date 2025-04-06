use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "reviews",
            &[
            
            ("id", ColType::PkAuto),
            
            ("movie_id", ColType::SmallIntegerNull),
            ("rating", ColType::SmallIntegerNull),
            ("comment", ColType::String),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "reviews").await
    }
}
