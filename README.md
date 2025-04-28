# Fastly Compute KV Store Demo

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Try out data stores for the Fastly Compute platform with this basic starter that demonstrates routing, as well as reading and writing KV Store data.

**For more details about starter kits for Compute, see the [Fastly Developer Hub](https://developer.fastly.com/solutions/starters/)**.

## Features

- Allow only requests with particular HTTP methods
- Match request URL path and methods for routing
- Write and Read Edge Objects

## Understanding the code

This starter is intentionally lightweight, and requires no dependencies aside from the [`fastly`](https://docs.rs/fastly) crate. It will help you understand the basics of processing requests at the edge using Fastly. This starter includes implementations of common patterns explained in our [using Compute@Edge](https://developer.fastly.com/learning/compute/rust/) and [VCL migration](https://developer.fastly.com/learning/compute/migrate/) guides.

The starter doesn't require the use of any backends. Once deployed, you will have a Fastly service running on Compute@ that can generate synthetic responses at the edge.

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
=======
# compute-kv-demo

This sample code demonstrates how to interact with Fastly's KV Store feature using the [Rust SDK](https://docs.rs/fastly/latest/fastly/).

Usage: copy and deploy as a Fastly Compute service. If you don't have a Fastly account, create one first.

These features need to be enabled if you want to test them. Contact support for help.

URL syntax: the path determines which feature is exercised, and the verb determines the action: lookup or write.
* /local_kv/key: interact with a POP-local key-value store. 
  * `GET`: lookup a key
  * `POST`: write a value for a key
* /object_store/key: interact with the global object store.
  * `GET`: lookup the value for a key
  * `POST`: write a value for a key
* anything else: invalid URL
>>>>>>> 268f1c5c414c918b6839329d9c7e91318ecf3aee
