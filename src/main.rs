use std::io::BufRead;
use std::io::BufReader;
use std::ops::Add;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::{Datelike, DateTime, NaiveDateTime, TimeZone, Utc};
use clap::{App, AppSettings, Arg};
use csv;
use nmea::Nmea;
use serde::Serialize;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use socketcan::CANSocket;

#[derive(Debug, Serialize)]
struct Record {
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "Unix-Timestamp")]
    unix_timestamp: String,
    #[serde(rename = "CAN-Interface")]
    interface: String,
    #[serde(rename = "CAN-ID")]
    can_id: String,
    #[serde(rename = "DLC")]
    dlc: u8,
    #[serde(rename = "Data")]
    data: String,
    #[serde(rename = "Longitude")]
    longitude: Option<String>,
    #[serde(rename = "Latitude")]
    latitude: Option<String>,
    #[serde(rename = "Altitude (m)")]
    altitude: Option<String>,
    #[serde(rename = "Speed (m/s)")]
    speed: Option<String>
}

fn get_gps_and_can_frame_timestamp(reader: &mut BufReader<Box<dyn SerialPort>>)
    -> (DateTime<Utc>, Instant) {
    loop {
        let can_frame_timestamp = Instant::now();

        let mut my_str = String::new();
        if let Ok(_) = reader.read_line(&mut my_str) {
            if my_str.starts_with("$GPRMC") {
                let mut nmea = Nmea::new();
                if let Ok(_) = nmea.parse(&my_str) {
                    let gps_data = nmea.fix_date;
                    let gps_time = nmea.fix_time;

                    if let (Some(date), Some(time)) = (gps_data, gps_time) {
                        let date = date.with_year(2000 + date.year()).unwrap_or(date);
                        let gps_timestamp = NaiveDateTime::new(date, time);
                        let gps_timestamp = Utc.from_utc_datetime(&gps_timestamp);
                        return (gps_timestamp, can_frame_timestamp);
                    }
                }
            }
        }
    }
}

fn main() {
    let matches = App::new("gpscandump - logs GPS and CAN-bus data")
        .bin_name("gpscandump")
        .setting(AppSettings::DisableVersionFlag)
        .arg(
            Arg::new("interface")
                .help("The CAN-interface")
                .short('i')
                .long("interface")
                .use_delimiter(false)
                .default_value("vcan0")
                .required(false)
        )
        .arg(
            Arg::new("serial_port")
                .help("The serial-port to use")
                .short('s')
                .long("serial-port")
                .use_delimiter(false)
                .default_value("/dev/ttyUSB0")
                .required(false)
        )
        .arg(
            Arg::new("output")
                .help("File where one saves the results")
                .short('o')
                .long("output")
                .use_delimiter(false)
                .default_value("log.csv")
                .required(false)
        )
        .get_matches();

    let can_interface_name = matches.value_of("interface").unwrap();
    let serial_port_name = matches.value_of("serial_port").unwrap();
    let output_file_path = matches.value_of("output").unwrap();

    let can_interface = if can_interface_name == "any" {
        CANSocket::open_if(0).expect("Failed to open CAN-interface")
    } else {
        CANSocket::open(can_interface_name).expect("Failed to open CAN-interface")
    };

    can_interface
        .set_read_timeout(Duration::from_millis(1))
        .expect("Failed to set timeout of 1ms");

    let serial_port = serialport::new(serial_port_name, 4800)
        .timeout(Duration::new(1,0))
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .open()
        .expect("Failed to open serial port");
    let mut reader = BufReader::new(serial_port);

    // original timestamp
    let (gps_timestamp, can_frame_timestamp) = get_gps_and_can_frame_timestamp(&mut reader);

    let m_longitude = Arc::new(Mutex::new(None));
    let m_latitude = Arc::new(Mutex::new(None));
    let m_altitude = Arc::new(Mutex::new(None));
    let m_speed = Arc::new(Mutex::new(None));

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let can_frame = can_interface.read_frame();
            let timestamp = Instant::now();
            if let Ok(can_frame) = can_frame {
                tx.send((timestamp, can_frame));
            }
        }
    });

    {
        let m_longitude = m_longitude.clone();
        let m_latitude = m_latitude.clone();
        let m_altitude = m_altitude.clone();
        let m_speed = m_speed.clone();

        thread::spawn(move || {
            let mut buffer = String::new();
            loop {
                if let Ok(_) = reader.read_line(&mut buffer) {
                    if buffer.starts_with("$GPRMC") {
                        let mut nmea = Nmea::new();
                        if let Ok(_) = nmea.parse(&buffer) {
                            {
                                let longitude = nmea.longitude();
                                let mut mutex = m_longitude.lock().unwrap();
                                *mutex = longitude;
                            }
                            {
                                let latitude = nmea.latitude();
                                let mut mutex = m_latitude.lock().unwrap();
                                *mutex = latitude;
                            }
                            {
                                let speed = nmea.speed_over_ground;
                                let mut mutex = m_speed.lock().unwrap();
                                *mutex = speed.map(|value| value * 0.514444);
                            }
                        }
                    } else if buffer.starts_with("$GPGGA") {
                        let mut nmea = Nmea::new();
                        if let Ok(_) = nmea.parse(&buffer) {
                            let altitude = nmea.altitude();
                            let mut mutex = m_altitude.lock().unwrap();
                            *mutex = altitude;
                        }
                    }
                }
                buffer.clear();
            }
        });
    }

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(output_file_path).expect("");

    loop {
        match rx.recv() {
            Ok((timestamp, can_frame)) => {
                let long = m_longitude
                    .lock()
                    .unwrap()
                    .map(|value| format!("{:.8}", value).replacen(".", ",", 1));

                let lat = m_latitude
                    .lock()
                    .unwrap()
                    .map(|value| format!("{:.8}", value).replacen(".", ",", 1));

                let alt = m_altitude
                    .lock()
                    .unwrap()
                    .map(|value| format!("{:.2}", value).replacen(".", ",", 1));

                let spd = m_speed
                    .lock()
                    .unwrap()
                    .map(|value| format!("{:.2}", value).replacen(".", ",", 1));


                let elapsed = timestamp.duration_since(can_frame_timestamp);
                let new_timestamp = gps_timestamp
                    .add(chrono::Duration::from_std(elapsed).unwrap());

                wtr.serialize(Record {
                    timestamp: new_timestamp.to_rfc3339(),
                    unix_timestamp: format!("{},{}",
                                            new_timestamp.timestamp(),
                                            new_timestamp.timestamp_subsec_nanos()),
                    interface: can_interface_name.to_owned(),
                    can_id: format!("{:08X}", can_frame.id()),
                    dlc: can_frame.data().len() as u8,
                    data: can_frame.data()
                        .iter()
                        .map(|value| format!("{:02X}", value))
                        .collect::<Vec<_>>()
                        .join(" "),
                    longitude: long,
                    latitude: lat,
                    altitude: alt,
                    speed: spd
                }).unwrap();
                wtr.flush();
            }
            Err(err) => println!("{}", err)
        }
    }
}
