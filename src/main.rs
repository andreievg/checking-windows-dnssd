use std::time::Duration;

use {astro_dnssd::DNSServiceBuilder, std::collections::HashMap};

const SERVICE_NAME: &'static str = "_omsupply._tcp";
const NAME: &'static str = "omSupplyServer";
const PROTOCOL_KEY: &'static str = "protocol";
const CLIENT_VERSION_KEY: &'static str = "client_version";
const HARDWARE_ID_KEY: &'static str = "hardware_id";
const CLIENT_VERSION: &'static str = "unspecified";

pub(crate) fn start_discovery(port: u16, hardware_id: String) {
    tokio::task::spawn(async move {
        let mut text_record = HashMap::<String, String>::new();
        text_record.insert(HARDWARE_ID_KEY.to_string(), hardware_id.to_string());
        text_record.insert(CLIENT_VERSION_KEY.to_string(), CLIENT_VERSION.to_string());
        text_record.insert(PROTOCOL_KEY.to_string(), "https".to_string());

        let registration_result = DNSServiceBuilder::new(SERVICE_NAME, port)
            .with_txt_record(text_record.clone())
            .with_name(NAME)
            .with_host("192.168.10.11")
            .register();

        match registration_result {
            Ok(_service_handle) => futures::future::pending::<()>().await,
            Err(e) => println!("Error registering discovery: {:?}", e),
        }
    });
}

#[tokio::main]
async fn main() {
    start_discovery(8000, "abc".to_string());
    tokio::time::sleep(Duration::from_secs(1000)).await;
}
