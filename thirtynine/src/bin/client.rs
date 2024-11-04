use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::{self, time};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("mqtt_client", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("shell/temp", QoS::AtMostOnce).await.unwrap();

    client.publish("shell/temp", QoS::AtMostOnce, false, "Test Message").await.unwrap();

    loop {
        let notification = eventloop.poll().await.unwrap();
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
            println!("Message from {}: {:?}", publish.topic, publish.payload);
        }

        time::sleep(time::Duration::from_secs(1)).await;
    }
}

