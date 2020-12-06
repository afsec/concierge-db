pub mod show_tables;
pub mod query;

// mod query {
//     use serde::Serialize;

//     #[derive(Debug, Serialize)]
//     struct Result(Vec<Line>);

//     #[derive(Debug, Serialize)]
//     struct Line(Vec<Column>);

//     #[derive(Debug, Serialize)]
//     struct Column {
//         field_name: String,
//         field_data: SQLiteType,
//     }

//     #[derive(Debug, Serialize)]
//     enum SQLiteType {
//         Integer(u16),
//         Real(f32),
//         Text(String),
//         Null,
//     }

//     impl Default for SQLiteType {
//         fn default() -> Self {
//             Self::Null
//         }
//     }
// }
