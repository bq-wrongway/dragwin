mod utils;

use iced::Length::Fill;
use iced::mouse::Interaction;
use iced::widget::container::{self, Style};
use iced::widget::{button, mouse_area, row, text};
use iced::window::drag_resize;
use iced::{Background, Element};
use iced::{Color, Task, Theme, theme, window};
use std::fmt::Debug;
use utils::dragwin::view;
pub fn main() -> iced::Result {
    iced::application(Example::new, Example::update, Example::view)
        .window(window::Settings {
            transparent: true,
            decorations: false,

            ..Default::default()
        })
        .theme(|_| Theme::CatppuccinMocha)
        .style(|_, _| theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .run()
}
#[derive(Default)]
struct Example {
    value: u32,
}
#[derive(Debug, Clone)]
pub enum Message {
    Shakira(utils::dragwin::Message),
}
impl Example {
    fn new() -> (Self, Task<Message>) {
        (Self { value: 9 }, Task::none())
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Shakira(m) => {
                match m {
                    utils::dragwin::Message::Drag => todo!(),
                    utils::dragwin::Message::Maximize => todo!(),
                    utils::dragwin::Message::NorthWest => todo!(),
                    utils::dragwin::Message::North => todo!(),
                    utils::dragwin::Message::NorthEast => todo!(),
                    utils::dragwin::Message::West => todo!(),
                    utils::dragwin::Message::East => todo!(),
                    utils::dragwin::Message::South => todo!(),
                    utils::dragwin::Message::SouthWest => todo!(),
                    utils::dragwin::Message::SouthEast => todo!(),
                    utils::dragwin::Message::Close => todo!(),
                }
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<Message> {
        utils::dragwin::view().map(|_f| Message::Shakira)
    }
}
