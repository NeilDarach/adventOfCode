use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut files = parser::parse(input);
    while files.step() {}
    Ok(files.checksum().to_string())
}

#[derive(Debug)]
pub struct Files {
    blocks: Vec<Option<u64>>,
    free_blocks: Vec<(usize, usize)>,
    used_blocks: Vec<(usize, usize)>,
}

impl Files {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            free_blocks: vec![],
            used_blocks: vec![],
        }
    }

    pub fn step(&mut self) -> bool {
        if self.used_blocks.is_empty() {
            return false;
        }
        let file = self.used_blocks.pop().unwrap();
        if let Some(i) = self
            .free_blocks
            .iter()
            .position(|&e| file.0 > e.0 && e.1 >= file.1)
        {
            let free = self.free_blocks[i];
            for i in 0..(file.1) {
                self.blocks.swap(free.0 + i, file.0 + i);
            }
            self.free_blocks[i] = (free.0 + file.1, free.1 - file.1);
        }
        true
    }

    pub fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, e)| {
                if e.is_some() {
                    e.unwrap() * i as u64
                } else {
                    0
                }
            })
            .sum()
    }
}

mod parser {
    use super::*;
    pub fn parse(input: &str) -> Files {
        let mut id = 0;
        let mut empty = false;
        let mut files = Files::new();
        for c in input.trim().chars() {
            let count = c.to_digit(10).expect("can't parse c");
            let block = if empty { None } else { Some(id) };
            for _ in 0..count {
                files.blocks.push(block);
            }
            empty = !empty;
            if empty {
                id += 1;
            }
        }
        let mut i = 0;
        for chunk in files.blocks.chunk_by(|a, b| a == b) {
            if chunk[0].is_some() {
                files.used_blocks.push((i, chunk.len()))
            } else {
                files.free_blocks.push((i, chunk.len()))
            }
            i += chunk.len();
        }
        files
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("2858", process(SAMPLE)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let files = parser::parse(SAMPLE);
        assert_eq!(Some(0), files.blocks[0]);
        assert_eq!(Some(0), files.blocks[1]);
        assert_eq!(None, files.blocks[2]);
        assert_eq!(None, files.blocks[3]);
        assert_eq!(None, files.blocks[4]);
        assert_eq!(Some(1), files.blocks[5]);
        assert_eq!(Some(1), files.blocks[6]);
        assert_eq!(Some(1), files.blocks[7]);
        assert_eq!(None, files.blocks[8]);
        assert_eq!(None, files.blocks[9]);
        assert_eq!(None, files.blocks[10]);
        assert_eq!(Some(2), files.blocks[11]);
        assert_eq!(None, files.blocks[12]);
        assert_eq!(None, files.blocks[13]);
        assert_eq!(None, files.blocks[14]);
        Ok(())
    }

    #[test]
    fn test_checksum() -> miette::Result<()> {
        let mut files = Files::new();
        files.blocks = vec![Some(0), Some(0), Some(9), Some(9), Some(8), None, None];
        assert_eq!(18 + 27 + 32, files.checksum());
        Ok(())
    }
}
