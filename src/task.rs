#![allow(dead_code)]
#![allow(unused_imports)]
use nix;
use std::{sync::{mpsc::channel, Arc, Mutex}, path::PathBuf, marker::PhantomData};
use rayon::{iter::ParallelBridge, prelude::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator}};
use std::fs::File;

use crate::task_calls::{is_pid, self};



#[derive(Debug)]
pub struct Task<'a>{
    pub pid: u64,
    pub cpu_usage: u64,
    pub mem_usage: u64,
    pub runtime: Option<f32>,
    // pub command: PathBuf, // command the program was launched with 
    pub command: Option<String>,

    pub parent: Option<&'a Task <'a>>,
}


impl<'a> Task<'a>{
}

// gets called only once at runtime
pub fn add_tasks_at_startup_p<'a>() -> Vec<Arc<Mutex<Task<'a>>>> {
    let mut tasks: Vec<Arc<Mutex<Task<'a>>>> = Vec::new();
    if let Ok(proc_entries) = std::fs::read_dir("/proc") {
        tasks = proc_entries
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .filter(|entry| is_pid(entry)) 
            .map(|entry| {
                let pid_string = entry.file_name().into_string().unwrap();
                let pid = pid_string.parse::<u64>().unwrap();
                Arc::new(Mutex::new(Task {
                    pid,
                    cpu_usage: 0,              
                    mem_usage: 0,              
                    runtime: task_calls::fetch_running_time(&pid),
                    command: task_calls::fetch_command(&pid), 
                    parent: None,                            
                }))
            })
            .collect();
    }
    tasks
}