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
    first_none: usize,
    last_some: usize,
}

impl Files {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            first_none: 0,
            last_some: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        if self.last_some > self.first_none {
            self.blocks.swap(self.last_some, self.first_none);
            while self.blocks[self.first_none].is_some() {
                self.first_none += 1;
            }
            while self.blocks[self.last_some].is_none() {
                self.last_some -= 1;
            }
            true
        } else {
            false
        }
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
        while files.blocks[files.first_none].is_some() {
            files.first_none += 1;
        }
        files.last_some = files.blocks.len() - 1;
        while files.blocks[files.last_some].is_none() {
            files.first_none -= 1;
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
        assert_eq!("1928", process(SAMPLE)?);
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
