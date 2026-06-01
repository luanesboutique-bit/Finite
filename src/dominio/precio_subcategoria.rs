use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PrecioSubcategoria {
    pub id: Option<i32>,
    pub subcategoria_id: i32,
    pub precio_normal: Decimal,
    pub precio_nocturno: Decimal,
    pub precio_domingo_festivo: Decimal,
}
