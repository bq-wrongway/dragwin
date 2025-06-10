mod utils;

use iced::Length::{Fill, Shrink};
use iced::border::rounded;
use iced::keyboard::{Modifiers, key};
use iced::widget::button::Style;
use iced::widget::image::Handle;
use iced::widget::{Image, button, canvas, center, center_x, container, stack, text, tooltip};
use iced::{
    Border, Color, Point, Rectangle, Renderer, Subscription, Task, Theme, Vector, keyboard, mouse,
    theme, window,
};
use iced::{Element, Font};
use image::{DynamicImage, open};
use std::fmt::Debug;
use utils::dragwin;
use utils::draw::DrawState;
use utils::mode::Mode;
use utils::state::State;
pub fn main() -> iced::Result {
    iced::application(Example::new, Example::update, Example::view)
        .font(include_bytes!("../fonts/dragwin.ttf").as_slice())
        .window(window::Settings {
            transparent: true,
            decorations: false,

            ..Default::default()
        })
        .theme(|_| Theme::CatppuccinMocha)
        .subscription(Example::subscription)
        .style(|_, _| theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .run()
}
#[derive(Debug)]
struct Example {
    gaus_blue: f32,
    img: DynamicImage,
    state: utils::state::State,
}
#[derive(Debug, Clone)]
pub enum Message {
    DragWin(utils::dragwin::Message),
    Undo,
}
impl Example {
    pub fn subscription(&self) -> Subscription<Message> {
        let app_key_listener = keyboard::on_key_press(|key, modifiers| match key {
            // key::Key::Named(named) => match named {
            //     key::Named::Escape => Some(Message::Cancel),
            //     key::Named::Enter => Some(Message::Done),
            //     _ => None,
            // },
            key::Key::Character(char) => match char.as_str() {
                "z" if modifiers == Modifiers::CTRL => Some(Message::Undo),
                _ => None,
            },
            _ => None,
        });

        Subscription::batch([app_key_listener])
    }
    fn new() -> (Self, Task<Message>) {
        //path to image
        let rgba = open("/home/melnibone/Downloads/tux.png")
            .unwrap()
            .into_rgba8();
        (
            Self {
                gaus_blue: 0.0,
                img: DynamicImage::ImageRgba8(rgba),
                state: State {
                    cache: canvas::Cache::default(),
                    shapes: Vec::new(),
                    mode: Mode::default(),
                    cursor_position: Point::ORIGIN,
                    scale_factor: 1.0,
                    windows: Vec::new(),
                },
            },
            Task::none(),
        )
    }
    fn push_shape(&mut self) {
        if let Mode::Draw {
            element: shape,
            state: status,
        } = &mut self.state.mode
        {
            if shape.tool.is_valid() {
                self.state.shapes.push(shape.clone());
            }
            shape.tool.reset();
            *status = DrawState::Idle;
        }
        self.state.cache.clear();
    }

    fn rotate_img(&mut self) {
        // self.img = self.img.huerotate(90);
        self.img = self.img.rotate90()
    }
    fn blur_img(&mut self, g: f32) {
        self.gaus_blue = g;
        self.img = self.img.huerotate(self.gaus_blue as i32)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWin(m) => utils::dragwin::update(m, self).map(Message::DragWin),
            Message::Undo => {
                if self.state.mode.is_draw_mode() {
                    self.state.shapes.pop();
                    self.state.cache.clear();
                }
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let handle = Handle::from_rgba(
            self.img.width(),
            self.img.height(),
            self.img.as_bytes().to_owned(),
        );
        utils::dragwin::view(
            center(
                stack!(Image::new(handle).expand(true))
                    .push(canvas(&self.state).width(Fill).height(Fill))
                    .width(Shrink)
                    .height(Shrink),
            )
            .style(|t| iced::widget::container::Style {
                border: Border {
                    color: Color::WHITE,
                    width: 2.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .center_x(Fill)
            .width(Fill)
            .height(Fill)
            .into(),
            //so calling on press with message from main.rs does not work since function takes message
            // from dragwin, but changing that also does not work
            // button("R").on_press(Message::RotateImage).into(),
            action(new_icon(), "rotate", Some(dragwin::Message::RotateImage)),
            self,
        )
        .map(Message::DragWin)
    }
}
fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(center_x(content).width(20).height(20).padding(0)).style(|t, s| match s {
        button::Status::Active => Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: rounded(4),
            text_color: t.extended_palette().secondary.strong.text.inverse(),
            ..Default::default()
        },
        button::Status::Hovered => Style {
            background: Some(iced::Background::Color(
                t.extended_palette().background.strongest.color,
            )),
            border: rounded(4),
            text_color: t.extended_palette().secondary.strong.text.inverse(),
            ..Default::default()
        },
        button::Status::Pressed => Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: rounded(4),
            text_color: t.extended_palette().secondary.strong.text,
            ..Default::default()
        },
        button::Status::Disabled => Style {
            background: Some(iced::Background::Color(Color::BLACK)),
            border: rounded(4),
            text_color: t.extended_palette().secondary.strong.text,
            ..Default::default()
        },
    });

    if let Some(on_press) = on_press {
        tooltip(action.on_press(on_press), label, tooltip::Position::Bottom)
            .style(container::rounded_box)
            .into()
    } else {
        action.style(button::secondary).into()
    }
}

fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0E801}')
}
fn maximize<'a, Message>() -> Element<'a, Message> {
    icon('\u{0E802}')
}
fn close<'a, Message>() -> Element<'a, Message> {
    icon('\u{0E803}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("dragwin");

    text(codepoint).font(ICON_FONT).into()
}
