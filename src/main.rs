mod utils;

use iced::Element;
use iced::Length::Fill;
use iced::widget::image::Handle;
use iced::widget::pop::Key;
use iced::widget::{Image, button, center, text};
use iced::{Color, Task, Theme, theme, window};
use image::{DynamicImage, RgbaImage, open};
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
    img: DynamicImage,
}
#[derive(Debug, Clone)]
pub enum Message {
    DragWin(utils::dragwin::Message),
}
impl Example {
    fn new() -> (Self, Task<Message>) {
        //path to image
        let rgba = open("/home/melnibone/Downloads/tux.png")
            .unwrap()
            .into_rgba8();
        (
            Self {
                value: 9,
                img: DynamicImage::ImageRgba8(rgba),
            },
            Task::none(),
        )
    }

    fn rotate_img(&mut self) {
        self.img = self.img.rotate180()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWin(m) => utils::dragwin::update(m, self).map(Message::DragWin),
        }
    }
    fn view(&self) -> Element<Message> {
        let handle = Handle::from_rgba(
            self.img.width(),
            self.img.height(),
            self.img.as_bytes().to_owned(),
        );
        utils::dragwin::view(
            center(Image::new(handle).expand(true)).into(),
            //so calling on press with message from main.rs does not work since function takes message
            // from dragwin, but changing that also does not work
            // button("R").on_press(Message::RotateImage).into(),
            text("Asd").into(),
        )
        .map(Message::DragWin)
    }
}
