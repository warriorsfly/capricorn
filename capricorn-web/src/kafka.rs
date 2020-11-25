use futures::StreamExt;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    ClientConfig,
};

fn init_consumer(brokers: String, topics: &[String]) -> Result<StreamConsumer, KafkaError> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", &brokers)
        .set("auto.offset.reset", "latest")
        .set("enable.partition.eof", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()?;

    let topics = topics
        .iter()
        .map(|topic| topic.as_str())
        .collect::<Vec<&str>>();

    consumer.subscribe(topics.as_slice())?;

    Ok(consumer)
}
