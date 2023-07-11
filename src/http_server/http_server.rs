use std::str::FromStr;

use crate::{
    ser_config::{DataResponse, JsonResponse, UserData},
    server::mysql_util::{get_playerspermissions, getplayerpermissions},
    yml_util::decrypt_name_t,
};

use rocket::{get, http::Status, post, Responder};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
extern crate crypto;

use crate::POOL;

// 自定义状态码并返回数据
#[derive(Responder)]
#[response(content_type = "json")]
pub struct HttpGetResponder((rocket::http::Status, String));
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    message: String,
}

#[get("/?<name>")]
pub fn getpermissions(name: Option<String>) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();
    let result = getplayerpermissions(&pool, &name);
    match result {
        Ok(Some(_player)) => {
            let status = Status::Ok;
            let message = serde_json::to_string(&Response {
                message: _player.permission_name,
            })
            .unwrap();
            return HttpGetResponder((status, message));
        }
        Ok(None) => {
            let status = Status::NotFound;
            let message = serde_json::to_string(&Response {
                message: "false".to_string(),
            })
            .unwrap();
            return HttpGetResponder((status, message));
        }
        Err(_err) => {
            let status = Status::NotFound;
            let message = serde_json::to_string(&Response {
                message: "false".to_string(),
            })
            .unwrap();
            return HttpGetResponder((status, message));
        }
    }
}
#[get("/?<name>&<pws>&<t>")]
pub fn getinformation(
    name: Option<String>,
    pws: Option<String>,
    t: Option<String>,
) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pws = pws.unwrap_or_default();
    let t = t.unwrap_or_default();

    let md5pws = decrypt_name_t(name.clone(), t);
    println!("正确密钥： {}", md5pws);

    if md5pws == pws {
        let pool: mysql::Pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        match getplayerpermissions(&pool, &name) {
            Ok(Some(player)) => {
                let json_string = serde_json::to_string(&player).unwrap();
                let status = Status::Ok;
                let message = json_string;
                return HttpGetResponder((status, message));
            }
            Ok(None) => {
                let status = Status::NotFound;
                let message = serde_json::to_string(&Response {
                    message: "false".to_string(),
                })
                .unwrap();
                return HttpGetResponder((status, message));
            }
            Err(_err) => {
                let status = Status::InternalServerError;
                let message = serde_json::to_string(&Response {
                    message: "false".to_string(),
                })
                .unwrap();
                return HttpGetResponder((status, message));
            }
        }
    } else {
        let status = Status::NotFound;
        let message = serde_json::to_string(&Response {
            message: "false".to_string(),
        })
        .unwrap();
        HttpGetResponder((status, message))
    }
}

#[post(
    "/",
    format = "application/x-www-form-urlencoded",
    data = "<user_data>"
)]
pub fn get_messageauthority(user_data: String) -> String {
    //getMessageauthority

    if let Ok(ws_data) = serde_json::from_str::<UserData>(&user_data) {
        // 在这里处理解析成功的逻辑
        let name = &ws_data.name;
        let pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        let result = getplayerpermissions(&pool, &name);
        let data = match result {
            Ok(Some(_player)) => DataResponse {
                code: "200".to_string(),
                msg: "true".to_string(),
            },
            Ok(None) => DataResponse {
                code: "403".to_string(),
                msg: "false".to_string(),
            },
            Err(err) => {
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

#[post(
    "/",
    format = "application/x-www-form-urlencoded",
    data = "<user_data>"
)]
pub fn get_login_chat(user_data: String) -> String {

    let mut name = String::new();
    let mut pws = String::new();
    let mut t = String::new();

    // 解析数据
    let data: Vec<&str> = user_data.split('&').collect();
    for item in data {
        let parts: Vec<&str> = item.split('=').collect();
        if parts.len() == 2 {
            match parts[0] {
                "name" => name = urlencoding::decode(parts[1]).unwrap_or_default(),
                "token" => pws = urlencoding::decode(parts[1]).unwrap_or_default(),
                "t" => t = urlencoding::decode(parts[1]).unwrap_or_default(),
                _ => {}
            }
        }
    }


    let md5pws = decrypt_name_t(name.to_owned(), t.to_owned());

    println!("正确密钥： {}", md5pws);
    if md5pws == pws {
        let pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        let result = getplayerpermissions(&pool, &name);
        let data = match result {
            Ok(Some(_player)) => DataResponse {
                code: "200".to_string(),
                msg: "true".to_string(),
            },
            Ok(None) => DataResponse {
                code: "403".to_string(),
                msg: "false".to_string(),
            },
            Err(err) => {
                DataResponse {
                    code: "500".to_string(),
                    msg: "error".to_string(),
                }
            }
        };
        let json_response = JsonResponse { data };
        format!("{}", serde_json::to_string(&json_response).unwrap())
    } else {
        let data = {
            DataResponse {
                code: "404".to_string(),
                msg: "false".to_string(),
            }
        };

        let json_response = JsonResponse { data: data };
        format!("{}", serde_json::to_string(&json_response).unwrap())
    }
}



#[derive(Debug, Deserialize)]
struct UserDatas {
    name: String,
    t: String,
    token: String,
}
#[post(
    "/",
    format = "application/x-www-form-urlencoded",
    data = "<user_data>"
)]
pub fn getplayerall(user_data: String) -> HttpGetResponder {
    
    println!("{}",user_data);
    let mut name = String::new();
    let mut t = String::new();
    let mut token = String::new();

    // 解析数据
    match from_str::<UserDatas>(&user_data) {
        Ok(data) => {
            name = data.name;
            t = data.t;
            token = data.token;
        }
        Err(err) => eprintln!("Failed to parse JSON: {}", err),
    }

    println!("name: {}", name);
    println!("t: {}", t);
    println!("token: {}", token);

    let md5pws = decrypt_name_t(name.clone(), token.clone());
    println!("正确密钥： {},{}", md5pws,t.clone());

    if md5pws == t {
        let pool: mysql::Pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        // 调用函数获取玩家权限信息
        let players_permissions = match get_playerspermissions(&pool) {
            Ok(Some(players)) => players.players,
            _ => Vec::new(),
        };
        let json_data = serde_json::to_string(&players_permissions).unwrap();

        let status = Status::Ok;
        let message = json_data;
        HttpGetResponder((status, message))
    } else {
        let status = Status::NotFound;
        let message = serde_json::to_string(&Response {
            message: "false".to_string(),
        })
        .unwrap();
        HttpGetResponder((status, message))
    }
}

// #[get("/")]
// pub fn index() -> &'static str {
//     "接口/getMessageauthority \n"
// }
