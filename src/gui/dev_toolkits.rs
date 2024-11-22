use std::sync::{Arc, RwLock};

use iced::{Color, Element, Length, Theme};
use iced::widget::{container, row, text};

use sidebar::{Message as SidebarMessage, Sidebar};

use crate::{env, main};
use crate::db::Database;
use crate::gui::page::{PageManager, PageMessage};

use super::setting;
use super::sidebar;

const TITLE: &str = "APP_TITLE";

pub struct DeveloperToolkits {
    sidebar: Sidebar,
    page: Arc<RwLock<PageManager>>,
    // current_page: Option<Element<'static, PageMessage>>,

}

#[derive(Debug, Clone)]
pub enum Message {
    SidebarEvent(SidebarMessage),
    PageEvent(PageMessage),
}

impl DeveloperToolkits {
    fn title(&self) -> String {
        env::load().get_var(TITLE).to_string()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::SidebarEvent(sidebar_msg) => {
                println!("dev_tookkits ------- call {:?}", sidebar_msg);
                let sidebar_event: Option<sidebar::Event> = self.sidebar.update(sidebar_msg);

                if let Some(event) = sidebar_event {
                    match event {
                        sidebar::Event::PageChange(page_name) => {
                            self.change_page(page_name);
                        }
                    }
                }


                // self.change_page(page_name);
            }
            Message::PageEvent(page_msg) => {
                self.page.write().unwrap().update(page_msg);
            }
        }
    }

    fn change_page(&mut self, page_name: String) {
        match self.page.write().unwrap().change_page(&page_name) {
            Err(e) => {
                panic!("Failed to change page: {:?}", e);
            },
            _ => {}
        }
    }

    fn get_view(&self) -> Element<Message> {
        if let Some(page_content) = self.page.write().unwrap().view_current_page() {
            page_content.map(Message::PageEvent)
        } else {
            text("Developzer Toolkits").size(50).into()
        }
    }


    fn view(&self) -> Element<Message> {
        // let main_content = text("Developzer Toolkits").size(50);

        let view = self.get_view();

    

        let row_content = row![
            self.sidebar.view().map(Message::SidebarEvent),
            container(view)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center(Length::Fill)
        ];

        container(row_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(Color::BLACK)),
                    ..container::Style::default()
                }
            })
            .into()
    }
}

impl Default for DeveloperToolkits {
    fn default() -> Self {
        let page = PageManager::new();
        let sidebar = Sidebar::new(&page);

        Self {
            sidebar,
            page: Arc::new(RwLock::new(page)),
            // current_page: None,
        }
    }
}

pub fn run(db: &Database) -> iced::Result {
    env::load_env_file("page", "page.env");
    let setup = setting::setup();

    iced::application(
        DeveloperToolkits::title,
        DeveloperToolkits::update,
        DeveloperToolkits::view,
    )
        .settings(setup.0)
        .window(setup.1)
        .run()
}