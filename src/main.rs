use std::fs::{File,read_dir};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn get_all_children_for_pid(pid: &str) -> Vec<String> {
    let mut children_map = HashMap::new();

    for d in read_dir("/proc").unwrap() {
        let pid = d.unwrap().file_name().to_string_lossy().into_owned();
        if pid.parse::<usize>().is_ok() {
            if let Some(ppid) = get_ppid_for(&pid) {
                let children = children_map.entry(ppid).or_insert_with(Vec::new);
                children.push(pid);
            }
        }
    }

    get_all_children_for_pid_from_map(&children_map, pid)
}

fn get_all_children_for_pid_from_map(map: &HashMap<String,Vec<String>>, pid: &str) -> Vec<String> {
    let mut pids = Vec::new();
    if let Some(children) = map.get(pid) {
        for pid in children {
            let grandchildren = get_all_children_for_pid_from_map(map, pid);
            pids.push(pid.to_string());
            for child in grandchildren {
                pids.push(child.to_string());
            }
        }
    }
    pids
}

fn get_ppid_for(pid: &str) -> Option<String> {
    let status = format!("/proc/{}/status", pid);

    let file = match File::open(&status) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let reader = BufReader::new(file);

    for l in reader.lines() {
        let line = match l {
            Ok(s) => s,
            Err(_) => return None,
        };
        if line.starts_with("PPid:") {
            return Some(line.split_whitespace().nth(1).unwrap().to_string());
        }
    }

    None
}

fn main() {
    for pid in std::env::args().skip(1) {
        let pids = get_all_children_for_pid(&pid);
        if !pids.is_empty() {
            println!("{}", pids.join(" "));
        }
    }
}

// vim: se sw=4:
