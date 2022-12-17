use crate::err::CustomError;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use crate::cfg;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = cfg::read("mysql");
        println!("DB Url: {}", db_url);
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}
