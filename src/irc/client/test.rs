use websocket::ClientBuilder;

#[test]
#[ignore = "requires dotenv to be set up."]
fn test_client_websocket() {
    let host = dotenv::var("WEBSOCKET_HOST").expect("");
    println!("Connectin websocket to {host}");
    let client = ClientBuilder::new(&host).unwrap().connect(None);
    if let Err(err) = client {
        println!("Failed to connect websocket to {host}:\n{err}");
        assert!(false);
    }
}
