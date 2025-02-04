# rust-notes-api

REST API development using Axum with Rust

Run project:

cargo run


Example (cURL) (bash):

curl -X GET http://127.0.0.1:3000/notes

curl -X POST http://127.0.0.1:3000/notes \     -H "Content-Type: application/json" \     -d '{"id": 1, "content": "Axum framework"}'

curl -X DELETE http://127.0.0.1:3000/notes/1


Example (PowerShell):

Invoke-WebRequest -Uri "http://127.0.0.1:3000/notes" -Method Get

Invoke-WebRequest -Uri "http://127.0.0.1:3000/notes" -Method Post -Headers @{"Content-Type" = "application/json"} -Body (@{id=1; content="Axum framework"} | ConvertTo-Json)

Invoke-WebRequest -Uri "http://127.0.0.1:3000/notes/1" -Method Delete
