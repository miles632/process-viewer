#![allow(dead_code)]
#![allow(unused_imports)]

mod task;
mod task_calls;
mod task_tree;

use std::{fs::read, ops::Deref, path::PathBuf};
use crate::task_calls::fetch_running_time;

fn main() {
    // let entries = std::fs::read_dir("/proc").unwrap();
    // for entry in entries {
    //     task_calls::fetch_command(&entry.unwrap());
    // }

    task_tree::ProcessTree::new();

    // let runningtime = fetch_running_time(&633).unwrap();
    // println!("{}", runningtime);


    // dbg!(vec);


}
