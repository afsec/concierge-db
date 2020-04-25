use serde::Serialize;

#[derive(Serialize, Debug)]
struct ShowTables {
    tables: Vec<String>,
}

#[derive(Serialize, Debug)]
struct ShowTablesNull {
    tables: (),
}

pub fn show_tables(rows: Vec<String>) -> String {
    if !rows.is_empty() {
        let response = ShowTables { tables: rows };
        let result: Result<String, serde_json::error::Error> =
            serde_json::to_string_pretty(&response);
        match result {
            Ok(table_names) => {
                dbg!(&table_names);
                table_names
            }
            Err(err) => {
                dbg!(err.to_string());
                err.to_string()
            }
        }
    } else {
        let response = ShowTablesNull { tables: () };
        let result: Result<String, serde_json::error::Error> =
            serde_json::to_string_pretty(&response);
        match result {
            Ok(table_names) => {
                dbg!(&table_names);
                table_names
            }
            Err(err) => {
                dbg!(err.to_string());
                err.to_string()
            }
        }
    }
}
