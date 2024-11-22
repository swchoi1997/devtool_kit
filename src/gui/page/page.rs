use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use iced::widget::image::Handle;

use crate::env::CustomError;

use super::Page;

/**
 * 페이지 관리를 위한 구조체
 */
#[derive(Clone)]
pub struct PageContainer {
    // page 관리를 위한 HashMap
    // String: Alias / Page: Page Trait 구현체
    orders: HashMap<u32, String>,
    pages: HashMap<String, Arc<RwLock<dyn Page>>>,
    // 현재 페이지
    current_page: Option<String>,
    // TODO Stack으로 뒤로가기 기능 구현
}


impl PageContainer {
    /**
     * PageContainer 생성자 함수.
     * 빈 페이지 목록과 None으로 설정된 현재 페이지를 가진 새로운 PageContainer를 반환.
     */
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            pages: HashMap::new(),
            current_page: None,
        }
    }

    /**
     * 페이지를 등록하는 메서드
     */
    pub fn add_page<P: Page + 'static>(&mut self, page: P) -> &mut Self {
        let next_order = self.orders
            .keys()
            .max()
            .map(|&odr| odr + 1)
            .unwrap_or(1);

        self.add_page_with_order(next_order, page.title().to_string(), page)
    }

    pub fn add_page_with_order<P: Page + 'static>(&mut self, order: u32, name: String, page: P) -> &mut Self {
        self.orders.insert(order, name);
        self.pages.insert(page.title().to_string(), Arc::new(RwLock::new(page)));

        self
    }

    pub fn get_page(&mut self, page_name: &str) -> Option<&mut Arc<RwLock<dyn Page>>> {
        self.pages.get_mut(page_name)
    }


    pub fn get_sidebar_items(&self) -> Result<Vec<(Handle, String)>, CustomError> {
        let mut sidebar_items = vec![];

        for odr in 0..self.orders.len() {
            let _search = (odr + 1) as u32;
            let _name = self.orders.get(&_search).unwrap();

            let _page = self.pages.get(_name)
                .unwrap()
                .as_ref()
                .read()
                .unwrap()
                .get_sidebar_item();
            // .borrow()
            // .get_sidebar_item();

            sidebar_items.push(_page);
        }

        if sidebar_items.is_empty() {
            return Err(CustomError::NoPageFound);
        }

        Ok(sidebar_items)
    }
}
