# RUST Practice
## Rocket web framework
#### _This requires nightly version of rust (for rocket framework)_
~~~
rustup override set nightly
~~~

### Diesel ORM & Postgres database
Setup database url as below in _.env_ file: See .env.example.
~~~
DATABASE_URL=postgres://<username>:<passwd>@<db_url>/<dbname>
~~~

Diesel installation: 
~~~
cargo install diesel_cli --no-default-features --features postgres
~~~
Diesel setup in project:
~~~
diesel setup
~~~

Migration folder for diesel ORM. This includes schema details for database setup.
~~~
migrations/0000*/up.sql >> define schema  
migrations/0000*/down.sql >> for reversal
~~~

Run diesel migrations (once DATABASE_URL) is correct: 
~~~
diesel migration run
~~~




