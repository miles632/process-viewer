#![allow(dead_code)]

use std::fmt::format;
use std::{path::PathBuf};
use std::fs;
use std::io;


pub fn fetch_command(pid: &u64) -> Option<String> {
    let cmdline_str = format!("/proc/{}/cmdline", pid);

    match fs::read_to_string(&cmdline_str) {
        Ok(command) => {
            // the upper operation will return an empty string
            // in case of zombie processes, hence this check
            if command.is_empty() {
                return None;
            }
            Some(command)
        },
        Err(_) => None,
    }
}
// checks to see if an entry in /proc is actually a true process
pub fn is_pid(entry: &fs::DirEntry) -> bool {
    if let Some(name) = entry.file_name().to_str() {
        if let Ok(_a)  = name.parse::<u64>() {
            return true;
        }
    }
    false
}
// fetches the time a process has been scheduled both in kernel- and usermode
pub fn fetch_scheduletime(pid: &u64) -> Option<(u64, u64)> {
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(stat_path).ok()?;
    let stat_content_lines: Vec<&str> = stat_content.split(' ').collect();
    // let utime: usize = stat_content_lines[13] as usize;
    // let stime = stat_content_lines[14] as usize;
    if let (Some(utime_str), Some(stime_str)) = 
    (stat_content_lines.get(13), stat_content_lines.get(14)) {
        if let (Ok(utime), Ok(stime)) = 
        (utime_str.parse::<u64>(), stime_str.parse::<u64>()) {
            return Some((utime, stime))
        }
    } 
    None
}

pub fn fetch_running_time(pid: &u64) -> Option<f64> {
    let system_uptime_fields = fs::read_to_string("/proc/uptime").ok()?;
    let system_uptime_fields: Vec<&str> = system_uptime_fields.split(' ').collect();
    let system_uptime = system_uptime_fields[0].parse::<f64>().ok()?;

    let stat_path = format!("/proc/{}/stat", pid);
    let stat_lines = fs::read_to_string(stat_path).ok()?;
    let stat_lines: Vec<&str> = stat_lines.split(' ').collect();
    let starttime = stat_lines[21].parse::<f64>().ok()?;

    let running_time = system_uptime as f64 - (starttime as f64 / 100.0);
    // let running_time = system_uptime as f64 - starttime;
    Some(running_time)
}
