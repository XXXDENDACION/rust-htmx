use diesel::prelude::*;
use rusthtmx::database::*;
use rusthtmx::models::*;
use rusthtmx::*;

fn main() {
    use crate::schema::todo;
    use crate::schema::todo::dsl::*;

    let connection = &mut establish_connection();
    for (order, text) in TODOS.iter() {
        let new_wordpair = NewTodo {
            title: text,
            pos: order,
        };

        let result = todo
            .filter(title.eq(text))
            .first::<Todo>(connection)
            .optional();

        match result {
            Ok(Some(_)) => {}
            Ok(None) => {
                diesel::insert_into(todo::table)
                    .values(&new_wordpair)
                    .execute(connection)
                    .expect("Error saving new wordpair");
            }
            Err(_) => println!("Error checking for existing wordpair"),
        }
    }

}

const TODOS: [(i32, &str); 5] = [
    (1, "test"),
    (2, "test1"),
    (3, "test2"),
    (4, "test3"),
    (5, "test4"),
];