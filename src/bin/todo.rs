use rusthtmx::models::*;

use rusthtmx::database::*;
use diesel::prelude::*;


fn main() {
    use rusthtmx::schema::todo::dsl::*;

    let connection = &mut establish_connection();
    let result = todo
        .select(Todo::as_select())
        .limit(100)
        .load(connection)
        .expect("Error loading todo");

    println!("Displaying {} Todo", result.len());

    for tod in result {
        println!("{:?}", tod.id);
    }
}