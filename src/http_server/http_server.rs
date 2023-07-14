use std::{
    f32::consts::E,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    ser_config::{UserData, UserDataPerm},
    server::mysql_util::{
        delete_perm, get_playerspermissions, getplayerInformation, getplayerpermission_grade,
        insert_perm, update_perm,
    },
    yml_util::decrypt_name_t,
};

use rocket::{catch, get, http::Status, post, Request, Responder};
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
    code: u32,
    message: String,
}

// 获取玩家权限信息
#[get("/?<name>")]
pub fn get_permissions(name: Option<String>) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();

    println!("name: {}", name);

    let result = getplayerInformation(&pool, &name);
    match result {
        Ok(Some(_player)) => {
            let status = Status::Ok;
            let message = serde_json::to_string(&Response {
                code: 200,
                message: _player.permission_name,
            })
            .unwrap();
            return HttpGetResponder((status, message));
        }
        Ok(None) => {
            return null_404_http_get_responder();
        }
        Err(_err) => {
            return null_404_http_get_responder();
        }
    }
}

// 获取登录玩家的信息
#[post("/", format = "application/json", data = "<user_data>")]
pub fn get_login_chat(user_data: String) -> HttpGetResponder {
    let mut name = String::new();
    let mut token = String::new();
    let mut t = String::new();

    println!("get_login_chat接受参数： {}", user_data);

    // 解析数据
    match from_str::<UserData>(&user_data) {
        Ok(data) => {
            name = data.name;
            token = data.token;
            t = data.t;
        }
        Err(err) => eprintln!("Failed to parse JSON: {}", err),
    }

    println!("name: {}", name);
    println!("t: {}", t);
    println!("token: {}", token);

    let md5pws = decrypt_name_t(name.to_owned(), t.to_owned());
    let pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();
    println!("正确密钥： {}", md5pws);
    let t_timestamp = t.parse::<i128>().expect("Failed to parse timestamp");
    let current_timestamp = getnewcurrent_timestamp();
    println!("服务端时间戳：{}", current_timestamp);
    println!("客户端端时间戳：{}", t_timestamp);
    let time_difference = current_timestamp - t_timestamp;
    println!("时间差：{}", time_difference.abs());

    if time_difference.abs() >= 60000 {
        null_401_http_get_responder()
    } else if md5pws == token {
        let result = getplayerInformation(&pool, &name);
        match result {
            Ok(Some(_player)) => {
                let json_data = serde_json::to_string(&_player).unwrap();
                let status = Status::Ok;
                let message = json_data;
                HttpGetResponder((status, message))
            }
            Ok(None) => null_404_http_get_responder(),
            Err(_err) => null_500_http_get_responder(),
        }
    } else {
        let status = Status::Forbidden;
        let message = serde_json::to_string(&Response {
            code: 403,
            message: "false".to_string(),
        })
        .unwrap();
        HttpGetResponder((status, message))
    }
}

