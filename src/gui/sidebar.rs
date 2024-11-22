use iced::{border, color, Color, Element, Length, Theme};
use iced::widget::{button, column, container, image, row, text};
use iced::widget::image::Handle;

use super::page::PageManager;

#[derive(Debug, Clone)]
pub enum Message {
    MenuSelected(usize, String),
}

#[derive(Debug, Clone)]
pub enum Event {
    PageChange(String)
}

pub struct Sidebar {
    menu_items: Vec<(Handle, String)>, // 메뉴 항목들을 저장하는 벡터
    selected: Option<usize>, // 현재 선택된 메뉴 항목의 인덱스
}


impl Sidebar {
    pub fn new(page: &PageManager) -> Self {
        let menu_items = page.get_sidebar_from_page().unwrap();

        Self {
            menu_items,
            selected: None,
        }
    }

    pub fn update(&mut self, _message: Message) -> Option<Event> {
        match _message {
            Message::MenuSelected(index, ref page_name) => {
                println!("sidebar ------- call {:?}", _message);
                self.selected = Some(index);

                Some(Event::PageChange(page_name.clone()))
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let menu_items: Element<_> = self.menu_items
            .iter()
            .enumerate()
            // fold를 사용하여 각 메뉴 항목을 순회하면서 column을 구성합니다.
            .fold(column![].spacing(10), |column, (i, (icon_handle, item))| {
                column.push(
                    button(
                        row![
                                image(icon_handle.clone())
                                    .width(Length::Fixed(12.0))
                                    .height(Length::Fixed(12.0)),
                                text(item).size(16)
                            ].spacing(15).align_y(iced::Alignment::Center)
                    )

                        .on_press(Message::MenuSelected(i, item.clone()))
                        .style(move |theme: &Theme, status| {
                            // let palette = theme.extended_palette();
                            let is_selected = Some(i) == self.selected;

                            match status {
                                button::Status::Pressed => {
                                    Self::default_style()
                                    // button::Style::default()
                                    //     .with_background(palette.primary.strong.color)
                                }
                                _ if is_selected => {
                                    Self::cliked_button_style()
                                }
                                _ => {
                                    Self::default_style()
                                }
                            }
                        }
                        )
                        .width(Length::Fill),
                )
            })
            .into();

        container(menu_items)
            .width(Length::Fixed(200.0))
            .height(Length::Fill)
            .style(|theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(theme.extended_palette().background.base.color)),
                    // background: Some(color!(0x111111).into()),
                    text_color: Some(Color::WHITE),
                    border: border::rounded(2),
                    ..container::Style::default()
                }
            })
            .into()
    }

    fn default_style() -> button::Style {
        button::Style {
            background: None,
            text_color: Color::WHITE,
            border: iced::Border::default()
                .rounded(iced::border::Radius::from(10.0)),
            shadow: iced::Shadow::default(),
        }
    }

    fn cliked_button_style() -> button::Style {
        let mut cliked_button = Self::default_style();
        cliked_button.background = Some(color!(0xA487EB).into());

        cliked_button
    }
}


// impl Default for Sidebar{

// }