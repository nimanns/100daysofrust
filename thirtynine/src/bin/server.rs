use rumqttd::Config;
use rumqttd::Broker;

fn main() {
    let config = config::Config::builder()
        .add_source(config::File::with_name("rumqttd.toml"))
        .build()
        .unwrap();

    let rumqttd_config: Config = config.try_deserialize().unwrap();

    println!("Starting MQTT broker on port 1883");
    let mut broker = Broker::new(rumqttd_config);
    
    broker.start().unwrap();
}
