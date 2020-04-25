pub fn connection() -> Result<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, String> {
    let manager = r2d2_sqlite::SqliteConnectionManager::file("database.sqlite3");
    match r2d2::Pool::builder().max_size(1).build(manager) {
        Ok(connection_pool) => Ok(connection_pool),
        Err(error) => Err(error.to_string()),
    }
}

pub fn config(pool: Result<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, String>) {
    use std::time::Duration;
    // let pool = connection().unwrap();
    let conn = pool.unwrap().get().unwrap();
    conn.busy_timeout(Duration::from_secs(5)).unwrap();
    // TODO: Adicionar PRAGMAs (Constraints, etc.)

    conn.execute_batch("PRAGMA journal_mode=WAL").unwrap();

    // TODO: Verificar se os PRAGMAs foram efetivados
    assert!(conn.is_autocommit());
}

// TODO: Verificar o uso de Traits do mesmo modo como foi implementado
//         na leitura dos HEADERS UserAgent e Authorization e verificar
//         se o Rocket Implementa algum tipo de Blanket Implementation
//         para `database connection`.
