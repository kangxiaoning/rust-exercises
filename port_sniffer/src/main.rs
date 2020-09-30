use clap::{load_yaml, App};

use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

fn scan(tx: Sender<u16>, begin_port: u16, stop_port: u16, ip_address: IpAddr) {
    let mut port: u16 = begin_port;
    loop {
        // println!("{}", port);
        match TcpStream::connect_timeout(
            &SocketAddr::new(ip_address, port),
            Duration::from_millis(100),
        ) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if stop_port <= port {
            break;
        }
        port += 1;
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let mut num_threads = matches
        .value_of("threads_number")
        .unwrap_or("1")
        .parse::<u16>()
        .unwrap();

    let ip_address = IpAddr::from_str(matches.value_of("IPADDRESS").unwrap()).unwrap();

    let mut begin_port = matches
        .value_of("begin_port")
        .unwrap_or("1")
        .parse::<u16>()
        .unwrap();

    let mut end_port = matches
        .value_of("end_port")
        .unwrap_or("1024")
        .parse::<u16>()
        .unwrap();

    if begin_port > end_port {
        std::mem::swap(&mut begin_port, &mut end_port);
    }

    let step = (end_port - begin_port) / num_threads;
    // 如果端口范围小于线程数
    if step == 0 {
        if end_port > begin_port {
            num_threads = end_port - begin_port
        } else {
            num_threads = 1
        }
    }

    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        let mut begin = begin_port + step * i;
        if i != 0 {
            begin += 1;
        }

        let mut end = begin_port + step * (i + 1);
        if i == num_threads - 1 {
            end = end_port
        };

        thread::spawn(move || {
            scan(tx, begin, end, ip_address);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
