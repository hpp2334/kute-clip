use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::models::application::APPLICATION_VERSION;
use crate::{libs::repository::DbConnection, result::KCResult};
use misty_vm::client::{AsReadonlyMistyClientHandle, MistyClientHandle};
use misty_vm::misty_service;
use misty_vm::services::MistyServiceTrait;

pub trait IDatabaseService: Send + Sync + 'static {
    fn set_app_dir(&self, app_dir: PathBuf);
    fn db_path(&self) -> PathBuf;
    fn conn(&self) -> KCResult<DbConnection>;
}
misty_service!(DatabaseService, IDatabaseService);

pub struct DatabaseServiceImpl {
    app_dir: Arc<Mutex<PathBuf>>,
}

impl DatabaseServiceImpl {
    pub fn new() -> Self {
        Self {
            app_dir: Default::default(),
        }
    }
}

impl IDatabaseService for DatabaseServiceImpl {
    fn set_app_dir(&self, app_dir: PathBuf) {
        *self.app_dir.lock().unwrap() = app_dir;
    }
    fn db_path(&self) -> PathBuf {
        let app_dir = self.app_dir.lock().unwrap().clone();
        let db_path = app_dir.join("store.db");
        db_path
    }
    fn conn(&self) -> KCResult<DbConnection> {
        let db_path = self.db_path().to_string_lossy().to_string();
        let conn = DbConnection::open(db_path)?;
        Ok(conn)
    }
}

pub fn get_conn<'a>(cx: impl AsReadonlyMistyClientHandle<'a>) -> KCResult<DbConnection> {
    DatabaseService::of(cx).conn()
}

pub fn init_repositories(cx: MistyClientHandle, app_dir: PathBuf) -> KCResult<()> {
    DatabaseService::of(cx).set_app_dir(app_dir.into());

    let db_path = DatabaseService::of(cx).db_path();

    if std::fs::metadata(&db_path).is_err() {
        let db_path = db_path.to_string_lossy();
        tracing::info!("Database not exists. Init database v{APPLICATION_VERSION}");
        tracing::info!("Database filepath: {db_path}");

        let conn = get_conn(cx)?;
        conn.execute_batch(include_str!("./schema/v1.sql"))?;
    } else {
        tracing::info!("Database already exists.");
    }

    Ok(())
}
