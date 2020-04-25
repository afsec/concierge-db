// use serde::Serialize;

// #[derive(Serialize, Debug)]
// struct ColumnInfo {
//     pub column_name: String,
//     pub column_type: String
// }


use super::ColumnInfo;

pub fn show_columns(columns: Vec<ColumnInfo>) -> String {
    // let response = ColumnInfo {
    //     column_name: columns.column_name,
    //     column_type: columns.column_type
    // };

    let result: Result<String, serde_json::error::Error> = serde_json::to_string_pretty(&columns);
      
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
