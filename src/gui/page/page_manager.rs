use iced::Element;
use iced::widget::image::Handle;

use crate::env::CustomError;
use crate::gui::page::bookmark::Bookmark;
use crate::gui::page::home::Home;

use super::PageMessage;
use super::page::PageContainer;

#[derive(Clone)]
pub struct PageManager {
    page_container: PageContainer,
    // 현재 페이지
    current_page: Option<String>,
}


impl PageManager {
    pub fn new() -> Self {
        let page_container = Self::init();

        Self {
            page_container,
            current_page: None,
        }
    }

    fn init() -> PageContainer {
        let mut page_container = PageContainer::new();

        page_container
            .add_page(Home::new_from_env())
            .add_page(Bookmark::new_from_env());

        page_container
    }

    /**
     * 현재 페이지를 변경하는 메서드.
     * 현재 페이지에서 `on_leave` 메서드를 호출하고, 새로운 페이지에 대해 `on_enter`를 호출.
     */
    pub fn change_page<'a>(&mut self, name: &str) -> Result<(), CustomError> {;
        if self.current_page.as_deref() == Some(name) {
            return Ok(());
        }

        if let Some(current) = &self.current_page {
            if let Some(current_page) = self.page_container.get_page(current) {
                current_page.write().unwrap().on_leave();
            }
        }

        if let Some(new_page) = self.page_container.get_page(name) {
            new_page.write().unwrap().on_enter();
            // let mut nxt_current_page = new_page.as_ref().borrow_mut();
            // nxt_current_page.on_enter();

            self.current_page = Some(name.to_string());

            return Ok(());
        }

        Err(CustomError::NoPageFound)
    }

    // pub fn change_page<'a>(&mut self, name: &str) -> Result<Option<Element<'a, PageMessage>>, CustomError> {
    //     if self.current_page.as_deref() == Some(name) {
    //         return Ok(self.view_current_page());
    //     }

    //     if let Some(current) = &self.current_page {
    //         if let Some(current_page) = self.page_container.get_page(current) {
    //             current_page.write().unwrap().on_leave();
    //         }
    //     }

    //     if let Some(new_page) = self.page_container.get_page(name) {
    //         new_page.write().unwrap().on_enter();
    //         // let mut nxt_current_page = new_page.as_ref().borrow_mut();
    //         // nxt_current_page.on_enter();

    //         self.current_page = Some(name.to_string());

    //         return Ok(self.view_current_page());
    //     }

    //     Err(CustomError::NoPageFound)
    // }


    pub fn view_current_page<'a>(&mut self) -> Option<Element<'a, PageMessage>> {
        self.current_page
            .as_ref()
            .and_then(|page_name| {
                self.page_container
                    .get_page(page_name)
                    .map(|page| {
                        page.read().unwrap().view()
                        // .borrow_mut().view()
                    })
            })
    }


    pub fn get_sidebar_from_page(&self) -> Result<Vec<(Handle, String)>, CustomError> {
        self.page_container.get_sidebar_items()
    }


    pub fn update(&mut self, message: PageMessage) {
        if let Some(current_page_name) = &self.current_page {
            if let Some(current_page) = self.page_container.get_page(current_page_name) {
                current_page.write().unwrap().update(message);
            }
        }
    }
}
