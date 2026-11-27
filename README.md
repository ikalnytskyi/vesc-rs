# VESC communication for Rust

A `no-std`/`no-alloc` Rust implementation of the VESC®[^1] firmware
communication protocol, making it ideal for embedded systems, robotics, and any
application that needs to communicate with VESC motor controllers.

[^1]: https://vesc-project.com/

## Supported commands

> [!NOTE]
>
> If you find a missing command, feel free to contribute! Adding a new command
> should be relatively easy. Just follow the well-established pattern (see
> commits [`b0287c7`] and [`1049f10`] for reference).
>
> [`b0287c7`]: https://github.com/ikalnytskyi/vesc-rs/commit/b0287c7
> [`1049f10`]: https://github.com/ikalnytskyi/vesc-rs/commit/1049f10

| Command ID | Command Name                      | Status |
|:----------:|-----------------------------------|--------|
| `0`        | `FwVersion`                       | ✅     |
| `4`        | `GetValues`                       | ✅     |
| `5`        | `SetDuty`                         | ✅     |
| `6`        | `SetCurrent`                      | ✅     |
| `7`        | `SetCurrentBrake`                 | ✅     |
| `8`        | `SetRpm`                          | ✅     |
| `9`        | `SetPos`                          | ✅     |
| `10`       | `SetHandbrake`                    | ✅     |
| `14`  | `GetMcConf`                        | ✅     |
| `29`       | `Reboot`                          | ✅     |
| `30`       | `Alive`                           | ✅     |
| `34`       | `ForwardCan`                      | ✅     |
| `50`       | `GetValuesSelective`              | ✅     |
| `110`      | `SetOdometer`                     | ✅     |
| `128`      | `GetStats`                        | ✅     |
| `129`      | `ResetStats`                      | ✅     |
| `156`      | `Shutdown`                        | ✅     |
| `157`      | `FwInfo`                          | ✅     |
| `159`      | `MotorEstop`                      | ✅     |

## Supported command replies

> [!NOTE]
>
> Many commands have no replies, so this list need not mirror the supported
> commands.

| Command ID | Command Name                      | Status |
|------------|-----------------------------------|--------|
| `0`        | `FwVersion`                       | ✅     |
| `4`        | `GetValues`                       | ✅     |
| `14`  | `GetMcConf`                        | ✅     |
| `50`       | `GetValuesSelective`              | ✅     |
| `128`      | `GetStats`                        | ✅     |
| `129`      | `ResetStats`                      | ✅     |
| `157`      | `FwInfo`                          | ✅     |

## Installation

Add this to your Cargo.toml:

```rust
[dependencies]
vesc = "0.4"
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

## Reading motor configuration

`Command::GetMcConf` reads configuration without writing settings or running
motor detection. `CommandReply::GetMcConf` exposes six current and field-weakening
settings in `McConf`; other configuration fields are not exposed.

The decoder selects the configuration layout using its signature, without
requiring a firmware version argument. Official configuration layouts used by
6.05, 6.06, 7.00 and 8.00 are supported; unknown signatures return
`DecodeError::UnsupportedMcConf`. A future firmware using a supported layout
works without a library change; a new layout needs explicit support.

Both short and long reply frames are supported. The largest supported
configuration frame is 499 bytes and fits `Decoder::default()`. A custom decoder
buffer must accommodate the complete reply frame.

```rust
use vesc::{Command, CommandReply};

let mut request = [0; 6];
let len = vesc::encode(Command::GetMcConf, &mut request).unwrap();
// Send request[..len] using the application's existing transport.

// After decoding the controller reply:
if let Some(CommandReply::GetMcConf(config)) = reply {
    let phase_current_limit = config.l_current_max * config.l_current_max_scale;
    let field_weakening_start = config.foc_fw_duty_start * config.l_max_duty;
    // Current is in amperes; duty is a fraction of one, not percent.
}
```

The library remains synchronous, `no_std` and allocation-free. No Embassy,
transport, XML, or firmware-detection dependency is introduced.

## License

This project is licensed under the [MIT license](LICENSE).
