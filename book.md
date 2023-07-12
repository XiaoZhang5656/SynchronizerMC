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
  - 404 Not Found：未找到玩家权限信息
  - 其他错误状态码
---
### `POST /getLoginChat`

该接口用于获取登录玩家的信息。
- 请求头：`"Content-Type: application/json"`
- 请求体参数：
```json
{
    "name": "banchen20",
    "token": "0ff04d57217e99641670e20d58a88c62",
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
  - 玩家验证失败：
    - 状态码：404 Not Found
    - 响应体：`{"message": "false"}`
  - 未找到玩家权限信息：
    - 状态码：403 Forbidden
    - 响应体：`{"message": "null"}`
  - 其他错误：
    - 状态码：403 Forbidden
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
  - 403 Forbidden : 客户端权限不足
  - 404 Not Found：密钥验证失败
  - 其他错误状态码

## 异常处理

### 404 Not Found

当访问的路径不存在时，返回提示信息和文档地址。

- 路径：任意不存在的路径
- 响应：返回提示信息和文档地址

## 其他说明
 -POST 请求格式
 - JSON

-GET 请求格式
 - 参数
 - 示例
```
http://127.0.0.1:8082/getpermissions?name=banchen21
```
