use mysql::{params, prelude::*};

use crate::ser_config::Player;

pub fn create_tables_with_foreign_key(pool: &mysql::Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;

    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            xuid VARCHAR(255) PRIMARY KEY,
            name VARCHAR(255) UNIQUE,
            online VARCHAR(255),
            ip VARCHAR(255),
            last_server VARCHAR(255),
            money INT,
            data JSON
        )
        "#,
    )?;

    conn.query_drop(
        r#"
    CREATE TABLE IF NOT EXISTS permissions (
        id INT PRIMARY KEY AUTO_INCREMENT,
        player_name VARCHAR(255),
        chat VARCHAR(20) DEFAULT 'true',
        FOREIGN KEY (player_name) REFERENCES players(name) ON DELETE CASCADE
    )
    "#,
    )?;

    Ok(())
}

pub fn insert_player(pool: &mysql::Pool, player: &Player) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r#"
        INSERT INTO players (name, xuid,online, ip, last_server, balance, permissionID)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        (
            &player.name,
            &player.xuid,
            &player.online,
            &player.ip,
            &player.last_server,
            &player.balance,
            &player.data,
        ),
    )?;
    conn.exec_drop(
        r#"
        INSERT INTO permissions (player_name)
        VALUES (?)
        "#,
        (&player.name,),
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
                online: row.get("online").unwrap(),
                ip: row.get("ip").unwrap(),
                last_server: row.get("last_server").unwrap(),
                balance: row.get("balance").unwrap(),
                data: row.get("data").unwrap(),
            }
        });

    Ok(res)
}
pub fn getplayerpermissions(pool: &mysql::Pool, name: &str) -> mysql::Result<String> {
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
                online: row.get("online").unwrap(),
                ip: row.get("ip").unwrap(),
                last_server: row.get("last_server").unwrap(),
                balance: row.get("money").unwrap(),
                data: row.get("data").unwrap(),
            }
        })
        .map(|player| serde_json::to_string(&player).unwrap_or_default())
        .unwrap_or_default(); // If no player found, return an empty string

    Ok(res)
}

pub fn update_player(pool: &mysql::Pool, player: &Player) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r#"
        UPDATE players
        SET online = :online, ip = :ip, last_server = :last_server, balance = :balance, data = :data
        WHERE name = :name
        "#,
        params! {
            "online" => &player.online,
            "ip" => &player.ip,
            "last_server" => &player.last_server,
            "balance" => player.balance,
            "data" => &player.data,
            "name" => &player.name,
        },
    )?;
    Ok(())
}
