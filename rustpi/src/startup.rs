use std::{cmp::{max, min}, time::Duration};

use embedded_graphics::{mono_font::{ascii::FONT_6X10, MonoTextStyle}, pixelcolor::Rgb888, prelude::{Point, Primitive, RgbColor}, primitives::{PrimitiveStyle, Rectangle}, Drawable};
use embedded_text::{alignment::HorizontalAlignment, style::{HeightMode, TextBoxStyleBuilder}, TextBox};
use log::info;
use sysinfo::Networks;

use crate::{led::{DrawableScreen, ScreenManager}, widgets::{SCREEN_HEIGHT, SCREEN_WIDTH}};


fn ease_out_cubic(x: f32, scaling_limit: f32) -> u32 {
    ((1.0 - (1.0 - x / scaling_limit).powf(3.0)) * scaling_limit).round() as u32
}

fn get_brightness(input: Rgb888, brightness: f32) -> Rgb888 {
    Rgb888::new(
        (brightness * input.r() as f32).round() as u8,
        (brightness * input.g() as f32).round() as u8,
        (brightness * input.b() as f32).round() as u8,
    )
}

pub async fn welcome(manager: &mut ScreenManager) {
    let centered_textbox_style = TextBoxStyleBuilder::new()
    .height_mode(HeightMode::Exact(
        embedded_text::style::VerticalOverdraw::Visible,
    ))
    .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
    .alignment(HorizontalAlignment::Center)
    .paragraph_spacing(0)
    .build();

    let character_style_target_color = Rgb888::new(0xEE, 0xF1, 0xBD);
    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0x84, 0xD2, 0xF6));

    let box_1_style = PrimitiveStyle::with_fill(Rgb888::new(0x49, 0x47, 0x5B));
    let box_2_style = PrimitiveStyle::with_fill(Rgb888::new(0x79, 0x94, 0x96));
    let box_3_style = PrimitiveStyle::with_fill(Rgb888::new(0x64, 0x45, 0x36));

    let bottom_corner = Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let top_corner = Point::new(0, 0);

    for i in 0..SCREEN_HEIGHT * 3 {
        manager.clear();

        Rectangle::with_corners(
            Point::new(0, SCREEN_HEIGHT as i32 - ease_out_cubic(i as f32 / 2.0, SCREEN_HEIGHT as f32) as i32), 
            bottom_corner
        )
            .into_styled(box_1_style)
            .draw(manager.get_canvas())
            .unwrap();

        Rectangle::with_corners(
            Point::new(0, SCREEN_HEIGHT as i32 - ease_out_cubic((i as f32 / 2.0) - 10.0, SCREEN_HEIGHT as f32) as i32), 
            bottom_corner
        )
            .into_styled(box_2_style)
            .draw(manager.get_canvas())
            .unwrap();

        Rectangle::with_corners(
            Point::new(0, SCREEN_HEIGHT as i32 - ease_out_cubic((i as f32 / 2.0) - 20.0, SCREEN_HEIGHT as f32) as i32), 
            bottom_corner
        )
            .into_styled(box_3_style)
            .draw(manager.get_canvas())
            .unwrap();

        character_style.text_color = Some(get_brightness(character_style_target_color, max(min(i - 40, 0) as f32 / (SCREEN_HEIGHT * 3) as f32, 1.0)));
        TextBox::with_textbox_style(
            "WMATA Metrorail Arrival Sign\nby Noah Gearhart\nfor Dark Wolf Solutions",
            Rectangle::with_corners(
                top_corner,
                bottom_corner
            ),
            character_style,
            centered_textbox_style,
        )
        .draw(manager.get_canvas())
        .unwrap();

        manager.run_updates_should_exit();
        tokio::time::sleep(Duration::from_nanos(100)).await;
    }
    
    tokio::time::sleep(Duration::from_secs(5)).await;

    
    for i in 0..SCREEN_HEIGHT * 3 {
        manager.clear();

        Rectangle::with_corners(
            top_corner, 
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 - ease_out_cubic((i as f32 / 2.0)  - 20.0, SCREEN_HEIGHT as f32) as i32)
        )
            .into_styled(box_1_style)
            .draw(manager.get_canvas())
            .unwrap();

        Rectangle::with_corners(
            top_corner, 
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 - ease_out_cubic((i as f32 / 2.0) - 10.0, SCREEN_HEIGHT as f32) as i32)
        )
            .into_styled(box_2_style)
            .draw(manager.get_canvas())
            .unwrap();
    
        Rectangle::with_corners(
            top_corner, 
            Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 - ease_out_cubic(i as f32 / 2.0, SCREEN_HEIGHT as f32) as i32)
        )
            .into_styled(box_3_style)
            .draw(manager.get_canvas())
            .unwrap();

        manager.run_updates_should_exit();
        tokio::time::sleep(Duration::from_nanos(100)).await;
    }
}

pub async fn check_for_network(manager: &mut ScreenManager) {
    manager.clear();
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
        MonoTextStyle::new(&FONT_6X10, Rgb888::new(0x84, 0xD2, 0xF6)),
        centered_textbox_style,
    )
    .draw(manager.get_canvas())
    .unwrap();

    manager.run_updates_should_exit();

    tokio::time::sleep(Duration::from_secs(2)).await;
    loop {
        let networks = Networks::new_with_refreshed_list();
        let discovered_ip = networks.iter().flat_map(|iface|
            iface.1.ip_networks().iter()
        ).find(|ip_addr| ip_addr.addr.is_ipv4() && !ip_addr.addr.is_loopback() && !ip_addr.addr.to_string().starts_with("172"));
        if discovered_ip.is_some() {
            info!(target: "startup", "IP Address: {}", discovered_ip.unwrap().addr);
            
            manager.clear();
            TextBox::with_textbox_style(
                "Connected to\nNetwork",
                Rectangle::with_corners(
                    Point::new(0, 0),
                    Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 / 2),
                ),
                MonoTextStyle::new(&FONT_6X10, Rgb888::new(0x84, 0xD2, 0xF6)),
                centered_textbox_style,
            )
            .draw(manager.get_canvas())
            .unwrap();

            TextBox::with_textbox_style(
                &format!("IP Address:\n{}", discovered_ip.unwrap().addr),
                Rectangle::with_corners(
                    Point::new(0, SCREEN_HEIGHT as i32 / 2),
                    Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32),
                ),
                MonoTextStyle::new(&FONT_6X10, Rgb888::new(0xD7, 0xB3, 0x77)),
                centered_textbox_style,
            )
            .draw(manager.get_canvas())
            .unwrap();
        
            manager.run_updates_should_exit();
            tokio::time::sleep(Duration::from_secs(5)).await;
            break;
        } else {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}
