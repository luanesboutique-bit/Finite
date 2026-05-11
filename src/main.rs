use std::sync::Arc;
use finit::infraestructura::api::rutas::{self as ax_routing, EstadoApp};
use finit::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use finit::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use finit::aplicacion::servicios::login_usuario::CasoUsoLoginUsuario;
use finit::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use finit::aplicacion::servicios::listar_subcategorias::CasoUsoListarSubcategorias;
use finit::aplicacion::servicios::consultar_perfil_colaborador::CasoUsoConsultarPerfilColaborador;
use finit::aplicacion::servicios::solicitud_servicio::CasoUsoSolicitudServicio;
use finit::aplicacion::servicios::listar_solicitudes::CasoUsoListarSolicitudes;
use finit::aplicacion::servicios::listar_colaboradores_marketplace::CasoUsoListarColaboradoresMarketplace;
use finit::aplicacion::servicios::gestionar_mensajes::CasoUsoGestionarMensajes;
use finit::aplicacion::servicios::actualizar_documentacion::CasoUsoActualizarDocumentacion;
use finit::aplicacion::servicios::configurar_precios_dinamicos::CasoUsoConfigurarPreciosDinamicos;
use finit::aplicacion::servicios::configurar_horarios::CasoUsoConfigurarHorarios;
use finit::infraestructura::{RepositorioMySQL, sqlite_repositorio::RepositorioSQLite};
use sqlx::{MySqlPool, SqlitePool};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok(); // Carga las variables de entorno

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no definida en .env");
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secreto_finit".to_string());
    let puerto = std::env::var("PUERTO").unwrap_or_else(|_| "3000".to_string());

    let (estado, motor_nombre) = if db_url.starts_with("sqlite:") {
        println!("Conectando a SQLite...");
        let pool = SqlitePool::connect(&db_url).await?;
        let repo = Arc::new(RepositorioSQLite::nuevo(pool));
        repo.inicializar_tablas().await?;
        (crear_estado(repo.clone(), jwt_secret), "SQLite")
    } else {
        println!("Conectando a MySQL...");
        let pool = MySqlPool::connect(&db_url).await?;
        let repo = Arc::new(RepositorioMySQL::nuevo(pool));
        repo.inicializar_tablas().await?;
        (crear_estado(repo.clone(), jwt_secret), "MySQL")
    };

    println!("🚀 Iniciando motor finit (Versión {})...", motor_nombre);

    // Configurar Rutas
    let app = ax_routing::crear_rutas(Arc::new(estado))
        .layer(CorsLayer::permissive());

    // Iniciar Servidor
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", puerto)).await?;
    println!("📡 Servidor finit iniciado en http://localhost:{}", puerto);
    
    axum::serve(listener, app).await?;

    Ok(())
}

fn crear_estado<R>(
    repo: Arc<R>,
    jwt_secret: String
) -> EstadoApp 
where R: finit::dominio::puertos::repositorio_usuario::RepositorioUsuario + 
         finit::dominio::puertos::repositorio_colaborador::RepositorioColaborador +
         finit::dominio::puertos::repositorio_categoria::RepositorioCategoria +
         finit::dominio::puertos::repositorio_servicio::RepositorioServicio +
         finit::dominio::puertos::repositorio_solicitud::RepositorioSolicitud +
         finit::dominio::puertos::repositorio_mensaje::RepositorioMensaje +
         finit::dominio::puertos::repositorio_disponibilidad::RepositorioDisponibilidad +
         finit::dominio::puertos::repositorio_configuracion_precio::RepositorioConfiguracionPrecio +
         'static
{
    let registro_colaborador = Arc::new(CasoUsoRegistroColaborador::nuevo(
        repo.clone(),
        repo.clone(),
        repo.clone(),
        jwt_secret.clone(),
    ));

    let registro_usuario = Arc::new(CasoUsoRegistroUsuario::nuevo(
        repo.clone(),
    ));

    let login_usuario = Arc::new(CasoUsoLoginUsuario::nuevo(
        repo.clone(),
        jwt_secret.clone(),
    ));

    let listar_categorias = Arc::new(CasoUsoListarCategorias::nuevo(
        repo.clone(),
    ));

    let listar_subcategorias = Arc::new(CasoUsoListarSubcategorias::nuevo(
        repo.clone(),
    ));

    let consultar_perfil_colaborador = Arc::new(CasoUsoConsultarPerfilColaborador::nuevo(
        repo.clone(),
        repo.clone(),
        repo.clone(),
    ));

    let solicitud_servicio = Arc::new(CasoUsoSolicitudServicio::nuevo(
        repo.clone(),
        repo.clone(),
    ));

    let listar_solicitudes = Arc::new(CasoUsoListarSolicitudes::nuevo(
        repo.clone(),
    ));

    let listar_colaboradores_marketplace = Arc::new(CasoUsoListarColaboradoresMarketplace::nuevo(
        repo.clone(),
        repo.clone(),
        repo.clone(),
    ));

    let gestionar_mensajes = Arc::new(CasoUsoGestionarMensajes::nuevo(
        repo.clone(),
    ));

    let actualizar_documentacion = Arc::new(CasoUsoActualizarDocumentacion::nuevo(
        repo.clone(),
    ));

    let configurar_precios_dinamicos = Arc::new(CasoUsoConfigurarPreciosDinamicos::nuevo(
        repo.clone(),
    ));

    let configurar_horarios = Arc::new(CasoUsoConfigurarHorarios::nuevo(
        repo.clone(),
    ));

    let verificar_colaborador = Arc::new(finit::aplicacion::servicios::verificar_colaborador::CasoUsoVerificarColaborador::nuevo(
        repo.clone(),
    ));

    let gestionar_estado_solicitud = Arc::new(finit::aplicacion::servicios::gestionar_estado_solicitud::CasoUsoGestionarEstadoSolicitud::nuevo(
        repo.clone(),
    ));

    EstadoApp {
        registro_colaborador,
        registro_usuario,
        login_usuario,
        listar_categorias,
        listar_subcategorias,
        consultar_perfil_colaborador,
        solicitud_servicio,
        listar_solicitudes,
        listar_colaboradores_marketplace,
        gestionar_mensajes,
        actualizar_documentacion,
        configurar_precios_dinamicos,
        configurar_horarios,
        verificar_colaborador,
        gestionar_estado_solicitud,
    }
}
