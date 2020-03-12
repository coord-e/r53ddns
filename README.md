# r53ddns

[![Actions Status](https://github.com/coord-e/r53ddns/workflows/Test%20and%20Lint/badge.svg)](https://github.com/coord-e/r53ddns/actions?workflow=Test+and+Lint)
[![Actions Status](https://github.com/coord-e/r53ddns/workflows/Release/badge.svg)](https://github.com/coord-e/r53ddns/actions?workflow=Release)
[![Crates.io](https://img.shields.io/crates/v/r53ddns)](https://crates.io/crates/r53ddns)
[![Crates.io](https://img.shields.io/crates/l/r53ddns)](https://crates.io/crates/r53ddns)
[![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/coorde/r53ddns)](https://hub.docker.com/r/coorde/r53ddns)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/coorde/r53ddns)](https://hub.docker.com/r/coorde/r53ddns)

`r53ddns` is a simple command-line utility to update `A` record in Route53 with current global IP address.

## Installation

Download the latest compiled binary from links below and put it in your executable path.

Platform|Download
--------|--------
Linux 64-bit|[r53ddns-x86_64-unknown-linux-musl](https://github.com/coord-e/r53ddns/releases/latest/download/r53ddns-x86_64-unknown-linux-musl)
macOS 64-bit|[r53ddns-x86_64-apple-darwin](https://github.com/coord-e/r53ddns/releases/latest/download/r53ddns-x86_64-apple-darwin)
Windows 64-bit|[r53ddns-x86_64-pc-windows-msvc.exe](https://github.com/coord-e/r53ddns/releases/latest/download/r53ddns-x86_64-pc-windows-msvc.exe)

### with Cargo

```shell
$ cargo install r53ddns
```

### with Docker

```shell
$ alias r53ddns="docker run coorde/r53ddns"
```

## Example configuration

`r53ddns` works very well even in non-user environments. The following is an example systemd.service(5) file to execute `r53ddns`.

```
[Unit]
Description=Update DDNS
# OnFailure=notify-failure@%n.service

[Service]
Type=Simple
ExecStart=/path/to/r53ddns -l Info -k <key id> -s <key secret> -z <zone id> -n <record name>

[Install]
WantedBy=multi-user.target
```

You can run this service daily using systemd.timer(5) to be up-to-date.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
