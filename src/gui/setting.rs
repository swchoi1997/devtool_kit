use iced::{Settings, Size};
use iced::window;
use iced::window::icon;

use crate::env;

const APP_ICON_PATH: &str = "APP_ICON_PATH";
const APP_ID: &str = "com.example.devtoolkits";
// const FONT_FAMILY_NAME: &str = "Your Font Family";
// const FONT_SIZE_BODY: f32 = 16.0;


pub fn setup() -> (Settings, window::Settings) {
    let iced_set = Settings {
        id: Some(String::from(APP_ID)),                 // 애플리케이션의 고유 식별자
        antialiasing: true,                             // 안티앨리어싱 활성화 여부
        ..Default::default()
    };

    let icon_path = env::load().get_var(APP_ICON_PATH).to_string();

    let window_set = window::Settings {
        size: Size::new(1024.0, 768.0), // 창의 초기 크기 (너비, 높이)
        position: window::Position::Default,        // 창의 초기 위치
        min_size: None,                             // 최소 창 크기 (필요시 Some((width, height)))
        max_size: None,                             // 최대 창 크기 (필요시 Some((width, height)))
        visible: true,                              // 창의 초기 가시성
        resizable: true,                            // 창 크기 조절 가능 여부
        decorations: true,                          // 창 테두리 및 제어 버튼 표시 여부
        transparent: false,                         // 창 배경 투명 여부
        icon: Some(icon::from_file(icon_path).unwrap()), //TODO icon경로 확인
        // 창 아이콘 (필요시 Some(icon)으로 설정)
        #[cfg(target_os = "linux")]
        platform_specific: window::PlatformSpecific {
            application_id: String::from(APP_ID),   // Linux용 애플리케이션 ID
        },
        exit_on_close_request: true,                // 창 닫기 요청 시 애플리케이션 종료 여부
        ..Default::default()
    };

    (iced_set, window_set)
}