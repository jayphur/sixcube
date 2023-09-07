use std::{thread, time::Duration};

use kiss3d::window::Window;
use sc_core::display::world::WorldDisplay;
use sc_prelude::*;

pub fn gfx_loop(window: Window, world: WorldDisplay) -> Result<()>{
    loop {
        thread::sleep(Duration::from_secs(4)); // "hard at work"
        thread::sleep(Duration::from_secs(4)); // "hard at work"
        return Err(anyhow!("test error"));
    }
}