use brickpack::sqlite::SqlitePool;
// pub type SqlitePool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub type SqlitePooledConnection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn connection() -> Result<SqlitePool, String> {
    // TODO: Use env var to define file location
    const DB_FILE: &str = "database.sqlite3";
    const MAX_THREADS: u32 = 1;

    let manager = r2d2_sqlite::SqliteConnectionManager::file(DB_FILE);
    match r2d2::Pool::builder().max_size(MAX_THREADS).build(manager) {
        Ok(connection_pool) => Ok(connection_pool),
        Err(error) => Err(error.to_string()),
    }
}

pub fn bootstrap(pool: &SqlitePool) {
    use std::time::Duration;
    let conn = pool.get().unwrap();
    conn.busy_timeout(Duration::from_secs(5)).unwrap();

    conn.execute_batch("PRAGMA journal_mode=WAL").unwrap();
    conn.execute_batch("PRAGMA foreign_keys=ON").unwrap();
    assert!(conn.is_autocommit());
}
