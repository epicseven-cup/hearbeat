use crate::AppState;
use actix_web::{Responder, web};
use std::collections::HashMap;
use std::os::macos::raw::stat;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

pub async fn heartbeat(state: Arc<AppState>) {
    let mut interval = interval(std::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        let keys: Vec<String> = {
            let m = state.resources.lock().unwrap();
            m.keys().cloned().collect()
        };

        let mut resource = HashMap::new();

        let client = reqwest::Client::builder().timeout(Duration::from_secs(2)).build().unwrap();
        for k in keys {
            println!("{} {}", k, state.resources.lock().unwrap().get(&k).unwrap());
            let statue = match client.get(&k).send().await {
                Ok(response) => {
                response.status().is_success()
                },

                Err(e) => {
                    false
                }
            };

            resource.insert(k, statue);
        }

        for (k, v) in resource {
            state.resources.lock().unwrap().insert(k, v);
        }
    }
}
