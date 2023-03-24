use druid::widget::{Button, Column, Container, Label, Radio, Row};
use druid::widget::{Flex, Padding};
use druid::{AppLauncher, Color, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::im::Vector;
use std::sync::Arc;
use crate::game_data::AppState;

pub struct ColorPreview {
    pieces : Vector<BoardPiece>,
}

impl ColorPreview {
    pub fn new(color : &Color) -> Board {
        ColorPreview {
            color,
        }
    }
 }

 impl Widget<AppState> for ColorPreview {

 }

