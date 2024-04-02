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

    println!("{:?}", result.len());

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
                pos: *item
            });
        }
    });

    // // todo!("Optimize algorithm");
    // if let Some(current_index) = todos.iter().position(|x| x.id == todo_id) {
    //     if current_index as i32 > target_index {
    //         todos.iter().enumerate().for_each(|(idx, item)| {
    //             if (idx as i32) > target_index && idx < current_index {
    //                 result.push(Todo {
    //                     id: item.id,
    //                     title: item.title.clone(),
    //                     pos: item.pos + 1
    //                 });
    //             }
    //
    //             if idx == current_index {
    //                 result.push( Todo {
    //                     id: item.id,
    //                     title: item.title.clone(),
    //                     pos: target_index
    //                 })
    //             }
    //         })
    //     }
    //
    //     if (current_index as i32) < target_index {
    //         todos.iter().enumerate().for_each(|(idx, item)| {
    //             if idx > current_index && (idx as i32) < target_index {
    //                 result.push(Todo {
    //                     id: item.id,
    //                     title: item.title.clone(),
    //                     pos: item.pos - 1
    //                 });
    //             }
    //
    //             if idx == current_index {
    //                 result.push( Todo {
    //                     id: item.id,
    //                     title: item.title.clone(),
    //                     pos: target_index
    //                 })
    //             }
    //         })
    //     }
    // }

    result
}

