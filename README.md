# gpscandump

![Crates.io](https://img.shields.io/crates/d/gpscandump)

*gpscandump* is a logging program that combines CAN-bus with GPS data to be more precise, i.e., not relying on the underlying operating system to retrieve timing and date information. The log contains the timestamp, the used CAN-bus interface, the CAN-ID, the DLC, the CAN-bus data, the longitude, the latitude, the elevation (altitude), and the speed over ground.

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
