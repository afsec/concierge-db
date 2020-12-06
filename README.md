# Concierge DB

Just serve a SQLite database through HTTP Rest API powered by Brickpack (https://github.com/afsec/brickpack/)

## Lean artifact (< 10 MB)
The whole microservice is built with static compiling using **MUSL** target.

## TODO

- [ ] Implement SQLite types: REAL, NULL, BLOB
- [ ] Implement env var defined LOG LEVEL
- [ ] Implement r2d2 pooling