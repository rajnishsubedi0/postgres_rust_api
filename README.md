<p style="font-size: 18px;">1) First of all setup database named <i>"test"</i> on postgres as mentioned on .env file. You can name any variable any thing.</p>
<p style="font-size: 18px;">2) Generate postgres table with following code on <i>test</i> named database from terminal.</p>
<p style="font-size: 16px;">
    
```
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL
);
```
3) After that enter ```diesel print-schema > src/schema.rs``` on terminal to generate schema.rs file as per your postgres table configuration. You can delete schema.rs from this clone and can generate that with that command.
4) Then enter ```cargo run```
5) To get users enter
```
curl http://127.0.0.1:8080/users
```
 or can open from browser with 
 ```
 localhost:8080/users
```
To add users enter
```
curl -X POST http://127.0.0.1:8080/users \
-H "Content-Type: application/json" \
-d '{"name": "Alice", "email": "alice@example.com"}'
```
or
You can also send data from rest api client on json format.
