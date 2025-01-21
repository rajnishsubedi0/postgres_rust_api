<p style="font-size: 18px;">1) First of all setup database named <i>test</i> on postgres as mentioned on .env file. You can name any variable any thing.</p><br><br>
<p style="font-size: 18px;">2) Generate postgres table with following code on <i>test</i> named database.</p><br>
<p style="font-size: 16px;">
<i>CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL
);</i></p>

