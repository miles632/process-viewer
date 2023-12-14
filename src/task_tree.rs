#![warn(unused_variables)]

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

use std::io::Read;
use std::fs::{read_to_string, File, read_dir, DirEntry};
use rayon::{prelude::ParallelIterator, iter::IntoParallelRefIterator};
use rayon::{iter::ParallelBridge};
use std::cell::RefCell;
use std::fmt::format;
use std::vec::Vec;

use crate::task::Process;
use crate::task_calls::{fetch_command, fetch_running_time, is_pid};


fn parse_from_status_slice(input: &str) -> u64 {
    // split the input by the tab character
    let parts: Vec<&str> = input.split('\t').collect();
    let value = parts[1].trim_end_matches(',').trim().parse::<u64>().unwrap();
    value
}

pub fn check_for_children_processes(ppid_pool: &Vec<(u64, u64)>, node: u64) -> Vec<u64> {
    let mut matching_children: Vec<u64> = Vec::new();
    for (pid, ppid) in ppid_pool {
        if *ppid == node {
            matching_children.push(*pid);
        }
    }
    if matching_children.is_empty() {
        return Vec::<u64>::new();
    } else { 
        matching_children
    }
}
#[derive(Debug)]
pub struct ProcessTree {
    pub process_node: Process,
    // children: Option<Box<Vec<Mutex<ProcessTree>>>>
    pub children: Vec<Arc<Mutex<ProcessTree>>>
}

impl ProcessTree {
    pub fn new() 
    -> Vec<ProcessTree>
    {
        let mut pid_ppid_array = Vec::<(u64, u64)>::new();

        if let Ok(dir_entries) = read_dir("/proc") {
            pid_ppid_array =
                dir_entries 
                    .par_bridge()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| is_pid(entry))
                    .map(|entry| {
                        let status_path = format!("/proc/{}/status", entry.file_name().to_string_lossy());
                        let mut status_buf = String::new();
                        File::open(status_path).unwrap().read_to_string(&mut status_buf).unwrap();
                        let status_contents: Vec<&str> = status_buf.split('\n').collect();

                        let pid = parse_from_status_slice(status_contents[5]);
                        let ppid = parse_from_status_slice(status_contents[6]);

                        (pid, ppid)
                    })
                    .collect::<Vec<(u64, u64)>>();
        } else {
            eprintln!("No /proc directory found")
        }

        // collect all trees into a vector, this will usually just be 
        // 2 trees with the main node pids 2 and 1 
        let root_pids = pid_ppid_array
            .iter()
            .filter(|root_pid_ppid| {
                root_pid_ppid.1 == 0
            })
            .map(|root_ppid_pid| {
                root_ppid_pid.0
            })
            .collect::<Vec<u64>>();

        let process_trees =
            root_pids.iter().map(|pid| {
                let task = Process::new(*pid);
                let children = 
                    Self::recursively_construct_children(pid, &pid_ppid_array);
                let ptree = ProcessTree {
                    process_node: task,
                    children: children,
                };
                ptree
            }).collect::<Vec<ProcessTree>>();
        // dbg!(&process_trees);
        process_trees
    }

    pub fn recursively_construct_children(
        pid: &u64, pid_ppid_array: &Vec<(u64,u64)>) 
        -> Vec<Arc<Mutex<ProcessTree>>> {
        if let children_processes = 
        check_for_children_processes(pid_ppid_array, *pid){
            let mut vec: Vec<Arc<Mutex<ProcessTree>>> = Vec::new();

            for child in children_processes {
                let new_subtree = ProcessTree {
                    process_node: Process::new(child),
                    children: Self::recursively_construct_children(&child, &pid_ppid_array),
                };
                vec.push(Arc::new(Mutex::new(new_subtree)));
            }
            vec
        } else { 
            return Vec::new();
        }
    }
    pub fn get_number_of_processes(&self) -> usize {
        let tree_children = self.children.clone();
        let mut amount: usize = 1;
        let nodes_amount = self.children.len();
        amount+=nodes_amount;
        self.children.iter().for_each(|child|{
            amount+=child.lock().unwrap().get_number_of_processes();
        });
        amount
    }
    // pub fn flatten_tree(&self) -> Vec<Arc<Mutex<Process>>> {
    //     let mut processes = Vec::<Arc<Mutex<Process>>>::new();

    //     processes.push(Arc::new(Mutex::new(self.process_node))); 

    //     // Recursively flatten the tree for each child
    //     for child in &self.children {
    //         child = child.into_inner().unwrap();
    //         processes.extend(child.flatten_tree());
    //     }

    //     processes
    // }
}