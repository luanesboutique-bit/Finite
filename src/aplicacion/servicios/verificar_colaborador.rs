use crate::dominio::colaborador::{EstadoVerificacion};
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoVerificarColaborador {
    repositorio_colaborador: Arc<dyn RepositorioColaborador>,
}

impl CasoUsoVerificarColaborador {
    pub fn nuevo(repositorio_colaborador: Arc<dyn RepositorioColaborador>) -> Self {
        Self { repositorio_colaborador }
    }

    pub async fn ejecutar(
        &self,
        colaborador_id: i32,
        estado: String,
        _comentario: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut colaborador = self.repositorio_colaborador.buscar_por_id(colaborador_id).await?
            .ok_or("Colaborador no encontrado")?;

        match estado.as_str() {
            "verificado" => {
                colaborador.es_verificado = true;
                colaborador.estado_verificacion = EstadoVerificacion::Verificado;
            },
            "rechazado" => {
                colaborador.es_verificado = false;
                colaborador.estado_verificacion = EstadoVerificacion::Rechazado;
            },
            _ => return Err("Estado de verificación no válido".into()),
        }

        self.repositorio_colaborador.actualizar(colaborador).await?;
        Ok(())
    }

    pub async fn listar_pendientes(&self) -> Result<Vec<crate::dominio::colaborador::Colaborador>, Box<dyn Error + Send + Sync>> {
        // Por simplicidad, buscamos todos y filtramos. 
        // En un sistema real, el repositorio debería tener un método específico.
        // Pero como RepositorioColaborador no tiene 'listar', usaremos SQL directo si fuera posible
        // o añadiremos el método al trait.
        
        // Vamos a añadir 'listar_pendientes' al RepositorioColaborador trait.
        self.repositorio_colaborador.listar_pendientes().await
    }
}
