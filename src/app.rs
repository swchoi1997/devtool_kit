use crate::gui;
use crate::db;
use crate::db::Database;

pub fn run() -> iced::Result{
    let db = connect_db();
    gui::run_dev_toolkits_app(&db)
}

async fn connect_db() -> Database {
    // 비동기 작업을 동기 함수에서 실행할 수 있도록 tokio 런타임 생성
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let db = Database::new("app_data.db").await.unwrap();

        db
    })
}

