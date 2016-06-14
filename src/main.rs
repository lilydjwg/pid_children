use std::fs::{File,read_dir};
use std::io::{Read, BufRead, BufReader};
use std::collections::HashMap;

fn get_children_for_pid(pid: &str) -> Option<Vec<String>> {
    let children = format!("/proc/{0}/task/{0}/children", pid);

    let mut file = match File::open(&children) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => return None,
    };
    if buf.is_empty() {
        return None;
    }

    Some(buf.split(' ').filter(|x| !x.is_empty()).map(|x| x.to_string()).collect())
}

fn get_all_children_for_pid(pid: &str) -> Vec<String> {
    let mut pids = Vec::new();

    let children = match get_children_for_pid(pid) {
        Some(c) => c,
        None => return pids,
    };

    for pid in children {
        let mut grandchildren = get_all_children_for_pid(&pid);
        pids.push(pid);
        if !grandchildren.is_empty() {
            pids.append(&mut grandchildren);
        }
    }

    pids
}

fn get_all_children_for_pid_old_kernel(pid: &str) -> Vec<String> {
    let mut children_map = HashMap::new();

    for d in read_dir("/proc").unwrap() {
        let path = d.unwrap().path();
        let pid = path.file_name().unwrap().to_string_lossy().into_owned();
        if pid.parse::<usize>().is_ok() {
            if let Some(ppid) = get_ppid_for(&pid) {
                let mut children = children_map.entry(ppid).or_insert_with(Vec::new);
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
            if !grandchildren.is_empty() {
                for child in grandchildren {
                    pids.push(child.to_string());
                }
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

fn has_children_file() -> bool {
    let p = std::path::Path::new("/proc/1/task/1/children");
    p.is_file()
}

fn main() {
    let use_children_file = has_children_file();
    for pid in std::env::args().skip(1) {
        let pids = if use_children_file {
            get_all_children_for_pid(&pid)
        } else {
            get_all_children_for_pid_old_kernel(&pid)
        };
        println!("{}", pids.join(" "));
    }
}
