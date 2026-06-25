use crate::dominio::precio_subcategoria::PrecioSubcategoria;
use crate::dominio::puertos::repositorio_precio_subcategoria::RepositorioPrecioSubcategoria;
use crate::infraestructura::RepositorioMySQL;
use async_trait::async_trait;
use std::error::Error;
use sqlx::Row;
use rust_decimal::Decimal;

#[async_trait]
impl RepositorioPrecioSubcategoria for RepositorioMySQL {
    async fn guardar(&self, precio: PrecioSubcategoria) -> Result<PrecioSubcategoria, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO precio_subcategoria (subcategoria_id, precio_normal, precio_medio, precio_urgente) VALUES (?, ?, ?, ?)")
            .bind(precio.subcategoria_id)
            .bind(precio.precio_normal.to_string())
            .bind(precio.precio_medio.to_string())
            .bind(precio.precio_urgente.to_string())
            .execute(&self.pool).await?;
        Ok(PrecioSubcategoria { id: Some(resultado.last_insert_id() as i32), ..precio })
    }
    async fn actualizar(&self, precio: PrecioSubcategoria) -> Result<PrecioSubcategoria, Box<dyn Error + Send + Sync>> {
        sqlx::query("UPDATE precio_subcategoria SET precio_normal = ?, precio_medio = ?, precio_urgente = ? WHERE subcategoria_id = ?")
            .bind(precio.precio_normal.to_string())
            .bind(precio.precio_medio.to_string())
            .bind(precio.precio_urgente.to_string())
            .bind(precio.subcategoria_id).execute(&self.pool).await?;
        Ok(precio)
    }
    async fn buscar_por_subcategoria(&self, subcategoria_id: i32) -> Result<Option<PrecioSubcategoria>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, subcategoria_id, precio_normal, precio_medio, precio_urgente FROM precio_subcategoria WHERE subcategoria_id = ?").bind(subcategoria_id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(PrecioSubcategoria {
                id: Some(r.get(0)), subcategoria_id: r.get(1),
                precio_normal: r.get::<String, _>(2).parse().unwrap_or(Decimal::ZERO),
                precio_medio: r.get::<String, _>(3).parse().unwrap_or(Decimal::ZERO),
                precio_urgente: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
            }))
        } else { Ok(None) }
    }
}
