#![allow(dead_code)]
#![allow(unused_imports)]

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
    // pub command: PathBuf, // command the program was launched with 
    pub command: Option<String>,

    // // pub parent: Option<&'a Task<'a>>,
    // pub children: Option<Vec<Mutex<Arc<Task<'a>>>>>,
}
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
    }
}