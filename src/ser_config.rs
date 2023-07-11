use serde::{Serialize, Deserialize};
use rocket::response::Responder;
//放置结构体

// 启动配置文件结构体
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Config{
    pub(crate) database_host:String,
    pub(crate) database_port:u32,
    pub(crate) database_username:String,
    pub(crate) database_password:String,
    pub(crate) database_dataname:String,
    pub(crate) ws_port:u32,
    pub(crate) ws_key:String,
    pub(crate) http_port:u32,
}

// 玩家数据结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub xuid: String,
    pub online: String,
    pub ip: String,
    pub last_server: String,
    pub balance: i32,
    pub data: String,
}










