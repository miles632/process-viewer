#![allow(dead_code)]
#![allow(unused_imports)]
#![recursion_limit = "1024"]

mod task;
mod task_calls;
mod task_tree;

use std::{fs::read, ops::Deref, path::PathBuf};
use crate::task_calls::fetch_running_time;
use std::thread;

fn main() {
    // let entries = std::fs::read_dir("/proc").unwrap();
    // for entry in entries {
    //     task_calls::fetch_command(&entry.unwrap());
    // }

    let stack_size = 8*1024*1024*1024;
    let builder = thread::Builder::new().stack_size(stack_size);
    builder.spawn(|| {
        task_tree::ProcessTree::new();
    }).unwrap().join().unwrap();

    

    // let runningtime = fetch_running_time(&633).unwrap();
    // println!("{}", runningtime);


    // dbg!(vec);


}
