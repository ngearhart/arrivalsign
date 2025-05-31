use std::time::Duration;

use embedded_graphics::{
    mono_font::{
        ascii::{
            FONT_5X7, FONT_6X10, FONT_8X13_BOLD
        },
        MonoTextStyle,
    },
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, RgbColor},
    primitives::{PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use log::{debug, info};
use sysinfo::Networks;
use tokio::{spawn, sync::watch::Sender, task::JoinHandle};

use crate::{
    led::{DrawableScreen, ScreenManager},
    widgets::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StartupMode {
    Waiting,
    WelcomeIn,
    WelcomeStatic,
    WelcomeOut,
    NetworkConnecting,
    NetworkStatus,
    Hidden,
    Done,
}

#[derive(Clone, Debug)]
pub struct StartupState {
    pub mode: StartupMode,
    discovered_ip: Option<String>,
    scroll_index: u32,
}

impl StartupState {
    pub fn blank() -> Self {
        StartupState {
            mode: StartupMode::Hidden,
            discovered_ip: None,
            scroll_index: 0,
        }
    }

    pub fn render(self: &Self, manager: &mut ScreenManager) {
        match self.mode {
            StartupMode::Waiting => draw_waiting(manager, self.scroll_index),
            StartupMode::WelcomeIn => draw_welcome_in(manager, self.scroll_index),
            StartupMode::WelcomeStatic => draw_welcome_text(manager, true),
            StartupMode::WelcomeOut => draw_welcome_out(manager, self.scroll_index),
            StartupMode::NetworkConnecting => draw_network_connecting(manager, self.scroll_index as f32 / 100.0),
            StartupMode::NetworkStatus => {
                if self.scroll_index <= 100 {
                    draw_network_with_ip(manager, self.discovered_ip.clone().unwrap(), self.scroll_index as f32 / 100.0, 1.0);
                } else {
                    draw_network_with_ip(manager, self.discovered_ip.clone().unwrap(), 2.0 - (self.scroll_index as f32 / 100.0), 2.0 - (self.scroll_index as f32 / 100.0));
                }
            }
            StartupMode::Hidden => (),
            StartupMode::Done => (),
        }
    }
}

fn ease_out_cubic(x: f32, scaling_limit: f32) -> u32 {
    ((1.0 - (1.0 - x / scaling_limit).powf(3.0)) * scaling_limit).round() as u32
}

fn draw_waiting(manager: &mut ScreenManager, seconds: u32) {
    let centered_textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::Exact(
            embedded_text::style::VerticalOverdraw::Visible,
        ))
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        .alignment(HorizontalAlignment::Center)
        .paragraph_spacing(0)
        .build();
    let character_style_target_color = Rgb888::new(80, 80, 80);
    let regular_character_style = MonoTextStyle::new(&FONT_6X10, character_style_target_color);
    let bottom_corner = Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let top_corner = Point::new(0, 0);
        TextBox::with_textbox_style(
        &format!("Waiting for power stability ({})", seconds),
        Rectangle::with_corners(
            top_corner,
            bottom_corner,
        ),
        regular_character_style,
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();
}

fn draw_welcome_text(manager: &mut ScreenManager, include_bg: bool) {
    let centered_textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::Exact(
            embedded_text::style::VerticalOverdraw::Visible,
        ))
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        .alignment(HorizontalAlignment::Center)
        .paragraph_spacing(0)
        .build();
    let character_style_target_color = Rgb888::new(0x0C, 0x12, 0x0C);
    let bold_character_style = MonoTextStyle::new(&FONT_8X13_BOLD, character_style_target_color);
    let regular_character_style = MonoTextStyle::new(&FONT_6X10, character_style_target_color);
    let small_character_style = MonoTextStyle::new(&FONT_5X7, character_style_target_color);
    let bottom_corner = Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let top_corner = Point::new(0, 0);
    let box_3_style = PrimitiveStyle::with_fill(Rgb888::new(0x42, 0x40, 0x64));

    if include_bg {
        Rectangle::with_corners(top_corner, bottom_corner)
            .into_styled(box_3_style)
            .draw(manager.get_canvas())
            .unwrap();
    }

    TextBox::with_textbox_style(
        "WMATA Metrorail Arrival Sign",
        Rectangle::with_corners(
            top_corner,
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 / 2),
        ),
        bold_character_style,
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();
    TextBox::with_textbox_style(
        "by Noah Gearhart",
        Rectangle::with_corners(
            Point::new(0, SCREEN_HEIGHT as i32 / 2),
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 * 3 / 4),
        ),
        regular_character_style,
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();
    TextBox::with_textbox_style(
        "for Dark Wolf Solutions",
        Rectangle::with_corners(Point::new(0, SCREEN_HEIGHT as i32 * 3 / 4), bottom_corner),
        small_character_style,
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();
}

fn draw_welcome_in(manager: &mut ScreenManager, i: u32) {
    let black_style = PrimitiveStyle::with_fill(Rgb888::BLACK);
    let box_1_style = PrimitiveStyle::with_fill(Rgb888::new(0x49, 0x47, 0x5B));
    let box_2_style = PrimitiveStyle::with_fill(Rgb888::new(0x79, 0x94, 0x96));
    let box_3_style = PrimitiveStyle::with_fill(Rgb888::new(0x42, 0x40, 0x64));

    let bottom_corner = Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let top_corner = Point::new(0, 0);

    Rectangle::with_corners(
        Point::new(
            0,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 20.0, SCREEN_HEIGHT as f32) as i32,
        ),
        bottom_corner,
    )
    .into_styled(box_3_style)
    .draw(manager.get_canvas())
    .unwrap();

    draw_welcome_text(manager, false);

    Rectangle::with_corners(
        top_corner,
        Point::new(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32 - ease_out_cubic(i as f32 / 2.0, SCREEN_HEIGHT as f32) as i32,
        ),
    )
    .into_styled(black_style)
    .draw(manager.get_canvas())
    .unwrap();

    Rectangle::with_corners(
        Point::new(
            0,
            SCREEN_HEIGHT as i32 - ease_out_cubic(i as f32 / 2.0, SCREEN_HEIGHT as f32) as i32,
        ),
        Point::new(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 10.0, SCREEN_HEIGHT as f32) as i32,
        ),
    )
    .into_styled(box_1_style)
    .draw(manager.get_canvas())
    .unwrap();

    Rectangle::with_corners(
        Point::new(
            0,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 10.0, SCREEN_HEIGHT as f32) as i32,
        ),
        Point::new(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 20.0, SCREEN_HEIGHT as f32) as i32
                - 1,
        ),
    )
    .into_styled(box_2_style)
    .draw(manager.get_canvas())
    .unwrap();
}

