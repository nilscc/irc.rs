use websocket::{stream::sync::NetworkStream, sync::Client, ClientBuilder, OwnedMessage};

use super::{capabilities::CapNegotiator, Capability};

fn connect() -> Client<Box<dyn NetworkStream + Send>> {
    let host = dotenv::var("WEBSOCKET_HOST").expect("");
    println!("Connectin websocket to {host}");
    match ClientBuilder::new(&host).unwrap().connect(None) {
        Err(err) => {
            panic!("Failed to connect websocket to {host}:\n{err}")
        }
        Ok(client) => client,
    }
}

#[test]
#[ignore = "requires dotenv to be set up."]
fn integrate_client_with_websocket() {
    let mut client = connect();
    let capabilities = CapNegotiator::request(vec![Capability("sasl".into())]);

    let msg = capabilities.ls(Some("302".into()));
    client
        .send_message(&OwnedMessage::Text(msg.to_string()))
        .unwrap();

    if let Ok(msg) = client.recv_message() {
        println!("{msg:?}");
    }
}
