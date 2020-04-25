use crate::api::Coluna;

pub fn read_all(rows: Vec<Vec<Coluna>>) -> String {

    let result: Result<String, serde_json::error::Error> = serde_json::to_string_pretty(&rows);
    match result {
        Ok(json) => {
            // dbg!(&json);
            json
        }
        Err(err) => {
            dbg!(err.to_string());
            err.to_string()
        }
    }
}
