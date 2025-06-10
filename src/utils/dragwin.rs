use iced::{
    Alignment::{self, Center},
    Background, Color, Element,
    Length::{Fill, Shrink},
    Point, Size, Task, Theme, border,
    mouse::Interaction,
    widget::{
        Column, Row, button,
        container::{self, Style},
        horizontal_space, mouse_area, row, text,
        text_input::focus,
    },
    window::{self, drag_resize},
};
use image::GenericImageView;

use crate::{Example, action, new_icon};

use super::{
    draw::{DrawElement, DrawState, Tool},
    mode::{CropState, Mode},
    state::{State, icon_button, toolbar_icon},
};

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
    MousePressed,
    MouseMoved(Point),
    MouseReleased,
    ChangeTool(Tool),
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
            example.blur_img(g);
            Task::none()
        }
        Message::MousePressed => {
            match &mut example.state.mode {
                Mode::Crop {
                    top_left,
                    bottom_right,
                    size,
                    state: status,
                } => {
                    *top_left = example.state.cursor_position;
                    *bottom_right = example.state.cursor_position;
                    *size = Size::ZERO;
                    *status = CropState::InProgress {
                        start: example.state.cursor_position,
                        end: example.state.cursor_position,
                    };
                }
                Mode::Draw {
                    element: shape,
                    state: status,
                } => {
                    if shape.tool.is_text_tool() && shape.tool.is_valid() {
                        example.state.shapes.push(shape.clone());
                        example.state.cache.clear();
                        shape.tool.reset();
                    }

                    shape.tool.initiate(example.state.cursor_position);
                    *status = DrawState::InProgress {
                        initial_pt: example.state.cursor_position,
                        final_pt: example.state.cursor_position,
                    };
                }
            }
            Task::none()
        }
        Message::MouseMoved(position) => {
            example.state.cursor_position = position;
            match &mut example.state.mode {
                Mode::Crop {
                    top_left,
                    bottom_right,
                    size,
                    state: status,
                } => match status {
                    CropState::FullScreen | CropState::Window(_) => {
                        example.state.mode.get_window_below_cursor(
                            &example.state.windows,
                            &example.state.cursor_position,
                            example.state.scale_factor,
                            example.img.dimensions(),
                        );
                    }
                    CropState::InProgress { start, end } => {
                        *end = position;
                        *top_left = Point::new(start.x.min(end.x), start.y.min(end.y));
                        *bottom_right = Point::new(start.x.max(end.x), start.y.max(end.y));
                        *size = Size::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);
                    }
                    _ => {}
                },
                Mode::Draw {
                    element: shape,
                    state: status,
                } => {
                    if shape.tool.is_text_tool() {
                        return Task::none();
                    };
                    if let DrawState::InProgress {
                        initial_pt,
                        final_pt,
                    } = status
                    {
                        *final_pt = position;
                        shape.tool.update(*initial_pt, *final_pt);
                    }
                }
            }
            Task::none()
        }
        Message::MouseReleased => {
            match &mut example.state.mode {
                Mode::Crop { state: status, .. } => {
                    if let CropState::InProgress { start, end } = status {
                        if start != end {
                            *status = CropState::Area;
                        } else {
                            example.state.mode.get_window_below_cursor(
                                &example.state.windows,
                                &example.state.cursor_position,
                                example.state.scale_factor,
                                example.img.dimensions(),
                            );
                        }
                    }
                }
                Mode::Draw {
                    element: shape,
                    state: status,
                } => {
                    if shape.tool.is_text_tool() {
                        *status = DrawState::TextInput;
                        return focus("text_input").into();
                    } else {
                        if shape.tool.is_valid() {
                            example.state.shapes.push(shape.clone());
                            example.state.cache.clear();
                            shape.tool.reset();
                        }
                        *status = DrawState::Idle;
                    }
                }
            };
            Task::none()
        }
        Message::ChangeTool(tool) => {
            example.push_shape();
            if let Mode::Draw { element: shape, .. } = &mut example.state.mode {
                shape.tool = tool;
            } else {
                example.state.mode = Mode::Draw {
                    element: DrawElement {
                        tool,
                        ..Default::default()
                    },
                    state: DrawState::Idle,
                }
            };
            Task::none()
        }
    }
}

pub fn view<'a>(
    content: Element<'a, Message>,
    toolbar: Element<'a, Message>,
    state: &Example,
    //doing this also does not work
    // toolbar: Element<'a, crate::Message>,
) -> Element<'a, Message> {
    let toolbar_column = Column::new()
        .push(
            Row::new().push(Row::from_iter(Tool::ALL.into_iter().map(|tool| {
                // action(new_icon(), "rotate", Some(Message::ChangeTool(tool)))
                toolbar_icon(
                    tool.icon(),
                    "some",
                    match &state.state.mode {
                        Mode::Draw { element, state } => tool == element.tool,
                        _ => false,
                    },
                    Message::ChangeTool(tool),
                )
            }))),
        )
        .align_x(Alignment::Center)
        .spacing(5);
    let base = iced::widget::center(
        iced::widget::column![
            mouse_area(
                iced::widget::container(
                    row![
                        //in this case tool bar is my button
                        toolbar,
                        toolbar_column,
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
