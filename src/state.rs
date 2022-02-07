use once_cell::sync::{Lazy, OnceCell};
use std::collections::{HashMap};
use std::sync::{Mutex};

pub static STATE: Lazy<Mutex<HashMap<&str, bool>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("listen_leader_beat", false);
    m.insert("start_server",false);
    Mutex::new(m)
});

