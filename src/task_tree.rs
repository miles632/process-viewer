use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc::channel;

use std::hash::Hash;
use std::io::Read;
use std::fs::{read_to_string, File, read_dir, DirEntry};
use std::collections::HashMap;
use rayon::prelude::ParallelIterator;
use rayon::{iter::ParallelBridge};
use std::cell::RefCell;

use crate::task::Task;
use crate::task_calls::is_pid;
struct TaskTree {
    task: Task,
    children: Option<Vec<Mutex<Box<Task>>>>
}

impl TaskTree {
    fn new() 
    // -> Result<TaskTree, String> 
    {
        let mut ppid_pid_map: Mutex<HashMap<u64, u64>> = Mutex::new(HashMap::new());
        let mut pid_pool: Vec<u64> = Vec::new();

        // // the pids are converted from DirEntry to string
        // if let Ok(dir_entries) = read_dir("/proc/") {
        //     dir_entries
        //         .par_bridge()
        //         .filter_map(|entry| entry.ok())
        //         .filter(|entry| {is_pid(entry)})
        //         .map(|entry| {
        //             let status_path = format!("/proc/{}/status", entry.file_name().into_string().unwrap());
        //             let mut status_buf = String::new();
        //             File::open(status_path).unwrap().read_to_string(&mut status_buf);
        //             let status_contents: Vec<&str> = status_buf.split('\n').collect();
        //             let ppid = status_contents[6].parse::<u64>().unwrap();
        //             let pid = status_contents[5].parse::<u64>().unwrap();
        //             ppid_pid_map.lock().unwrap().insert(ppid, pid);
        //             pid_pool.push(pid);

        //             entry
        //         });
        // }
        let terminate_flag = Arc::new(AtomicBool::new(false));
        let handles: Vec<_> = {
            if let Ok(dir_entries) = read_dir("/proc") {
                dir_entries.par_bridge().for_each(|entry| {
                    if terminate_flag.load(order)


                });
            }
        };


    }
}