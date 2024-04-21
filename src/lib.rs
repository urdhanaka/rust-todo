use std::{env, fs::OpenOptions};

pub struct ToDo {
    pub todo_path: String,
    pub name: String,
    pub todo: Vec<String>,
}

impl ToDo {
    pub fn build() -> Self {
        let todo_path: String = env::var("TODO_PATH").unwrap_or(env::var("HOME").unwrap());

        ToDo {
            todo_path,
            name: String::from(""),
            todo: Vec::new(),
        }
    }

    pub fn new(self, new: &str) -> Result<Self, String> {
        Ok(Self {
            name: new.to_string(),
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