fn draw_welcome_out(manager: &mut ScreenManager, i: u32) {
    let black_style = PrimitiveStyle::with_fill(Rgb888::BLACK);
    let box_1_style = PrimitiveStyle::with_fill(Rgb888::new(0x49, 0x47, 0x5B));
    let box_2_style = PrimitiveStyle::with_fill(Rgb888::new(0x79, 0x94, 0x96));
    let box_3_style = PrimitiveStyle::with_fill(Rgb888::new(0x42, 0x40, 0x64));

    let bottom_corner = Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let top_corner = Point::new(0, 0);

    Rectangle::with_corners(
        top_corner,
        Point::new(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32 - ease_out_cubic(i as f32 / 2.0, SCREEN_HEIGHT as f32) as i32,
        ),
    )
    .into_styled(box_3_style)
    .draw(manager.get_canvas())
    .unwrap();

    draw_welcome_text(manager, false);

    Rectangle::with_corners(
        Point::new(
            0,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 10.0, SCREEN_HEIGHT as f32) as i32,
        ),
        Point::new(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 20.0, SCREEN_HEIGHT as f32) as i32,
        ),
    )
    .into_styled(box_1_style)
    .draw(manager.get_canvas())
    .unwrap();

    Rectangle::with_corners(
        Point::new(
            0,
            SCREEN_HEIGHT as i32 - ease_out_cubic(i as f32 / 2.0, SCREEN_HEIGHT as f32) as i32,
        ),
        Point::new(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 10.0, SCREEN_HEIGHT as f32) as i32,
        ),
    )
    .into_styled(box_2_style)
    .draw(manager.get_canvas())
    .unwrap();

    Rectangle::with_corners(
        Point::new(
            0,
            SCREEN_HEIGHT as i32
                - ease_out_cubic((i as f32 / 2.0) - 20.0, SCREEN_HEIGHT as f32) as i32,
        ),
        bottom_corner,
    )
    .into_styled(black_style)
    .draw(manager.get_canvas())
    .unwrap();
}

fn draw_network_connecting(manager: &mut ScreenManager, opacity: f32) {
    let centered_textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::Exact(
            embedded_text::style::VerticalOverdraw::Visible,
        ))
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        .alignment(HorizontalAlignment::Center)
        .paragraph_spacing(0)
        .build();

    TextBox::with_textbox_style(
        "Connecting to\nNetwork",
        Rectangle::with_corners(
            Point::new(0, 0),
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 / 2),
        ),
        MonoTextStyle::new(&FONT_6X10, get_color_with_opacity(Rgb888::new(0x84, 0xD2, 0xF6), opacity)),
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();
}

fn get_color_with_opacity(color: Rgb888, opacity: f32) -> Rgb888 {
    return Rgb888::new(
        (color.r() as f32 * opacity).round() as u8,
        (color.g() as f32 * opacity).round() as u8,
        (color.b() as f32 * opacity).round() as u8,
    )
}

