use std::thread::{self, JoinHandle};

use kiss3d::window::Window;
use sc_core::display::world::WorldDisplay;
use sc_core::obj::world::World;
use sc_prelude::*;
use sc_prelude::sync::RwLock;
mod gfx;

pub struct SoloInstance{
    world: RwLock<World>,
}
impl SoloInstance{
    pub fn new(world: World) -> Self{
        Self { 
            world: RwLock::new(world) 
        }
    }
    pub fn run(mut self) -> Result<()>{
        let gfx = self.spawn_gfx()?;
        
        Ok(())
    }
    // ///////////////////////////////////

    fn spawn_gfx(&mut self) -> Result<JoinHandle<Result<()>>>{
        use thread::Builder;
        let display = WorldDisplay::new(self.world.write());
        Ok(
            Builder::new()
            .name("Graphics".into())
            .spawn(move || {
                    let display = display;
                    gfx::gfx_loop(Window::new("title bruh"))
                }
            )
        ?)
    }    
}
