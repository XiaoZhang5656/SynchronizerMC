use crate::{
    server::mysql_util::{get_playerspermissions, getplayerpermissions},
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
    message: String,
}

// 获取玩家权限信息
#[get("/?<name>")]
pub fn get_messageauthority(name: Option<String>) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();

    println!("name: {}", name);

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
                message: "null".to_string(),
            })
            .unwrap();
            return HttpGetResponder((status, message));
        }
        Err(_err) => {
            let status = Status::NotFound;
            let message = serde_json::to_string(&Response {
                message: "null".to_string(),
            })
            .unwrap();
            return HttpGetResponder((status, message));
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
    println!("pws: {}", token);

    let md5pws = decrypt_name_t(name.to_owned(), t.to_owned());
    let pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();
    println!("正确密钥： {}", md5pws);
    if md5pws == token {
        let result = getplayerpermissions(&pool, &name);
        match result {
            Ok(Some(_player)) => {
                let json_data = serde_json::to_string(&_player).unwrap();
                let status = Status::Ok;
                let message = json_data;
                HttpGetResponder((status, message))
            }
            Ok(None) => {
                let status = Status::Forbidden;
                let message = serde_json::to_string(&Response {
                    message: "null".to_string(),
                })
                .unwrap();
                HttpGetResponder((status, message))
            }
            Err(_err) => {
                let status = Status::Forbidden;
                let message = serde_json::to_string(&Response {
                    message: "null".to_string(),
                })
                .unwrap();
                HttpGetResponder((status, message))
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

#[derive(Debug, Deserialize)]
struct UserData {
    name: String,
    t: String,
    token: String,
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

    if md5pws == token {
        let pool: mysql::Pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        // 调用函数获取玩家权限信息
        match get_playerspermissions(&pool) {
            Ok(Some(players)) => {
                let json_data = serde_json::to_string(&players).unwrap();
                let status = Status::Ok;
                let message =json_data;
                HttpGetResponder((status, message))
            }
            _ => {
                let status = Status::Forbidden;
                let message = serde_json::to_string(&Response {
                    message: "null".to_string(),
                })
                .unwrap();
                HttpGetResponder((status, message))
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

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    let str =
        "文档地址：https://github.com/banchen19/SynchronizerMC/blob/master/book.md".to_string();
    format!("Sorry, '{}' is not a valid path.\n{}", req.uri(), str)
}

// #[get("/")]
// pub fn index() -> &'static str {
//     "接口/getMessageauthority \n"
// }
