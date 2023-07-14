
use mysql::{params, prelude::*};

use crate::{
    http_server::http_server::{null_403_http_get_responder, HttpGetResponder, Response},
    ser_config::{Player, Players, UserData, UserDataPerm},
};

pub fn create_tables_with_foreign_key(pool: &mysql::Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;

    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS `tb_permission` (
            `permission_id` int(11) UNIQUE NOT NULL COMMENT '权限id',
            `permission_name` varchar(255) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL COMMENT '权限name',
            PRIMARY KEY (`permission_id`) USING BTREE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8 ROW_FORMAT=COMPACT;
        INSERT IGNORE INTO tb_permission (permission_id, permission_name) VALUES (0, 'bds'), (99, 'root');
        "#,
    )?;
    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS `tb_player` (
            `pl_xuid` varchar(255) UNIQUE NOT NULL COMMENT '玩家唯一标识',
            `pl_name` varchar(255) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL COMMENT '玩家名字',
            `pl_llmoney` int(255) DEFAULT NULL COMMENT '玩家游戏余额',
            `pl_ip` varchar(255) DEFAULT NULL  COMMENT '玩家IP地址',
            `pl_online` int(2)  DEFAULT 0 COMMENT '玩家是否在线',
            `pl_device` varchar(11) DEFAULT NULL COMMENT '玩家设备',
            `pl_permission` int(11) DEFAULT 0 COMMENT '玩家权限等级',
            `pl_server_name` varchar(255) DEFAULT NULL COMMENT '玩家服务器',
            `online_time` int(255) DEFAULT NULL COMMENT '玩家在线时长',
            PRIMARY KEY (`pl_xuid`) USING BTREE,
            KEY `device_Key` (`pl_permission`) USING BTREE,
            CONSTRAINT `permission_Key` FOREIGN KEY (`pl_permission`) REFERENCES `tb_permission` (`permission_id`) ON DELETE SET NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8 ROW_FORMAT=COMPACT;
        "#,
    )?;
    Ok(())
}
pub fn insert_player(pool: &mysql::Pool, player: Player) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO tb_player (
            pl_xuid,
            pl_name,
            pl_llmoney,
            pl_ip,
            pl_online,
            pl_device,
            pl_server_name,
            online_time
        ) VALUES (
            :pl_xuid,
            :pl_name,
            :pl_llmoney,
            :pl_ip,
            :pl_online,
            :pl_device,
            :pl_server_name
            :online_time
        )",
        params! {
            "pl_xuid" => player.pl_xuid,
            "pl_name" => player.pl_name,
            "pl_llmoney" => player.pl_llmoney,
            "pl_ip" => player.pl_ip,
            "pl_online" => player.pl_online,
            "pl_device" => player.pl_device,
            "pl_server_name" => player.pl_server_name,
            "online_time" => player.online_time
        },
    )
}

pub fn getplayerInformation(pool: &mysql::Pool, name: &str) -> mysql::Result<Option<Player>> {
    let mut conn = pool.get_conn()?;
    let res = conn
        .exec_first(
            "SELECT
            tb_player.pl_xuid,
            tb_player.pl_name,
            tb_player.pl_llmoney,
            tb_player.pl_ip,
            tb_player.pl_online,
            tb_player.pl_server_name,
            tb_player.pl_device,
            tb_permission.permission_name,
            tb_player.online_time
        FROM
            tb_player
            JOIN tb_permission ON tb_player.pl_permission = tb_permission.permission_id
        WHERE
            tb_player.pl_name = :name",
            params! {
                "name" => name,
            },
        )
        .map(|row| {
            row.map(
                |(
                    pl_xuid,
                    pl_name,
                    pl_llmoney,
                    pl_ip,
                    pl_online,
                    pl_server_name,
                    pl_device,
                    permission_name,
                    online_time,
                )| Player {
                    pl_xuid,
                    pl_name,
                    pl_llmoney,
                    pl_ip,
                    pl_online,
                    pl_server_name,
                    pl_device,
                    permission_name,
                    online_time,
                },
            )
        });
    match res {
        Ok(Some(player)) => Ok(Some(player)),
        _ => Ok(None),
    }
}

