use std::fmt;

use bevy::window::WindowMode;
use clap::{Parser, ValueEnum};
use doome_engine::{HEIGHT, WIDTH};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long)]
    width: Option<f32>,

    #[arg(long)]
    height: Option<f32>,

    #[arg(long, default_value_t = ArgsWindowMode::Windowed)]
    mode: ArgsWindowMode,
}

impl Args {
    #[cfg(target_arch = "wasm32")]
    pub fn get() -> Self {
        Self {
            width: None,
            height: None,
            mode: ArgsWindowMode::Windowed,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get() -> Self {
        Self::parse()
    }

    pub fn width(&self) -> f32 {
        self.width.unwrap_or(WIDTH as f32)
    }

    pub fn height(&self) -> f32 {
        self.height.unwrap_or(HEIGHT as f32)
    }

    pub fn mode(&self) -> WindowMode {
        match self.mode {
            ArgsWindowMode::Windowed => WindowMode::Windowed,
            ArgsWindowMode::Fullscreen => WindowMode::Fullscreen,
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ArgsWindowMode {
    Windowed,
    Fullscreen,
}

impl fmt::Display for ArgsWindowMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().unwrap().get_name().fmt(f)
    }
}
