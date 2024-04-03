use crate::database::establish_connection;
use crate::models::*;
use diesel::prelude::*;
use rand::Rng;

pub fn get_random_todo_from_db() -> (i32, String) {
    use crate::schema::todo::dsl::*;
    let connection = &mut establish_connection();

    let result = todo
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading wordpairs");

    let n_words = result.len();
    let random_index = rand::thread_rng().gen_range(0..n_words);
    let item = &result[random_index];

    (
        item.id.clone(),
        item.title.clone()
    )

}

pub fn get_todos() -> Vec<Todo> {
    use crate::schema::todo::dsl::*;
    let connection = &mut establish_connection();

    let result = todo
        .select((id, title, pos))
        .order(pos.asc())
        .load(connection)
        .expect("Error loading wordpairs");

    result
}

pub fn reorder_todo(new_orders: Vec<i32>) -> Vec<Todo> {
    use crate::schema::todo::dsl::*;
    let connection = &mut establish_connection();

    let mut result = Vec::new();
    let todos: Vec<Todo> = todo
        .select((id, title, pos))
        .order(pos.asc())
        .load(connection)
        .expect("Error loading wordpairs");

    new_orders.iter().enumerate().for_each(|(idx, item)| {
        if let Some(element) = todos.iter().find(|&x| x.id == *item) {
            result.push(Todo {
                id: element.id,
                title: element.title.clone(),
                pos: idx as i32
            });

            diesel::update(todo)
                .filter(id.eq(element.id))
                .set(pos.eq(idx as i32))
                .execute(connection)/**/
                .expect("Error saving reorder todos!");
        }
    });


    result
}

