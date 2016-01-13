extern crate serial;
extern crate time;

use std::env;
use std::io;
use time::Duration as Cargo_Duration;
use std::time::Duration; // need this for serial timeout
use std::thread; // used for sleep_ms
use std::net::{TcpListener, TcpStream};
//use std::io::Read; // to read from TCP socket

use std::io::prelude::*;
use serial::prelude::*;

struct servo_positions {
    y_deg:u8, // 8 bits is all micro uses for ints anyway
    x_deg:u8
}

fn main() {
        thread::sleep_ms(350);
        for tmp_arg in env::args_os().skip(1) {
            let arg = &tmp_arg.into_string().unwrap();
            let listener = TcpListener::bind("localhost:5643").unwrap(); // using port 5643

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        process_commands(stream,arg); // no point in threading, we only have one serial interface
                    }
                    Err(e) => { println!("Connection attempted but failed: {}",e); }
                }
            }
            drop(listener); // drop connection
        }
}

fn process_commands (stream: TcpStream,arg:&String) { // based on https://avacariu.me/articles/2015/rust-echo-server-example
    println!("Command Processing STARTED");
    let mut packetsSec = 0;
    let mut startTime = time::precise_time_s();

    let mut servoPositions = servo_positions{y_deg:0,x_deg:0};
    let mut port = serial::open(arg).unwrap();
    configureSerialPort(&mut port).unwrap();


    let mut tmp_stream = &stream;
    for byte in tmp_stream.bytes() { // read individual bytes
        //println!("{}", byte.unwrap());
        match byte {
            Err(e) => panic!("Error reading byte from TCP stream: {}", e),
            Ok(b) => {
                //println!("{}",b /*as char*/); // currently just prints out decimal value of each byte
                if b == 0xFE { // if 254 in ascii then decode rest and send
                    //println!("byte was magic");
                    let mut tmp_buf = [0;3];
                    let mut handle = tmp_stream.take(3);
                    handle.read(&mut tmp_buf);
                    //println!("The rest of the packet: {},{},{}",tmp_buf[0],tmp_buf[1],tmp_buf[2]);
                    if tmp_buf[2] == 0xFF { // if last byte matches termination, then continue reading
                            packetsSec += 1; // increment valid packet count
                            servoPositions.y_deg = tmp_buf[0] as u8;
                            servoPositions.x_deg = tmp_buf[1] as u8;
                            servoSend(&mut port,&servoPositions); // send the data over serial
                    }
                    //println!("Current Time {}",time::precise_time_s());
                    if (startTime + 1.0) < time::precise_time_s() {
                        println!("Packets/Sec: {}",packetsSec);
                        startTime = time::precise_time_s();
                        packetsSec = 0;
                    }
                }
            }
        }
    }
    println!("Command Processing STOPPED");
}

fn askServoPos (servoPositions: &mut servo_positions) {
    let stdin = io::stdin();

    // read y_pos
    let input = &mut String::new();
    print!("Enter the y_deg to send in degrees: ");
    io::stdout().flush();
    let y_pos_result = stdin.read_line(input);
    servoPositions.y_deg = input.trim().parse::<u8>().unwrap();
    println!("y_pos set: {}",servoPositions.y_deg);

    // read x_pos
    let input = &mut String::new();
    print!("Enter the x_deg to send in degrees: ");
    io::stdout().flush();
    let x_pos_result = stdin.read_line(input);
    servoPositions.x_deg = input.trim().parse::<u8>().unwrap();
    println!("x_pos set: {}",servoPositions.x_deg);
}


fn servoSend <T: SerialPort>(port: &mut T, servoPositions: &servo_positions) {
    //println!("Prepping Packet");
    let mut send_buf : Vec<u8> = vec!(0xFE, servoPositions.y_deg as u8, servoPositions.x_deg as u8, 0xFF,0x0D,0x0A);
    //println!("Sending Packet...");
    let writeResult = port.write(&send_buf);

    //println!("Packet Sent");
}

// TODO: this should read till no input from buffer, not sure how to do that yet :\
fn readPort <T: SerialPort>(port: &mut T) {
    let mut rcv_buf: Vec<u8> = (0..128).collect();
    port.read(&mut rcv_buf[..]).is_ok();
    let stringBuffer = String::from_utf8(rcv_buf).unwrap();
    print!("{}",stringBuffer);
}


fn configureSerialPort<T: SerialPort>(port: &mut T) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));
    try!(port.set_timeout(Duration::from_millis(1000)));

    Ok(())
}
