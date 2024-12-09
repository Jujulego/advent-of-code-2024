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

#[derive(Clone, Debug)]
struct FileSystem {
    layout: Vec<u32>,
}

impl FileSystem {
    fn file_blocks(&self) -> AmphipodBlockIterator<'_> {
        AmphipodBlockIterator {
            layout: &self.layout[..],
            first_span_blocks: self.layout.first().copied(),
            first_span_id: 0,
            last_span_blocks: self.layout.last().copied(),
            last_span_id: self.layout.len() - 1,
        }
    }

    fn file_spans(&self) -> AmphipodSpanIterator<'_> {
        AmphipodSpanIterator {
            layout: &self.layout[..],
            first_span_id: 0,
            last_span_id: self.layout.len() - 1,
        }
    }

    fn layout(&self) -> &[u32] {
        &self.layout
    }
}

/////////////////////////////////////////////////////////////////////
// Amphipod Block
/////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum AmphipodBlock {
    File { id: usize },
    Empty,
}

/////////////////////////////////////////////////////////////////////
// Amphipod Block Iterator
/////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct AmphipodBlockIterator<'a> {
    layout: &'a [u32],
    first_span_blocks: Option<u32>,
    first_span_id: usize,
    last_span_blocks: Option<u32>,
    last_span_id: usize,
}

impl<'a> AmphipodBlockIterator<'a> {
    fn first_span_blocks(&self) -> Option<&u32> {
        if self.last_span_id == self.first_span_id {
            zip(self.last_span_blocks.as_ref(), self.first_span_blocks.as_ref())
                .next()
                .map(|(first, last)| min(last, first))
        } else {
            self.first_span_blocks.as_ref()
        }
    }

    fn last_span_blocks(&self) -> Option<&u32> {
        if self.last_span_id == self.first_span_id {
            zip(self.last_span_blocks.as_ref(), self.first_span_blocks.as_ref())
                .next()
                .map(|(first, last)| min(last, first))
        } else {
            self.last_span_blocks.as_ref()
        }
    }

    fn build_block(&self, id: usize) -> AmphipodBlock {
        if id % 2 == 0 {
            AmphipodBlock::File { id: id / 2 }
        } else {
            AmphipodBlock::Empty
        }
    }
}

impl<'a> Iterator for AmphipodBlockIterator<'a> {
    type Item = AmphipodBlock;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.first_span_blocks() {
                Some(0) => {
                    if self.layout.is_empty() {
                        self.first_span_blocks = None;
                        continue;
                    }

                    self.layout = &self.layout[1..];
                    self.first_span_blocks = self.layout.first().copied();

                    self.first_span_id += 1;
                }
                Some(blocks) => {
                    self.first_span_blocks = Some(blocks - 1);
                    break Some(self.build_block(self.first_span_id));
                }
                None => break None
            }
        }
    }
}

impl<'a> DoubleEndedIterator for AmphipodBlockIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.last_span_blocks() {
                Some(0) => {
                    if self.layout.is_empty() {
                        self.last_span_blocks = None;
                        continue;
                    }

                    self.layout = &self.layout[..self.layout.len() - 1];
                    self.last_span_blocks = self.layout.last().copied();

                    if self.last_span_blocks.is_none() {
                        break None;
                    }

                    self.last_span_id -= 1;
                }
                Some(blocks) => {
                    self.last_span_blocks = Some(blocks - 1);
                    break Some(self.build_block(self.last_span_id));
                }
                None => break None
            }
        }
    }
}

impl<'a> FusedIterator for AmphipodBlockIterator<'a> {}

/////////////////////////////////////////////////////////////////////
// Amphipod Span
/////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AmphipodSpan {
    File { id: usize, size: u32 },
    Empty { size: u32 },
}

impl AmphipodSpan {
    fn is_empty(&self) -> bool {
        matches!(self, AmphipodSpan::Empty { .. })
    }

    fn is_file(&self) -> bool {
        matches!(self, AmphipodSpan::File { .. })
    }

    fn id(&self) -> Option<&usize> {
        match self {
            AmphipodSpan::File { id, .. } => Some(id),
            AmphipodSpan::Empty { .. } => None
        }
    }

    fn size(&self) -> &u32 {
        match self {
            AmphipodSpan::File { size, .. } => size,
            AmphipodSpan::Empty { size } => size
        }
    }

