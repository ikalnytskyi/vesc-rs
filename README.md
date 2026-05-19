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

| Command ID | Command Name                      | Status |
|:----------:|-----------------------------------|--------|
| `4`        | `GetValues`                       | ✅     |
| `6`        | `SetCurrent`                      | ✅     |
| `7`        | `SetCurrentBrake`                 | ✅     |
| `8`        | `SetRpm`                          | ✅     |
| `10`       | `SetHandbrake`                    | ✅     |
| `34`       | `ForwardCan`                      | ✅     |
| `50`       | `GetValuesSelective`              | ✅     |

## Supported command replies

> [!NOTE]
>
> Many commands have no replies, so this list need not mirror the supported
> commands.

| Command ID | Command Name                      | Status |
|------------|-----------------------------------|--------|
| `4`        | `GetValues`                       | ✅     |
| `50`       | `GetValuesSelective`              | ✅     |

## Installation

Add this to your Cargo.toml:

```rust
[dependencies]
vesc = "0.1"
```

## Usage

```rust
let mut buf = [0u8; 16];
let size = vesc::encode(Command::SetRpm(5000), &mut buf).unwrap();
tx.write_all(&buf[..size]).unwrap();
```

```rust
let mut decoder = Decoder::default();
let mut buf = [0u8; 512];

loop {
    let size = rx.read(&mut buf).unwrap()
    decoder.feed(&buf[..size]).unwrap();

    for reply in self.decoder.by_ref() {
        // process `reply`
    }
}
```

## License

This project is licensed under the [MIT license](LICENSE).
