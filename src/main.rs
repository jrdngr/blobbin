pub mod app;
pub mod blob;
pub mod math;
pub mod world;

use app::App;
use world::World;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const BLOB_SIZE: f64 = 5.0;

const BLOB_COUNT: usize = 100;

const REPEL_FORCE: f64 = 0.0001;
const REPEL_DISTANCE: f64 = 20.0;
const FRICTION_FORCE: f64 = 0.025;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut world = World {
        width: WIDTH as usize,
        height: HEIGHT as usize,
        blob_size: BLOB_SIZE,
        repel_force: REPEL_FORCE,
        repel_distance: REPEL_DISTANCE,
        friction_force: FRICTION_FORCE,
        blobs: Vec::new(),
    };
    world.add_random_blobs(BLOB_COUNT);

    let app = App::new(world);

    app.run()
}
