use std::fmt::Display;

use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

use serde::Serialize;

pub mod resources;

pub use resources::*;

#[derive(Debug, Default, Serialize)]
pub struct Counter {
    pub iterations: usize,
    pub bytes: usize,
}

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "iterations: {} bytes: {}", self.iterations, self.bytes)
    }
}

#[derive(Debug, Default, Serialize)]
pub struct DIC {
    pub cmd: Counter,
    pub data: Counter,
}

impl WriteOnlyDataCommand for DIC {
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        self.cmd.iterations += 1;
        self.cmd.bytes += data_format_bytes(cmd);

        Ok(())
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        self.data.iterations += 1;
        self.data.bytes += data_format_bytes(buf);

        Ok(())
    }
}

impl Display for DIC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CMD[{}] DATA[{}]", self.cmd, self.data)
    }
}

fn data_format_bytes(input: DataFormat<'_>) -> usize {
    match input {
        DataFormat::U8(buf) => buf.len(),
        DataFormat::U16(buf) => buf.len() * 2,
        DataFormat::U16BE(buf) => buf.len() * 2,
        DataFormat::U16LE(buf) => buf.len() * 2,
        DataFormat::U8Iter(iter) => iter.count(),
        DataFormat::U16BEIter(iter) => iter.count() * 2,
        DataFormat::U16LEIter(iter) => iter.count() * 2,
        _ => panic!("Unable to determine data format"),
    }
}
