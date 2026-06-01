use crate::dominio::precio_subcategoria::PrecioSubcategoria;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioPrecioSubcategoria: Send + Sync {
    async fn guardar(&self, precio: PrecioSubcategoria) -> Result<PrecioSubcategoria, Box<dyn Error + Send + Sync>>;
    async fn actualizar(&self, precio: PrecioSubcategoria) -> Result<PrecioSubcategoria, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_subcategoria(&self, subcategoria_id: i32) -> Result<Option<PrecioSubcategoria>, Box<dyn Error + Send + Sync>>;
}
