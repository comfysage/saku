use crate::task::Task;

use super::args::Args;

pub struct ArgListener {
    word: String,
    task: Option<Task>,
    listeners: Vec<ArgListener>,
}

impl ArgListener {
    pub fn init(&self, args: &mut Args) -> Result<(), String> {
        if let Some(task) = &self.task {
            task.init(args.get())?;
            return Ok(());
        }
        for i in 0..self.listeners.len() {
            if &self.listeners[i].word == args.current() {
                args.change(1);
                self.listeners[i].init(args)?;
            }
        }
        Err("no match found".to_string())
    }
    pub fn new(word: &str, task: Option<Task>) -> Self {
        Self {
            word: word.to_string(),
            task: task,
            listeners: vec![],
        }
    }
    pub fn add_listener(&mut self, listener: ArgListener) -> Result<(), String> {
        self.listeners.push(listener);
        Ok(())
    }
    pub fn on(self, root: &mut ArgListener) -> Result<(), String> {
        root.add_listener(self)?;
        Ok(())
    }
}
