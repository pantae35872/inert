use std::{fs::File, io::Write, time::Duration};

use rppal::uart::{self, Parity, Queue, Status, Uart};
use thiserror::Error;
use tokio::task::LocalEnterGuard;

pub struct Esp32Cam {
    buffer: Vec<u8>,
    uart: Uart,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] uart::Error),
}

impl Esp32Cam {
    pub fn new() -> Result<Self, Error> {
        let uart = Uart::new(115_200, Parity::None, 8, 1)?;

        assert!(
            !uart.is_read_blocking(),
            "esp32 cam shouldn't be blocking read"
        );

        assert!(
            !uart.is_write_blocking(),
            "esp32 cam shouldn't be blocking write"
        );
        Ok(Self {
            buffer: Vec::new(),
            uart,
        })
    }

    pub fn capture(&mut self) -> Result<&'_ [u8], Error> {
        self.uart.flush(Queue::Input)?;
        self.uart.write(&[0x55])?;
        self.uart.drain()?;

        let mut length = [0u8; 4];
        assert_eq!(
            self.uart.read(&mut length)?,
            4,
            "Esp32 cam is not writing the length fast enough.. rare"
        );
        let length = u32::from_le_bytes(length) as usize;

        self.buffer.resize(length, 0);
        let mut file = File::create("/home/john/img.jpeg").unwrap();

        let mut read_pos = 0;
        while read_pos < length {
            let readed = self
                .uart
                .read(&mut self.buffer[read_pos..][..(length - read_pos).min(512)])?;
            file.write_all(&self.buffer[read_pos..][..readed]).unwrap();
            read_pos += readed;
        }

        Ok(&self.buffer)
    }
}
