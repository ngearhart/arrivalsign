#[cfg(all(feature = "rpi", feature = "simulator"))]
compile_error!("feature \"rpi\" and feature \"simulator\" cannot be enabled at the same time");

mod firebase;
mod widgets;

use chrono::Utc;
use dotenv::dotenv;
use env_logger::Builder;
use firebase::{AlertWidget, ArrivalWidget, LoadableWidget};
use tokio::{select, spawn, sync::{mpsc, oneshot, watch, Mutex}, time::interval};
use widgets::arrival::{get_latest_state, render_arrival_display, spawn_arrival_update_task, ArrivalDisplayable, ArrivalState, SimpleArrivalDisplayable, TrainDisplayEntry};
use std::{io::Read, ops::Deref, sync::{Arc, LazyLock, RwLock}, thread, time::Duration};

use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
    text::Text,
    pixelcolor::BinaryColor
};

use itertools::{join, Itertools};

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorEvent, Window};
#[cfg(feature = "rpi")]
use rpi_led_matrix::{args, LedMatrix};

#[cfg(feature = "simulator")]
use embedded_graphics_simulator::SimulatorDisplay;

const WIDTH: u32 = 64 * 2;
const HEIGHT: u32 = 32 * 2;

// Change depending on your monitor resolution.
#[cfg(feature = "simulator")]
const WINDOW_SCALING: u32 = 8;

#[cfg(feature = "rpi")]
fn get_canvas() {

}

#[cfg(feature = "simulator")]
fn get_canvas() -> SimulatorDisplay<Rgb888> {
    return SimulatorDisplay::<Rgb888>::new(Size::new(WIDTH, HEIGHT));
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

    // Create styles used by the drawing operations.
    // let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::new(64, 0, 128), 1);
    // let fill = PrimitiveStyle::with_fill(Rgb888::new(0, 128, 32));
    // let text_style = MonoTextStyle::new(&FONT_4X6, Rgb888::new(0xff, 0xff, 0xff));

    // let yoffset = 10;

    // // Draw a 3px wide outline around the matrix.

    let mut canvas = get_canvas();

    // let display_size = canvas.size();
    // let (width, height) = (display_size.width, display_size.height);
    // Rectangle::with_corners(
    //     Point::zero(),
    //     Point::new(width as i32 - 1, height as i32 - 1),
    // )
    // .into_styled(thin_stroke)
    // .draw(&mut canvas)
    // .unwrap();

    // // Draw a triangle.
    // Triangle::new(
    //     Point::new(4, 8 + yoffset),
    //     Point::new(4 + 8, 8 + yoffset),
    //     Point::new(4 + 4, yoffset),
    // )
    // .into_styled(fill)
    // .draw(&mut canvas)
    // .unwrap();

    // // Draw a filled square
    // Rectangle::with_corners(Point::new(52, yoffset), Point::new(16, 16))
    //     .into_styled(fill)
    //     .draw(&mut canvas)
    //     .unwrap();

    // // Draw a circle with a 3px wide stroke.
    // Circle::new(Point::new((width) / 2, yoffset - 2), 5)
    //     .into_styled(fill)
    //     .draw(&mut canvas)
    //     .unwrap();

    // // Draw centered text.
    // let eg_text = "EG+";
    // Text::new(eg_text, Point::new(16, 16), text_style)
    //     .draw(&mut canvas)
    //     .unwrap();

    // let rpi_text = "RPi";
    // Text::new(rpi_text, Point::new(16, 22), text_style)
    //     .draw(&mut canvas)
    //     .unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .scale(WINDOW_SCALING)
        .build();
    let mut window = Window::new("Metro Sign Simulator", &output_settings);

    let a = ArrivalWidget::load().await;
    let b = AlertWidget::load().await;

    // println!("{}", b.get_messages().join(", "));
    // println!("{}", b.alerts.iter().cloned().map(|alert| alert.message).join(", "));

    // let state = get_latest_state(a).await.unwrap();
    // println!("{}", join(state.iter().map(|item| {
    //    item.pretty_print()
    // }), "\n"));

    // let (tx, mut rx) = mpsc::channel(32);
    let (tx, mut rx) = watch::channel(ArrivalState { messages: Vec::new(), last_update: Utc::now() });
    spawn_arrival_update_task(tx);


    let mut interval = interval(Duration::from_millis(100));


    'running: loop {

        let mut messages: Vec<SimpleArrivalDisplayable> = Vec::new();

        let res = rx.has_changed();
        if res.is_ok() {
            messages = rx.borrow_and_update().messages.clone();
        }

        canvas.clear(Rgb888::BLACK).unwrap();
        render_arrival_display(messages, &mut canvas);
        window.update(&canvas);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }

        // tokio::select! {
        //     _ = interval.tick() => {
        //         
        //     },
        //     msg = &mut rx => {
        //         println!("State updated");
        //         if msg.is_ok() {
        //             messages = msg.messages;
        //         }
        //         break;
        //     }
        // }

        // match rx.recv().await {
        //     Some(msg) => {
        //     },
        //     None => ()
        // }

    
        // interval.tick().await;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}