use iced::widget::{button, column, text};
use iced::{Alignment::Center, Element, Length::Fill};
use crate::app::{App, message::Message};

pub fn view(app: &App) -> Element<'_, Message> {
    let hello_message = text("hello world 🤡");
    let start_button = button("Start").on_press(Message::Start);
    let stream_log = text(app.stream_value);

    column![hello_message, start_button, stream_log]
        .width(Fill)
        .height(Fill)
        .align_x(Center)
        .spacing(20)
        .padding(200)
        .into()
}