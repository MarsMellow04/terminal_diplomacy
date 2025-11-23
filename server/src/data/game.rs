use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "game_phase")]
pub enum GamePhase {
    #[sea_orm(string_value = "SpringMovement")]
    SpringMovement,
    #[sea_orm(string_value = "SpringRetreat")]
    SpringRetreat,
    #[sea_orm(string_value = "FallMovement")]
    FallMovement,
    #[sea_orm(string_value = "FallRetreat")]
    FallRetreat,
    #[sea_orm(string_value = "WinterBuild")]
    WinterBuild,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub game_id: i32,
    pub name: String,
    pub year: i32,
    pub game_phase: GamePhase, 
    pub created_at: time::PrimitiveDateTime,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {
    
}