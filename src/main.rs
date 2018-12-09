enum Detail {
    Uniform(UniformDetail),
    Composite(CompositeDetail),
}

struct UniformDetail {
    name: String,
}

struct CompositeDetail {
    name: String,
    details: Vec<Detail>,
}

struct Product(Detail);

struct Task;

struct FullTaskList(Vec<Task>);

struct WorkRegistry {
    available_tasks: Vec<Task>,
    completed_tasks: Vec<Task>,
    actual_task: Option<Task>,
}

struct Creator {
    work_registry: WorkRegistry,
    details: Vec<Detail>,
    products: Vec<Product>,
}

struct Game {
    tasks: FullTaskList,
    creator: Creator,
}

fn main() {
    println!("Hello, world!");
}
