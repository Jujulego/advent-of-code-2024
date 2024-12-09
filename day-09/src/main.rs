use std::cmp::min;
use std::iter::{zip, FusedIterator};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

/////////////////////////////////////////////////////////////////////
// File System
/////////////////////////////////////////////////////////////////////

struct FileSystem {
    layout: Vec<u8>,
}

impl FileSystem {
    fn file_blocks(&self) -> FileBlockIterator<'_> {
        FileBlockIterator {
            layout: &self.layout[..],
            first_part_blocks: self.layout.first().copied(),
            first_part_id: 0,
            last_part_blocks: self.layout.last().copied(),
            last_part_id: self.layout.len() - 1,
        }
    }
}

/////////////////////////////////////////////////////////////////////
// File Block
/////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum FileBlock {
    File(usize),
    Empty,
}

/////////////////////////////////////////////////////////////////////
// File Block Iterator
/////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct FileBlockIterator<'a> {
    layout: &'a [u8],
    first_part_blocks: Option<u8>,
    first_part_id: usize,
    last_part_blocks: Option<u8>,
    last_part_id: usize,
}

impl<'a> FileBlockIterator<'a> {
    fn first_part_blocks(&self) -> Option<&u8> {
        if self.last_part_id == self.first_part_id {
            zip(self.last_part_blocks.as_ref(), self.first_part_blocks.as_ref())
                .next()
                .map(|(first, last)| min(last, first))
        } else {
            self.first_part_blocks.as_ref()
        }
    }

    fn last_part_blocks(&self) -> Option<&u8> {
        if self.last_part_id == self.first_part_id {
            zip(self.last_part_blocks.as_ref(), self.first_part_blocks.as_ref())
                .next()
                .map(|(first, last)| min(last, first))
        } else {
            self.last_part_blocks.as_ref()
        }
    }
}

impl<'a> Iterator for FileBlockIterator<'a> {
    type Item = FileBlock;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.first_part_blocks() {
                Some(0) => {
                    if self.layout.is_empty() {
                        self.first_part_blocks = None;
                        continue;
                    }

                    self.layout = &self.layout[1..];
                    self.first_part_blocks = self.layout.first().copied();

                    self.first_part_id += 1;
                }
                Some(blocks) => {
                    self.first_part_blocks = Some(blocks - 1);

                    if self.first_part_id % 2 == 0 {
                        break Some(FileBlock::File(self.first_part_id / 2));
                    } else {
                        break Some(FileBlock::Empty);
                    }
                }
                None => break None
            }
        }
    }
}

impl<'a> DoubleEndedIterator for FileBlockIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.last_part_blocks() {
                Some(0) => {
                    if self.layout.is_empty() {
                        self.last_part_blocks = None;
                        continue;
                    }

                    self.layout = &self.layout[..self.layout.len() - 1];
                    self.last_part_blocks = self.layout.last().copied();

                    if self.last_part_blocks.is_none() {
                        break None;
                    }

                    self.last_part_id -= 1;
                }
                Some(blocks) => {
                    self.last_part_blocks = Some(blocks - 1);

                    if self.last_part_id % 2 == 0 {
                        break Some(FileBlock::File(self.last_part_id / 2));
                    } else {
                        break Some(FileBlock::Empty);
                    }
                }
                None => break None
            }
        }
    }
}

impl<'a> FusedIterator for FileBlockIterator<'a> {}

/////////////////////////////////////////////////////////////////////
// main
/////////////////////////////////////////////////////////////////////

fn main() {
    let filesystem = FileSystem {
        layout: read_lines!("day-09/input.txt")
            .next().unwrap()
            .chars()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>(),
    };

    let mut blocks = filesystem.file_blocks();
    let mut idx = 0;
    let mut part01 = 0;

    while let Some(block) = blocks.next() {
        if let FileBlock::File(id) = block {
            part01 += id * idx;
            idx += 1;
        } else {
            while let Some(block) = blocks.next_back() {
                if let FileBlock::File(id) = block {
                    part01 += id * idx;
                    idx += 1;
                    break;
                }
            }
        }
    }

    println!("part 01: {part01}");
}
