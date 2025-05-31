#[cfg(all(feature = "rpi", feature = "simulator"))]
compile_error!("feature \"rpi\" and feature \"simulator\" cannot be enabled at the same time");

mod firebase;
mod led;
mod startup;
mod widgets;

use chrono::Utc;
use dotenv::dotenv;
use led::{DrawableScreen, ScreenManager};
use log::{debug, info};
use startup::{draw_boot, spawn_startup_task, StartupMode, StartupState};
use std::{env, time::Duration};
use tokio::sync::watch;
use widgets::{
    alerts::{render_alert_display, spawn_alert_update_task, AlertMode, AlertState},
    arrival::{
        render_arrival_display, spawn_arrival_update_task, ArrivalState, SimpleArrivalDisplayable,
    },
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Skip the welcome sequence
    #[arg(short, long)]
    skip_welcome: bool,

    /// Just show "booting" message and exit
    #[arg(short, long)]
    boot: bool,

    /// Number of seconds to show "waiting for power stability"
    #[arg(short, long, default_value_t = 0)]
    delay_seconds: u8,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let args = Args::parse();
    
    info!(target: "main", "Starting WMATA Metro Arrival Sign by Noah Gearhart");

    debug!(target: "main", "Initializing screen");
    let mut manager = ScreenManager::init();
    debug!(target: "main", "Done");

    if args.boot {
        draw_boot(&mut manager).await;
        return;
    }

    if args.skip_welcome {
        info!(target: "main", "Skipping welcome");
    } else {
        info!(target: "main", "Running welcome sequence. Use `--skip-welcome` argument to skip. In dev, use `cargo run -- --skip-welcome`.");
        let (startup_tx, mut startup_rx) = watch::channel(StartupState::blank());
        let startup_task = spawn_startup_task(startup_tx, args.delay_seconds.into());

        let mut startup_state: StartupState = StartupState::blank();
        'startup: loop {
            manager.clear();

            let startup_res = startup_rx.has_changed();
            if startup_res.is_ok() {
                startup_state = startup_rx.borrow_and_update().clone();
            }
            if startup_state.mode == StartupMode::Done || startup_task.is_finished() {
                break 'startup;
            } else {
                startup_state.render(&mut manager);
            }

            if manager.run_updates_should_exit() {
                break 'startup;
            }

            #[cfg(feature = "simulator")]
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    let mut loading_message: Vec<SimpleArrivalDisplayable> = Vec::new();
    loading_message.push(SimpleArrivalDisplayable::loading());
    let (mut arrival_tx, mut arrival_rx) = watch::channel(ArrivalState {
        messages: loading_message,
        last_update: Utc::now(),
    });
    let mut arrival_update_task = spawn_arrival_update_task(arrival_tx);

    let (mut alert_tx, mut alert_rx) = watch::channel(AlertState::blank());
    let mut alert_update_task = spawn_alert_update_task(alert_tx);

    'running: loop {
        manager.clear();
        let mut messages: Vec<SimpleArrivalDisplayable> = Vec::new();
        let mut alert_state: AlertState = AlertState::blank();

        // Check if threads have exited (probably in error)
        if arrival_update_task.is_finished() {
            info!(target: "main", "Detected arrival update task exited. Restarting...");
            (arrival_tx, arrival_rx) = watch::channel(ArrivalState {
                messages: Vec::new(),
                last_update: Utc::now(),
            });
            arrival_update_task = spawn_arrival_update_task(arrival_tx);
        }
        if alert_update_task.is_finished() {
            info!(target: "main", "Detected alert update task exited. Restarting...");
            (alert_tx, alert_rx) = watch::channel(AlertState::blank());
            alert_update_task = spawn_alert_update_task(alert_tx);
        }

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
