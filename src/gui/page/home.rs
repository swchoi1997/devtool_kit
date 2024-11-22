use iced::{
    Element,
    Length, widget::{
        container,
        image::Handle,
        text,
    },
};

use crate::env;

use super::{Page, PageMessage};

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug)]
pub struct Home {
    icon_path: Handle,
    title: String,
    upper_menu: String,
}

impl Home {
    fn new(icon_path: String, title: String, upper_menu: String) -> Self {
        Self {
            icon_path: Handle::from_path(icon_path),
            title,
            upper_menu,
        }
    }

    pub fn new_from_env() -> Self {
        let home = env::from_key("page")
            .get_var_json_map("HOME")
            .unwrap();

        Self {
            icon_path: Handle::from_path(home.get("icon").unwrap()),
            title: home.get("title").unwrap().to_string(),
            upper_menu: home.get("upper_menu").unwrap().to_string(),
        }
    }
}

impl Page for Home {
    // fn on_enter(&mut self) {
    //
    // }
    //
    // fn on_leave(&mut self) {
    //
    // }

    fn icon_path(&self) -> &Handle {
        &self.icon_path
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn upper_menu(&self) -> &str {
        &self.upper_menu
    }

    fn update(&mut self, message: PageMessage) {
        todo!()
    }

    fn view<'a>(&self) -> Element<'a, PageMessage> {
        container(text("HOME").size(30))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}