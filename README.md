# gpscandump

![Crates.io](https://img.shields.io/crates/d/gpscandump)

*gpscandump* is a logging program that combines CAN-bus with GPS data to be more precise, i.e., not relying on the underlying operating system to retrieve timing and date information. The log contains the timestamp, the used CAN-bus interface, the CAN-ID, the DLC, the CAN-bus data, the longitude, the latitude, the elevation (altitude), and the speed over ground.

## Build

1. Install [Rust](https://www.rust-lang.org/)

   ​	Remark: On Windows Rust requires a **C/C++** compiler, e.g., Visual Studio or [MSYS2](https://www.msys2.org/)

2. Install the Rust **ARM** target

   1. `rustup target add armv7-unknown-linux-gnueabihf`
   2. Install **ARM** linker on Ubuntu this can be done with `sudo apt install gcc-arm-linux-gnueabihf`

3. Build the binary

   1. **x86**: `cargo build --target x86_64-unknown-linux-gnu --release` 
   2. **ARM**: `cargo build --target armv7-unknown-linux-gnueabihf --release`

## Install

If the repository has been cloned on a Raspberry Pi already, you can just execute

`make install`

Which builds and installs all the necessary files and enables the systemd services. 

The make file executes the following steps:

1. Copy `scripts/gpscandumps.sh` into `/usr/bin`

2. Copy `gpscandump` into `/usr/bin`

   ​	Use `target/armv7-unknown-linux-gnueabihf/gpscandump` for the **Raspberry Pi 4**

3. Copy the content of `services` into `/etc/systemd/system`

4. Create the directory `gpscandump` in `/home/pi` with `mdkir /home/pi/gpscandumps`

5. Enable `ecap-gpscan-dumps.service` with `sudo systemctl enable ecap-gpscan-dumps.service`

6. Start `ecap-gpscan-dumps.service` with `sudo systemctl start ecap-gpscan-dumps.service`

## Usage
To set the appropriate Baudrate adjust it in `/etc/systemd/system/ecap-can.service`.

Therefore change the line:

`ExecStartPre ... bitrate {BAUDRATE in Bits per second}`

Don't forget to restart the systemd service afterwards with:

`sudo systemctl restart ecap-can-hw.service`

Another helper script can be used to restart the logging service and clean the directory with the gpscandums. Execute:

`restart-ecapservices.sh` 

```bash
gpscanlogger - logs GPS and CAN-bus data 

USAGE:
    gpscanlogger [OPTIONS]

OPTIONS:
    -h, --help                         Print help information
    -i, --interface <interface>        The CAN-interface [default: vcan0]
    -o, --output <output>              File where one saves the results [default: log.csv]
    -s, --serial-port <serial_port>    The serial-port to use [default: /dev/ttyUSB0]
```
