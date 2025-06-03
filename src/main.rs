mod utils;

use iced::Element;
use iced::Length::Fill;
use iced::widget::text;
use iced::{Color, Task, Theme, theme, window};
use std::fmt::Debug;
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
    DragWin(utils::dragwin::Message),
}
impl Example {
    fn new() -> (Self, Task<Message>) {
        (Self { value: 9 }, Task::none())
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWin(m) => utils::dragwin::update(m).map(Message::DragWin),
        }
    }
    fn view(&self) -> Element<Message> {
        utils::dragwin::view(
            text("a").height(Fill).width(Fill).center().into(),
            text("asda").into(),
        )
        .map(Message::DragWin)
    }
}
