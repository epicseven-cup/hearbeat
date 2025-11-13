use std::collections::HashMap;
use std::os::macos::raw::stat;
use std::sync::Arc;
use actix_web::{web, Responder};
use tokio::time::interval;
use crate::AppState;

pub async fn heartbeat(state:Arc<AppState>) {

    tokio::spawn(async move {

        let mut interval = interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let keys: Vec<String> = {
                let m = state.resources.lock().unwrap();
                m.keys().cloned().collect()
            };

            let mut resource = HashMap::new();
            for k in keys {
                println!("{}", k);
                let statue = reqwest::get(k.clone()).await.unwrap().status().is_success();
                resource.insert(k, statue);
            }

            for (k, v) in resource {
                state.resources.lock().unwrap().insert(k, v);
            }
        }
    });

}