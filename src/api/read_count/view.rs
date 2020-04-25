use serde::Serialize;

#[derive(Serialize, Debug)]
struct RowCount {
    rows: u16,
}

pub fn read_count(rows: u16) -> String {
    let response = RowCount { rows };

    let result: Result<String, serde_json::error::Error> = serde_json::to_string_pretty(&response);

    // let tables: Result<String, serde_json::error::Error> = serde_json::to_string_pretty(&rows);
    match result {
        Ok(names) => {
            dbg!(&names);
            names
        }
        Err(err) => {
            dbg!(err.to_string());
            err.to_string()
        }
    }
}
