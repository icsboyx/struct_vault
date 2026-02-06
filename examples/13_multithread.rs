//! Example 13: Multithreaded usage with shared config.
//!
//! This shows how to protect the config with a Mutex while multiple threads
//! update and save it.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use struct_vault::{PersistentStructConfig, vault_config};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct SharedConfig {
    names: Vec<String>,
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
            println!("Thread {} starting", i);
            let mut cfg = shared.lock().expect("lock config");
            if i == 0 {
                for sec in (1..=5).rev() {
                    println!("Thread 0 holding lock: {}s", sec);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            cfg.count += 1;
            cfg.names.push(format!("Updated by thread {}", i));
            cfg.enabled = !cfg.enabled;
            cfg.vault_save().expect("save config");
            println!(
                "Thread {} saved: names={:?} count={} enabled={}",
                i, cfg.names, cfg.count, cfg.enabled
            );
        }));
    }

    for h in handles {
        h.join().expect("thread join");
    }

    let final_cfg = shared.lock().expect("lock config");
    println!("Final config: {:?}", *final_cfg);
}
