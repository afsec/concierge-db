# Concierge DB

Just serve a SQLite database through HTTP Rest API

## Lean artifact (~4 MB)
The whole microservice is built with static compiling using **MUSL** target.
```
$ ls -lh
total 3,7M
-rwxrwxr-x 1 user user 3,6M mai 15 23:11 concierge-db
```

## Startup message
```
$ cd dist/
$ ./concierge-db 
concierge-db v0.1.0
Listening at: http://0.0.0.0:3341
```
