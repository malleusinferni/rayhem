use sdl2::{self, Sdl, EventPump};
use specs::{self, Planner};

use display;

use input::*;
use movement::*;
use map::*;

pub fn new<'r>() -> Engine<'r> {
    let sdl = sdl2::init().unwrap();

    let window = sdl.video().unwrap()
        .window("Rayhem", 640, 480)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .unwrap();

    sdl.mouse().set_relative_mouse_mode(true);

    let renderer = window.renderer()
        .present_vsync()
        .build()
        .unwrap();

    let mut planner = {
        let mut world = specs::World::new();

        world.register::<Pos3D>();
        world.register::<Vel3D>();
        world.register::<IsPlayer>();

        world.add_resource(LevelMap::new());

        world.create_now()
            .with(Pos3D::new(13.0, 5.0, 6.0, 90.0))
            .with(Vel3D::new())
            .with(IsPlayer {})
            .build();

        Planner::new(world, 4)
    };

    let event_pump = sdl.event_pump().unwrap();

    let display_handler = ::display::init(&mut planner, renderer);

    planner.add_system(MovePlayer{}, "Input", 4);
    planner.add_system(ApplyVelocity{}, "Movement", 3);

    let ctx = Ctx::new();

    Engine {
        sdl: sdl,
        ctx: ctx,
        event_pump: event_pump,
        planner: planner,
        display: display_handler,
    }
}

#[derive(Clone)]
pub struct Ctx {
    pub should_quit: bool,
    pub turn_amount: f32,
    pub walking: bool,

    pub elapsed: f64,
    pub began: f64,
    pub dt: f64,
}

impl Ctx {
    fn new() -> Self {
        use time;

        Ctx {
            should_quit: false,
            turn_amount: 0.0,
            walking: false,

            elapsed: 0.0,
            began: time::precise_time_s(),
            dt: 0.0,
        }
    }

    fn update(&mut self, event_pump: &mut EventPump) {
        self.turn_amount = 0.0;

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit { .. } => { self.should_quit = true; },

                Event::KeyDown { keycode: Some(k), .. } => match k {
                    Keycode::Q => { self.should_quit = true; },

                    _ => (),
                },

                Event::MouseMotion { xrel, yrel, .. } => {
                    let _ = yrel; // TODO: Implement vlook
                    self.turn_amount += xrel as f32;
                },

                _ => (),
            }
        }

        use sdl2::keyboard::Scancode;

        let kb = event_pump.keyboard_state();

        self.walking = kb.is_scancode_pressed(Scancode::W);

        use time;

        let elapsed = time::precise_time_s() - self.began;
        self.dt = elapsed - self.elapsed;
        self.elapsed = elapsed;
    }
}

pub struct Engine<'r> {
    sdl: Sdl,
    ctx: Ctx,
    event_pump: EventPump,
    planner: Planner<Ctx>,
    display: display::Handler<'r>,
}

impl<'r> Engine<'r> {
    pub fn run(&mut self) {
        while !self.ctx.should_quit {
            self.ctx.update(&mut self.event_pump);
            self.planner.dispatch(self.ctx.clone());
            self.display.draw(self.planner.mut_world());
        }
    }
}
