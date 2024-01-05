rust試したくて作ったWebサーバ

### プロジェクト立ち上げ
```
docker-compose build
docker-compose run actix-web-todo cargo init
docker-compose run actix-web-todo cargo add actix-web
docker-compose run actix-web-todo cargo add rusqlite
docker-compose run actix-web-todo cargo add cargo-watch
```

### サンプルリクエスト
```
# Todo作成
curl -X POST http://localhost:3456/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"DemoTitle", "contents":"DemoContents"}'

# Todo取得
curl http://localhost:3456/todos/1
```

### sqlite
```bash
docker exec -it actix-web-todo_actix-web-todo_1 bash
sqlite3 todo.db
```
