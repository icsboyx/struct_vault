//! Example 15: Async concurrency with shared config.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use struct_vault::{PersistentStructConfig, async_feature, vault_config};
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct AsyncSharedConfig {
    names: Vec<String>,
    count: i32,
    enabled: bool,
}

#[tokio::main]
async fn main() {
    let mut base = AsyncSharedConfig::default();
    base.vault_config_default("async_multithread_example");
    async_feature::vault_load_or_default_async(&mut base).await;

    let shared = Arc::new(Mutex::new(base));

    let mut handles = Vec::new();
    for i in 0..4 {
        let shared = Arc::clone(&shared);
        handles.push(tokio::spawn(async move {
            if i == 0 {
                for sec in (1..=5).rev() {
                    println!("Task 0 modifying in : {}s", sec);
                    sleep(Duration::from_secs(1)).await;
                }
            }
            let mut cfg = shared.lock().await;
            cfg.count += 1;
            cfg.names.push(format!("Updated by task {}", i));
            cfg.enabled = !cfg.enabled;
            async_feature::vault_save_async(&*cfg).await.expect("save config async");
            println!(
                "Task {} saved: names={:?} count={} enabled={}",
                i, cfg.names, cfg.count, cfg.enabled
            );
        }));
    }

    for h in handles {
        h.await.expect("task join");
    }

    let final_cfg = shared.lock().await;
    println!("Final config: {:?}", *final_cfg);
}
