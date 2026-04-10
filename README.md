# Arbhx Core VFS

Arbhx is a capability-based virtual filesystem abstraction that unifies access to different storage systems under a single interface. It supports both async and blocking APIs and is designed to work across local filesystems, network protocols, archives, and in-memory stores.

The system focuses on composability, backend flexibility, and explicit capability discovery rather than rigid class inheritance hierarchies.

---

## Features

- Read-only filesystem interface
- Writable filesystem interface
- Random-access (seekable) writes
- Full read/write/seek capability interface
- Metadata access without opening files
- Directory listing with filtering and streaming support
- Optional storage usage reporting
- Async core with synchronous compatibility layer



## Design Overview

Arbhx is built around capability-based backends.

Rather than requiring a backend to implement a fixed interface, each backend advertises what it supports at runtime.

A profile may provide:

- VfsReader – read-only access
- VfsWriter – requential writes
- vfsPortore VfsSeekWriter – random-access writes
- VvfFull – full read/write/seek access

Capabilities are exposed via upgrade methods.

## Async and Sync Layers

Arbhx supports both `sync` and `async` APIs. They are designed to work concurrently with each other. To see this, use 
the `arbhx-sync` crate for a Tokio Implementation.


## Thread Safety
This system is designed to be thread safe. It uses `Arc` extensively and requires `Send`+`Sync`+`'static` on all implementatoins to prevent lifetime issues.

## License
This software is bound under the MIT and Apache 2.0 licenses.