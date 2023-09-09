# Todo Rust api
Get ready to rustle up some todos with our Rust-powered API! Actix Web brings the speed, and Rust ensures our code is as safe as a vault. Our API will so fast that we have to rewrite the laws of physics in Rust and Actix Web just to keep up with it! 🚀😄
# Dependencies
```toml
actix-cors = "0.6.4"
actix-web = "4.4.0"
chrono = { version= "0.4.30", features = ["serde"]}
env_logger = "0.10.0"
serde = { version = "1.0.188", features = ["derive"]}
uuid = { version = "1.4.1", features = ["v4"]}
```
# Create a new Todo items

```bash
curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy milk", "description": "Buy 2 litres of milk"}' http://localhost:8080/api/todos
```
```json
{
  "id": "bb0ae8fc-92de-414d-a1aa-310d32c6398d",
  "title": "Buy milk",
  "description": "Buy 2 litres of milk",
  "created_at": "2023-09-09T08:25:08.668394954Z",
  "updated_at": "2023-09-09T08:25:08.668397813Z"
}
```

# Get all Todo items
```bash
curl -s http://localhost:8080/api/todos | jq
```
```json
[
  {
    "id": "bb0ae8fc-92de-414d-a1aa-310d32c6398d",
    "title": "Buy milk",
    "description": "Buy 2 litres of milk",
    "created_at": "2023-09-09T08:25:08.668394954Z",
    "updated_at": "2023-09-09T08:25:08.668397813Z"
  }
]
```
# Get a Todo item by id
```bash
curl -s http://localhost:8080/api/todos/bb0ae8fc-92de-414d-a1aa-310d32c6398d | jq
```
```json
{
  "id": "bb0ae8fc-92de-414d-a1aa-310d32c6398d",
  "title": "Buy milk",
  "description": "Buy 2 litres of milk",
  "created_at": "2023-09-09T08:25:08.668394954Z",
  "updated_at": "2023-09-09T08:25:08.668397813Z"
}
```
# update a Todo item by id
```bash
curl -s -X PUT -H "Content-Type: application/json" -d '{"title":"Buy a skateboard", "description": "Buy 2 skateboards"}' http://localhost:8080/api/todos/bb0ae8fc-92de-414d-a1aa-310d32c6398d | jq
```
```json
{
  "id": "bb0ae8fc-92de-414d-a1aa-310d32c6398d",
  "title": "Buy a skateboard",
  "description": "Buy 2 skateboards",
  "created_at": "2023-09-09T08:25:08.668394954Z",
  "updated_at": "2023-09-09T08:25:08.668397813Z"
}
```
# delete a Todo item by id
```bash
curl -s -X DELETE http://localhost:8080/api/todos/bb0ae8fc-92de-414d-a1aa-310d32c6398d | jq
```
```json
{
  "id": "bb0ae8fc-92de-414d-a1aa-310d32c6398d",
  "title": "Buy a skateboard",
  "description": "Buy 2 skateboards",
  "created_at": "2023-09-09T08:25:08.668394954Z",
  "updated_at": "2023-09-09T08:25:08.668397813Z"
}
```

