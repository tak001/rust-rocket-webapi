use serde::{Deserialize, Serialize};

/// TODOのモデルはmodels.rsに定義
#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub done: bool,
}