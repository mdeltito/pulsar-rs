use std::time::Duration;

use futures::TryStreamExt;
use pulsar::{
    ConnectionRetryOptions, Consumer, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor,
};
use regex::Regex;
use tracing::{debug, error, info};

#[derive(Debug)]
struct TestMessage {
    data: Vec<u8>,
}

impl DeserializeMessage for TestMessage {
    type Output = Result<TestMessage, std::io::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        Ok(TestMessage {
            data: payload.data.clone(),
        })
    }
}

fn main() -> Result<(), pulsar::Error> {
    tracing_subscriber::fmt::init();

    let addr = "pulsar://localhost:6650";
    let tns = "platform/pipeline";
    let topic = format!("persistent://{tns}/test");
    let topic_regex = Regex::new(format!("{topic}.*").as_str()).unwrap();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let pulsar = Pulsar::builder(addr, TokioExecutor)
            .with_connection_retry_options(ConnectionRetryOptions {
                max_retries: u32::MAX,

                // mdeltito: 2s keepalive interval
                // `check_connections` interval is also modified to 1s, which aggressively
                // purges "unused" connections.
                keep_alive: Duration::from_secs(2),
                ..ConnectionRetryOptions::default()
            })
            .build()
            .await?;

        let mut consumer: Consumer<TestMessage, _> = pulsar
            .consumer()
            .with_lookup_namespace(tns)
            .with_topic_regex(topic_regex)
            .with_topic_refresh(std::time::Duration::from_secs(10))
            .with_consumer_name("test_consumer")
            .with_subscription_type(SubType::Exclusive)
            .with_subscription("test_subscription")
            .build()
            .await?;

        info!("Consumer connected and waiting for messages from {}", topic);

        let mut counter = 0usize;
        while let Some(msg) = consumer.try_next().await? {
            consumer.ack(&msg).await?;

            let data = match msg.deserialize() {
                Ok(data) => data,
                Err(e) => {
                    error!("Could not deserialize message: {:?}", e);
                    continue;
                }
            };

            counter += 1;
            info!(
                "Message #{}: id={:?}, payload_size={} bytes",
                counter,
                msg.message_id(),
                data.data.len()
            );
            debug!("Payload: {:?}", String::from_utf8_lossy(&data.data));
        }

        Ok(())
    })
}
