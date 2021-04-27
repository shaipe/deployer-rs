//! copyright © ecdata.cn 2021 - present
//! Docker应用部署
//! create by shaipe 20210125

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Docker {
    pub name: String,
    pub image: String,
}