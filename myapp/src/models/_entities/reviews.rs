use crate::models::_entities::movies;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "reviews")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub movie_id: i16,
    pub rating: i16,
    pub comment: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "movies::Entity",
        from = "Column::MovieId",
        to = "movies::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Movie,
}

impl Related<movies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movie.def()
    }
}

// impl ActiveModelBehavior for ActiveModel {}
