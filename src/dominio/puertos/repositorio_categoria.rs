use crate::dominio::categoria::{Categoria, Subcategoria};
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioCategoria: Send + Sync {
    async fn listar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>>;
    async fn listar_subcategorias(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>>;
    async fn buscar_subcategoria_por_id(&self, id: i32) -> Result<Option<Subcategoria>, Box<dyn Error + Send + Sync>>;
    async fn guardar_subcategoria(&self, categoria_id: i32, nombre: String, descripcion: Option<String>) -> Result<Subcategoria, Box<dyn Error + Send + Sync>>;
    async fn actualizar_subcategoria(&self, id: i32, nombre: String, descripcion: Option<String>) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn eliminar_subcategoria(&self, id: i32) -> Result<(), Box<dyn Error + Send + Sync>>;
}
