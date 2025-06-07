use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: &'static DatabaseConnection,
}

impl AppState {
    pub fn new(db: &'static DatabaseConnection) -> Self {
        Self { db }
    }
}
