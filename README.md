# DoIP Reader Writer

The `doip_rw` crate provides a set of encoders and decoders for Diagnostics Over Internet Protocol (DoIP) messages.
The encoding and decoding are supported by synchronous IO such as `Reader` and `Writer`.

For asynchronous IO, only `DiagnosticMessage` struct should be augmented, as the other messages are always very small.

## Features
- zero copy serialization/deserialization
- deserialization "in place" to replace an existing DoIP payload
- for larger messages such as `DiagnosticMessage` both owned and borrowed buffer are available

## Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
doip_rw = "0.1.0"
```

## Usage
A simple synchronous client example is provided in [simple_client](examples/simple_client.rs)
It can be run against a DoIP server on localhost, tcp port 13400, by invoking:
```
cargo run --example simple_client
```

A simple vehicle announcement would look like :
```rust
    let udp = UdpSocket::bind("0.0.0.0:13400").unwrap();
    let mut vin: Vin = [0u8; 17];
    vin.copy_from_slice("VF1YYYYYZTT      ".as_bytes());
    let announce = VehicleIdentificationResponse {
        vin,
        logical_address: 0xed00,
        eid: [0xaa, 0xbb, 0xcc, 0xdd, 0x00, 0x38],
        gid: None,
        further_action: FurtherActionRequired::NoFurtherActionRequired,
        vin_gid_sync_status: VinGidSyncStatus::Synchronized,
    };
    let mut buf = vec![];
    write_message(&announce, &mut Cursor::new(&mut buf)).unwrap();
    udp.set_broadcast(true).unwrap();
    udp.send_to(&buf, "255.255.255.255:13400").unwrap();
```

## Documentation
Comprehensive API documentation is available on [docs.rs](https://docs.rs/doip_rw/).

## Contributing
Contributions are welcome! Feel free to open issues, submit pull requests, or suggest features. Please follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) when contributing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
