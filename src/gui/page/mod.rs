mod home;
mod page;
mod bookmark;
mod page_manager;

use std::any::Any;
use std::sync::Arc;

pub use page_manager::PageManager;
use iced::widget::image::Handle;
use iced::Element;

#[derive(Debug, Clone)]
pub enum PageMessage {
    // PageSelected(Arc<dyn Any + Send + Sync>),
    Home(home::Message),
    Bookmark(bookmark::Message),
    // 다른 페이지 메시지들...
}


pub trait Page{
    /**
     * Default Constructor
     * 
     */
    // fn new(&self, icon_path: String, title: String, upper_menu: String) -> Self;

    /**
     * Page Enter
     */
    fn on_enter(&mut self){
        dbg!(format!("{:?} enter", self.title()));

    }

    /**
     * Page Leave
     */
    fn on_leave(&mut self){
        dbg!(format!("{:?} leave", self.title()));

    }

    /**
     * Sidebar Maker
     */
    fn get_sidebar_item(&self) -> (Handle, String) {
        (self.icon_path().clone(), self.title().to_string())
    }

    fn icon_path(&self) -> &Handle;
    fn title(&self) -> &str;
    fn upper_menu(&self) -> &str;

    fn view<'a>(&self) -> Element<'a, PageMessage>;
    fn update(&mut self, message: PageMessage);


}