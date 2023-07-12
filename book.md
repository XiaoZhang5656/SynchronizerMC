# 项目文档

## 接口说明

### `GET /getpermissions`

获取玩家权限信息。

- 请求方式：GET
- 路径：/getpermissions
- 参数：
  - name：玩家名称
- 响应：
  - 200 OK：成功，返回玩家权限信息
    - 响应体：`{"message":"bds"}`
  - 404 Not Found：资源不存在
    - 响应体：`{"message":"null"}`
---
### `POST /getLoginChat`

该接口用于获取登录玩家的信息。
- 请求头：`"Content-Type: application/json"`
- 请求体参数：
```json
{
    "name": "banchen21",
    "token": "65f254760c19ef38a9e808478fcef633",
    "t": "99973077"
}
```
- 响应：
  - 成功：
    - 状态码：200 OK
    - 响应体：包含玩家权限信息的 JSON 字符串
    ```json
    {
    "pl_xuid": "2535447156610197",
    "pl_name": "banchen21",
    "pl_llmoney": 18889699,
    "pl_ip": "127.0.0.1:19137",
    "pl_online": "true",
    "pl_server_name": "天台一号",
    "pl_device": "Win10",
    "permission_name": "bds"
    }
    ```

  - 验证失败：
    - 状态码：403 Forbidden
    - 响应体：`{"message": "false"}`
  - 未找到玩家登录信息：
    - 状态码：404 Not Found
    - 响应体：`{"message": "null"}`
  - 服务端无法处理这种未知错误：
    - 状态码：500 Internal Server Error
    - 响应体：`{"message": "null"}`
---
### `POST /getplayerall`

获取所有玩家权限信息。

- 请求方式：POST
- 路径：/getplayerall
- 参数：
  - 用户数据（JSON）：
    - name：玩家名称
    - t：t 值
    - token：令牌
- 响应：
  - 200 OK：成功，返回所有玩家权限信息
  ```json
    {
    "players": [
        {
            "pl_xuid": "13213",
            "pl_name": "xiao",
            "pl_llmoney": 2222,
            "pl_ip": "127.0.0.1",
            "pl_online": "true",
            "pl_server_name": "零",
            "pl_device": "android",
            "permission_name": "bds"
        },
        {
            "pl_xuid": "2535447156610197",
            "pl_name": "banchen21",
            "pl_llmoney": 18889699,
            "pl_ip": "127.0.0.1:19137",
            "pl_online": "true",
            "pl_server_name": "天台一号",
            "pl_device": "Win10",
            "permission_name": "bds"
        }
    ]
    }
    ```
  - 验证失败：
    - 状态码：403 Forbidden
    - 响应体：`{"message": "false"}`
  - 未找到玩家登录信息：
    - 状态码：404 Not Found
    - 响应体：`{"message": "null"}`
  - 服务端无法处理这种未知错误：
    - 状态码：500 Internal Server Error
    - 响应体：`{"message": "null"}`


## 其他说明
 -POST 请求格式
 - JSON

-GET 请求格式
 - 参数
 - 示例
```
http://127.0.0.1:8082/getpermissions?name=banchen21
```
