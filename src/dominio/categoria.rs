use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Categoria {
    pub id: Option<i32>,
    pub nombre: String,
    #[serde(skip_deserializing)]
    #[sqlx(skip)]
    pub subcategorias: Option<Vec<Subcategoria>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subcategoria {
    pub id: Option<i32>,
    pub categoria_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    #[serde(skip_deserializing)]
    #[sqlx(skip)]
    pub precio_normal: Option<Decimal>,
    #[serde(skip_deserializing)]
    #[sqlx(skip)]
    pub precio_medio: Option<Decimal>,
    #[serde(skip_deserializing)]
    #[sqlx(skip)]
    pub precio_urgente: Option<Decimal>,
}
