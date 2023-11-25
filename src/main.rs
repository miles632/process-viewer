#![allow(dead_code)]
#![allow(unused_imports)]
#![recursion_limit = "1024"]

mod task;
mod task_calls;
mod task_tree;

use std::{fs::read, ops::Deref, path::PathBuf};
use task::Process;
use task_tree::ProcessTree;

use crate::task_calls::fetch_running_time;
use std::thread;

fn main() {
    // let ptree = ProcessTree::new();
    // println!("{}"ptree[1].get_number_of_processes());

    

    // let runningtime = fetch_running_time(&633).unwrap();
    // println!("{}", runningtime);


    // dbg!(vec);


}
