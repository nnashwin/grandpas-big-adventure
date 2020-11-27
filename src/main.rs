use std::env;
use std::path;

use ggez::{self, *};
use ggez::nalgebra::Point2;


mod input;
mod resources;
mod scenes;
mod types;
mod util;
mod world;

struct WindowSettings {
    resize_projection: bool,
}

struct MainState {
    input_binding: input::Binding,
    scenes: scenes::Stack,
    window_settings: WindowSettings,
}

impl MainState {
    fn new(ctx: &mut Context, resource_path: &path::Path) -> Self {
        let world = world::World::new(resource_path);
        let mut scenestack = scenes::Stack::new(ctx, world);
        // let initial_scene = Box::new(scenes::title::TitleScene::new(ctx, &mut scenestack.world));
        let initial_scene = Box::new(scenes::useript::UserInputScene::new(ctx, &mut scenestack.world));
        scenestack.push(initial_scene);

        Self {
            input_binding: input::create_input_binding(),
            scenes: scenestack,
            window_settings: WindowSettings {
                resize_projection: false,
            },
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update(ctx);
        }
        self.scenes.world.resources.sync(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from((0.0, 0.0, 0.4, 0.0)));
        self.scenes.draw(ctx);

        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {}", fps));

        graphics::draw(
            ctx,
            &fps_display,
            (Point2::new(50.0, 550.0), graphics::WHITE),
        )?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
                self.scenes.input(ev, true);
                self.scenes.world.input.update_effect(ev, true);
            }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, false);
            self.scenes.world.input.update_effect(ev, false);
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        match graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height)) {
            Ok(()) => println!("Resized window to {} x {}", width, height),
            Err(e) => println!("Errored out and couldn't resize window {}", e),
        }
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, _character: char) {
        self.scenes.text_input_event(_ctx, _character);
    }
}

fn main() {
    util::setup_logging();

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    println!("Resource dir: {:?}", resource_dir);

    let cb = ContextBuilder::new("game-template", "ggez")
        .window_setup(conf::WindowSetup::default().title("game template"))
        .window_mode(conf::WindowMode::default()
                     .dimensions(800.0, 600.0)
                     .resizable(true))
        .add_resource_path(&resource_dir);
    let (ctx, ev) = &mut cb.build().unwrap();

    let state = &mut MainState::new(ctx, &resource_dir);
    if let Err(e) = event::run(ctx, ev, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
    
}
