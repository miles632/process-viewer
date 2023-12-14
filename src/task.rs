#![allow(dead_code)]
#![allow(unused_imports)]
<<<<<<< HEAD

use std::{sync::{mpsc::channel, Arc, Mutex}, path::PathBuf, marker::PhantomData, fs};
use std::fmt::format;
use rayon::{iter::ParallelBridge, prelude::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator}};
use std::fs::File;

use crate::task_calls::{is_pid, self, fetch_command, fetch_running_time};
use crate::task_tree::*;

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u64,
    // pub cpu_usage: u64,
    // pub mem_usage: u64,
    pub runtime: Option<f64>,
=======
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
>>>>>>> parent of e7bdfa2 (changed up stuff a bit)
    // pub command: PathBuf, // command the program was launched with 
    pub command: Option<String>,

    pub parent: Option<&'a Task <'a>>,
}
<<<<<<< HEAD
impl Process {
    pub fn new(pid: u64) -> Self {
        Process {
            pid: pid,
            command: fetch_command(&pid),
            runtime: fetch_running_time(&pid)
        }
    }
    pub fn update(&mut self) {
        self.runtime = fetch_running_time(&self.pid);
        self.command = fetch_command(&self.pid);
=======


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
>>>>>>> parent of e7bdfa2 (changed up stuff a bit)
    }
}