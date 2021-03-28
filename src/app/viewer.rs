use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, SeekFrom};

const VIEWER_X_START: u16 = 15;
const VIEWER_Y_START: u16 = 3;
const BYTE_NUMBER: usize = 16;
const LINE_COUNT: u16 = VIEWER_X_START + 16 * 2 + 15;

pub struct BufferCache {
    pub offset: u64,
    pub length: u64,
    pub buffer: Vec<u8>,
}

impl BufferCache {
    pub fn new(offset: u64, length: u64) -> Self {
        BufferCache {
            offset,
            length,
            buffer: vec![0; length as usize],
        }
    }
}

pub struct ViewerState {
    pub file_handle: Option<File>,
    pub caches: Vec<BufferCache>,
    pub cursor: (u16, u16),
    pub offset: usize,
    pub height: u16,
}

impl ViewerState {
    pub fn new() -> Self {
        ViewerState {
            file_handle: None,
            caches: vec![],
            cursor: (VIEWER_X_START, VIEWER_Y_START),
            offset: 0,
            height: 0,
        }
    }

    pub fn open(&mut self, path: String) -> Result<(), Error> {
        self.file_handle = Some(File::open(path)?);
        Ok(())
    }

    fn is_range_cached(&self, offset: u64, length: u64) -> isize {
        for index in 0..self.caches.len() {
            let buffer = &self.caches[index];
            if buffer.offset >= offset && buffer.offset + buffer.length <= offset + length {
                return index as isize;
            }
        }

        -1
    }

    pub fn read(&mut self, offset: u64, length: u64) -> &[u8] {
        let index = self.is_range_cached(offset, length);
        if index >= 0 {
            let buffer = &self.caches[index as usize];
            let start: usize = (buffer.offset - offset) as usize;
            let end: usize = (offset + length - buffer.offset - buffer.length) as usize;
            return &buffer.buffer[start..end];
        }

        let mut cache = BufferCache::new(offset, 1048576);

        self.file_handle
            .as_ref()
            .unwrap()
            .seek(SeekFrom::Start(offset))
            .unwrap();

        let actual_length = self
            .file_handle
            .as_ref()
            .unwrap()
            .read(&mut cache.buffer)
            .unwrap() as u64;

        self.caches.push(cache);

        &self.caches[self.caches.len() - 1].buffer[0..actual_length as usize]
    }

    pub fn on_left(&mut self) {
        if self.file_handle.is_none() {
            return;
        }

        if self.cursor.0 - 1 >= VIEWER_X_START {
            self.cursor.0 -= 1;
        }
    }

    pub fn on_right(&mut self) {
        if self.file_handle.is_none() {
            return;
        }

        if self.cursor.0 + 1 <= LINE_COUNT {
            self.cursor.0 += 1;
        }
    }

    pub fn on_up(&mut self) {
        if self.file_handle.is_none() {
            return;
        }

        if self.cursor.1 - 1 >= VIEWER_Y_START {
            self.cursor.1 -= 1;
        } else if self.offset >= BYTE_NUMBER {
            self.offset -= BYTE_NUMBER;
        }
    }

    pub fn on_down(&mut self) {
        if self.file_handle.is_none() {
            return;
        }

        let meta = self.file_handle.as_ref().unwrap().metadata().unwrap();

        let cursor_offset =
            self.offset + (((self.cursor.1 as usize) - (VIEWER_Y_START as usize)) * BYTE_NUMBER);

        if cursor_offset >= meta.len() as usize {
            return;
        }

        if self.cursor.1 + 1 < self.height {
            self.cursor.1 += 1;
        } else {
            self.offset += 16;
        }
    }

    pub fn on_page_down(&mut self) {
        if self.file_handle.is_none() {
            return;
        }

        self.offset += ((self.height - VIEWER_Y_START) * BYTE_NUMBER as u16) as usize;
    }

    pub fn on_page_up(&mut self) {
        if self.file_handle.is_none() {
            return;
        }

        let offset = ((self.height - 3) * BYTE_NUMBER as u16) as usize;

        if self.offset < offset {
            self.offset = 0;
        } else {
            self.offset -= offset;
        }
    }

    pub fn on_jump(&mut self, offset: u64) {
        let floor = offset - (offset % 16);
        self.offset = floor as usize;
    }
}
