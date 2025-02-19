use std::{fmt::Debug, time::Duration};

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_7X14_BOLD},
        MonoTextStyle,
    },
    pixelcolor::Rgb888,
    prelude::{DrawTarget, Point, Primitive, RgbColor},
    primitives::{PrimitiveStyle, Rectangle},
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use log::{debug, info};
use rand::seq::IteratorRandom;
use tokio::{spawn, sync::watch::Sender, task::JoinHandle};

use crate::firebase::{AlertWidget, LoadableWidget};
use embedded_graphics::Drawable;

use super::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertMode {
    IntroA,
    IntroB,
    MessageA,
    MessageB,
    Hidden,
}

#[derive(Clone, Debug)]
pub struct AlertState {
    pub mode: AlertMode,
    pub currently_shown_message: String,
    scroll_index: u32,
}

impl AlertState {
    pub fn blank() -> Self {
        AlertState {
            mode: AlertMode::Hidden,
            currently_shown_message: String::from(""),
            scroll_index: 0,
        }
    }
}

pub fn spawn_alert_update_task(state_tx: Sender<AlertState>) -> JoinHandle<()> {
    spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
            debug!(target: "alert_state_update", "Loading new state...");

            let new_state = AlertWidget::load().await;
            debug!(target: "alert_state_update", "{:?}", new_state.alerts);
            info!(target: "alert_state_update", "New state loaded. Sending to main thread.");
            let intro_a = AlertState {
                mode: AlertMode::IntroA,
                currently_shown_message: new_state
                    .alerts
                    .iter()
                    .choose(&mut rand::rng())
                    .unwrap()
                    .message
                    .to_string(),
                scroll_index: 0,
            };
            let intro_b = AlertState {
                mode: AlertMode::IntroB,
                currently_shown_message: intro_a.currently_shown_message.clone(),
                scroll_index: 0,
            };
            let mut message_a = AlertState {
                mode: AlertMode::MessageA,
                currently_shown_message: intro_a.currently_shown_message.clone(),
                scroll_index: 0,
            };
            let mut message_b = AlertState {
                mode: AlertMode::MessageB,
                currently_shown_message: intro_a.currently_shown_message.clone(),
                scroll_index: 0,
            };
            if new_state.alerts.len() > 0 {
                // Show intro
                for _ in 1..3 {
                    state_tx.send(intro_a.clone()).unwrap();
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    state_tx.send(intro_b.clone()).unwrap();
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }

                // Show real message
                for i in 0..3 {
                    message_a.scroll_index = i * 4;
                    message_b.scroll_index = i * 4;
                    state_tx.send(message_a.clone()).unwrap();
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    message_a.scroll_index = i * 4 + 1;
                    message_b.scroll_index = i * 4 + 1;
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    state_tx.send(message_b.clone()).unwrap();
                    message_a.scroll_index = i * 4 + 2;
                    message_b.scroll_index = i * 4 + 2;
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    message_a.scroll_index = i * 4 + 3;
                    message_b.scroll_index = i * 4 + 3;
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
            state_tx.send(AlertState::blank()).unwrap();
            tokio::time::sleep(Duration::from_secs(rand::random_range(60..300))).await;
        }
    })
}

pub fn render_alert_display<D>(state: AlertState, canvas: &mut D)
where
    D: DrawTarget<Color = Rgb888>,
    <D as DrawTarget>::Error: Debug,
{
    let border_rect_style = PrimitiveStyle::with_fill(Rgb888::YELLOW);
    let invisible_style = PrimitiveStyle::with_fill(Rgb888::BLACK);
    const RECT_WIDTH: i32 = 5;

    for i in 0..(SCREEN_WIDTH as i32 / RECT_WIDTH / 2 + 1) {
        // Top row
        Rectangle::with_corners(
            Point::new(i * RECT_WIDTH * 2, 0),
            Point::new(RECT_WIDTH + i * RECT_WIDTH * 2 - 1, RECT_WIDTH - 1),
        )
        .into_styled(
            if state.mode == AlertMode::IntroA || state.mode == AlertMode::MessageA {
                border_rect_style
            } else {
                invisible_style
            },
        )
        .draw(canvas)
        .unwrap();

        Rectangle::with_corners(
            Point::new((i * RECT_WIDTH * 2) + RECT_WIDTH, 0),
            Point::new(
                (RECT_WIDTH + i * RECT_WIDTH * 2 - 1) + RECT_WIDTH,
                RECT_WIDTH - 1,
            ),
        )
        .into_styled(
            if state.mode == AlertMode::IntroA || state.mode == AlertMode::MessageA {
                invisible_style
            } else {
                border_rect_style
            },
        )
        .draw(canvas)
        .unwrap();

        // Bottom row
        Rectangle::with_corners(
            Point::new(i * RECT_WIDTH * 2, SCREEN_HEIGHT as i32 - RECT_WIDTH),
            Point::new(RECT_WIDTH + i * RECT_WIDTH * 2 - 1, SCREEN_HEIGHT as i32),
        )
        .into_styled(
            if state.mode == AlertMode::IntroA || state.mode == AlertMode::MessageA {
                invisible_style
            } else {
                border_rect_style
            },
        )
        .draw(canvas)
        .unwrap();

        Rectangle::with_corners(
            Point::new(
                (i * RECT_WIDTH * 2) + RECT_WIDTH,
                SCREEN_HEIGHT as i32 - RECT_WIDTH,
            ),
            Point::new(
                (RECT_WIDTH + i * RECT_WIDTH * 2 - 1) + RECT_WIDTH,
                SCREEN_HEIGHT as i32,
            ),
        )
        .into_styled(
            if state.mode == AlertMode::IntroA || state.mode == AlertMode::MessageA {
                border_rect_style
            } else {
                invisible_style
            },
        )
        .draw(canvas)
        .unwrap();
    }

    let big_text_style = MonoTextStyle::new(&FONT_7X14_BOLD, Rgb888::new(255, 255, 255));
    let small_text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255, 255, 255));
    let centered_textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::Exact(
            embedded_text::style::VerticalOverdraw::Visible,
        ))
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        .alignment(HorizontalAlignment::Center)
        .paragraph_spacing(6)
        .build();
    let top_aligned_textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::Exact(
            embedded_text::style::VerticalOverdraw::FullRowsOnly,
        ))
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Top)
        .alignment(HorizontalAlignment::Center)
        .paragraph_spacing(6)
        .build();
    let bounds = Rectangle::with_corners(
        Point::new(0, RECT_WIDTH),
        Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 - RECT_WIDTH),
    );

    if state.mode == AlertMode::IntroA || state.mode == AlertMode::IntroB {
        TextBox::with_textbox_style(
            "Metro Alert",
            bounds,
            big_text_style,
            centered_textbox_style,
        )
        .draw(canvas)
        .unwrap();
    } else if state.mode == AlertMode::MessageA || state.mode == AlertMode::MessageB {
        let height = centered_textbox_style.measure_text_height(
            &small_text_style,
            &state.currently_shown_message,
            bounds.size.width,
        );
        if height > bounds.size.height {
            TextBox::with_textbox_style(
                &state.currently_shown_message,
                bounds,
                small_text_style,
                top_aligned_textbox_style,
            )
            .set_vertical_offset(-2 * state.scroll_index as i32) // Magic number: line height
            .draw(canvas)
            .unwrap();
        } else {
            TextBox::with_textbox_style(
                &state.currently_shown_message,
                bounds,
                small_text_style,
                centered_textbox_style,
            )
            .draw(canvas)
            .unwrap();
        }
    }
}
