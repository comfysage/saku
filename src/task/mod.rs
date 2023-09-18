use crate::Error;

use self::{show::cli_show, install::cli_install};

pub mod show;
pub mod install;

type Tasks = Vec<Task>;

pub enum Task {
    /* Install pkg $ yuki add
     *  - Clone
     *  - Build
     *  - Pack
     */
    Install,
    /* Uninstall pkg $ yuki remove
     *  - Remove Pack
     *  - Remove Clone
     */
    Uninstall,
    Add,
    Remove,

    Clone,
    Build,
    Pack,

    RemoveClone,
    RemovePack,

    ConfigInit,
    ConfigCreate,

    Show,
    List,
}

impl Task {
    pub fn init(&self, args: Vec<String>) -> Result<(), Error> {
        let tasks = Task::expand(self);
        Task::run_all(tasks, &args)?;
        Ok(())
    }
    pub fn expand(task: &Task) -> Tasks {
        match task {
            Task::Install => vec![Task::Clone, Task::Build, Task::Pack],
            Task::Uninstall => vec![Task::RemovePack, Task::RemoveClone],
            Task::Add => vec![Task::Add],
            Task::Remove => vec![Task::Remove],
            Task::Clone => vec![Task::Clone],
            Task::Build => vec![Task::Build],
            Task::Pack => vec![Task::Pack],
            Task::RemoveClone => vec![Task::RemoveClone],
            Task::RemovePack => vec![Task::RemovePack],
            Task::Show => vec![Task::Show],
            Task::List => vec![Task::List],
            Task::ConfigInit => vec![Task::ConfigInit],
            Task::ConfigCreate => vec![Task::ConfigInit],
        }
    }
    pub fn run_all(tasks: Tasks, args: &Vec<String>) -> Result<(), Error> {
        for task in tasks {
            Task::run(task, args)?;
        }
        Ok(())
    }
    pub fn run(task: Task, args: &Vec<String>) -> Result<(), Error> {
        match task {
            Task::Install => cli_install(args.clone()),
            Task::Uninstall => todo!(),
            Task::Add => todo!(),
            Task::Remove => todo!(),
            Task::Clone => todo!(),
            Task::Build => todo!(),
            Task::Pack => todo!(),
            Task::RemoveClone => todo!(),
            Task::RemovePack => todo!(),
            Task::Show => cli_show(args.clone()),
            Task::List => todo!(),
            Task::ConfigInit => todo!(),
            Task::ConfigCreate => todo!(),
        }?;
        Ok(())
    }
}
