#[cfg(all(feature = "rpi", feature = "simulator"))]
compile_error!("feature \"rpi\" and feature \"simulator\" cannot be enabled at the same time");

mod firebase;
mod led;
mod widgets;
mod startup;

use chrono::Utc;
use dotenv::dotenv;
use led::{ DrawableScreen, ScreenManager};
use startup::{check_for_network, welcome};
use tokio::sync::watch;
use std::time::Duration;
use widgets::{
    alerts::{render_alert_display, spawn_alert_update_task, AlertMode, AlertState},
    arrival::{
        render_arrival_display, spawn_arrival_update_task,
        ArrivalState, SimpleArrivalDisplayable,
    },
};


#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let mut manager = ScreenManager::init();
    welcome(&mut manager).await;
    // check_for_network(&mut manager).await;


    let mut loading_message: Vec<SimpleArrivalDisplayable> = Vec::new();
    loading_message.push(SimpleArrivalDisplayable::loading());
    let (arrival_tx, mut arrival_rx) = watch::channel(ArrivalState {
        messages: loading_message,
        last_update: Utc::now(),
    });
    spawn_arrival_update_task(arrival_tx);

    let (alert_tx, mut alert_rx) = watch::channel(AlertState::blank());
    spawn_alert_update_task(alert_tx);

    'running: loop {
        manager.clear();

        let mut messages: Vec<SimpleArrivalDisplayable> = Vec::new();
        let mut alert_state: AlertState = AlertState::blank();

        let arrival_res = arrival_rx.has_changed();
        if arrival_res.is_ok() {
            messages = arrival_rx.borrow_and_update().messages.clone();
        }

        let alert_res = alert_rx.has_changed();
        if alert_res.is_ok() {
            alert_state = alert_rx.borrow_and_update().clone();
        }

        if alert_state.mode != AlertMode::Hidden {
            render_alert_display(alert_state, manager.get_canvas());
        } else {
            render_arrival_display(messages, manager.get_canvas());
        }
        if manager.run_updates_should_exit() {
            break 'running;
        }

        #[cfg(feature = "simulator")]
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
