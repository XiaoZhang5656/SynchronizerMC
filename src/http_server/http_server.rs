use std::f32::consts::E;

use crate::server::mysql_util::*;
use serde::{Deserialize, Serialize};

use crate::POOL;

#[derive(Debug, Deserialize)]
struct UserData {
    name: String,
}

#[derive(Debug, Serialize)]
struct DataResponse {
    code: String,
    msg: String,
}
#[derive(Debug, Serialize)]
struct JsonResponse {
    data: DataResponse,
}
#[post(
    "/getMessageauthority",
    format = "application/json",
    data = "<user_data>"
)]
pub fn handle_request(user_data: String) -> String {
    if let Ok(ws_data) = serde_json::from_str::<UserData>(&user_data) {
        // 在这里处理解析成功的逻辑
        let name = &ws_data.name;
        let pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        let result = get_player_by_name(&pool, &name);
        let data = match result {
            Ok(Some(player)) => {
                if player.chat_perm.parse::<bool>().unwrap_or(false)&&player.game_perm.parse::<bool>().unwrap_or(false) {
                    DataResponse {
                        code: "200".to_string(),
                        msg: "true".to_string(),
                    }
                } else {
                    DataResponse {
                        code: "403".to_string(),
                        msg: "false".to_string(),
                    }
                }
            }

            Ok(None) => DataResponse {
                code: "404".to_string(),
                msg: "false".to_string(),
            },
            Err(err) => {
                println!("Error: {:?}", err);
                DataResponse {
                    code: "500".to_string(),
                    msg: "error".to_string(),
                }
            }
        };
        let json_response = JsonResponse { data };
        format!("{}", serde_json::to_string(&json_response).unwrap())
    } else {
        // 在这里处理解析失败的逻辑
        "Failed to parse JSON data".to_string()
    }
}

#[post("/getLoginChat", format = "application/json", data = "<user_data>")]
pub fn get_login_chat(user_data: String) -> String {
    if let Ok(ws_data) = serde_json::from_str::<UserData>(&user_data) {
        // 在这里处理解析成功的逻辑
        let name = &ws_data.name;
        let pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        let result = get_player_by_name(&pool, &name);
        let data = match result {
            Ok(Some(player)) => {
                if player.game_perm.parse::<bool>().unwrap_or(false) {
                    DataResponse {
                        code: "200".to_string(),
                        msg: "true".to_string(),
                    }
                } else {
                    DataResponse {
                        code: "403".to_string(),
                        msg: "false".to_string(),
                    }
                }
            }
            Ok(None) => DataResponse {
                code: "404".to_string(),
                msg: "false".to_string(),
            },
            Err(err) => {
                println!("Error: {:?}", err);
                DataResponse {
                    code: "500".to_string(),
                    msg: "error".to_string(),
                }
            }
        };
        let json_response = JsonResponse { data };
        format!("{}", serde_json::to_string(&json_response).unwrap())
    } else {
        // 在这里处理解析失败的逻辑
        "Failed to parse JSON data".to_string()
    }
}
