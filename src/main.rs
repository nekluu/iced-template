use crate::app::App;

mod app;

fn main() -> iced::Result {
    iced::application(App::default,App::update,App::view).theme(App::theme).run()
}