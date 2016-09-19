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
        .present_vsync()
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

        world.create_now()
            .with(Pos3D::new(13.0, 5.0, 6.0, 90.0))
            .with(IsPlayer {})
            .build();

        Planner::new(world, 4)
    };

    let event_pump = sdl.event_pump().unwrap();

    let (display_agent, display_sys) = DisplaySys::new(renderer);

    planner.add_system(MovePlayer{}, "Input", 3);
    planner.add_system(MoveCamera{}, "Camera", 2);
    planner.add_system(display_sys, "Display", 1);

    let ctx = Ctx::new();

    Engine {
        sdl: sdl,
        ctx: ctx,
        event_pump: event_pump,
        planner: planner,
        display: display_agent,
    }
}

#[derive(Clone)]
pub struct Ctx {
    pub should_quit: bool,
    pub turning: Turning,
    pub walking: bool,
}

impl Ctx {
    fn new() -> Self {
        Ctx {
            should_quit: false,
            turning: Turning::Straight,
            walking: false,
        }
    }

    fn update(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit { .. } => { self.should_quit = true; },

                Event::KeyDown { keycode: Some(k), .. } => match k {
                    Keycode::Q => { self.should_quit = true; },

                    _ => (),
                },

                _ => (),
            }
        }

        use sdl2::keyboard::Scancode;

        let kb = event_pump.keyboard_state();

        self.turning = {
            let left = kb.is_scancode_pressed(Scancode::Left);
            let right = kb.is_scancode_pressed(Scancode::Right);
            match (left, right) {
                (true, false) => Turning::Left,
                (false, true) => Turning::Right,
                _ => Turning::Straight,
            }
        };

        self.walking = kb.is_scancode_pressed(Scancode::W);
    }
}

pub struct Engine<'r> {
    sdl: Sdl,
    ctx: Ctx,
    event_pump: EventPump,
    planner: Planner<Ctx>,
    display: DisplayAgent<'r>,
}

impl<'r> Engine<'r> {
    pub fn run(&mut self) {
        while !self.ctx.should_quit {
            self.ctx.update(&mut self.event_pump);
            self.planner.dispatch(self.ctx.clone());
            self.display.draw();
        }
    }
}
