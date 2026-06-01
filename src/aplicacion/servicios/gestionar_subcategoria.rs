use crate::dominio::categoria::Subcategoria;
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoGestionarSubcategoria {
    repositorio_categoria: Arc<dyn RepositorioCategoria>,
}

impl CasoUsoGestionarSubcategoria {
    pub fn nuevo(repositorio_categoria: Arc<dyn RepositorioCategoria>) -> Self {
        Self { repositorio_categoria }
    }

    pub async fn crear(&self, categoria_id: i32, nombre: String, descripcion: Option<String>) -> Result<Subcategoria, Box<dyn Error + Send + Sync>> {
        self.repositorio_categoria.guardar_subcategoria(categoria_id, nombre, descripcion).await
    }

    pub async fn actualizar(&self, id: i32, nombre: String, descripcion: Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.repositorio_categoria.actualizar_subcategoria(id, nombre, descripcion).await
    }

    pub async fn eliminar(&self, id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.repositorio_categoria.eliminar_subcategoria(id).await
    }
}
