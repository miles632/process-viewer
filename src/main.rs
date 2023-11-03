#![allow(dead_code)]
#![allow(unused_imports)]

mod task;
mod task_calls;

use std::{fs::read, ops::Deref, path::PathBuf};
fn main() {
    // let entries = std::fs::read_dir("/proc").unwrap();
    // for entry in entries {
    //     task_calls::fetch_command(&entry.unwrap());
    // }

    let vec = task::add_tasks_at_startup_p();
    dbg!(vec);


    // dbg!(vec);


}
