use std::path::Path;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    email: String
}

fn main() {
    let json_file_path = Path::new("../sample.json");
    let file = File::open(json_file_path).expect("file not found");
    let users:Vec<User> = serde_json::from_reader(file).expect("error while reading");
   for user in users {
       println!("Hello {} aka {} {}", user.username, user.first_name, user.last_name)
   }
}
