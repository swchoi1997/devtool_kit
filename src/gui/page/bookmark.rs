use std::{env::consts, fs::{self, File}, io::Write, path::PathBuf};

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
pub struct Bookmark {
    icon_path: Handle,
    title: String,
    upper_menu: String,
    bookmarks: Vec<Package>,
}

impl Bookmark {
    fn new(icon_path: String, title: String, upper_menu: String) -> Self {
        Self {
            icon_path: Handle::from_path(icon_path),
            title,
            upper_menu,
            bookmarks: Vec::new(),
        }
    }

    pub fn new_from_env() -> Self {
        let home = env::from_key("page")
            .get_var_json_map("BOOKMARK")
            .unwrap();

        Self {
            icon_path: Handle::from_path(home.get("icon").unwrap()),
            title: home.get("title").unwrap().to_string(),
            upper_menu: home.get("upper_menu").unwrap().to_string(),
            bookmarks: Vec::new(),
        }
    }

    fn load_saved_bookmark(&self) -> Vec<Package> {

        Vec::new()
    }
}

impl Page for Bookmark {
    fn on_enter(&mut self) {
        // 북마크 정보 초기화
        self.bookmarks = self.load_saved_bookmark();
    }

    // fn on_leave(&mut self) {
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

    fn update(&mut self, message: PageMessage) {}

    fn view<'a>(&self) -> Element<'a, PageMessage> {
        container(text("BOOKMARK").size(30))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
struct Package {
    order: u32,
    category: String,
    bookmark: Data,
}

#[derive(Debug, Clone)]
struct Data {
    order: u32,             // 순서
    _type: BookmarkType,    // URL, FOLDER 
    alias: String,          // 별칭
    url: String,             // 경로
}


#[derive(Debug, Clone)]
enum BookmarkType {
    Url,
    Folder,
}