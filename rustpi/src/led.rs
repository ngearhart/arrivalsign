

use embedded_graphics::prelude::RgbColor;
#[cfg(feature = "rpi")]
use rpi_led_panel::{RGBMatrixConfig, RGBMatrix, Canvas};
#[cfg(feature = "rpi")]
use std::fmt::Debug;

#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorEvent, Window, SimulatorDisplay};
#[cfg(feature = "simulator")]
use embedded_graphics::{pixelcolor::Rgb888, prelude::{DrawTarget, Size}};

use crate::widgets::{SCREEN_HEIGHT, SCREEN_WIDTH};

// Change depending on your monitor resolution.
#[cfg(feature = "simulator")]
const WINDOW_SCALING: u32 = 8;


pub trait DrawableScreen
{
    fn clear(&mut self);
    fn run_updates_should_exit(&mut self) -> bool;
    fn init() -> Self;
}

#[cfg(feature = "rpi")]
pub struct ScreenManager {
    matrix: RGBMatrix,
    pub canvas: Box<Canvas>
}

#[cfg(feature = "rpi")]
impl DrawableScreen for ScreenManager {

    fn run_updates(&mut self) {
        self.canvas.fill(0, 0, 0);
        self.canvas = self.matrix.update_on_vsync(self.canvas);
    }

    fn init() -> Self {
        let config = RGBMatrixConfig::default();
        let (matrix, canvas) = RGBMatrix::new(config, 0).expect("Matrix initialization failed");
        ScreenManager {
            matrix,
            canvas
        }
    }
}

#[cfg(feature = "simulator")]
pub struct ScreenManager {
    window: Window,
    pub canvas: SimulatorDisplay<Rgb888>
}

#[cfg(feature = "simulator")]
impl DrawableScreen for ScreenManager {

    fn clear(&mut self) {
        self.canvas.clear(Rgb888::BLACK).unwrap();
    }

    fn run_updates_should_exit(&mut self) -> bool {
        self.window.update(&self.canvas);

        self.window.events().any(|e| e == SimulatorEvent::Quit)
    }

    fn init() -> Self {
        let output_settings = OutputSettingsBuilder::new().scale(WINDOW_SCALING).build();
        ScreenManager {
            canvas: SimulatorDisplay::<Rgb888>::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
            window: Window::new("Metro Sign Simulator", &output_settings)
        }
    }
}
