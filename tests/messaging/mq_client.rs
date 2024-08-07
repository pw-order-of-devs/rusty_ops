use rstest::rstest;
use testcontainers::runners::AsyncRunner;
use testcontainers::Image;
use testcontainers_modules::rabbitmq::RabbitMq;

use crate::utils::mq_connect;

#[rstest]
#[case(RabbitMq::default(), "rabbit", 5672)]
#[tokio::test]
async fn create_queue_test<I: Image + Default>(
    #[case] image: I,
    #[case] mq_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let mq = image
        .start()
        .await
        .expect("initializing test container failed");
    let mq_client = mq_connect(&mq, mq_type, port).await;
    let result = mq_client.create_queue("test_queue").await;
    let _ = mq.stop().await;
    assert!(result.is_ok());
}

#[rstest]
#[case(RabbitMq::default(), "rabbit", 5672)]
#[tokio::test]
async fn delete_queue_test<I: Image + Default>(
    #[case] image: I,
    #[case] mq_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let mq = image
        .start()
        .await
        .expect("initializing test container failed");
    let mq_client = mq_connect(&mq, mq_type, port).await;
    let _ = mq_client.create_queue("test_queue").await;
    let result = mq_client.delete_queue("test_queue").await;
    let _ = mq.stop().await;
    assert!(result.is_ok());
}

#[rstest]
#[case(RabbitMq::default(), "rabbit", 5672)]
#[tokio::test]
async fn publish_test<I: Image + Default>(
    #[case] image: I,
    #[case] mq_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let mq = image
        .start()
        .await
        .expect("initializing test container failed");
    let mq_client = mq_connect(&mq, mq_type, port).await;
    let _ = mq_client.create_queue("test_queue").await;
    let result = mq_client.publish("test_queue", "test_message").await;
    let _ = mq.stop().await;
    assert!(result.is_ok());
}

#[rstest]
#[case(RabbitMq::default(), "rabbit", 5672)]
#[tokio::test]
async fn get_consumer_test<I: Image + Default>(
    #[case] image: I,
    #[case] mq_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let mq = image
        .start()
        .await
        .expect("initializing test container failed");
    let mq_client = mq_connect(&mq, mq_type, port).await;
    let _ = mq_client.create_queue("test_queue").await;
    let _ = mq_client.publish("test_queue", "test_message").await;
    let result = mq_client.get_consumer("test_queue").await;
    assert!(result.is_ok());
    let message = result.unwrap().next().await;
    assert!(message.is_some());
    assert!(message.clone().unwrap().is_ok());
    assert_eq!(
        "test_message",
        String::from_utf8(message.unwrap().unwrap()).unwrap()
    );
    let _ = mq.stop().await;
}
