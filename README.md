# Concierge DB

Just serve a SQLite database through HTTP Rest API powered by Brickpack (https://github.com/afsec/brickpack/)

## Lean artifact (~4 MB)
The whole microservice is built with static compiling using **MUSL** target.

```
$ ls -lh
total 3,8M
-rwxrwxr-x 1 user user 3,7M mai 30 16:57 concierge-db
-rw-r--r-- 1 user user  36K mai 30 02:06 database.sqlite3
```

## Startup message
```
$ cd dist/
$ BP_SERVER_TOKEN="SomeRandomString" ./concierge-db 
{"level":30,"time":1590872751580,"msg":"Logger started","level":"Info"}
{"level":30,"time":1590872751580,"msg":"Starting App [concierge-db v0.4.5]:"}
{"level":30,"time":1590872751580,"msg":"Powered by Brickpack Web Framework v0.7.8"}
{"level":30,"time":1590872751580,"msg":"System Endpoints:"}
{"level":30,"time":1590872751580,"msg":"                       GET   - /"}
{"level":30,"time":1590872751580,"msg":"                       GET   - /auth"}
{"level":30,"time":1590872751581,"msg":"                       PATCH - /maintenance"}
{"level":30,"time":1590872751581,"msg":"Application Endpoints:"}
{"level":30,"time":1590872751581,"msg":"                       POST  - /api/count-rows"}
{"level":30,"time":1590872751581,"msg":"                       POST  - /api/read-all"}
{"level":30,"time":1590872751581,"msg":"                       POST  - /api/insert-row"}
{"level":30,"time":1590872751581,"msg":"                       POST  - /api/show-columns"}
{"level":30,"time":1590872751581,"msg":"                       POST  - /api/update-field"}
{"level":30,"time":1590872751581,"msg":"                       POST  - /api/show-tables"}
{"level":30,"time":1590872751581,"msg":"BP_SERVER_TOKEN: SomeRandomString"}
{"level":30,"time":1590872751582,"msg":"Server listening","address":"http://0.0.0.0:3341","target":"release","tls":false}

```
