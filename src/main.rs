#[macro_use]
extern crate rocket;

use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
};

mod http_server;
mod yml_util;
use crate::http_server::http_server::*;

use mysql::Pool;
use rocket::Config;
use yml_util::generate_random_key;

use ws::listen;
mod server;
use crate::server::ws_server::ServerHandler;

use server::mysql_util::create_player_table;

pub struct WsServer {
    port: i32,
    key: String,
}
pub struct PersonalData {
    username: String,
    password: String,
    host: String,
    port: u32,
    database: String,
}
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref POOL: Mutex<Option<Pool>> = Mutex::new(None);
}
// 建立数据库连接并返回连接池
fn establish_connection(perso: &PersonalData) -> Pool {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        perso.username, perso.password, perso.host, perso.port, perso.database
    );
    mysql::Pool::new(url).expect("Failed to create connection pool")
}

#[tokio::main]
async fn main() {
    let key = generate_random_key(10);
    let config = yml_util::Config {
        host: "localhost".to_string(),
        port: 3306,
        username: "bc_test".to_string(),
        password: "123456".to_string(),
        database: "my_test".to_string(),
        ws_port: 8080,
        ws_key: key,
    };
    let file_path = "config.yml";
    match fs::metadata(file_path) {
        Err(_) => {
            println!("文件不存在, 开始写入");
            if let Err(err) = yml_util::write_config_to_yml(&config, file_path) {
                println!("无法写入配置文件：{}", err);
            }
            read_yml_to_str(file_path).await;
        }
        Ok(_) => {
            println!("文件存在");
            read_yml_to_str(file_path).await;
        }
    }
}
async fn read_yml_to_str(file_path: &str) {
    if let Ok(perso) = yml_util::read_yml(file_path) {
        let ws_server: WsServer = WsServer {
            port: perso.ws_port,
            key: perso.ws_key,
        };
        let perso = PersonalData {
            username: perso.username,
            password: perso.password,
            host: perso.host,
            port: perso.port,
            database: perso.database,
        };

        // 使用建立连接的函数
        let pool: Pool = establish_connection(&perso);
        println!("Mysql已连接");

        *POOL.lock().unwrap() = Some(pool.clone());
        // 创建玩家表
        create_player_table(&pool).expect("Failed to create player table");

        // 启动 WebSocket 服务器
        let ws_server_task = tokio::spawn(async move {
            println!("WS服务端已启动");
            println!("Port: {}", ws_server.port);
            println!("Key: {}", ws_server.key);

            if let Err(error) = listen(format!("0.0.0.0:{}", ws_server.port), |out| ServerHandler {
                out,
                key: ws_server.key.clone(),
                connections: Arc::new(Mutex::new(Vec::new())),
                pool: pool.clone(),
            }) {
                // 通知用户故障
                println!("创建 WebSocket 失败，原因: {:?}", error);
            }
        });
        // 启动 WebSocket 服务器
        let http_server_task = tokio::spawn(async move {
            // 启动 HTTP 服务器
            let config = Config::figment().merge(("port", ws_server.port));
            let _ = rocket::custom(config)
                .mount("/", routes![handle_request])
                .mount("/", routes![get_login_chat])
                .launch()
                .await;
        });

        // 等待 WebSocket 服务器和 HTTP 服务器的任务完成
        ws_server_task.await.unwrap();
        http_server_task.await.unwrap();
        // 等待线程完成
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
