use crate::mem::Mem;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fs, io};

pub struct Eeprom {
    memory: Vec<u8>,
}

impl Eeprom {
    pub fn new(image: &[u8]) -> Self {
        let memory = image.to_vec();

        Self { memory }
    }

    pub fn new_with_size(size: usize, image: &[u8]) -> Self {
        let mut memory = Vec::new();

        if size > image.len() {
            let padding_size = size - image.len();
            memory.resize(padding_size, 0);
        }

        let slice = if size < image.len() {
            &image[..size]
        } else {
            image
        };
        memory.extend_from_slice(slice);

        Self { memory }
    }

    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let read_bytes = fs::read(path)?;

        Ok(Self::new(&read_bytes))
    }

    pub fn from_file_with_size(size: usize, path: impl AsRef<Path>) -> io::Result<Self> {
        let read_bytes = fs::read(path)?;

        Ok(Self::new_with_size(size, &read_bytes))
    }
}

impl Mem for Eeprom {
    fn size(&self) -> usize {
        self.memory.len()
    }
}

impl Deref for Eeprom {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}

impl DerefMut for Eeprom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BYTES: [u8; 8] = [5, 29, 1, 9, 12, 4, 38, 15];

    #[test]
    fn should_read_bytes() {
        let eeprom = Eeprom::new(&TEST_BYTES);
        assert_eq!(4, eeprom[5]);
    }

    #[test]
    fn should_add_padding() {
        let eeprom = Eeprom::new_with_size(16, &TEST_BYTES);
        assert_eq!(0, eeprom[7]);
    }

    #[test]
    fn should_add_image_after_padding() {
        let eeprom = Eeprom::new_with_size(16, &TEST_BYTES);
        assert_eq!(5, eeprom[8])
    }
}
