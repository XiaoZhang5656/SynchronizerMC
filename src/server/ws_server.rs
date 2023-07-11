use std::sync::{Arc, Mutex};

use crate::ser_config::Player;
use serde_json;
use ws::{Handler, Handshake, Message, Result};
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
        

        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        let mut connections = self.connections.lock().unwrap();
        // 移除已关闭的连接
        connections.retain(|sender| sender.connection_id() != self.out.connection_id());
    }
}

fn parse_player_data(player_data: &serde_json::Value) -> Player {
    Player {
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
        online: player_data
            .get("online")
            .and_then(|game_perm| game_perm.as_str())
            .unwrap_or("true")
            .to_string(),
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
            .unwrap_or(0) as i32,
        data: player_data
            .get("data")
            .map(|data| data.to_string())
            .unwrap_or_else(|| "".to_string()),
    }
}