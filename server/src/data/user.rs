use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub password_hash: String, 
    pub created_at: time::PrimitiveDateTime,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {
    
}