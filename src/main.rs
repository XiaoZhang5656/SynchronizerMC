use rocket::catchers;
//rocket框架配置
use rocket::routes;
use rocket::Config as OtherConfig;
extern crate rocket;

// 结构体引用
mod ser_config;
use ser_config::Config;

// yml工具引用
mod yml_util;
use yml_util::generate_random_key;

// use std::error::Error;
// 异步
use std::{
    error::Error,
    fs,
    sync::{Arc, Mutex},
    thread,
};

// ws服务端
use ws::listen;
mod server;
use crate::server::ws_server::ServerHandler;

// MySQL
use mysql::Pool;
use server::mysql_util::create_tables_with_foreign_key;

// http服务端
mod http_server;
use crate::http_server::http_server::*;

//MySQL的Pool连接-全局变量
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref POOL: Mutex<Option<Pool>> = Mutex::new(None);
}

// 美化输出
use colored::*;

// 异步启动
#[tokio::main]
async fn main() {
    let file_path = "config.yml";

    inti_config(file_path.to_string().clone());

    match read_yml_to_str(&file_path).await {
        Ok(config) => {
            start_mysql(config.clone());

            let ws_server_task = tokio::spawn(start_ws_server(config.clone()));
            let http_server_task = tokio::spawn(start_http_server(config.clone()));

            tokio::try_join!(ws_server_task, http_server_task).unwrap();
        }
        Err(err) => {
            println!("读取配置文件失败：{}", err);
            // 错误处理逻辑...
        }
    }
    // 等待线程完成
    thread::sleep(std::time::Duration::from_secs(1));
}

//初始化
fn inti_config(file_path: String) {
    let key = generate_random_key(16);
    let config = crate::ser_config::Config {
        database_host: "localhost".to_string(),
        database_port: 3306,
        database_dataname: "my_test".to_string(),
        database_password: "123456".to_string(),
        database_username: "bc_test".to_string(),
        ws_port: 8081,
        ws_key: key,
        http_port: 8082,
    };
    match fs::metadata(&file_path) {
        Err(_) => {
            let text = "文件不存在, 开始写入".to_string();
            println!("{}", text.yellow());
            if let Err(err) = yml_util::write_config_to_yml(&config, &file_path) {
                println!("无法写入配置文件：{}", err);
            }
        }
        Ok(_) => {
            let text = "配置文件存在".to_string();
            println!("{}", text.green());
        }
    }
}

// 读取配置文件
async fn read_yml_to_str(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config = yml_util::read_yml(file_path)?;
    Ok(config)
}

// 建立数据库连接并返回连接池
fn establish_connection(config: &Config) -> Pool {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.database_username,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_dataname
    );
    mysql::Pool::new(url).expect("Failed to create connection pool")
}

// 启动MySQL连接
fn start_mysql(config: Config) {
    // 使用建立连接的函数
    let pool: Pool = establish_connection(&config);

    *POOL.lock().unwrap() = Some(pool.clone());
    println!("{}", "Mysql已连接".green());
    // 创建玩家表
    create_tables_with_foreign_key(&pool).expect("Failed to create player table");
}

// 启动ws端
async fn start_ws_server(config: Config) {
    // 启动 WebSocket 服务器
    let ws_server_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let text = "WS服务端已启动".to_string();
        println!("{}", text.green());
        println!("WS服务端Port: {}", config.ws_port);
        println!("WS服务端Key: {}", config.ws_key);

        if let Err(error) = listen(format!("0.0.0.0:{}", config.ws_port), |out| ServerHandler {
            out,
            key: config.ws_key.clone(),
            connections: Arc::new(Mutex::new(Vec::new())),
        }) {
            // 通知用户故障
            println!("创建 WebSocket 失败，原因: {:?}", error);
        }
    });
    let text = "Http服务端已启动".to_string();
    println!("{}", text.green());
    println!("Http服务端port: {}", config.http_port);
    ws_server_task.await.unwrap();
}

// 启动http端
async fn start_http_server(config: Config) {
    // 启动 HTTP 服务器
    let http_server_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let config = OtherConfig::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", config.http_port));
        let _ = rocket::custom(config)
            .mount("/getpermissions", routes![get_permissions])
            .mount("/getLoginChat", routes![get_login_chat])
            .mount("/getplayerall", routes![getplayerall])
            .mount("/perm_mg", routes![perm_mg])
            .register("/", catchers![not_found])
            // .mount("/", routes![index])
            .launch()
            .await;
    });
    http_server_task.await.unwrap();
}
