extern crate cc;

fn main() {
    if cfg!(target_os = "macos") {
        cc::Build::new().file("support/thread_priority_helper.c").compile("thread_priority_helper");
    }
}
