#![warn(unused_variables)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, ops::Deref};
use std::rc::Rc;

use std::hash::Hash;
use std::io::Read;
use std::fs::{read_to_string, File, read_dir, DirEntry};
use std::collections::HashMap;
use rayon::{prelude::ParallelIterator, iter::IntoParallelRefIterator};
use rayon::{iter::ParallelBridge};
use std::cell::RefCell;

use crate::task::Task;
use crate::task_calls::is_pid;


fn parse_from_status_slice(input: &str) -> Option<u64> {
    // Split the input by the tab character
    let parts: Vec<&str> = input.split('\t').collect();

    // Ensure that the split resulted in at least two parts
    if parts.len() == 2 {
        // Try to parse the second part as a u64
        if let Ok(value) = parts[1].trim_end_matches(',').trim().parse::<u64>() {
            return Some(value);
        }
    }

    // Return None if parsing fails or the input format is incorrect
    None
}
pub struct TaskTree {
    task: Task,
    children: Option<Vec<Mutex<Box<Task>>>>
}

impl TaskTree {
    pub fn new() 
    // -> Arc<Mutex<HashMap<u64, u64>>>
    // -> TaskTree
    {
        // let terminate_flag = Arc::new(AtomicBool::new(false));
        let ppid_pid_map: Arc<Mutex<HashMap<Option<u64>, Option<u64>>>> = {
            let mut map: Arc<Mutex<HashMap<Option<u64>, Option<u64>>>> = Arc::new(Mutex::new(HashMap::new()));
            if let Ok(dir_entries) = read_dir("/proc") {
                dir_entries
                    .par_bridge()
                    .filter_map(|entry| entry.ok()) 
                    .filter(|entry| is_pid(entry))
                    .for_each(|entry| {
                        let status_path = format!("/proc/{}/status", entry.file_name().to_string_lossy());
                        let mut status_buf = String::new();
                        File::open(status_path).unwrap().read_to_string(&mut status_buf).unwrap();
                        let status_contents: Vec<&str> = status_buf.split('\n').collect();

                        let pid = parse_from_status_slice(status_contents[5]);
                        let ppid = parse_from_status_slice(status_contents[6]);
 
                        let map = map.clone();
                        let mut map_lock = map.lock().unwrap();
                        map_lock.insert(ppid, pid);
                    });
            }
            // dbg!(&map);
            map
        };

        // pid pool that I can use to 
        // too lazy to implement paralellism, doesn't really matter here anyway
        let mut temporary_pid_pool: Vec<Option<u64>> = Vec::new();
        for (_, pid) in ppid_pid_map.lock().unwrap().clone().into_iter(){
            temporary_pid_pool.push(pid);
        }



    }
}

        