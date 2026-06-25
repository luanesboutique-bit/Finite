use crate::dominio::precio_subcategoria::PrecioSubcategoria;
use crate::dominio::puertos::repositorio_precio_subcategoria::RepositorioPrecioSubcategoria;
use std::error::Error;
use std::sync::Arc;
use rust_decimal::Decimal;

pub struct CasoUsoConfigurarPrecioSubcategoria {
    repositorio_precio: Arc<dyn RepositorioPrecioSubcategoria>,
}

impl CasoUsoConfigurarPrecioSubcategoria {
    pub fn nuevo(repositorio_precio: Arc<dyn RepositorioPrecioSubcategoria>) -> Self {
        Self { repositorio_precio }
    }

    pub async fn ejecutar(
        &self,
        subcategoria_id: i32,
        precio_normal: Decimal,
        precio_medio: Decimal,
        precio_urgente: Decimal,
    ) -> Result<PrecioSubcategoria, Box<dyn Error + Send + Sync>> {
        let precio_existente = self.repositorio_precio.buscar_por_subcategoria(subcategoria_id).await?;

        let precio = PrecioSubcategoria {
            id: precio_existente.as_ref().and_then(|p| p.id),
            subcategoria_id,
            precio_normal,
            precio_medio,
            precio_urgente,
        };

        if precio_existente.is_some() {
            self.repositorio_precio.actualizar(precio).await
        } else {
            self.repositorio_precio.guardar(precio).await
        }
    }
}
