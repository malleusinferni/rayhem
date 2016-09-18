use sdl2::{self, Sdl, EventPump};
use specs::{self, Planner};

use system::*;

pub fn new<'r>() -> Engine<'r> {
    //if cfg!(target_os = "macos") {
    //    sdl2::hint::set("SDL_VIDEO_MAC_FULLSCREEN_SPACES", "0");
    //}

    let sdl = sdl2::init().unwrap();

    let window = sdl.video().unwrap()
        .window("Rayhem", 640, 480)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .unwrap();

    let renderer = window.renderer()
        .build()
        .unwrap();

    let mut planner = {
        use component::*;
        use resource::*;

        let mut world = specs::World::new();
        world.register::<Pos3D>();
        world.register::<Sprite3D>();
        world.register::<Billboard>();
        world.register::<IsPlayer>();
        world.add_resource(Camera3D::new(Vec2u::new(640, 480)));
        world.add_resource(LevelMap::new());
        Planner::new(world, 4)
    };

    let event_pump = sdl.event_pump().unwrap();

    let (display_agent, display_sys) = DisplaySys::new(renderer);

    planner.add_system(display_sys, "Display", 1);

    let ctx = Ctx::new();

    Engine {
        sdl: sdl,
        ctx: ctx,
        input: event_pump,
        planner: planner,
        display: display_agent,
    }
}

#[derive(Clone)]
pub struct Ctx {
    // Nothing
}

impl Ctx {
    fn new() -> Self { Ctx { } }

    fn update(&mut self) {
        // Do nothing
    }
}

pub struct Engine<'r> {
    sdl: Sdl,
    ctx: Ctx,
    input: EventPump,
    planner: Planner<Ctx>,
    display: DisplayAgent<'r>,
}

impl<'r> Engine<'r> {
    pub fn run(&mut self) {
        for _ in 0 .. 180 {
            self.ctx.update();
            let _events = self.input.poll_iter().count();
            self.planner.dispatch(self.ctx.clone());
            self.display.draw();
        }
    }
}
