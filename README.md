# rs-distributed-compute
Distributed Computation for Finding sum of Prime numbers less than `N` with task management in Rust

### Postgres and ORM

[ORM](https://www.sea-ql.org/SeaORM/docs/index/)

### Configuration format 

- Flat structure like Springboot's `application.properties`
- In a `toml` file

### DB Details

Table = 1

1. id -> uuid, string
2. algorithm -> name of algorithm, string
3. input -> input param, string
4. output -> result, string
5. created_at -> creation date, date
6. updated_at -> update date, date

### Web framework reference

[Reference](https://docs.rs/axum/0.7.6/axum/index.html)

[Example](https://github.com/tokio-rs/axum/tree/main/examples)

### Parallelism map, filter, reduce 

[Exploring concurrency in rust](https://codedamn.com/news/rust/advanced-concurrency-rust-exploring-parallelism-rayon)

[Map-reduce in parallel - rust cookbook](https://github.com/rust-lang-nursery/rust-cookbook/blob/master/src/concurrency/parallel/rayon-map-reduce.md)

`Use rayon join if the algorithm can be divided and conquered using recursion`