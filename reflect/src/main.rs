#![allow(unused)]
use reflect_derive::Reflect;

fn main() {
    let user = User::default();
    println!("Struct name: {}", user.struct_name());
    println!("Fields name: {:?}", user.fields_name())
}

#[derive(Default, Reflect)]
struct User {
    age: String,
    name: String,
    email: String,
}
