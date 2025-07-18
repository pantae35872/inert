use std::{fs::File, io::Write, time::Duration};

use rppal::uart::{self, Parity, Queue, Uart};
use thiserror::Error;

#[repr(u8)]
enum Esp32Command {
    Capture = 0x55,
    Init = 0x56,
}

#[repr(u8)]
enum Esp32Result {
    Success,
    NoCam,
    CamCaptureFailed,
    CamAlreadyInitialized,
}

impl From<u8> for Esp32Result {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Esp32Result::Success,
            0x1 => Esp32Result::NoCam,
            0x2 => Esp32Result::CamCaptureFailed,
            0x3 => Esp32Result::CamAlreadyInitialized,
            _ => panic!("Invalid esp32 cam result"),
        }
    }
}

pub struct Esp32Cam {
    buffer: Vec<u8>,
    uart: Uart,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] uart::Error),
    #[error("Esp32 camera failed")]
    CamFailed,
    #[error("Esp32 camera already initialized")]
    CamAlreadyInitialized,
}

impl Esp32Cam {
    pub fn new() -> Result<Self, Error> {
        let mut uart = Uart::new(57600, Parity::None, 8, 1)?;

        uart.set_read_mode(4, Duration::default())?;

        assert!(
            !uart.is_write_blocking(),
            "esp32 cam shouldn't be blocking write"
        );

        Ok(Self {
            buffer: Vec::new(),
            uart,
        })
    }

    fn command(&mut self, command: Esp32Command) -> Result<(), Error> {
        self.uart.write(&[command as u8])?;

        let mut result = [0u8; 1];
        self.uart.read(&mut result)?;
        let result = Esp32Result::from(result[0]);
        match result {
            Esp32Result::NoCam | Esp32Result::CamCaptureFailed => Err(Error::CamFailed),
            Esp32Result::CamAlreadyInitialized => Err(Error::CamAlreadyInitialized),
            Esp32Result::Success => Ok(()),
        }
    }

    pub fn capture(&mut self) -> Result<&'_ [u8], Error> {
        self.uart.flush(Queue::Input)?;
        self.uart.drain()?;

        match self.command(Esp32Command::Init) {
            Err(Error::CamAlreadyInitialized) | Ok(_) => {}
            Err(err) => return Err(err),
        };

        self.command(Esp32Command::Capture)?;

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