pub fn get_playerspermissions(pool: &mysql::Pool) -> mysql::Result<Option<Players>> {
    let mut conn = pool.get_conn()?;
    let res = conn.exec_iter(
        "SELECT
            tb_player.pl_xuid,
            tb_player.pl_name,
            tb_player.pl_llmoney,
            tb_player.pl_ip,
            tb_player.pl_online,
            tb_player.pl_server_name,
            tb_player.pl_device,
            tb_player.online_time,
            tb_permission.permission_name
        FROM
            tb_player
            JOIN tb_permission ON tb_player.pl_permission = tb_permission.permission_id",
        (),
    );
    // println!("{:?}",res);
    let mut players = Vec::new();
    for row in res? {
        let row = row?;
        let player = Player {
            pl_xuid: row.get("pl_xuid").unwrap(),
            pl_name: row.get("pl_name").unwrap(),
            pl_llmoney: row.get("pl_llmoney").unwrap(),
            pl_ip: row.get("pl_ip").unwrap(),
            pl_online: row.get("pl_online").unwrap(),
            pl_server_name: row.get("pl_server_name").unwrap(),
            pl_device: row.get("pl_device").unwrap(),
            permission_name: row.get("permission_name").unwrap(),
            online_time: row.get("online_time").unwrap(),
        };
        // let json_string = serde_json::to_string(&player).unwrap();
        players.push(player);
    }

    if players.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Players { players }))
    }
}

pub fn update_player(pool: &mysql::Pool, player: Player) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "UPDATE tb_player SET
         pl_ip = :ip,
         pl_online = :online,
         pl_server_name = :server_name,
         pl_device = :device,
         online_time = :online_time
         WHERE pl_name = :name",
        params! {
            "ip" => player.pl_ip,
            "online" => player.pl_online,
            "server_name" => player.pl_server_name,
            "device" => player.pl_device,
            "online_time" => player.online_time,
            "name" => player.pl_name,
        },
    )?;
    Ok(())
}

pub fn on_leftupdate_player(pool: &mysql::Pool, player: Player) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "UPDATE tb_player SET
         pl_llmoney = :llmoney,
         pl_online = :online,
         pl_server_name = :server_name,
         pl_device = :device,
         online_time = :online_time
         WHERE pl_name = :name",
        params! {
            "llmoney" => player.pl_llmoney,
            "online" => player.pl_online,
            "server_name" => player.pl_server_name,
            "device" => player.pl_device,
            "name" => player.pl_name,
            "online_time" => player.online_time,
        },
    )?;
    Ok(())
}

// 添加权限
pub fn insert_perm(pool: &mysql::Pool, user_dataperm: UserDataPerm) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO tb_permission (
            permission_id,
            permission_name
        ) VALUES (
            :permission_id,
            :permission_name
        )",
        params! {
            "permission_id" => user_dataperm.perm_int,
            "permission_name" => user_dataperm.perm_str,
        },
    )?;

    Ok(())
}

// 删除权限
pub fn delete_perm(pool: &mysql::Pool, user_dataperm: UserDataPerm) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "DELETE FROM tb_permission
        WHERE permission_id = :permission_id",
        params! {
            "permission_id" => user_dataperm.perm_int,
        },
    )?;

    Ok(())
}


pub fn update_perm(pool: &mysql::Pool, user_dataperm: UserDataPerm) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "UPDATE tb_permission SET
        permission_name = :permission_name
         WHERE permission_name = :permission_name",
        params! {
            "permission_name" => user_dataperm.perm_int,
        },
    )?;
    Ok(())
}

// 获取玩家权限等级
pub fn getplayerpermission_grade(pool: &mysql::Pool, name: &str) -> mysql::Result<Option<u128>> {
    let mut conn = pool.get_conn()?;
    let res = conn
        .exec_first(
            "SELECT
            pl_permission
        FROM
            tb_player
        WHERE
            pl_name = :name",
            params! {
                "name" => name,
            },
        )
        .map(|row| {
            row.map(
                |(pl_permission,)| {
                    pl_permission
                },
            )
        });
    match res {
        Ok(Some(player)) => Ok(Some(player)),
        _ => Ok(None),
    }
}
