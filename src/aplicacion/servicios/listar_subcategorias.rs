use crate::dominio::categoria::Subcategoria;
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use crate::dominio::puertos::repositorio_precio_subcategoria::RepositorioPrecioSubcategoria;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoListarSubcategorias {
    repo_categoria: Arc<dyn RepositorioCategoria>,
    repo_precio: Arc<dyn RepositorioPrecioSubcategoria>,
}

impl CasoUsoListarSubcategorias {
    pub fn nuevo(
        repo_categoria: Arc<dyn RepositorioCategoria>,
        repo_precio: Arc<dyn RepositorioPrecioSubcategoria>,
    ) -> Self {
        Self { repo_categoria, repo_precio }
    }

    pub async fn ejecutar(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let mut subcategorias = self.repo_categoria.listar_subcategorias(categoria_id).await?;
        for sub in &mut subcategorias {
            if let Some(id) = sub.id {
                if let Ok(Some(precios)) = self.repo_precio.buscar_por_subcategoria(id).await {
                    sub.precio_normal = Some(precios.precio_normal);
                    sub.precio_medio = Some(precios.precio_medio);
                    sub.precio_urgente = Some(precios.precio_urgente);
                }
            }
        }
        Ok(subcategorias)
    }

    pub async fn buscar_por_id(&self, id: i32) -> Result<Option<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let mut subcategoria = self.repo_categoria.buscar_subcategoria_por_id(id).await?;
        if let Some(sub) = &mut subcategoria {
            if let Ok(Some(precios)) = self.repo_precio.buscar_por_subcategoria(id).await {
                sub.precio_normal = Some(precios.precio_normal);
                sub.precio_medio = Some(precios.precio_medio);
                sub.precio_urgente = Some(precios.precio_urgente);
            }
        }
        Ok(subcategoria)
    }
}
