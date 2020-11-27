use ggez::{self, GameResult};
use ggez::event::{KeyCode};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Text, TextFragment};
use ggez_goodies::scene;
use log::*;
use specs::{self};
use warmy;

use crate::input;
use crate::scenes;
use crate::types::Point2;
use crate::world::World;

pub const WINDOW_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const TEXT_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

const INPUT_MAX_CHAR: usize = 30;

struct RectDim {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl RectDim {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        RectDim { x, y, w, h }
    }
}

pub struct UserInputScene {
    done: bool,
    font: Font,
    text_input_rendered: bool,
    input_text: String,
}

impl UserInputScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let font = Font::new(ctx, "/fonts/DejaVuSerif.ttf").unwrap(); 

        let text_input_rendered = false;
        let mut input_text = "".to_string();

        UserInputScene {
            done,
            font,
            input_text,
            text_input_rendered,
        }
    }
}

impl scene::Scene<World, input::Event> for UserInputScene {
    fn update(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> scenes::Switch {
        if self.done {
            self.done = false;
            scene::SceneSwitch::Push(Box::new(scenes::menu::MenuScene::new(ctx, gameworld)))
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> GameResult<()> {
        let (drawable_width, drawable_height) = graphics::drawable_size(ctx);
        graphics::clear(ctx, graphics::BLACK);
        let rd = RectDim::new(drawable_width / 4.0, drawable_height / 3.3, drawable_width / 2.0, drawable_height / 3.3);

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(rd.x, rd.y, rd.w, rd.h),
            WINDOW_COLOR,
            )?;

        let input_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(rd.x + 30.0, rd.y + (rd.h - 60.0), rd.w - 60.0, 30.0),
            Color::from((50, 50, 50, 255)),
            )?;

        let text = Text::new(TextFragment {
            text: (self.input_text).to_string(),
            color: Some(TEXT_COLOR),
            font: Some(self.font),
            ..Default::default()
        });

        graphics::draw(ctx, &rect, (Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &input_rect, (Point2::new(0.0, 0.0),))?;
        graphics::draw(
            ctx,
            &text,
            DrawParam::default().dest(Point2::new(rd.x + 38.0, rd.y + (rd.h - 54.0)),),
            )?;
        Ok(())
    }
    
    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        if gameworld.input.get_button_pressed(input::Button::Delete) {
            let mut next_str = self.input_text.to_owned();
            next_str.pop();
            self.input_text = next_str;
        }
        if gameworld.input.get_button_pressed(input::Button::Confirm) {
            println!("enter pushed");
        }
    }

    fn name(&self) -> &str {
        "UserInputScene"
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, _character: char) {
        println!("delete is pressed: {}", ggez::input::keyboard::is_key_pressed(_ctx, KeyCode::Back));
        match ggez::input::keyboard::is_key_pressed(_ctx, KeyCode::Back) {
            true => (),
            false => {
                let mut next_str = self.input_text.to_owned();
                if next_str.chars().count() < INPUT_MAX_CHAR {
                    next_str.push_str(&_character.to_string());
                    self.input_text = next_str;
                }
            }
        }
    }
}
