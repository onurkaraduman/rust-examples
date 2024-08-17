use std::{collections::HashMap, process::id};

struct TodoItem {
    id: i64,
    name: String,
    completed: bool,
}

struct TodoList {
    name: String,
    items: HashMap<i64, TodoItem>,
}

impl TodoList {
    fn new(name: String) -> TodoList {
        TodoList {
            name: name,
            items: HashMap::new(),
        }
    }

    fn add(&mut self, item_name: String) {
        let next_id = self.items.len() as i64 + 1;
        let todo_item = TodoItem {
            id: next_id,
            name: item_name.clone(),
            completed: false,
        };
        self.items.insert(next_id, todo_item);
        println!("Added new Item: {}: {}", next_id, item_name);
    }

    fn update(&mut self, id: i64, new_name: String) {
        if let Some(item) = self.items.get_mut(&id) {
            item.name = new_name;
            println!("Updated an Item: {}", &id);
        } else {
            print!("{} could not be found", &id);
        }
    }

    fn delete(&mut self, id: i64) {
        if let Some(item) = self.items.get_mut(&id) {
            self.items.remove(&id);
            println!("Deleted an Item: {}", &id);
        } else {
            print!("{} could not be found", &id);
        }
    }

    fn complete(&mut self, id: i64) {
        if let Some(item) = self.items.get_mut(&id) {
            item.completed = true;
            println!("Completed an Item: {}", &id);
        } else {
            print!("{} could not be found", &id);
        }
    }

    fn list(&self) {
        println!("----- List of {} ----", self.name);
        for (id, item) in &self.items {
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

    todo_list
}
