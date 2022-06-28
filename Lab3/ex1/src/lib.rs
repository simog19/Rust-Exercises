use std::mem;

pub struct CircularBuffer<T> {
    len: usize,
    data: Vec<T>,
    wrpos: usize,
    rdpos: usize
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buf = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buf.push(T::default());
        }
        CircularBuffer{
            data: buf,
            wrpos: 0,
            rdpos: 0,
            len: 0
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        return if self.len == self.data.len() {
            Err(Error::FullBuffer)
        } else {
            self.data[self.wrpos] = _element;
            self.wrpos = (self.wrpos + 1) % self.data.len();
            self.len += 1;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        }
        else {
            //Replaces dest with the default value of T, returning the previous dest value.
            let element = mem::take(&mut self.data[self.rdpos]);
            self.rdpos = (self.rdpos + 1) % self.data.len();
            self.len -= 1;
            return Ok(element);
        }
    }

    pub fn clear(&mut self) {
        while self.len>0{
            self.read().unwrap();
        }
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.len == self.data.len(){
            self.read().unwrap();
        }
        self.write(_element);
    }
}
