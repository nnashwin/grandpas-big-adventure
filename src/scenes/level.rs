use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use log::*;
use specs::{self};
use warmy;
use ggez::nalgebra::Point2;

use crate::input;
use crate::scenes;
use crate::world::World;

pub struct LevelScene {
    done: bool,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;

        LevelScene {
            done,
        }
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> scenes::Switch {
        if self.done {
            self.done = false;
            scene::SceneSwitch::Push(Box::new(scenes::menu::MenuScene::new(ctx, gameworld)))
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        debug!("Input: {:?}", ev);
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, _character: char) {
        println!("Text input character: {}",_character);
    }
}
