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
    menu_idx: i32,
    next_scene: &'static str,
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

        let mut menu_idx = 0;
        let mut next_scene = "";
        let done = false;
        TitleScene {
            menu_idx,
            next_scene,
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
        let vert_axis = _gameworld.input.get_axis_raw(input::Axis::Vert);
        
        match _gameworld.input.get_axis_raw(input::Axis::Vert) {
            1.0 => println!("up was pressed"),
            -1.0 => println!("down was pressed"),
            _ => (),
        }

        if _gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    } 
}
