use crate::{server::mysql_util::getplayerpermissions, yml_util::decrypt_name_t};

use rocket::{get, http::Status, post, Responder};
use serde::{Deserialize, Serialize};
extern crate crypto;

use crate::POOL;

// 自定义状态码并返回数据
#[derive(Responder)]
#[response(content_type = "json")]
pub struct HttpGetResponder((rocket::http::Status, String));

#[get("/?<name>")]
pub fn getpermissions(name: Option<String>) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    println!("name: {}", name);

    // 获取玩家权限组全部信息


    let status = Status::NotFound;
    let message = serde_json::to_string(&"{\"message\": false}").unwrap();
    HttpGetResponder((status, message))
}
#[get("/?<name>&<pws>&<t>")]
pub fn getinformation(name: Option<String>, pws: Option<String>, t: Option<String>) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pws = pws.unwrap_or_default();
    let t = t.unwrap_or_default();
    let md5pws = decrypt_name_t(name.to_string(), t.to_string());
    println!("{}", md5pws);
    if md5pws == pws {
        // getinformation
        let pool: mysql::Pool = POOL
            .lock()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone();
        let permissions = getplayerpermissions(&pool, &name);
        match permissions {
            Ok(result) => {
                if !result.is_empty() {
                    let status = Status::Ok;
                    let message = result.to_string();
                    return HttpGetResponder((status, message));
                } else {
                    let status = Status::NoContent;
                    let message = serde_json::to_string(&"{\"message\": false}").unwrap();
                    return HttpGetResponder((status, message))
                }
            }
            Err(err) => {
                let status = Status::Forbidden;
                let message = serde_json::to_string(&"{\"message\": false}").unwrap();
                return HttpGetResponder((status, message))
            }
        }
    }
    let status = Status::NotFound;
    let message = serde_json::to_string(&"{\"message\": false}").unwrap();
    HttpGetResponder((status, message))
}

// #[get("/")]
// pub fn index() -> &'static str {
//     "接口/getMessageauthority \n"
// }
