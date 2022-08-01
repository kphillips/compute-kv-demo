# compute-kv-demo

This sample code demonstrates how to interact with Fastly's experimental Object Store and LocalKV Store features using the [Rust SDK](https://docs.rs/fastly/0.8.6/fastly/?search=) and a [Helper Crate](https://docs.rs/fastly-kv-preview/latest/fastly_kv_preview/index.html).

Usage: copy and deploy as a Fastly Compute@Edge service. If you don't have a Fastly account, create one first.

These features need to be enabled if you want to test them. Contact support for help.

URL syntax: the path determines which feature is exercised, and the verb determines the action: lookup or write.
* /local_kv/key: interact with a POP-local key-value store. 
  * `GET`: lookup a key
  * `POST`: write a value for a key
* /object_store/key: interact with the global object store.
  * `GET`: lookup the value for a key
  * `POST`: write a value for a key
* anything else: invalid URL
