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

fn check_for_children_processes(ppid_pool: &Vec<u64>, node: u64) -> Vec<u64> {
    let mut matching_children: Vec<u64> = Vec::new();
    for ppid in ppid_pool {
        if *ppid == node {
            matching_children.push(*ppid);
        }
    }
    matching_children
}
#[derive(Debug)]
pub struct ProcessTree {
    task: Box<Process>,
    children: Vec<Mutex<Box<ProcessTree>>>
}

impl ProcessTree {
    pub fn new() 
    // -> Arc<Mutex<HashMap<u64, u64>>>
    // -> TaskTree
    {
        let mut ppid_pid_array = Vec::<(u64, u64)>::new();

        if let Ok(dir_entries) = read_dir("/proc") {
            ppid_pid_array =
                dir_entries
                    .par_bridge()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| is_pid(entry))
                    .map(|entry| {
                        let status_path = format!("/proc/{}/status", entry.file_name().to_string_lossy());
                        let mut status_buf = String::new();
                        File::open(status_path).unwrap().read_to_string(&mut status_buf).unwrap();
                        let status_contents: Vec<&str> = status_buf.split('\n').collect();

                        let pid = parse_from_status_slice(status_contents[6]);
                        let ppid = parse_from_status_slice(status_contents[5]);

                        (pid, ppid)
                    })
                    .collect::<Vec<(u64, u64)>>();
        } else {
            eprintln!("No /proc directory found")
        }

        let process_trees= {
            let root_pids = ppid_pid_array
                .iter()
                .filter(|ppid_pid| {
                    ppid_pid.0 == 0
                })
                .map(|ppid_pid| {
                    ppid_pid.1
                })
                .collect::<Vec<u64>>();

            let ppid_pool = &ppid_pid_array.iter().map(|ppid_pid| {
                ppid_pid.0
            }).collect::<Vec<u64>>();

            let Process_Trees =
                root_pids.iter().map(|pid| {
                    let tree = ProcessTree {
                        task: Box::new(Process::new(*pid)),
                        children: Self::recursively_construct_children(pid, ppid_pool),
                    };
                    tree
                }).collect::<Vec<ProcessTree>>();
        };
        dbg!(process_trees)

    }
   fn recursively_construct_children(pid: &u64, ppid_pool: &Vec<u64>) -> Vec<Mutex<Box<ProcessTree>>> {
        let children_processes = check_for_children_processes(ppid_pool, *pid);

        let mut vec = Vec::<Mutex<Box<ProcessTree>>>::new();

        for child in children_processes {
            let new_subtree = ProcessTree {
                task: Box::new(Process::new(child)),
                children: Self::recursively_construct_children(&child, &ppid_pool),
            };
            vec.push(Mutex::new(Box::new(new_subtree)));
        }
        vec
    }



}

        