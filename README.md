# gpscandump

![Crates.io](https://img.shields.io/crates/d/gpscandump)

This is a logging tool that receives GPS-data from a serial port and CAN-bus data from CAN-interface supported by the Linux kernel. The idea behind this tool was that the underlying logging device does not always have access to the current time. Consequently, the timestamps are only correct relatively to one and another. To mitigate this problem, the timestamps are synchronized by using GPS-data. The resulting log contains the following columns:

- Timestamp in [RFC-3339](https://datatracker.ietf.org/doc/html/rfc3339) formatting
- Unix-Timestamp
- CAN-bus interface
- DLC (number of received data bytes)
- CAN-bus data
- Longitude
- Latitude
- Altitude (elevation) in meter
- Speed over ground in m/s

Currently, `gpscandump` only supports CAN-interfaces the Linux kernel can use, as well as a specific [GPS-dongle](https://www.globalsat.com.tw/en/product-199952/Cable-GPS-with-USB-interface-SiRF-Star-IV-BU-353S4.html).

## Installation

```bash
cargo install gpscandump
```

## Usage

```bash
gpscandump - logs GPS and CAN-bus data 

USAGE:
    gpscandump [OPTIONS]

OPTIONS:
    -h, --help                         Print help information
    -i, --interface <interface>        The CAN-interface [default: vcan0]
    -o, --output <output>              File where one saves the results [default: log.csv]
    -s, --serial-port <serial_port>    The serial-port to use [default: /dev/ttyUSB0]
```

## License / Terms of Usage

The source code of this project is licensed under the MIT license. This implies that you are free to use, share, and adapt it. However, please give appropriate credit by citing the project.

## Contact

If you have problems using the software, find mistakes, or have general questions please use the [issue tracker](https://github.com/tsabelmann/gpscandump-rs/issues) to contact us.

## Contributors

- [Tim Lucas Sabelmann](https://github.com/tsabelmann)
