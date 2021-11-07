#![allow(unused)]
mod mac;
use mac::Mac;
def_mac!(TodoMac, "todo", &["id", "title", "description"]);
fn main() {
    let todo = TodoMac::new();
    println!("the table name is: {:?}", todo.table_name());
}
