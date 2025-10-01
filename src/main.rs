use fastly::http::{Method, StatusCode};
use fastly::kv_store::{KVStoreError, LookupResponse};
use fastly::{
    mime,
    panic_with_status,
    KVStore,
    Body, Request, Response, Error};

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // Log out which version of the Fastly Service is responding to this request.
    // This is useful to know when debugging.
    if let Ok(fastly_service_version) = std::env::var("FASTLY_SERVICE_VERSION") {
        println!("FASTLY_SERVICE_VERSION: {}", fastly_service_version);
    }

    //KV store name must be defined in advance via API
    let kv_store_name = "demo_kv_store";
    
    // For debugging
    let cache_server_host = std::env::var("FASTLY_HOSTNAME").unwrap_or_else(|_| String::new());

    //let path_re = regex::Regex::new(r#"^/(?P<action>simple_cache|kv_store)/(?P<key>[[:alnum:]]+)$"#).unwrap();
    let path_re = regex::Regex::new(r#"^/(?P<action>kv_store)/(?P<key>[[:alnum:]]+)$"#).unwrap();

    match path_re.captures(req.get_path()) {
        Some(caps) => {
            match caps.name("action").unwrap().as_str() {
                /*"simple_cache" => {                    
                    eprintln!("Fastly host: {}", cache_server_host);
                    let key: CacheKey = match caps.name("key") {
                        Some(urlkey) => {
                            let key = urlkey.as_str();
                            eprintln!("Key: {}", key);
                            let mut hasher = Sha256::new();
                            hasher.update(key);
                            hasher
                                .finalize()
                                .try_into()
                                .expect("sha256 hash is 32 bytes")
                        },
                        None => panic_with_status!(400, "Key must not be empty"),
                    };
                    match req.get_method() {
                        // Lookup for a GET
                        &Method::GET => match local_store.lookup(&key)? {
                            //Some (value) => Ok(Response::from_body(value).with_content_type(mime::TEXT_PLAIN_UTF_8)),
                            /* Debug */
                                Some(mut value) => {
                                value.append(Body::from(cache_server_host));
                                Ok(Response::from_body(value).with_content_type(mime::TEXT_PLAIN_UTF_8))
                            }, 
                            //None => Ok(Response::from_status(StatusCode::NOT_FOUND)),
                            // Debug
                                None => Ok(Response::from_body(cache_server_host).with_content_type(mime::TEXT_PLAIN_UTF_8))
                        },
                        //Write for a POST
                        &Method::POST => {
                            local_store.insert(&key, req.take_body(), Duration::from_secs(60))?;
                            Ok(Response::from_status(StatusCode::CREATED))
                        },
                        _ => Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)),
                    }
                },*/
                "kv_store" => {
                    /*
                    Use the KV Store when any of these conditions are true:
                     * you need the data to be replicated globally
                     * you need to be able to write keys from your Compute application
                     * you need the data to be durable.
                    */
                    eprintln!("Fastly host: {}", cache_server_host);
                    eprintln!("Opening KV Store {}", kv_store_name);
                    // open a KV store
                    let kv_store = match KVStore::open(kv_store_name).unwrap_or_else(|_| {
                        panic_with_status!(501, "kv_store API error");
                    }) {                        
                        Some(store) => store,
                        // ...or panic if it doesn't exist
                        None => panic_with_status!(501, "Store {} not found!", kv_store_name),
                        //None => Ok(Response::with_body_text_plain("Configured KV Store {} not found", kv_store_name))
                    };
                    let key = caps.name("key").unwrap().as_str().to_owned();
                    eprintln!("Key: {}", key);
                    match req.get_method() {
                        &Method::GET => match kv_store.lookup(&key) {
                            Ok(mut value) => Ok(Response::from_body(value.take_body()).with_content_type(mime::TEXT_PLAIN_UTF_8)),
                            Err(_e) => Ok(Response::from_status(StatusCode::NOT_FOUND)),
                        },
                        &Method::POST => {
                            kv_store.insert(&key, req.take_body())?;
                            Ok(Response::from_status(StatusCode::CREATED))
                        },
                        _ => Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)),
                    }
                },
                _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
                    .with_body_text_plain("The page you requested could not be found\n"))
            }
        },
        None => {
            eprintln!("Fastly host: {}", cache_server_host);
            Ok(Response::from_status(StatusCode::NOT_FOUND)
                .with_body_text_plain("Invalid URL\n"))
        },
    }
}
