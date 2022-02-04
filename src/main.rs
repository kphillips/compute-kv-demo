use std::convert::TryInto;
use std::time::Duration;

use fastly::http::{Method, StatusCode};
use fastly::{mime, panic_with_status, Error, Request, Response, Body};
use fastly_kv_preview::local_kv::{Key, LocalStore};
use fastly_kv_preview::object_store::ObjectStore;
use sha2::{Digest, Sha256};

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    //Object store name must be defined in advance via API
    //Note that due to a regression, we're currently using the LINK name,
    //not the OBJECT STORE name, as of 2022-02-02.
    let object_store_name = "demo_object_store";
    // For debugging
    let cache_server_host = std::env::var("FASTLY_HOSTNAME").unwrap_or_else(|_| String::new());

    let path_re = regex::Regex::new(r#"^/(?P<action>local_kv|object_store)/(?P<key>[[:alnum:]]+)$"#).unwrap();
    
    match path_re.captures(req.get_path()) {
        Some(caps) => {
            match caps.name("action").unwrap().as_str() {
                "local_kv" => {                    
                    // Open the store
                    eprintln!("Fastly host: {}", cache_server_host);
                    let mut local_store = LocalStore::open().unwrap_or_else(|_| {
                        panic_with_status!(501, "cached_kvstore API not available on this host");
                    });
                    let key: Key = match caps.name("key") {
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
                },
                "object_store" => {
                    // Use Global Object Store
                    eprintln!("Fastly host: {}", cache_server_host);
                    let mut object_store = match ObjectStore::open(object_store_name).unwrap_or_else(|_| {
                        panic_with_status!(501, "cached_object_store API not available on this host");
                    }) {
                        // open an existing object store
                        Some(store) => store,
                        // ...or panic if it doesn't exist
                        None => panic_with_status!(501, "Store {} not found!", object_store_name),
                    };
                    let key = caps.name("key").unwrap().as_str().to_owned();
                    eprintln!("Key: {}", key);
                    match req.get_method() {
                        &Method::GET => match object_store.lookup(&key)? {
                            Some(value) => Ok(Response::from_body(value).with_content_type(mime::TEXT_PLAIN_UTF_8)),
                            None => Ok(Response::from_status(StatusCode::NOT_FOUND)),
                        },
                        &Method::POST => {
                            object_store.insert(&key, req.take_body())?;
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
                .with_body_text_plain("Invlaid URL\n"))
        },
    }
}