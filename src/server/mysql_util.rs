use mysql::{params, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub xuid: String,
    pub chat_perm: String,
    pub game_perm: String,
    pub online: bool,
    pub ip: String,
    pub last_server: String,
    pub(crate) balance: i32,
    pub data: String,
}



pub fn create_player_table(pool: &mysql::Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(255),
            xuid VARCHAR(255) UNIQUE,
            chat_perm VARCHAR(255),
            game_perm VARCHAR(255),
            online BOOLEAN,
            ip VARCHAR(255),
            last_server VARCHAR(255),
            balance INT,
            data JSON
        )        
        "#,
    )?;
    Ok(())
}
pub fn insert_player(
    pool: &mysql::Pool,
    player: &Player,
) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r#"
        INSERT INTO players (name, xuid, chat_perm, game_perm, online, ip, last_server, balance, data)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        (
            &player.name,
            &player.xuid,
            &player.chat_perm,
            &player.game_perm,
            &player.online,
            &player.ip,
            &player.last_server,
            &player.balance,
            &player.data,
        ),
    )?;
    Ok(())
}

pub fn get_player_by_name(pool: &mysql::Pool, name: &str) -> mysql::Result<Option<Player>> {
    let mut conn = pool.get_conn()?;
    let res = conn
        .exec_first(
            "SELECT * FROM players WHERE name = :name",
            params! {
                "name" => name,
            },
        )?
        .map(|row| {
            let row: mysql::Row = row;
            Player {
                name: row.get("name").unwrap(),
                xuid: row.get("xuid").unwrap(),
                chat_perm: row.get("chat_perm").unwrap(),
                game_perm: row.get("game_perm").unwrap(),
                online: row.get("online").unwrap(),
                ip: row.get("ip").unwrap(),
                last_server: row.get("last_server").unwrap(),
                balance: row.get("balance").unwrap(),
                data: row.get("data").unwrap(),
            }
        });

    Ok(res)
}

pub fn update_player(
    pool: &mysql::Pool,
    player: &Player,
) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r#"
        UPDATE players
        SET chat_perm = :chat_perm, game_perm = :game_perm, online = :online, ip = :ip, last_server = :last_server, balance = :balance, data = :data
        WHERE name = :name
        "#,
        params! {
            "chat_perm" => &player.chat_perm,
            "game_perm" => &player.game_perm,
            "online" => player.online,
            "ip" => &player.ip,
            "last_server" => &player.last_server,
            "balance" => player.balance,
            "data" => &player.data,
            "name" => &player.name,
        },
    )?;
    Ok(())
}

