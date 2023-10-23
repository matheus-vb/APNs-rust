use std::fs::File;
use std::env;
use a2::{Client, DefaultNotificationBuilder, Endpoint, NotificationBuilder, NotificationOptions};

pub async fn send_notification(device_token: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const key_file: &'static str  = "auth-key.p8";
    let mut team_id = String::new();
    let mut key_id = String::new();
    let mut message = String::from("Message from Rust!");
    let mut topic: Option<String> = None;

    key_id = env::var("KEY_ID").expect("KEY_ID must be set");
    team_id = env::var("TEAM_ID").expect("TEAM_ID must be set");
    let rawTopic = env::var("BUNDLE_ID").expect("BUNDLE_ID must be set");

    topic = Some(rawTopic.to_string());

    // Read the private key from disk
    let mut private_key = File::open(key_file).unwrap();

    println!("Key file: {:?}", private_key);
    println!("Key id: {:?}", key_id);
    println!("Topic: {:?}", topic);
    println!("Team id: {:?}", team_id);

    let endpoint = Endpoint::Sandbox;

    let client = Client::token(&mut private_key, key_id, team_id, endpoint).unwrap();
    
    let options = NotificationOptions {
        apns_topic: topic.as_deref(),
        ..Default::default()
    };
    
    // Notification payload
    let builder = DefaultNotificationBuilder::new()
        .set_body(message.as_ref())
        .set_sound("default")
        .set_badge(1u32);

    let payload = builder.build(device_token.as_ref(), options);
    let response = client.send(payload).await?;
    
    println!("Sent: {:?}", response);

    Ok(())
}