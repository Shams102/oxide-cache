# Oxide Cache

A Redis-inspired asynchronous in-memory cache server built in Rust using Tokio.

This project explores the fundamentals of backend infrastructure engineering, including:

* asynchronous networking
* TCP socket communication
* concurrent task scheduling
* protocol parsing
* shared state synchronization
* mutex contention
* scalable server architecture
* message-passing concurrency
* async-safe resource management

The project is being built as a systems/backend engineering learning project with a focus on understanding how modern infrastructure systems such as Redis work internally.

---

# Features

## Current Features

* Async TCP server using Tokio
* Concurrent client handling with `tokio::spawn`
* Redis protocol frame parsing using `mini-redis`
* Shared global in-memory datastore using `Arc<Mutex<HashMap>>`
* Redis-style GET and SET command support
* Multi-client socket handling
* Multi-binary Cargo project structure (`server.rs` + `client.rs`)
* Message-passing architecture using Tokio channels
* Dedicated manager task for Redis connection handling
* Concurrent async client-side command execution
* `mpsc` channels for request dispatching
* `oneshot` channels for asynchronous responses
* Shared-state synchronization across multiple concurrent connections
* Concurrent request orchestration through spawned Tokio tasks

## Planned Features

* Mutex sharding
* Pub/Sub messaging
* Key expiration (TTL)
* Persistence to disk
* Replication
* Request pipelining
* Benchmarking and load testing
* Metrics and observability
* Connection pooling
* Distributed cache experiments

---

# Tech Stack

* Rust
* Tokio
* mini-redis
* TCP sockets
* Async/await
* Concurrent task scheduling
* `Arc<Mutex<_>>`
* `mpsc` and `oneshot` channels

---

# Project Structure

```text
src/
 └── bin/
      ├── server.rs
      └── client.rs
```

* `server.rs` → Redis-like asynchronous TCP server implementation
* `client.rs` → asynchronous message-passing client architecture

---

# Architecture

The project currently follows a concurrent message-passing architecture.

```text
spawned async tasks
        ↓
Tokio mpsc channels
        ↓
manager task
        ↓
single Redis connection
        ↓
Redis-like async server
        ↓
shared synchronized datastore
        ↓
Tokio oneshot response channels
        ↓
spawned async tasks
```

Instead of sharing the Redis connection directly across tasks using a mutex, a dedicated manager task owns the connection and processes commands received through channels.

This architecture avoids:

* mutex contention on shared client connections
* holding locks across `.await`
* deadlocks caused by async lock misuse
* unsafe shared mutable ownership
* connection ownership conflicts

It also models real backend infrastructure patterns used in:

* async runtimes
* actor systems
* distributed services
* scalable backend systems
* connection pools
* message brokers

---

# Concepts Explored

This project focuses heavily on systems and backend engineering concepts:

* async runtimes
* task scheduling
* lightweight async tasks vs OS threads
* TCP sockets and networking
* Redis protocol parsing
* concurrent task execution
* shared state synchronization
* mutex contention
* async-safe architecture patterns
* message-passing concurrency
* bounded queues and backpressure
* `mpsc` and `oneshot` channels
* manager-task architecture
* request/response coordination
* ownership and borrowing in async Rust
* memory-safe multithreading
* backend scalability patterns

---

# Running the Project

## Start the server

```bash
cargo run --bin server
```

## Run the client

Open another terminal:

```bash
cargo run --bin client
```

---

# Example Output

```text
GET RESPONSE = Ok(Ok(Some(b"bar")))
SET RESPONSE = Ok(Ok(()))
```

Due to concurrent task scheduling, execution order may vary between runs.

---

# Why This Project?

Most backend frameworks abstract away networking and concurrency internals.

This project intentionally rebuilds foundational backend concepts from scratch in order to understand:

* how Redis works internally
* how async runtimes schedule tasks
* how socket servers handle concurrent clients
* how shared state is synchronized safely
* how message-passing architectures work
* how Rust prevents data races and memory bugs
* how scalable backend infrastructure systems are designed

---

# Next Steps

The next planned improvements are:

* Mutex sharding for reduced contention
* Pub/Sub messaging support
* Key expiration and TTL cleanup tasks
* Persistent storage
* Request pipelining
* Connection pooling
* Benchmarking under concurrent load
* Distributed replication experiments
* Metrics and observability

---


# License

MIT