fn draw_network_with_ip(manager: &mut ScreenManager, ip: String, in_opacity: f32, out_opacity: f32) {
    let centered_textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::Exact(
            embedded_text::style::VerticalOverdraw::Visible,
        ))
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        .alignment(HorizontalAlignment::Center)
        .paragraph_spacing(0)
        .build();
    TextBox::with_textbox_style(
        "Connected to\nNetwork",
        Rectangle::with_corners(
            Point::new(0, 0),
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 / 2),
        ),
        MonoTextStyle::new(&FONT_6X10, get_color_with_opacity(Rgb888::new(0x84, 0xD2, 0xF6), out_opacity)),
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();

    TextBox::with_textbox_style(
        &format!("IP Address:\n{}", ip),
        Rectangle::with_corners(
            Point::new(0, SCREEN_HEIGHT as i32 / 2),
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32),
        ),
        MonoTextStyle::new(&FONT_6X10, get_color_with_opacity(Rgb888::new(0xD7, 0xB3, 0x77), in_opacity)),
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();
}

pub fn draw_boot(manager: &mut ScreenManager) {
    manager.clear();
    let centered_textbox_style = TextBoxStyleBuilder::new()
    .height_mode(HeightMode::Exact(
        embedded_text::style::VerticalOverdraw::Visible,
    ))
    .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
    .alignment(HorizontalAlignment::Center)
    .paragraph_spacing(0)
    .build();
    let character_style_target_color = Rgb888::new(80, 80, 80);
    let regular_character_style = MonoTextStyle::new(&FONT_6X10, character_style_target_color);
    let bottom_corner = Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let top_corner = Point::new(0, 0);
        TextBox::with_textbox_style(
        &format!("Booting"),
        Rectangle::with_corners(
            top_corner,
            bottom_corner,
        ),
        regular_character_style,
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();
    manager.run_updates_should_exit();
}

pub fn spawn_startup_task(state_tx: Sender<StartupState>, wait_seconds: u32) -> JoinHandle<()> {
    spawn(async move {
        debug!(target: "startup_state_update", "Running welcome");
        if wait_seconds > 0 {
            for i in 0..wait_seconds {
                state_tx
                    .send(StartupState {
                        mode: StartupMode::Waiting,
                        discovered_ip: None,
                        scroll_index: wait_seconds - i,
                    })
                    .unwrap();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            state_tx
                .send(StartupState {
                    mode: StartupMode::Hidden,
                    discovered_ip: None,
                    scroll_index: 0,
                })
                .unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        for i in 0..SCREEN_HEIGHT * 3 {
            state_tx
                .send(StartupState {
                    mode: StartupMode::WelcomeIn,
                    discovered_ip: None,
                    scroll_index: i,
                })
                .unwrap();
            tokio::time::sleep(Duration::from_millis(15)).await;
        }

        state_tx
            .send(StartupState {
                mode: StartupMode::WelcomeStatic,
                discovered_ip: None,
                scroll_index: 0,
            })
            .unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;

        for i in 0..SCREEN_HEIGHT * 3 {
            state_tx
                .send(StartupState {
                    mode: StartupMode::WelcomeOut,
                    discovered_ip: None,
                    scroll_index: i,
                })
                .unwrap();
            tokio::time::sleep(Duration::from_millis(15)).await;
        }

        debug!(target: "startup_state_update", "Running network");
        for i in 0..50 {
            state_tx
                .send(StartupState {
                    mode: StartupMode::NetworkConnecting,
                    discovered_ip: None,
                    scroll_index: i * 2,
                })
                .unwrap();
            tokio::time::sleep(Duration::from_millis(15)).await;
        }

        state_tx
            .send(StartupState {
                mode: StartupMode::NetworkConnecting,
                discovered_ip: None,
                scroll_index: 100,
            })
            .unwrap();
        tokio::time::sleep(Duration::from_secs(2)).await;

        loop {
            let networks = Networks::new_with_refreshed_list();
            let discovered_ip = networks
                .iter()
                .flat_map(|iface| iface.1.ip_networks().iter())
                .find(|ip_addr| {
                    ip_addr.addr.is_ipv4()
                        && !ip_addr.addr.is_loopback()
                        && !ip_addr.addr.to_string().starts_with("172")
                });
            if discovered_ip.is_some() {
                info!(target: "startup_state_update", "IP Address: {}", discovered_ip.unwrap().addr);

                for i in 0..50 {
                    state_tx
                        .send(StartupState {
                            mode: StartupMode::NetworkStatus,
                            discovered_ip: Some(discovered_ip.unwrap().addr.to_string()),
                            scroll_index: i * 2,
                        })
                        .unwrap();
                    tokio::time::sleep(Duration::from_millis(15)).await;
                }

                state_tx
                    .send(StartupState {
                        mode: StartupMode::NetworkStatus,
                        discovered_ip: Some(discovered_ip.unwrap().addr.to_string()),
                        scroll_index: 100,
                    })
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(5)).await;
    
                for i in 50..100 {
                    state_tx
                        .send(StartupState {
                            mode: StartupMode::NetworkStatus,
                            discovered_ip: Some(discovered_ip.unwrap().addr.to_string()),
                            scroll_index: i * 2,
                        })
                        .unwrap();
                    tokio::time::sleep(Duration::from_millis(15)).await;
                }
                break;
            } else {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }

        info!(target: "startup_state_update", "Startup done");
        state_tx
            .send(StartupState {
                mode: StartupMode::Done,
                discovered_ip: None,
                scroll_index: 0,
            })
            .unwrap();
    })
}
