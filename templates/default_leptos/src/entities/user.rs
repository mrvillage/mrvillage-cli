use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
#[cfg(feature = "ssr")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(DeriveEntityModel))]
#[cfg_attr(feature = "ssr", sea_orm(table_name = "user"))]
pub struct Model {
    #[cfg_attr(feature = "ssr", sea_orm(primary_key))]
    pub id:         i32,
    pub name:       String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(EnumIter, DeriveRelation))]
pub enum Relation {}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl ActiveModelBehavior for ActiveModel {}
    }
}
