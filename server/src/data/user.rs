use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    username: String,
    #[sea_orm(column_type = "Text")]
    password_hash: String, 
    created_at: time::PrimitiveDateTime,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {
    
}