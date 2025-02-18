#[cfg(all(feature = "rpi", feature = "simulator"))]
compile_error!("feature \"rpi\" and feature \"simulator\" cannot be enabled at the same time");

mod firebase;
mod widgets;

use chrono::Utc;
use dotenv::dotenv;
use tokio::sync::watch;
use std::time::Duration;
use widgets::{
    alerts::{render_alert_display, spawn_alert_update_task, AlertMode, AlertState},
    arrival::{
        render_arrival_display, spawn_arrival_update_task,
        ArrivalState, SimpleArrivalDisplayable,
    },
};

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
};

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorEvent, Window};
#[cfg(feature = "rpi")]
use rpi_led_matrix::{args, LedMatrix};

#[cfg(feature = "simulator")]
use embedded_graphics_simulator::SimulatorDisplay;

// Change depending on your monitor resolution.
#[cfg(feature = "simulator")]
const WINDOW_SCALING: u32 = 8;

#[cfg(feature = "rpi")]
fn get_canvas() {}

#[cfg(feature = "simulator")]
fn get_canvas() -> SimulatorDisplay<Rgb888> {
    use widgets::{SCREEN_HEIGHT, SCREEN_WIDTH};

    return SimulatorDisplay::<Rgb888>::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT));
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // let app = args::add_matrix_args(
    //     App::new("C++ Library Example")
    //         .about("shows basic usage of matrix arguments")
    //         .version(crate_version!())
    //         .arg(
    //             arg!(--loops <LOOPS> "number of cycles to spin the line")
    //                 .default_value("5")
    //                 .required(false),
    //         ),
    // );
    // let matches = app.get_matches();
    // let (options, rt_options) = args::matrix_options_from_args(&matches);

    // let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    // let mut canvas = matrix.canvas();


    let mut canvas = get_canvas();

    let output_settings = OutputSettingsBuilder::new().scale(WINDOW_SCALING).build();
    let mut window = Window::new("Metro Sign Simulator", &output_settings);

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

        canvas.clear(Rgb888::BLACK).unwrap();

        if alert_state.mode != AlertMode::Hidden {
            render_alert_display(alert_state, &mut canvas);
        } else {
            render_arrival_display(messages, &mut canvas);
        }

        window.update(&canvas);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
