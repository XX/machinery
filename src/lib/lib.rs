#![allow(dead_code)]

pub mod unit;
pub mod comp;
pub mod task;

use crate::comp::{Component, Machine};
use crate::task::{FullTaskList, WorkRegistry};

pub struct Creator {
    work_registry: WorkRegistry,
    components: Vec<Component>,
    machines: Vec<Machine>,
}

pub enum ActionMode {
    Design,
    Work,
}

pub struct Game {
    tasks: FullTaskList,
    creator: Creator,
    mode: ActionMode,
}