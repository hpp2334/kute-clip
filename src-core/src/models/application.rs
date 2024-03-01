use misty_vm::client::MistyReadonlyClientHandle;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::{repositories::get_conn, result::KCResult};


pub const APPLICATION_VERSION: i32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationMetaModel {
    pub version: i32,
    pub preference: String,
}

impl Default for ApplicationMetaModel {
    fn default() -> Self {
        Self { version: APPLICATION_VERSION, preference: Default::default() }
    }
}

pub fn load_application_meta(cx: MistyReadonlyClientHandle) -> KCResult<ApplicationMetaModel> {
    let conn = get_conn(cx)?;
    let meta = conn.query::<ApplicationMetaModel>("SELECT * FROM application_meta", params![])?.pop().unwrap_or_default();
    Ok(meta)
}