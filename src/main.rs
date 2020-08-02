pub mod app;
pub mod blob;
pub mod config;
pub mod ecs;
pub mod math;
pub mod world;

use app::App;
use config::Config;
use world::World;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

const BLOB_COUNT: usize = 10;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = Config::load_default_config_file()?;

    let mut world = World {
        width: WIDTH as usize,
        height: HEIGHT as usize,
        config,
        blobs: Vec::new(),
    };
    world.add_random_blobs(BLOB_COUNT);

    let app = App::new(world);

    app.run()
}
