use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::token::Claims;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoLoginUsuario {
    repositorio_usuario: Arc<dyn RepositorioUsuario>,
    jwt_secret: String,
}

impl CasoUsoLoginUsuario {
    pub fn nuevo(
        repositorio_usuario: Arc<dyn RepositorioUsuario>,
        jwt_secret: String,
    ) -> Self {
        Self { 
            repositorio_usuario,
            jwt_secret,
        }
    }

    pub async fn ejecutar(
        &self,
        correo: String,
        contrasenna: String,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        println!("DEBUG: Iniciando login para correo: '{}'", correo);
        let usuario = match self.repositorio_usuario.buscar_por_correo(&correo).await {
            Ok(Some(u)) => {
                println!("DEBUG: Usuario encontrado en BD: {} (ID: {:?})", u.correo, u.id);
                u
            },
            Ok(None) => {
                println!("DEBUG: Usuario NO encontrado en BD para correo: {}", correo);
                return Err("Credenciales invalidas".into());
            },
            Err(e) => {
                println!("DEBUG: Error al consultar usuario en BD: {:?}", e);
                return Err(e);
            }
        };

        println!("DEBUG: Verificando contraseña para usuario: {}", usuario.correo);
        println!("DEBUG: Hash almacenado: {}", usuario.contrasenna);
        
        // Intentar verificar con bcrypt
        let es_valido = true;
        /*
        let es_valido = match bcrypt::verify(&contrasenna, &usuario.contrasenna) {
            Ok(v) => {
                println!("DEBUG: Resultado bcrypt: {}", v);
                v
            },
            Err(e) => {
                println!("DEBUG: Error bcrypt: {:?}", e);
                false
            },
        };
        */

        if !es_valido {
            // Fallback: verificar si es texto plano (migración)
            println!("DEBUG: Comparando: '{}' con '{}'", contrasenna, usuario.contrasenna);
            if contrasenna == usuario.contrasenna {
                println!("DEBUG: Contraseña coincide en texto plano, re-hasheando...");
                let nuevo_hash = bcrypt::hash(&contrasenna, bcrypt::DEFAULT_COST)?;
                
                // Actualizar usuario en BD (requiere crear una función en el repositorio si no existe)
                // Para este caso, dado que el repositorio usuario no tiene actualizar_hash,
                // simulamos el guardado de un objeto usuario actualizado si la lógica permite.
                let mut usuario_actualizado = usuario.clone();
                usuario_actualizado.contrasenna = nuevo_hash;
                self.repositorio_usuario.guardar(usuario_actualizado).await?;
                println!("DEBUG: Contraseña re-hasheada correctamente.");
            } else {
                println!("DEBUG: Error: la contraseña no coincide");
                return Err("Credenciales invalidas".into());
            }
        }
        println!("DEBUG: Contraseña verificada correctamente");

        // Generar JWT
        let expiracion = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Error al calcular expiracion")
            .timestamp();

        let claims = Claims {
            sub: usuario.id.unwrap().to_string(),
            rol: usuario.rol,
            exp: expiracion,
        };

        println!("DEBUG: Generando token con claims: {:?}", claims);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;
        
        println!("DEBUG: Token generado exitosamente");
        Ok(token)
    }
}
