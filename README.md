# VESC communication for Rust

A `no-std`/`no-alloc` Rust implementation of the VESC®[^1] firmware
communication protocol, making it ideal for embedded systems, robotics, and any
application that needs to communicate with VESC motor controllers.

[^1]: https://vesc-project.com/

## Supported commands

> [!NOTE]
>
> If you find a missing command, feel free to contribute! Adding a new command
> should be relatively easy. Just follow the well-established pattern.

| Command Name                      | Status |
|-----------------------------------|--------|
| `GetValues`                       | ✅     |
| `SetCurrent`                      | ✅     |
| `SetRpm`                          | ✅     |
| `SetHandbrake`                    | ✅     |
| `ForwardCan`                      | ✅     |
| `GetValuesSelective`              | ✅     |

## Supported command replies

> [!NOTE]
>
> Many commands have no replies, so this list need not mirror the supported
> commands.

| Command Name                      | Status |
|-----------------------------------|--------|
| `GetValues`                       | ✅     |
| `GetValuesSelective`              | ✅     |

## Installation

Add this to your Cargo.toml:

```rust
[dependencies]
vesc = "0.1"
```

## License

This project is licensed under the [MIT license](LICENSE).
