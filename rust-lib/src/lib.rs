/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;
    use std::ffi::CString;

    use reqwest::Client;
    use tokio::runtime::Runtime;
    use std::time::Duration;
    use tokio::time;

    use log::{info};
    use android_log;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_androidrustffi_MainActivity_greeting(
        env: JNIEnv,
        _: JClass,
    ) -> jstring {

        let world_ptr = CString::new("Hello world from Rust world").unwrap();
        let output = env
            .new_string(world_ptr.to_str().unwrap())
            .expect("Couldn't create java string!");
        output.into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_androidrustffi_RustService_startRuntime(
        mut env: JNIEnv,
        _: JClass,
    ) {

        android_log::init("AndroidRustFFI").unwrap();
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

                info!("TEsting LOGS");

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

                if let Ok(class) = env
                .find_class("com/example/androidrustffi/RustService") {
                    let result = env.call_method(class, "rustFfiCallback", "()", &[]);
                }
                
            }
        });
    }
}


