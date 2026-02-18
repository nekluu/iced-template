pub mod message;
pub mod ui;
pub mod utils;

use iced::{Task, Theme};
pub use message::Message;

#[derive(Default, Clone, Copy, Debug)]
pub struct App {
    pub stream_value: usize,
}

impl App {
    pub fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => Task::run(utils::stream_logic(), Message::Data),
            Message::Data(data) => {
                self.stream_value = data;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        ui::view(self)
    }
}