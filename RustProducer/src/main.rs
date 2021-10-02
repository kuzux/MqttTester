use paho_mqtt as mqtt;
use futures::{executor::block_on, FutureExt, stream::StreamExt, select};
use std::time::Duration;

fn main() {
    let addr = "tcp://localhost:6788";

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(addr)
        .client_id("rust-producer")
        .finalize();
    let mut client = mqtt::AsyncClient::new(create_opts).unwrap();

    if let Err(err) = block_on(async {
        let mut stream = client.get_stream(128); // Buffer size

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .mqtt_version(mqtt::MQTT_VERSION_3_1)
            .finalize();

        client.connect(conn_opts).await?;
        client.subscribe("control", mqtt::QOS_1);

        let mut running: bool = false;
        let mut curr_index: i32 = 0;

        let mut sleeper = async_std::task::sleep(Duration::from_millis(20)).fuse();
        let mut pin_sleeper = Box::pin(sleeper);

        loop {
            let mut receiver = stream.next().fuse();

            select! {
                () = pin_sleeper => {
                    if(running) {
                        let msg = mqtt::Message::new("data", curr_index.to_string(), mqtt::QOS_0);
                        client.publish(msg).await?;
                        curr_index += 1;
                    }
                    sleeper = async_std::task::sleep(Duration::from_millis(20)).fuse();
                    pin_sleeper = Box::pin(sleeper);
                }
                maybe_msg = receiver => {
                    // We can detect a conection loss if either value of these optionals is None
                    let msg = maybe_msg.unwrap().unwrap();
                    if(msg.payload_str() == "start") {
                        running = true;
                        curr_index = 0;
                    } else if(msg.payload_str() == "stop") {
                        running = false;
                    }
                }
            }
        }

        return Ok::<(), mqtt::Error>(());
    }) {
        eprintln!("{}", err);
    }
}
