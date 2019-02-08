use std::sync::atomic::AtomicUsize;

pub static mut GLOBAL_INDEX: AtomicUsize = AtomicUsize::new(0);
