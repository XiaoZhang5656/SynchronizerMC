use std::str::FromStr;

use crate::{
    ser_config::{DataResponse, JsonResponse, UserData},
    server::mysql_util::{get_playerspermissions, getplayerpermissions},
    yml_util::decrypt_name_t,
};

use rocket::{get, http::Status, post, Responder, catch};
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

#[post(
    "/",
    format = "application/json",
    data = "<user_data>"
)]
pub fn get_messageauthority(user_data: String) -> HttpGetResponder {
    //getMessageauthority
    

    let mut name = String::new();

    println!("get_login_chat接受参数： {}", user_data);

    // 解析数据
    match from_str::<UserDatas>(&user_data) {
        Ok(data) => {
            name = data.name;
        }
        Err(err) => eprintln!("Failed to parse JSON: {}", err),
    }

    let pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();
    let result = getplayerpermissions(&pool, &name);
    match result {
        Ok(Some(_player)) =>{
            let status = Status::Ok;
            let message = _player.permission_name;
            HttpGetResponder((status, message))
        },
        Ok(None) => {
            let status = Status::Forbidden;
                let message = "false".to_string();
                HttpGetResponder((status, message))
        },
        Err(err) =>
        {
            let status = Status::NotFound;
            let message = "false".to_string();
            HttpGetResponder((status, message))
        },
    }
}

#[post(
    "/",
    format = "application/json",
    data = "<user_data>"
)]
pub fn get_login_chat(user_data: String) -> HttpGetResponder {
    let mut name = String::new();
    let mut pws = String::new();
    let mut t = String::new();

    println!("get_login_chat接受参数： {}", user_data);

    // 解析数据
    match from_str::<UserDatas>(&user_data) {
        Ok(data) => {
            name = data.name;
            pws = data.t;
            t = data.pws;
        }
        Err(err) => eprintln!("Failed to parse JSON: {}", err),
    }

    println!("name: {}", name);
    println!("t: {}", t);
    println!("pws: {}", pws);

    let md5pws = decrypt_name_t(name.to_owned(), t.to_owned());
    let pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();
    println!("正确密钥： {}", md5pws);
    if md5pws == pws {
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
                let message = "false".to_string();
                HttpGetResponder((status, message))
            }
            Err(err) => {
                let status = Status::NotFound;
                let message = "false".to_string();
                HttpGetResponder((status, message))
            }
        }
    } else {
        let status = Status::NotFound;
        let message = "false".to_string();
        HttpGetResponder((status, message))
    }
}

#[derive(Debug, Deserialize)]
struct UserDatas {
    name: String,
    pws: String,
    t: String,
}
#[post(
    "/",
    format = "application/json",
    data = "<user_data>"
)]
pub fn getplayerall(user_data: String) -> HttpGetResponder {
    println!("getplayerall: {}", user_data);
    let mut name = String::new();
    let mut t = String::new();
    let mut pws = String::new();

    // 解析数据
    match from_str::<UserDatas>(&user_data) {
        Ok(data) => {
            name = data.name;
            t = data.t;
            pws = data.pws;
        }
        Err(err) => eprintln!("Failed to parse JSON: {}", err),
    }

    println!("name: {}", name);
    println!("t: {}", t);
    println!("pws: {}", pws);

    let md5pws = decrypt_name_t(name.clone(), t.clone());
    println!("正确密钥： {},{}", md5pws, t.clone());

    if md5pws == pws {
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
                let message = json_data;
                HttpGetResponder((status, message))
            }
            _ => {
                let status = Status::Forbidden;
                let message = "false".to_string();
                HttpGetResponder((status, message))
            }
        }
    } else {
        let status = Status::NotFound;
        let message = "false".to_string();
        HttpGetResponder((status, message))
    }
}
use rocket::Request;
#[catch(404)]
pub fn not_found(req: &Request) ->String{
    let str="文档地址：https://github.com/banchen19/SynchronizerMC/blob/master/book.md".to_string();
    format!("Sorry, '{}' is not a valid path.\n{}", req.uri(),str)
}
// #[get("/")]
// pub fn index() -> &'static str {
//     "接口/getMessageauthority \n"
// }
