use iced::{
    Alignment::Center,
    Background, Color, Element,
    Length::{Fill, Shrink},
    Task, Theme, border,
    mouse::Interaction,
    widget::{
        container::{self, Style},
        horizontal_space, mouse_area, row, text,
    },
    window::{self, drag_resize},
};

use crate::Example;

#[derive(Debug, Clone)]
pub enum Message {
    Drag,
    Maximize,
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    South,
    SouthWest,
    SouthEast,
    Close,
    RotateImage,
    Blur(f32),
}

pub fn update(message: Message, example: &mut Example) -> Task<Message> {
    match message {
        Message::Drag => window::get_latest().and_then(window::drag),
        Message::Maximize => {
            println!("toggle!");
            // Task::none()
            window::get_latest().and_then(window::toggle_maximize)
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
        Message::West => window::get_latest().and_then(|f| drag_resize(f, window::Direction::West)),
        Message::East => window::get_latest().and_then(|f| drag_resize(f, window::Direction::East)),
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
        Message::RotateImage => {
            println!("rotating...");
            example.rotate_img();
            Task::none()
        }
        Message::Blur(g) => {
            println!("rotating...");
            example.blur_img(g);
            Task::none()
        }
    }
}

pub fn view<'a>(
    content: Element<'a, Message>,
    toolbar: Element<'a, Message>,
    //doing this also does not work
    // toolbar: Element<'a, crate::Message>,
) -> Element<'a, Message> {
    let base = iced::widget::center(
        iced::widget::column![
            mouse_area(
                iced::widget::container(
                    row![
                        //in this case tool bar is my button
                        toolbar,
                        horizontal_space(),
                        text("SomeImage"),
                        horizontal_space(),
                        iced::widget::button(text("x").center())
                            .height(Fill)
                            .width(30)
                            .on_press(Message::Close)
                            .padding(0)
                            .style(|t: &Theme, _| iced::widget::button::Style {
                                background: Some(Background::Color(
                                    t.extended_palette().secondary.strong.color
                                )),
                                border: border::rounded(5),
                                ..Default::default()
                            }),
                        iced::widget::button(text("max").center())
                            .height(Fill)
                            .width(30)
                            .on_press(Message::Maximize)
                            .padding(0)
                            .style(|t: &Theme, _| iced::widget::button::Style {
                                background: Some(Background::Color(
                                    t.extended_palette().secondary.strong.color
                                )),
                                border: border::rounded(5),
                                ..Default::default()
                            })
                    ]
                    .padding(5)
                    .spacing(5)
                )
                .width(Fill)
                .height(40)
            )
            .on_double_click(Message::Maximize)
            .on_press(Message::Drag),
        ]
        .push(content),
    )
    .style(|t: &Theme| Style {
        background: Some(Background::Color(t.palette().background)),
        border: iced::Border {
            color: t.palette().warning,
            width: 1.,
            radius: 8.into(),
        },
        ..Default::default()
    })
    .align_x(Center)
    .center_x(Fill)
    .width(Fill)
    .height(Fill);
    let bottom_row = row![
        mouse_area(
            iced::widget::container(row![])
                .width(5)
                .height(2)
                .style(|_| border_container())
        )
        .on_press(Message::SouthWest)
        .interaction(Interaction::ResizingDiagonallyUp),
        mouse_area(
            iced::widget::container(row![])
                .width(Fill)
                .height(2)
                .style(|_| border_container())
        )
        .on_press(Message::South)
        .interaction(Interaction::ResizingVertically),
        mouse_area(
            iced::widget::container(row![])
                .width(5)
                .height(2)
                .style(|_| border_container())
        )
        .on_press(Message::SouthEast)
        .interaction(Interaction::ResizingDiagonallyDown),
    ];

    let top_row = row![
        mouse_area(
            iced::widget::container(row![])
                .width(5)
                .height(2)
                .style(|_| border_container())
        )
        .on_press(Message::NorthWest)
        .interaction(Interaction::ResizingDiagonallyDown),
        mouse_area(
            iced::widget::container(row![])
                .width(Fill)
                .height(2)
                .style(|_| border_container())
        )
        .on_press(Message::North)
        .interaction(Interaction::ResizingVertically),
        mouse_area(
            iced::widget::container(row![])
                .width(5)
                .height(2)
                .style(|_| border_container())
        )
        .on_press(Message::NorthEast)
        .interaction(Interaction::ResizingDiagonallyUp),
    ];

    let rebase: Element<_> = iced::widget::center(iced::widget::column![
        top_row,
        iced::widget::container(
            row![
                mouse_area(
                    iced::widget::container(row![])
                        .width(2)
                        .height(Fill)
                        .style(|_| border_container())
                )
                .on_press(Message::West)
                .interaction(Interaction::ResizingHorizontally),
                base,
                mouse_area(
                    iced::widget::container(row![])
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
    .align_x(Center)
    .center_x(Fill)
    .width(Fill)
    .height(Fill)
    .into();
    rebase
}
fn border_container() -> Style {
    container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        ..Default::default()
    }
}
