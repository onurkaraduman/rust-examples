use std::{collections::HashMap, process::id};

struct TodoItem {
    id: i64,
    name: String,
    completed: bool,
}

struct TodoList {
    name: String,
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new(name: String) -> TodoList {
        TodoList {
            name: name,
            items: Vec::new(),
        }
    }

    fn add(&mut self, item_name: String) {
        let next_id = self.items.len() as i64;
        let todo_item = TodoItem {
            id: next_id,
            name: item_name.clone(),
            completed: false,
        };
        self.items.push(todo_item);
        println!("Added new Item: {}: {}", next_id, item_name);
    }

    fn update(&mut self, id: i64, new_name: String) {
        if let Some(item) = self.items.iter_mut().find(|it| it.id == id) {
            item.name = new_name;
            println!("Updated the item with id: {}", &id);
        } else {
            print!("{} could not be found", &id);
        }
    }

    fn delete(&mut self, id: i64) {
        let index = self.items.iter().position(|it| it.id == id).unwrap();
        self.items.remove(index);
        println!("Deleted an Item: {}", &id);
    }

    fn complete(&mut self, id: i64) {
        if let Some(item) = self.items.iter_mut().find(|it| it.id == id) {
            item.completed = true;
            println!("Completed an Item: {}", &id);
        } else {
            print!("{} could not be found", &id);
        }
    }

    fn list(&self) {
        println!("----- List of {} ----", self.name);
        for item in &self.items {
            let status = if item.completed { "[X]" } else { "[ ]" };
            println!("{} {}", status, item.name);
        }
    }
}

fn main() {
    println!("--- Todo List App ----");
    let mut todo_list = TodoList::new(String::from("Office"));
    todo_list.add(String::from("Daily Standup")); // id:0
    todo_list.add(String::from("Requirement Meeting with Business")); // id:1
    todo_list.add(String::from("Coffee Break")); // id:2
    todo_list.add(String::from("Lunch Break")); // id:3
    todo_list.list();

    todo_list.delete(1); // delete Requirement Meeting with Business
    todo_list.list();

    todo_list.update(2, String::from("Interview"));
    todo_list.list();

    todo_list.complete(0);
    todo_list.list();
}
