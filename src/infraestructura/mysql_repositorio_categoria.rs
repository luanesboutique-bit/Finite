use crate::dominio::categoria::{Categoria, Subcategoria};
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioCategoria for RepositorioMySQL {
    async fn listar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, Categoria>(
            "SELECT id, nombre FROM categoria"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn listar_subcategorias(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, Subcategoria>(
            "SELECT id, categoria_id, nombre, descripcion FROM subcategoria WHERE categoria_id = ?"
        )
        .bind(categoria_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn buscar_subcategoria_por_id(&self, id: i32) -> Result<Option<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, Subcategoria>(
            "SELECT id, categoria_id, nombre, descripcion FROM subcategoria WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }

    async fn guardar_subcategoria(&self, categoria_id: i32, nombre: String, descripcion: Option<String>) -> Result<Subcategoria, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO subcategoria (categoria_id, nombre, descripcion) VALUES (?, ?, ?)")
            .bind(categoria_id).bind(&nombre).bind(&descripcion).execute(&self.pool).await?;
        Ok(Subcategoria { id: Some(resultado.last_insert_id() as i32), categoria_id, nombre, descripcion })
    }
    async fn actualizar_subcategoria(&self, id: i32, nombre: String, descripcion: Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("UPDATE subcategoria SET nombre = ?, descripcion = ? WHERE id = ?")
            .bind(&nombre).bind(&descripcion).bind(id).execute(&self.pool).await?;
        Ok(())
    }
    async fn eliminar_subcategoria(&self, id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("DELETE FROM subcategoria WHERE id = ?").bind(id).execute(&self.pool).await?;
        Ok(())
    }
}
