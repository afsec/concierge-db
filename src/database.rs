pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type DbConnection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn connection() -> Result<Pool, String> {
    const DB_FILE: &str = "database.sqlite3";
    const MAX_THREADS: u32 = 1;
    let manager = r2d2_sqlite::SqliteConnectionManager::file(DB_FILE);
    match r2d2::Pool::builder().max_size(MAX_THREADS).build(manager) {
        Ok(connection_pool) => Ok(connection_pool),
        Err(error) => Err(error.to_string()),
    }
}

pub fn bootstrap(pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>) {
    use std::time::Duration;
    let conn = pool.get().unwrap();
    conn.busy_timeout(Duration::from_secs(5)).unwrap();

    // TODO: Add more PRAGMAs (Constraints, etc.)
    conn.execute_batch("PRAGMA journal_mode=WAL").unwrap();
    conn.execute_batch("PRAGMA foreign_keys=ON").unwrap();

    // TODO: Check if PRAGMAs were applied
    assert!(conn.is_autocommit());
}
