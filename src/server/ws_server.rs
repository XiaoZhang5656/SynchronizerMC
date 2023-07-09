use std::sync::{Arc, Mutex};

use crate::server::mysql_util::*;
use ws::{Handler, Handshake, Message, Result};

use serde::{Deserialize, Serialize};
use serde_json;
pub(crate) struct ServerHandler {
    pub(crate) out: ws::Sender,
    pub(crate) key: String,
    pub(crate) connections: Arc<Mutex<Vec<ws::Sender>>>,
    pub(crate) pool: mysql::Pool,
}
#[derive(Debug, Deserialize)]
struct WsData {
    key: String,
    typestr: String,
    data: String,
}
#[derive(Serialize, Deserialize)]
struct Person {
    typestr: String,
    data: String,
}
#[derive(Serialize, Deserialize)]
struct PlayerPws {
    name: String,
    chat: String,
}
#[derive(Serialize)]
struct ResponseData {
    code: String,
    msg: String,
}
impl Handler for ServerHandler {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        // 获取客户端地址
        let client_addr = shake.peer_addr;
        println!("新连接来自: {}", client_addr.unwrap());
        let mut connections = self.connections.lock().unwrap();
        connections.push(self.out.clone());

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // 处理在此连接上接收的消息
        println!("默认服务器收到消息 '{}'", msg);
        if let Ok(json_str) = msg.as_text() {
            if let Ok(ws_data) = serde_json::from_str::<WsData>(json_str) {
                // 检查 ws_data.key 是否与 self.key 相等
                if ws_data.key == self.key {
                    match ws_data.typestr.as_str() {
                        "broadcast" => {
                            let person = Person {
                                typestr: ws_data.typestr.clone(),
                                data: ws_data.data.clone(),
                            };
                            let connections = self.connections.lock().unwrap();
                            for sender in connections.iter() {
                                let json_string = serde_json::to_string(&person).unwrap();
                                println!("chat类型分布消息： {}", json_string);
                                if let Err(err) = sender.send(json_string) {
                                    println!("发送消息失败: {:?}", err);
                                }
                            }
                        }
                        "comm" => {
                            let person = Person {
                                typestr: ws_data.typestr.clone(),
                                data: ws_data.data.clone(),
                            };
                            let connections = self.connections.lock().unwrap();
                            for sender in connections.iter() {
                                let json_string = serde_json::to_string(&person).unwrap();
                                println!("comm类型分布指令： {}", json_string);
                                if let Err(err) = sender.send(json_string) {
                                    println!("发送指令失败: {:?}", err);
                                }
                            }
                        }
                        "insert_player" => {
                            let player_json =
                                serde_json::from_str::<serde_json::Value>(&ws_data.data.as_str());
                            if let Ok(player_data) = player_json {
                                // 执行插入玩家数据的操作
                                let player_instance = Player {
                                    name: player_data
                                        .get("name")
                                        .and_then(|name| name.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    xuid: player_data
                                        .get("xuid")
                                        .and_then(|xuid| xuid.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    chat_perm: player_data
                                        .get("chat_perm")
                                        .and_then(|chat_perm| chat_perm.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    game_perm: player_data
                                        .get("game_perm")
                                        .and_then(|game_perm| game_perm.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    online: player_data
                                        .get("online")
                                        .and_then(|online| online.as_i64())
                                        .map(|val| val != 0)
                                        .unwrap_or(false),
                                    ip: player_data
                                        .get("ip")
                                        .and_then(|ip| ip.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    last_server: player_data
                                        .get("last_server")
                                        .and_then(|last_server| last_server.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    balance: player_data
                                        .get("balance")
                                        .and_then(|balance| balance.as_i64())
                                        .unwrap_or(0)
                                        as i32,
                                    data: player_data
                                        .get("data")
                                        .map(|data| data.to_string())
                                        .unwrap_or_else(|| "".to_string()),
                                };

                                let result = insert_player(&self.pool, &player_instance);
                                match result {
                                    Ok(_) => println!("玩家{}提交数据", &player_instance.name),
                                    Err(_err) => {
                                        let _ = update_player(&self.pool, &player_instance);

                                        let result =
                                            get_player_by_name(&self.pool, &player_instance.name);

                                        match result {
                                            Ok(Some(player)) => {
                                                let person = Person {
                                                    typestr: "updata".to_owned(),
                                                    data: serde_json::to_string(&player).unwrap(),
                                                };
                                                let connections = self.connections.lock().unwrap();
                                                for sender in connections.iter() {
                                                    let json_string =
                                                        serde_json::to_string(&person).unwrap();
                                                    println!(
                                                        "存在返回类型分布消息： {}",
                                                        json_string
                                                    );
                                                    if let Err(err) = sender.send(json_string) {
                                                        println!(
                                                            "存在返回类型发送消息失败: {:?}",
                                                            err
                                                        );
                                                    }
                                                }
                                            }
                                            Ok(None) => println!("Player not found"),
                                            Err(err) => println!("Error: {:?}", err),
                                        }
                                    }
                                }
                            } else {
                                println!("无法解析玩家数据");
                            }
                        }
                        _ => {
                            // 对于其他未知的typestr值执行默认操作
                            println!("Unknown typestr: {}", ws_data.typestr);
                        }
                    }
                }
            } else {
                // 无法将消息解析为Ws_Data结构体
                println!("Failed to parse Ws_Data from message");
                let _ = self.out.close(ws::CloseCode::Other(404));
            }
        } else {
            let _ = self.out.close(ws::CloseCode::Other(404));
        }

        Ok(())
    }
    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        let mut connections = self.connections.lock().unwrap();
        // 移除已关闭的连接
        connections.retain(|sender| sender.connection_id() != self.out.connection_id());
    }
}
