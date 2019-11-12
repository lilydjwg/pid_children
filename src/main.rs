mod util;

fn main() {
    for pid in std::env::args().skip(1) {
        let pids = util::get_all_children_for_pid(&pid);
        if !pids.is_empty() {
            println!("{}", pids.join(" "));
        }
    }
}

// vim: se sw=4:
