# rs-distributed-compute
Distributed Computation for ML training with task management in Rust

### NATS Demo cluster and Code example 

Demo - `demo.nats.io`

[Nats by Example](https://natsbyexample.com/)

### MongoDB 

[Reference](https://www.mongodb.com/docs/languages/rust/)

### Configuration format 

- Flat structure like Springboot's `application.properties`
- In a `toml` file

### Status of a task

| Status     | Caller  | Actor   | Comment                               |
|------------|---------|---------|---------------------------------------|
| CREATED    | User    | Manager | POST /TASK                            |
| PENDING    | Manager | Manager | MANAGER CODE BEFORE PUBLISH TASK      |
| COMPLETED  | Worker  | Manager | MANAGER CONSUMES COMPLETION EVENT     |
| DISCARDED  | User    | Manager | PATCH /TASK                           |

### DB Details

Collection = 2

Status Collection:

1. id -> uuid, string
2. status -> status of task, string
3. created_at -> creation date, date
4. updated_at -> update date, date
5. worker -> worker id, string
6. wait_time -> wait duration before task started in ms, int
7. execution_time -> execution time of task in ms, int
8. task_log_id -> task log id


Task Log Collection:

1. id -> uuid, string
2. inp_num -> input number, number
3. sum -> sum of prime numbers less than equal to inp_num



### Web framework reference

[Reference](https://docs.rs/axum/0.7.6/axum/index.html)

[Example](https://github.com/tokio-rs/axum/tree/main/examples)