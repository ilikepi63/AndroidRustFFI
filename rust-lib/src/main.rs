use reqwest::Client;
use std::error::Error;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time;

pub fn main() -> () {
    // Create the runtime
    let rt = Runtime::new().unwrap();

    // Execute the future, blocking the current thread until completion
    rt.block_on(async {
        // get the url from the environment variables(can be compiled at build time)
        // we want to force the binary to panic here, so we know the
        let url = "http://eolr5l8a0t401et.m.pipedream.net";

        // want it to panic here with a human readable error at startup.
        let hostname = "HOSTNAME";

        // create the reqwest client.
        let client = reqwest::Client::new();

        // setup an interval each 10 seconds
        let mut interval = time::interval(Duration::from_secs(10));

        let payload = "{\"name\":\"Payload\"}";

        loop {
            interval.tick().await;

            // get the bssid information from the environment
            // if it fails, we would just continue on to getting the next lot of information
            match client
                .post(url)
                .body(payload.to_string())
                .header("Content-Type", "application/json")
                .header("X-Device-ID", hostname.trim())
                .send()
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    // if we need some sort of alert that the request hasn't gone through,
                    // we can add it in here.
                    eprintln!("Error with sending payload: {:?}", e)
                }
            };
        }
    });
}
