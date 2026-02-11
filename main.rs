use iced::{
    Alignment::Center,
    Element, Event,
    Length::Fill,
    Subscription, Task, event,
    futures::{Stream, stream},
    mouse,
    widget::{button, column, text},
};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
#[derive(Clone, Copy, Debug)]
enum Message {
    Start,
    Data(usize),
}
#[derive(Default, Clone, Copy, Debug)]
struct App {
    stream_value: usize,
}
impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => Task::run(stream(), Message::Data),
            Message::Data(data) => {
                self.stream_value = data;
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let hello_message = text("hello world 🤡");
        let button = button("Start").on_press(Message::Start);
        let stream_log = text(self.stream_value);
        column![hello_message, button, stream_log]
            .width(Fill)
            .height(Fill)
            .align_x(Center)
            .spacing(20)
            .padding(200)
            .into()
    }
    /*
    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| {
            match event {
                Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                    match delta {
                        mouse::ScrollDelta::Lines { y, .. } => Some(Message::WheelMoved(y)), //Add "WhellMoved(f32)" in your "Message" enum
                        mouse::ScrollDelta::Pixels { y, .. } => Some(Message::WheelMoved(y)),
                    }
                }
                _ => None,
            }
        })
    }
    */
}

fn stream() -> impl Stream<Item = usize> {
    let counter = 0;
    stream::unfold(counter, move |mut state| async move {
        if state >= 1000 {
            None
        } else {
            state += 1;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            Some((state, state))
        }
    })
}
