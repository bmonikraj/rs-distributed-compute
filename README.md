# rs-distributed-compute
Distributed Computation for ML training with task management in Rust

### Minio Play Console and Server 

[Reference](https://min.io/docs/minio/macos/administration/minio-console.html#minio-console)

### NATS Demo cluster and Code example 

Demo - `demo.nats.io`

[Nats by Example](https://natsbyexample.com/)

### SQLite 

[Reference](https://www.sqlite.org/)

### Configuration format 

- Flat structure like Springboot's `application.properties`
- In a `toml` file

### Status of a task

| Status     | Caller  | Actor   | Comment                               |
|------------|---------|---------|---------------------------------------|
| CREATED    | User    | Manager | POST /TASK                            |
| PENDING    | Manager | Manager | MANAGER CODE BEFORE PUBLISH TASK      |
| COMPLETED  | Worker  | Manager | MANAGER CONSUMES COMPLETION EVENT     |
| DISCARDING | User    | Manager | PATCH /TASK                          |
| DISCARDED  | Worker  | Manager | MANAGER CONSUMES MODEL DELETION EVENT |

### DB Details

Table = 1

Table Columns
1. id -> uuid, string
2. data -> url, string
3. model_type -> model type, string
4. model_path -> minio model path, string
5. status -> status of task, string
6. created_at -> creation date, date
7. updated_at -> update date, date
8. description -> description of task, string [patchable]
9. worker -> worker id, string
10. wait_time -> wait duration before task started in ms, int
11. execution_time -> execution time of task in ms, int
12. is_discarded -> bool [patchable] (is_discarded==false, model discard, set=true; Else error)


### Web framework reference

[Reference](https://docs.rs/axum/0.7.6/axum/index.html)

[Example](https://github.com/tokio-rs/axum/tree/main/examples)