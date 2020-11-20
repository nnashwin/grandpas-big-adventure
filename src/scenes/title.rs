use ggez;
use ggez::graphics::{draw, Color, DrawParam, Font, Text};
use ggez_goodies::scene;
use log::*;

use crate::input;
use crate::world::World;
use crate::scenes;
use crate::types::*;
use std::collections::BTreeMap;

const BTN_ARR: &'static [&'static str; 2] = &["start_button", "continue_button"];

#[derive(Debug)]
struct TextButton {
    point: Point2,
    text: Text,
}

pub struct TitleScene {
    menu_idx: usize,
    next_scene: &'static str,
    selected_color: Color,
    texts: BTreeMap<&'static str, TextButton>, 
    done: bool,
}


impl TitleScene {
    pub fn new(ctx: &mut ggez::Context, _world: &mut World) -> Self {
        let font = Font::new(ctx, "/fonts/DejaVuSerif.ttf").unwrap(); 
        let mut start_game_button = Text::new(("Start Game", font, 20.0));
        let mut continue_button = Text::new(("Continue Game", font, 20.0));
        let mut texts = BTreeMap::new();
        texts.insert("start_button", TextButton{text: start_game_button.clone(), point: Point2::new(200.0, 300.0)});
        texts.insert("continue_button", TextButton{text: continue_button.clone(), point: Point2::new(200.0, 400.0)});


        let selected_color = Color::from_rgba(128, 0, 128, 1);

        let mut menu_idx = 0;
        let mut next_scene = "";
        let done = false;
        TitleScene {
            menu_idx,
            next_scene,
            selected_color,
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
        let selected_key = BTN_ARR[self.menu_idx];
        println!("key selected is: {}", selected_key);
        for (_key, btn) in &self.texts {
            if *_key == "start_button" {
                draw(
                    ctx,
                    &btn.text,
                    DrawParam::default().dest(btn.point),
                )?
            } else {
                draw(
                    ctx,
                    &btn.text,
                    DrawParam::default().dest(btn.point),
                    )?;
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "TitleScene"
    }

    fn input(&mut self, _gameworld: &mut World, _ev: input::Event, _started: bool) {
        let vert_axis = _gameworld.input.get_axis_raw(input::Axis::Vert);
        
        match _gameworld.input.get_axis_raw(input::Axis::Vert) {
            1.0 => {
                // self.selected_key
                println!("up was pressed");
                self.menu_idx = if self.menu_idx > 0 {
                    self.menu_idx - 1
                } else {
                    BTN_ARR.len() - 1
                }
            },
            -1.0 => { 
                self.menu_idx = (self.menu_idx + 1) % BTN_ARR.len();
            },
            _ => (),
        }

        if _gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    } 
}
