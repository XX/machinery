use std::collections::HashMap;
use crate::comp::detail::DetailKindDiscriminants;

pub struct Task {
    max_details: Option<u32>,
    allowed_details: HashMap<DetailKindDiscriminants, u32>,
    required_details: HashMap<DetailKindDiscriminants, u32>,
}

pub struct FullTaskList(pub Vec<Task>);

pub struct WorkRegistry {
    available_tasks: Vec<Task>,
    completed_tasks: Vec<Task>,
    actual_task: Option<Task>,
}