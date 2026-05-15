use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

/// Multiple commands sent over one channel
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },

    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

/// Type alias for response sender
type Responder<T> =
    oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {

    // Create channel
    let (tx, mut rx) = mpsc::channel(32);

    // Manager task
    let manager = tokio::spawn(async move {

        // Single Redis connection
        let mut client =
            client::connect("127.0.0.1:6379")
                .await
                .unwrap();

        // Receive commands
        while let Some(cmd) = rx.recv().await {

            match cmd {

                Command::Get { key, resp } => {

                    let res = client.get(&key).await;

                    // Ignore send errors
                    let _ = resp.send(res);
                }

                Command::Set { key, val, resp } => {

                    let res = client.set(&key, val).await;

                    let _ = resp.send(res);
                }
            }
        }
    });

    // Clone sender for second task
    let tx2 = tx.clone();

    // GET task
    let t1 = tokio::spawn(async move {

        let (resp_tx, resp_rx) =
            oneshot::channel();

        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        // Send command
        tx.send(cmd).await.unwrap();

        // Wait for response
        let res = resp_rx.await;

        println!("GET RESPONSE = {:?}", res);
    });

    // SET task
    let t2 = tokio::spawn(async move {

        let (resp_tx, resp_rx) =
            oneshot::channel();

        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // Send command
        tx2.send(cmd).await.unwrap();

        // Wait for response
        let res = resp_rx.await;

        println!("SET RESPONSE = {:?}", res);
    });

    // Wait for tasks
    t1.await.unwrap();
    t2.await.unwrap();

    // Drop final sender so rx closes
    

    manager.await.unwrap();
}