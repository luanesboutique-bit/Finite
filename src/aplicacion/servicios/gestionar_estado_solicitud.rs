use crate::dominio::solicitud::{EstadoSolicitud};
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoGestionarEstadoSolicitud {
    repositorio_solicitud: Arc<dyn RepositorioSolicitud>,
}

impl CasoUsoGestionarEstadoSolicitud {
    pub fn nuevo(repositorio_solicitud: Arc<dyn RepositorioSolicitud>) -> Self {
        Self { repositorio_solicitud }
    }

    pub async fn ejecutar(
        &self,
        solicitud_id: i32,
        nuevo_estado: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut solicitud = self.repositorio_solicitud.buscar_por_id(solicitud_id).await?
            .ok_or("Solicitud no encontrada")?;

        let estado = match nuevo_estado.as_str() {
            "aceptado" => EstadoSolicitud::AceptadoPorColaborador,
            "rechazado" => EstadoSolicitud::Cancelado,
            "terminado" => EstadoSolicitud::Terminado,
            _ => return Err("Estado no válido".into()),
        };

        solicitud.estado = estado;
        self.repositorio_solicitud.actualizar_estado(solicitud.id.unwrap(), solicitud.estado).await?;
        Ok(())
    }
}
