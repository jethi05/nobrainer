use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: usize,
    id: usize,
    title: String,
    completed: bool,
}


const JSON: &str = r#"
{
    "userId": 1,
    "id": 2,
    "title": "kek",
    "completed": false
}
"#;


fn main() {
    let res = from_str::<Todo>(JSON);
    println!("{:?}", res);
}
