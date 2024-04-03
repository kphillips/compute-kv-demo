# Compute@Edge KV Store Demo

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Try out new state features for the Fastly Compute@Edge environment with this basic starter that demonstrates routing, as well as reading and writing KV Store data.

**For more details about starter kits for Compute@Edge, see the [Fastly Developer Hub](https://developer.fastly.com/solutions/starters/)**.

## Features

- Allow only requests with particular HTTP methods
- Match request URL path and methods for routing
- Write and Read Edge Objects

## Understanding the code

This starter is intentionally lightweight, and requires no dependencies aside from the [`fastly`](https://docs.rs/fastly) crate. It will help you understand the basics of processing requests at the edge using Fastly. This starter includes implementations of common patterns explained in our [using Compute@Edge](https://developer.fastly.com/learning/compute/rust/) and [VCL migration](https://developer.fastly.com/learning/compute/migrate/) guides.

The starter doesn't require the use of any backends. Once deployed, you will have a Fastly service running on Compute@Edge that can generate synthetic responses at the edge.

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
