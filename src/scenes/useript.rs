use ggez::{self, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Text, TextFragment};
use ggez_goodies::scene;
use log::*;
use specs::{self};
use warmy;

use crate::input;
use crate::scenes;
use crate::types::Point2;
use crate::world::World;

#[derive(Debug)]
struct TextBox {
    point: ggez::nalgebra::Point2<f32>,
    text: Text,
}

pub struct UserInputScene {
    done: bool,
    font: Font,
    text_input_rendered: bool,
    text_input: TextBox,
}

impl UserInputScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let font = Font::new(ctx, "/fonts/DejaVuSerif.ttf").unwrap(); 

        let text_input_rendered = false;
        let mut text = Text::new(TextFragment{
            text: "jkasjkjafksdjfkdsajfkjasdkfj".to_string(),
            color: Some(graphics::BLACK),
            font: Some(font),
            ..Default::default()
        });

        let text_input = TextBox{point: Point2::new(200.0, 300.0), text: text};

        UserInputScene {
            done,
            font,
            text_input,
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
        let (width, height) = graphics::drawable_size(ctx);
        graphics::clear(ctx, graphics::BLACK);
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(width / 4.0, height / 3.3, width / 2.0, height / 3.3),
            graphics::WHITE,
            )?;

        graphics::draw(ctx, &rect, (Point2::new(0.0, 0.0),))?;
        graphics::draw(
            ctx,
            &self.text_input.text,
            DrawParam::default().dest(self.text_input.point),
            )?;
        Ok(())
    }
    
    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        if gameworld.input.get_button_pressed(input::Button::Confirm) {
            println!("enter pushed")
        }
    }

    fn name(&self) -> &str {
        "UserInputScene"
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, _character: char) {
        let mut next_str = self.text_input.text.contents().to_owned();
        next_str.push_str(&_character.to_string());

        self.text_input.text = Text::new(TextFragment {
            text: next_str,
            color: Some(graphics::BLACK),
            font: Some(self.font),
            ..Default::default()
        });
    }
}
