use std::fs::File;
use std::io;
use std::io::Read;

// File reader with fixed-size internal buffer.
pub struct ChunkReader {
    file: File,
    buffer: Vec<u8>,
    capacity: usize,
    cursor: usize,
}

impl ChunkReader {
    pub fn new(file: File, buffer_size: usize) -> Self {
        ChunkReader {
            file,
            buffer: vec![0; buffer_size],
            capacity: 0,
            cursor: 0,
        }
    }

    #[inline]
    fn buffer_exhausted(&self) -> bool {
        self.cursor == self.capacity
    }

    fn fill_buffer(&mut self) -> io::Result<bool> {
        let read_bytes = self.file.read(&mut self.buffer)?;
        if read_bytes == 0 {
            return Ok(false);
        }

        self.capacity = read_bytes;
        self.cursor = 0;
        Ok(true)
    }
}

impl Iterator for ChunkReader {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer_exhausted() {
            let has_more = self.fill_buffer().unwrap_or(false);
            if !has_more {
                return None;
            }
        }

        let next = self.buffer[self.cursor];
        self.cursor += 1;
        Some(next)
    }
}
