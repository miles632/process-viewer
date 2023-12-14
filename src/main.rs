#![allow(dead_code)]
#![allow(unused_imports)]
#![recursion_limit = "1024"]

mod task;
mod task_calls;
mod task_tree;

use std::{fs::read, ops::Deref, path::PathBuf};
use task::Process;
use task_tree::ProcessTree;
use std::sync::Mutex;

use crate::task_calls::fetch_running_time;
use std::thread;

fn main() {
    let trees = ProcessTree::new();
    // trees.iter().for_each(|tree|{
    //     println!("number of processes of tree {}: {}", tree.task.pid, tree.get_number_of_processes());
    // });
    trees.iter().for_each(|tree|{
        dbg!(tree.process_node.runtime);
    })
}
