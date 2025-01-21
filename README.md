<p style="font-size: 18px;">1) First of all setup database named <i>"test"</i> on postgres as mentioned on .env file. You can name any variable any thing.</p>
<p style="font-size: 18px;">2) Generate postgres table with following code on <i>test</i> named database.</p>
<p style="font-size: 16px;">
<i>CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL
);</i></p>

3) After enter "diesel print-schema > src/schema.rs" on terminal to generate schema.rs file as per your postgres table configuration. You can delete schema.rs from this clone and can generate that with that command.
4) Then enter "cargo run"
To get users enter "curl http://127.0.0.1:8080/users" or can open from browser with localhost:8080/users
To add users enter <p style="font-size: 20px;">"curl -X POST http://127.0.0.1:8080/users \
<p style="font-size: 20px;">-H "Content-Type: application/json" \
<p style="font-size: 20px;">-d '{"name": "Alice", "email": "alice@example.com"}'"
