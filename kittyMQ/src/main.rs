use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Sender, Receiver};
use std::error::Error;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct KittyMessage {
    topic: String,
    payload: String,
    headers: HashMap<String, String>,
}

impl KittyMessage {
    pub fn new(topic: &str, payload: &str) -> Self {
        KittyMessage {
            topic: topic.to_string(),
            payload: payload.to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Clone)]
pub struct KittyMQ {
    topics: Arc<Mutex<HashMap<String, Sender<KittyMessage>>>>,
}

impl KittyMQ {
    pub fn new() -> Self {
        KittyMQ {
            topics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_topic(&self, topic: &str) -> Result<(), Box<dyn Error>> {
        let mut topics = self.topics.lock().unwrap();
        if !topics.contains_key(topic) {
            let (sender, _) = broadcast::channel(100);
            topics.insert(topic.to_string(), sender);
        }
        Ok(())
    }

    pub async fn publish(&self, message: KittyMessage) -> Result<(), Box<dyn Error>> {
        let topics = self.topics.lock().unwrap();
        if let Some(sender) = topics.get(&message.topic) {
            sender.send(message)?;
        }
        Ok(())
    }

    pub fn subscribe(&self, topic: &str) -> Result<Receiver<KittyMessage>, Box<dyn Error>> {
        let topics = self.topics.lock().unwrap();
        if let Some(sender) = topics.get(topic) {
            Ok(sender.subscribe())
        } else {
            Err("Topic not found".into())
        }
    }
}

#[async_trait]
pub trait KittyHandler: Send + Sync {
    async fn handle(&self, message: KittyMessage);
}

pub struct KittySubscriber {
    name: String,
}

#[async_trait]
impl KittyHandler for KittySubscriber {
    async fn handle(&self, message: KittyMessage) {
        println!("[{}] received message on '{}': {:?}",
                self.name, message.topic, message.payload);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let broker = KittyMQ::new();
    
    broker.create_topic("orders")?;
    broker.create_topic("notifications")?;
    
    let subscriber1 = KittySubscriber {
        name: "subscriber1".to_string(),
    };
    let subscriber2 = KittySubscriber {
        name: "subscriber2".to_string(),
    };
    
    let mut rx1 = broker.subscribe("orders")?;
    let mut rx2 = broker.subscribe("notifications")?;
    
    let broker_clone = broker.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            subscriber1.handle(msg).await;
        }
    });
    
    let broker_clone2 = broker_clone.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            subscriber2.handle(msg).await;
        }
    });
    
    broker.publish(KittyMessage::new("orders", "test order")
        .with_header("priority", "high")).await?;
    broker.publish(KittyMessage::new("notifications", "test")
        .with_header("type", "maintenance")).await?;
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    Ok(())
}
