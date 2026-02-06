//! Example 13: Multithreaded usage with shared config.
//!
//! This shows how to protect the config with a Mutex while multiple threads
//! update and save it.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use struct_vault::{vault_config, PersistentStructConfig};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct SharedConfig {
    name: String,
    count: i32,
    enabled: bool,
}

fn main() {
    let mut base = SharedConfig::default();
    base.vault_config_default("multithread_example");
    base.vault_load_or_default();

    let shared = Arc::new(Mutex::new(base));

    let mut handles = Vec::new();
    for i in 0..4 {
        let shared = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let mut cfg = shared.lock().expect("lock config");
            cfg.count += 1;
            cfg.name = format!("Updated by thread {}", i);
            cfg.enabled = !cfg.enabled;
            cfg.vault_save().expect("save config");
            println!("Thread {} saved: count={}", i, cfg.count);
        }));
    }

    for h in handles {
        h.join().expect("thread join");
    }

    let final_cfg = shared.lock().expect("lock config");
    println!("Final config: {:?}", *final_cfg);
}
