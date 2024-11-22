use iced::Element;
use iced::Length::Fill;
use iced::widget::{container, text};

#[derive(Debug, Clone)]
pub enum Message {}

pub struct HelloWorld;

impl HelloWorld {
    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, _message: Message) {
        // This application has no interactions
    }

    fn view(&self) -> Element<Message> {
        container(text("Hello World").size(50))
            .center_x(Fill)
            .center_y(Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
impl Default for HelloWorld {
    fn default() -> Self {
        Self
    }
}

pub fn run() -> iced::Result {
    iced::application(HelloWorld::title, HelloWorld::update, HelloWorld::view)
        .centered()
        .run()
}