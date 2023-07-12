use std::sync::{Arc, Mutex};

use crate::{
    ser_config::{DataChat, Person, Player, WsData},
    POOL,
};
use serde_json;
use ws::{Handler, Handshake, Message, Result};

use super::mysql_util::{getplayerpermissions, insert_player, on_leftupdate_player, update_player};
pub(crate) struct ServerHandler {
    pub(crate) out: ws::Sender,
    pub(crate) key: String,
    pub(crate) connections: Arc<Mutex<Vec<ws::Sender>>>,
}
impl Handler for ServerHandler {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        // 获取客户端地址
        let client_addr = shake.peer_addr;
        println!("Ws端接受新连接来自: {}", client_addr.unwrap());
        let mut connections = self.connections.lock().unwrap();
        connections.push(self.out.clone());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // 处理在此连接上接收的消息
        println!("默认服务器收到消息 '{}'", msg);

        if let Ok(ws_data) = serde_json::from_str::<WsData>(msg.as_text().unwrap()) {
            if ws_data.key == self.key {
                on_msg_util(self, ws_data)
            } else {
                println!("密钥验证失败，自动断开'{:p}'", &self.out);
                let _ = self.out.close(ws::CloseCode::Other(404));
            }
        } else {
            println!("首次无法解析数据，自动断开'{:p}'", &self.out);
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

fn on_msg_util(_server_handler: &mut ServerHandler, ws_data: WsData) {
    let typestr = ws_data.typestr;
    match typestr.as_str() {
        "chat" => {
            if let Ok(ws_data) = serde_json::from_str::<DataChat>(&ws_data.data) {
                on_chat(_server_handler, ws_data)
            } else {
                println!("无法解析消息，自动断开'{:p}'", &_server_handler.out);
                let _ = _server_handler.out.close(ws::CloseCode::Other(404));
            }
        }
        "onJoin_player" => {
            // 玩家加入游戏

            if let Ok(player_str) = serde_json::from_str::<Player>(&ws_data.data) {
                let pool: mysql::Pool = POOL
                    .lock()
                    .unwrap()
                    .as_ref()
                    .expect("Pool not initialized")
                    .clone();
                match insert_player(&pool, player_str) {
                    Ok(()) => println!("插入成功！新玩家数据:{}", &ws_data.data),
                    Err(_err) => {
                        println!("插入失败！返回云端玩家数据:{}", &ws_data.data);

                        if let Ok(player_str) = serde_json::from_str::<Player>(&ws_data.data) {
                            match update_player(&pool, player_str) {
                                Ok(()) => {
                                    println!("成功更新玩家数据:{}", &ws_data.data)
                                }
                                Err(_err) => println!("更新玩家数据失败:{}", &ws_data.data),
                            }
                        }

                        if let Ok(player_str) = serde_json::from_str::<Player>(&ws_data.data) {
                            match getplayerpermissions(&pool,&player_str.pl_name) {
                                Ok(Some(player)) => {
                                    let json_string = serde_json::to_string(&player).unwrap();
                                    to_send_chat_bds(
                                        _server_handler.connections.clone(),
                                        "updata".to_owned(),
                                        "null".to_owned(),
                                        json_string,
                                    )
                                }
                                Ok(None) => {
                                    to_send_chat_bds(
                                        _server_handler.connections.clone(),
                                        "updata".to_owned(),
                                        "null".to_owned(),
                                        "null".to_owned(),
                                    )
                                }
                                Err(_err) => {
                                    to_send_chat_bds(
                                        _server_handler.connections.clone(),
                                        "updata".to_owned(),
                                        "null".to_owned(),
                                        "null".to_owned(),
                                    )
                                }
                            }
                        }
                    }
                }
            } else {
                println!("无法解析玩家信息，自动断开'{:p}'", &_server_handler.out);
                let _ = _server_handler.out.close(ws::CloseCode::Other(404));
            }
        }
        "updata" => {
            // 玩家退出游戏
            let pool: mysql::Pool = POOL
                .lock()
                .unwrap()
                .as_ref()
                .expect("Pool not initialized")
                .clone();
            if let Ok(player_str) = serde_json::from_str::<Player>(&ws_data.data) {
                match on_leftupdate_player(&pool, player_str) {
                    Ok(()) => {
                        println!("成功更新玩家数据:{}", &ws_data.data)
                    }
                    Err(_err) => println!("更新玩家数据失败:{}", &ws_data.data),
                }
            }
        }
        _ => {
            println!("未知数据类型，自动断开'{:p}'", &_server_handler.out);
            let _ = _server_handler.out.close(ws::CloseCode::Other(404));
        }
    }
}

fn on_chat(_server_handler: &mut ServerHandler, _datachat: DataChat) {
    let pool: mysql::Pool = POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("Pool not initialized")
        .clone();
    match getplayerpermissions(&pool, &_datachat.player) {
        Ok(Some(player)) => to_send_chat_bds(
            _server_handler.connections.clone(),
            "chat".to_owned(),
            player.permission_name,
            _datachat.chat,
        ),
        Ok(None) => to_send_chat_bds(
            _server_handler.connections.clone(),
            "chat".to_owned(),
            "null".to_owned(),
            _datachat.chat,
        ),
        Err(_err) => to_send_chat_bds(
            _server_handler.connections.clone(),
            "chat".to_owned(),
            "null".to_owned(),
            _datachat.chat,
        ),
    }
}
fn to_send_chat_bds(
    connections: Arc<Mutex<Vec<ws::Sender>>>,
    typestra: String,
    perm: String,
    msg: String,
) {
    let person = Person {
        typestr: typestra,
        perm,
        data: msg,
    };
    let connections = connections.lock().unwrap();
    for sender in connections.iter() {
        let json_string = serde_json::to_string(&person).unwrap();
        println!("玩家发送消息chat类型分布消息： {}", json_string);
        if let Err(err) = sender.send(json_string) {
            println!("玩家发送消息发送消息失败: {:?}", err);
        }
    }
}
