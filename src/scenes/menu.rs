use ggez;
use ggez::graphics::{draw, DrawParam, Font, Text};
use ggez_goodies::scene;
use log::*;

use crate::input;
use crate::world::World;
use crate::scenes;
use crate::types::*;
use std::collections::BTreeMap;

pub struct MenuScene {
    texts: BTreeMap<&'static str, Text>,
    done: bool,
}

impl MenuScene {
    pub fn new(ctx: &mut ggez::Context, _world: &mut World) -> Self {
        let font = Font::new(ctx, "/fonts/DejaVuSerif.ttf").unwrap();
        let input_text = Text::new(("Press Any Key to Start", font, 20.0));
        let title_text = Text::new(("Main Menu", font, 48.0));
        let mut texts = BTreeMap::new();
        texts.insert("start_button", input_text.clone());
        texts.insert("title_text", title_text.clone());

        let done = false;
        MenuScene {
            texts,
            done,
        }
    }
}

impl scene::Scene<World, input::Event> for MenuScene {
    fn update(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> scenes::Switch {
        if self.done {
            self.done = false;
            scene::SceneSwitch::Push(Box::new(scenes::level::LevelScene::new(ctx, gameworld)))
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, _gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        for (_key, text) in &self.texts {
            draw(
                ctx,
                text,
                DrawParam::default().dest(Point2::new(200.0,300.0)),
            )?;
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "MenuScene"
    }

    fn input(&mut self, gameworld: &mut World, _ev: input::Event, _started: bool) {
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    }
}
