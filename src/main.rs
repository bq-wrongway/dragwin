mod utils;

use iced::Length::{Fill, Shrink};
use iced::border::rounded;
use iced::widget::button::Style;
use iced::widget::canvas::{Geometry, Stroke};
use iced::widget::image::Handle;
use iced::widget::{Image, button, canvas, center, center_x, container, stack, text, tooltip};
use iced::{Border, Color, Point, Rectangle, Renderer, Task, Theme, Vector, mouse, theme, window};
use iced::{Element, Font};
use image::{DynamicImage, open};
use std::fmt::Debug;
use utils::dragwin;
pub fn main() -> iced::Result {
    iced::application(Example::new, Example::update, Example::view)
        .font(include_bytes!("../fonts/dragwin.ttf").as_slice())
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
#[derive(Debug)]
struct Example {
    gaus_blue: f32,
    img: DynamicImage,
    state: State,
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
                gaus_blue: 0.0,
                img: DynamicImage::ImageRgba8(rgba),
                state: State {
                    system_cache: canvas::Cache::default(),
                },
            },
            Task::none(),
        )
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
    icon('\u{0e800}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("dragwin");

    text(codepoint).font(ICON_FONT).into()
}
#[derive(Debug)]
struct State {
    system_cache: canvas::Cache,
}
impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let system = self.system_cache.draw(renderer, bounds.size(), |frame| {
            frame.stroke_rectangle(
                Point { x: 30., y: 40. },
                iced::Size {
                    width: 200.,
                    height: 200.,
                },
                Stroke {
                    width: 3.0,
                    style: canvas::Style::Solid(Color::from_rgba(0.9, 0.3, 0.2, 1.)),
                    ..Default::default()
                },
            );
        });

        vec![system]
    }
}
