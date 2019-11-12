mod util;

fn main() {
    let my_pid = unsafe { libc::getpid() }.to_string();
    let parent_pid = unsafe { libc::getppid() }.to_string();

    for pid in util::get_all_children_for_pid(&parent_pid) {
        if pid != my_pid {
            let pid = pid.parse().unwrap();
            unsafe { libc::kill(pid, libc::SIGKILL); }
        }
    }
}

// vim: se sw=4:
