use serde::{Serialize, Deserialize};
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
    pub pl_online: u8,
    pub pl_server_name: String,
    pub pl_device: String,
    pub permission_name: String,
    pub online_time:i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsData {
    pub(crate)key:String,
    pub(crate)typestr:String,
    pub(crate)data:serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerToData {
    pub(crate)player_name: String,
    pub(crate)perm: String,
    pub(crate)data: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SerToChatData {
    pub(crate)typestr: String,
    pub(crate)serverver_name: String,
    pub(crate)data: serde_json::Value,
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
    pub players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionMg {
    typestr:String,
    perm_name:String
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub(crate)name: String,
    pub(crate)t: String,
    pub(crate)token: String,
}


#[derive(Debug, Deserialize)]
pub struct UserDataPerm {
    pub(crate)name: String,
    pub(crate)t: String,
    pub(crate)token: String,
    pub(crate)typestr:String,
    pub(crate)perm_int: u128,
    pub(crate)perm_str: String,
}