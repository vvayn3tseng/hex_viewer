use std::fs::File;
use std::io::prelude::*;

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
}

impl ViewerState {
    pub fn new() -> Self {
        ViewerState {
            file_handle: None,
            caches: vec![],
        }
    }

    pub fn open(&mut self, path: String) {
        self.file_handle = Some(File::create(path).unwrap());
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
            .read(&mut cache.buffer)
            .unwrap();
        self.caches.push(cache);

        &self.caches[self.caches.len() - 1].buffer[0..length as usize]
    }
}
