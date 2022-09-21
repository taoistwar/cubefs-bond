use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct Properties {
    name: String,
    sex: Option<String>,
}

fn main() {
    let json = r#"{
    "name": "xx"
  }"#;
    let obj = serde_json::from_str(json);
    let obj: Properties = obj.unwrap();
    dbg!(&obj);
    let json = serde_json::to_string_pretty(&obj);
    let json = json.unwrap();
    println!("{}", json);
}
