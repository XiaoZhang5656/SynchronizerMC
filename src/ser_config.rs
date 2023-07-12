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

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub pl_xuid: String,
    pub pl_name: String,
    pub pl_llmoney: i32,
    pub pl_ip: String,
    pub pl_online: String,
    pub pl_server_name: String,
    pub pl_device: String,
    pub permission_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsData {
    pub(crate)key:String,
    pub(crate)data:String,
    pub(crate)typestr:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataChat {
    pub(crate)player:String,
    pub(crate)chat:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub(crate)typestr: String,
    pub(crate)data: String,
    pub(crate)perm: String,
}

#[derive(Debug, Serialize)]
pub struct DataResponse {
    pub(crate)code: String,
    pub(crate)msg: String,
}

#[derive(Debug, Serialize)]
pub struct JsonResponse {
    pub(crate)data: DataResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    pub players: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionMg {
    typestr:String,
    perm_name:String
}