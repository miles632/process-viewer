#![allow(dead_code)]
#![allow(unused_imports)]

mod task;
mod task_calls;
mod task_tree;

use std::{fs::read, ops::Deref, path::PathBuf};
fn main() {
    // let entries = std::fs::read_dir("/proc").unwrap();
    // for entry in entries {
    //     task_calls::fetch_command(&entry.unwrap());
    // }

    let test_map = task_tree::TaskTree::new();
    dbg!(test_map);


    // dbg!(vec);


}