    fn size_mut(&mut self) -> &mut u32 {
        match self {
            AmphipodSpan::File { size, .. } => size,
            AmphipodSpan::Empty { size } => size
        }
    }
}

/////////////////////////////////////////////////////////////////////
// Amphipod Span Iterator
/////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct AmphipodSpanIterator<'a> {
    layout: &'a [u32],
    first_span_id: usize,
    last_span_id: usize,
}

impl<'a> Iterator for AmphipodSpanIterator<'a> {
    type Item = AmphipodSpan;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.layout.is_empty() {
            let id = self.first_span_id;
            let size = self.layout[0];

            self.first_span_id += 1;
            self.layout = &self.layout[1..];

            if id % 2 == 0 {
                Some(AmphipodSpan::File { id: id / 2, size })
            } else {
                Some(AmphipodSpan::Empty { size })
            }
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for AmphipodSpanIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.layout.is_empty() {
            let id = self.last_span_id;
            let size = self.layout.last().copied().unwrap();

            if self.last_span_id > 0 {
                self.last_span_id -= 1;
            }

            self.layout = &self.layout[..self.layout.len() - 1];

            if id % 2 == 0 {
                Some(AmphipodSpan::File { id: id / 2, size })
            } else {
                Some(AmphipodSpan::Empty { size })
            }
        } else {
            None
        }
    }
}

impl<'a> FusedIterator for AmphipodSpanIterator<'a> {}

/////////////////////////////////////////////////////////////////////
// main
/////////////////////////////////////////////////////////////////////

fn main() {
    let filesystem = FileSystem {
        layout: read_lines!("day-09/input.txt")
            .next().unwrap()
            .chars()
            .map(|c| (c as u8 - b'0') as u32)
            .collect::<Vec<_>>(),
    };

    // let initialSize = filesystem.layout().iter().sum::<u32>();
    // println!("initialSize: {initialSize}");

    // Part 01
    let mut blocks = filesystem.file_blocks();
    let mut idx = 0;
    let mut part01 = 0;

    while let Some(block) = blocks.next() {
        if let AmphipodBlock::File { id } = block {
            part01 += id * idx;
            idx += 1;
        } else {
            while let Some(block) = blocks.next_back() {
                if let AmphipodBlock::File { id } = block {
                    part01 += id * idx;
                    idx += 1;
                    break;
                }
            }
        }
    }

    println!("part 01: {part01}");

    // Part 02
    let mut reworked = filesystem.file_spans().collect::<Vec<_>>();
    // assert_eq!(initialSize, reworked.iter().map(|s| *s.size()).sum::<u32>());

    for file in filesystem.file_spans().rev().filter(AmphipodSpan::is_file) {
        let file_idx = reworked.iter().position(|x| x == &file).unwrap();

        let span = reworked[..file_idx].iter()
            .enumerate()
            .filter(|(_, span)| span.is_empty())
            .find(|(_, span)| span.size() >= file.size());

        if let Some((idx, &span)) = span {
            if idx == file_idx - 1 {
                *reworked[file_idx + 1].size_mut() += *reworked[idx].size();
                *reworked[idx].size_mut() = 0;
            } else {
                reworked.remove(file_idx);

                if reworked.len() == file_idx {
                    *reworked[file_idx - 1].size_mut() += file.size();
                } else {
                    *reworked[file_idx - 1].size_mut() += reworked[file_idx].size() + file.size();
                    reworked.remove(file_idx);
                }

                reworked.remove(idx);
                reworked.insert(idx, AmphipodSpan::Empty { size: span.size() - file.size() });
                reworked.insert(idx, file);
                reworked.insert(idx, AmphipodSpan::Empty { size: 0 });
            }
        }

        // assert_eq!(initialSize, reworked.iter().map(|s| *s.size()).sum::<u32>(), "error on {:?}", file.id());
    }

    let mut part02 = 0;
    let mut idx = 0;

    for span in &reworked {
        if let AmphipodSpan::File { id, .. } = span {
            part02 += id * (idx..idx + *span.size() as usize).sum::<usize>();
        }

        idx += *span.size() as usize;
    }

    // println!("{:?}", reworked.iter().map(|s| *s.size()).sum::<u32>());
    println!("part 02: {part02}");
}
