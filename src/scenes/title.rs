use ggez;
use ggez::graphics::{draw, Drawable, DrawParam, Font, Text};
use ggez_goodies::scene;
use log::*;

use crate::input;
use crate::world::World;
use crate::scenes;
use crate::types::*;
use std::collections::BTreeMap;

struct TextButton {
    point: Point2,
    text: Text,
}

pub struct TitleScene {
    texts: BTreeMap<&'static str, TextButton>, 
    done: bool,
}

impl TitleScene {
    pub fn new(ctx: &mut ggez::Context, _world: &mut World) -> Self {
        let font = Font::new(ctx, "/fonts/DejaVuSerif.ttf").unwrap(); 
        let start_game_button = Text::new(("Start Game", font, 20.0));
        let continue_button = Text::new(("Continue Game", font, 20.0));
        let mut texts = BTreeMap::new();
        texts.insert("start_button", TextButton{text: start_game_button.clone(), point: Point2::new(200.0, 300.0)});
        texts.insert("continue_button", TextButton{text: continue_button.clone(), point: Point2::new(200.0, 400.0)});

        let done = false;
        TitleScene {
            texts,
            done,
        }
    }
}

impl scene::Scene<World, input::Event> for TitleScene {
    fn update(&mut self, _gameworld: &mut World, ctx: &mut ggez::Context) -> scenes::Switch {
        if self.done {
            self.done = false;
            scene::SceneSwitch::Push(Box::new(scenes::level::LevelScene::new(ctx, _gameworld)))
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, _gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        for (_key, btn) in &self.texts {
            draw(
                ctx,
                &btn.text,
                DrawParam::default().dest(btn.point),
            )?;
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "TitleScene"
    }

    fn input(&mut self, _gameworld: &mut World, _ev: input::Event, _started: bool) {
        if _gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    } 
}
