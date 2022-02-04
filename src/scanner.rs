use std::sync::{Arc, Barrier};
use std::{thread};
use std::fmt;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Default)]
pub(crate) struct Options {
    pub response: bool,
    // Check if the connection returns any bytes
    pub udp: bool,
    // use udp
    pub tcp: bool, // use tcp
}

pub(crate) struct Scanner {
    options: Options,

    target: String,
    start_port: u16,
    max_port: u16,
    timeout: Duration,
}

#[derive(Debug)]
pub enum ScannerError {
    InvalidPortRange,
    InvalidTarget,
}

impl std::error::Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScannerError::InvalidPortRange => write!(f, "The given port range is invalid."),
            ScannerError::InvalidTarget => write!(f, "The given target could not be resolved."),
        }
    }
}

pub(crate) fn build_options() -> Options {
    Options::default()
}

impl Options {
    // Immutable access.
    /**
    pub(crate) fn udp(&self) -> &bool {
        &self.udp
    }
    pub(crate) fn tcp(&self) -> &bool {
        &self.tcp
    }
    pub(crate) fn response(&self) -> &bool {
        &self.response
    }
    **/

    // Mutable access.
    pub(crate) fn udp_mut(&mut self) -> &mut bool {
        &mut self.udp
    }
    pub(crate) fn tcp_mut(&mut self) -> &mut bool {
        &mut self.tcp
    }
    pub(crate) fn response_mut(&mut self) -> &mut bool {
        &mut self.response
    }
}

pub(crate) fn build_scanner(options: Options) -> Scanner {
    Scanner {
        options,
        target: "".to_string(),
        start_port: 0,
        max_port: 0,
        timeout: Duration::from_secs(10),
    }
}


impl Scanner {
    pub fn set_target(&mut self, target: String) {
        self.target = target
    }

    pub fn set_port_range(&mut self, start_port: u16, max_port: u16) {
        self.start_port = start_port;
        self.max_port = max_port;
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout
    }

    pub fn start(&mut self) -> Result<(), ScannerError> {
        // check for errors
        self.validate()?;

        self.start_threads()
    }

    fn validate(&mut self) -> Result<(), ScannerError> {
        if self.max_port <= self.start_port {
            // Return error
            return Err(ScannerError::InvalidPortRange);
        }
        Ok(())
    }

    fn start_threads(&mut self) -> Result<(), ScannerError>  {
        let length = self.max_port - self.start_port;
        let mut handles = Vec::with_capacity(length as usize);
        let barrier = Arc::new(Barrier::new(length as usize));

        if let Ok(socket_addresses) = format!("{}:0", self.target).to_socket_addrs() {
            let sockets: Vec<SocketAddr> = socket_addresses.collect();

            if sockets.is_empty() {
                return Err(ScannerError::InvalidTarget)
            }
            let ip = sockets[0].ip();

            for port in self.start_port..self.max_port {
                let c = Arc::clone(&barrier);
                let timeout = self.timeout;

                let handler = thread::spawn(move || {
                    let target = SocketAddr::new(ip.clone(), port);
                    c.wait();
                    match TcpStream::connect_timeout(&target, timeout) {
                        Ok(_) => println!("{}", port),
                        Err(_) => ()
                    }
                });

                handles.push(handler);
            }

            for handle in handles {
                handle.join().unwrap();
            }
            return Ok(())
        }

        return Err(ScannerError::InvalidTarget)
    }
}