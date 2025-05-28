use iced::Length::Fill;
use iced::mouse::Interaction;
use iced::widget::container::Style;
use iced::widget::{button, column, container, mouse_area, row, text};
use iced::window::drag_resize;
use iced::{Background, Color, Task, Theme, theme, window};
use iced::{Element, Length};
use std::fmt::Debug;

pub fn main() -> iced::Result {
    iced::application(Example::new, Example::update, Example::view)
        .window(window::Settings {
            transparent: true,
            decorations: false,

            ..Default::default()
        })
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
enum Message {
    Drag,
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    South,
    SouthWest,
    SouthEast,
    Close,
}

impl Example {
    fn new() -> (Self, Task<Message>) {
        (Self { value: 9 }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Drag => {
                self.value += 1;
                window::get_latest().and_then(window::drag)
                // Task::none()
            }
            Message::NorthWest => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::NorthWest))
            }
            Message::North => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::North))
            }
            Message::NorthEast => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::NorthEast))
            }
            Message::West => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::West))
            }
            Message::East => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::East))
            }
            Message::South => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::South))
            }
            Message::SouthWest => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::SouthWest))
            }
            Message::SouthEast => {
                window::get_latest().and_then(|f| drag_resize(f, window::Direction::SouthEast))
            }
            Message::Close => window::get_latest().and_then(window::close),
        }
    }

    fn view(&self) -> Element<Message> {
        let bottom_row = row![
            mouse_area(
                container(row![])
                    .width(5)
                    .height(2)
                    .style(|_| border_container())
            )
            .on_press(Message::SouthWest)
            .interaction(Interaction::ResizingDiagonallyUp),
            mouse_area(
                container(row![])
                    .width(Length::Fill)
                    .height(2)
                    .style(|_| border_container())
            )
            .on_press(Message::South)
            .interaction(Interaction::ResizingVertically),
            mouse_area(
                container(row![])
                    .width(5)
                    .height(2)
                    .style(|_| border_container())
            )
            .on_press(Message::SouthEast)
            .interaction(Interaction::ResizingDiagonallyDown),
        ];

        let top_row = row![
            mouse_area(
                container(row![])
                    .width(5)
                    .height(2)
                    .style(|_| border_container())
            )
            .on_press(Message::NorthWest)
            .interaction(Interaction::ResizingDiagonallyDown),
            mouse_area(
                container(row![])
                    .width(Length::Fill)
                    .height(2)
                    .style(|_| border_container())
            )
            .on_press(Message::North)
            .interaction(Interaction::ResizingVertically),
            mouse_area(
                container(row![])
                    .width(5)
                    .height(2)
                    .style(|_| border_container())
            )
            .on_press(Message::NorthEast)
            .interaction(Interaction::ResizingDiagonallyUp),
        ];

        container(column![
            top_row,
            container(
                row![
                    mouse_area(
                        container(row![])
                            .width(2)
                            .height(Fill)
                            .style(|_| border_container())
                    )
                    .on_press(Message::West)
                    .interaction(Interaction::ResizingHorizontally),
                    container(column![
                        mouse_area(
                            container(
                                row![
                                    button(text("x").center())
                                        .padding(0)
                                        .width(20)
                                        .on_press(Message::Close)
                                ]
                                .padding(4)
                            )
                            .style(|t: &Theme| container::Style {
                                background: Some(Background::Color(t.palette().success)),
                                border: iced::Border {
                                    color: t.palette().warning,
                                    width: 2.0,
                                    radius: 8.into()
                                },
                                ..Default::default()
                            })
                            .width(Length::Fill)
                            .height(30)
                        )
                        .on_press(Message::Drag),
                    ])
                    // .padding(10)
                    .style(|t: &Theme| Style {
                        background: Some(Background::Color(t.palette().primary)),
                        border: iced::Border {
                            color: t.palette().danger,
                            width: 2.,
                            radius: 8.into(),
                        },
                        ..Default::default()
                    })
                    .width(Fill)
                    .height(Fill),
                    mouse_area(
                        container(row![])
                            .width(2)
                            .height(Fill)
                            .style(|_| border_container())
                    )
                    .on_press(Message::East)
                    .interaction(Interaction::ResizingHorizontally),
                ]
                .width(Fill)
                .height(Fill)
            )
            .width(Fill)
            .height(Fill),
            bottom_row
        ])
        .width(Fill)
        .height(Fill)
        .into()
    }
}

fn border_container() -> Style {
    container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        ..Default::default()
    }
}
