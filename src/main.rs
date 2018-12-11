#![allow(dead_code)]

mod unit;
mod comp;
mod task;

use crate::comp::{Component, Machine};
use crate::task::{FullTaskList, WorkRegistry};

struct Creator {
    work_registry: WorkRegistry,
    components: Vec<Component>,
    machines: Vec<Machine>,
}

enum ActionMode {
    Design,
    Work,
}

struct Game {
    tasks: FullTaskList,
    creator: Creator,
    mode: ActionMode,
}

fn main() {
    println!("Hello, world!");
}
