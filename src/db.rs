use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();
//Conexión a la base
lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("No existe la base de datos");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Error al crear elpool")
    };
}

//Inicializar la conexión
pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Error al conectar a la base de datos");
    embedded_migrations::run(&conn).unwrap();
}

//Reconocer la base de datos
pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}