#[post("/", format = "application/json", data = "<user_data>")]
pub fn getplayerall(user_data: String) -> HttpGetResponder {
    println!("getplayerall: {}", user_data); //
    let mut name = String::new();
    let mut t = String::new();
    let mut token = String::new();

    // 解析数据
    match from_str::<UserData>(&user_data) {
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
    let md5pws = decrypt_name_t(name.clone(), t.clone());
    println!("正确密钥： {},{}", md5pws, t.clone());
    let pool: mysql::Pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();
    let t_timestamp = t.parse::<i128>().expect("Failed to parse timestamp");
    let current_timestamp = getnewcurrent_timestamp();
    println!("服务端时间戳：{}", current_timestamp);
    println!("客户端端时间戳：{}", t_timestamp);
    let time_difference = current_timestamp - t_timestamp;
    println!("时间差：{}", time_difference.abs());

    if time_difference.abs() >= 60000 {
        null_401_http_get_responder()
    } else if md5pws == token {
        match getplayerInformation(&pool, &name) {
            Ok(Some(_player)) => match get_playerspermissions(&pool) {
                Ok(Some(players)) => {
                    let json_data = serde_json::to_string(&players).unwrap();
                    let status = Status::Ok;
                    let message = json_data;
                    HttpGetResponder((status, message))
                }
                _ => {
                    let status = Status::NotFound;
                    let message = serde_json::to_string(&Response {
                        code: 404,
                        message: "null".to_string(),
                    })
                    .unwrap();
                    HttpGetResponder((status, message))
                }
            },
            Ok(None) => null_404_http_get_responder(),
            Err(_err) => {
                let status = Status::InternalServerError;
                let message = serde_json::to_string(&Response {
                    code: 500,
                    message: "error".to_string(),
                })
                .unwrap();
                HttpGetResponder((status, message))
            }
        }
        // 调用函数获取玩家权限信息
    } else {
        let status = Status::Forbidden;
        let message = serde_json::to_string(&Response {
            code: 403,
            message: "false".to_string(),
        })
        .unwrap();
        HttpGetResponder((status, message))
    }
}

// ***************************************************************权限处理
#[post("/", format = "application/json", data = "<user_data>")]
pub fn perm_mg(user_data: String) -> HttpGetResponder {
    println!("getplayerall: {}", user_data); //
    let mut name = String::new();
    let mut t = String::new();
    let mut token = String::new();
    let mut typestr = String::new();
    let mut perm_int: u128 = 0;

    // 解析数据
    match from_str::<UserDataPerm>(&user_data) {
        Ok(data) => {
            name = data.name;
            t = data.t;
            token = data.token;
            typestr = data.typestr;
            perm_int = data.perm_int;
        }
        Err(err) => eprintln!("Failed to parse JSON: {}", err),
    }

    println!("name: {}", name);
    println!("t: {}", t);
    println!("token: {}", token);
    let md5pws = decrypt_name_t(name.clone(), t.clone());
    println!("正确密钥： {},{}", md5pws, t.clone());
    let pool: mysql::Pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();

    let t_timestamp = t.parse::<i128>().expect("Failed to parse timestamp");
    let current_timestamp = getnewcurrent_timestamp();
    println!("服务端时间戳：{}", current_timestamp);
    println!("客户端端时间戳：{}", t_timestamp);
    let time_difference = current_timestamp - t_timestamp;
    println!("时间差：{}", time_difference.abs());

    if time_difference.abs() >= 60000 {
        null_401_http_get_responder()
    } else if md5pws == token {
        match getplayerpermission_grade(&pool, &name) {
            Ok(Some(perm_player_int)) => {
                if perm_int < perm_player_int {
                    return perm_type_mg(&pool, typestr, user_data);
                } else {
                    return null_403_http_get_responder();
                }
            }
            Ok(None) => return null_404_http_get_responder(),
            Err(_err) => return null_500_http_get_responder(),
        }
        // 调用函数获取玩家权限信息
    } else {
        null_403_http_get_responder()
    }
}

fn perm_type_mg(pool: &mysql::Pool, typestr: String, user_data: String) -> HttpGetResponder {
    match from_str::<UserDataPerm>(&user_data) {
        Ok(data) => match typestr.as_str() {
            "add" => match insert_perm(&pool, data) {
                Ok(_) => null_200_http_get_responder(),
                Err(_) => null_500_http_get_responder(),
            },
            "remove" => match delete_perm(&pool, data) {
                Ok(_) => null_200_http_get_responder(),
                Err(_) => null_500_http_get_responder(),
            },
            "updata" => match update_perm(&pool, data) {
                Ok(_) => null_200_http_get_responder(),
                Err(_) => null_500_http_get_responder(),
            },
            _ => null_500_http_get_responder(),
        },
        Err(_err) => null_500_http_get_responder(),
    }
}

fn null_404_http_get_responder() -> HttpGetResponder {
    let status = Status::NotFound;
    let message = serde_json::to_string(&Response {
        code: 404,
        message: "null".to_string(),
    })
    .unwrap();
    HttpGetResponder((status, message))
}
pub fn null_403_http_get_responder() -> HttpGetResponder {
    let status = Status::Forbidden;
    let message = serde_json::to_string(&Response {
        code: 403,
        message: "false".to_string(),
    })
    .unwrap();
    HttpGetResponder((status, message))
}
pub fn null_200_http_get_responder() -> HttpGetResponder {
    let status = Status::Ok;
    let message = serde_json::to_string(&Response {
        code: 200,
        message: "true".to_string(),
    })
    .unwrap();
    HttpGetResponder((status, message))
}
pub fn null_500_http_get_responder() -> HttpGetResponder {
    let status = Status::InternalServerError;
    let message = serde_json::to_string(&Response {
        code: 500,
        message: "error".to_string(),
    })
    .unwrap();
    HttpGetResponder((status, message))
}
pub fn null_401_http_get_responder() -> HttpGetResponder {
    let status = Status::Unauthorized;
    let message = serde_json::to_string(&Response {
        code: 401,
        message: "unauthorized".to_string(),
    })
    .unwrap();
    HttpGetResponder((status, message))
}
#[catch(404)]
pub fn not_found(req: &Request) -> String {
    let str = "默认  文档地址：https://github.com/banchen19/SynchronizerMC/blob/master/book.md"
        .to_string();
    format!("Sorry, \n'{}' is not a valid path.\n{}", req.uri(), str)
}

fn getnewcurrent_timestamp() -> i128 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i128;
    time
}
// #[get("/")]
// pub fn index() -> &'static str {
//     "接口/getMessageauthority \n"
// }
