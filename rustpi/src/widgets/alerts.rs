use std::time::Duration;

use log::{debug, info};
use tokio::{spawn, sync::watch::Sender, task::JoinHandle};

use crate::firebase::{Alert, AlertWidget, LoadableWidget};

pub struct AlertState {
    pub alerts: Vec<Alert>
}

pub fn spawn_alert_update_task(state_tx: Sender<AlertState>) -> JoinHandle<()> {
    spawn(async move {
        loop {
            debug!(target: "alert_state_update", "Loading new state...");

            let new_state = AlertWidget::load().await;
            debug!(target: "alert_state_update", "{:?}", new_state.alerts);

            info!(target: "alert_state_update", "New state loaded. Sending to main thread.");
            state_tx.send(AlertState {
                alerts: new_state.alerts.clone()
            }).unwrap();
            tokio::time::sleep(Duration::from_secs(120)).await;
        }
    })
}