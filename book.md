 ### Post   ：http://fanbk.tpddns.cn:8082/getplayerall
 ---
 请求要求：
 请求头
 ```
 Content-Type

 application/json
 ```
 请求体
```json
{
    "name": "banchen21",
    "pws": "11ce903d1c5ed3dfe86d6841bd40b30d",
    "t": "99973077"
}
```
返回内容
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
---
 ### Post ：http://fanbk.tpddns.cn:8082/getLoginChat
 请求头
 ```
 Content-Type

application/json
 ```
 请求体
```json
{
    "name": "banchen21",
    "t": "11ce903d1c5ed3dfe86d6841bd40b30d",
    "pws": "99973077"
}
```
 返回内容
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
---
### Get http://fanbk.tpddns.cn:8082/getMessageauthority?name=banchen21
返回内容
```json
 {"message":"bds"}
```


备注：
```
Ws端启动参数：

Routes:
   >> (get_login_chat) POST /getLoginChat/ application/x-www-form-urlencoded
   >> (getplayerall) POST /getplayerall/ application/x-www-form-urlencoded
   >> (getpermissions) GET /getpermissions/?<name>
   >> (get_messageauthority) POST /getMessageauthority/ application/x-www-form-urlencoded

```