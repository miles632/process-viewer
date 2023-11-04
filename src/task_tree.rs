use std::sync::Arc;

struct TaskTree <'a>{
    task: Task,
    children: Option<Vec<Arc<Task<'a>>>>
}

impl TaskTree {

}