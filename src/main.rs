use std::fs::File;
use std::io::Read;

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

fn main() {
    for pid in std::env::args().skip(1) {
        let pids = get_all_children_for_pid(&pid);
        println!("{}", pids.join(" "));
    }
}
