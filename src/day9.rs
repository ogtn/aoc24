use aoc_runner_derive::aoc;

fn parse(input: &str) -> Vec<i32> {
    let mut disk = Vec::new();
    let mut is_file = true;
    let mut id = 0;

    for size in input.bytes() {
        let size = size - b'0';

        if is_file {
            for _ in 0..size {
                disk.push(id);
            }

            id += 1;
            is_file = false;
        } else {
            for _ in 0..size {
                disk.push(-1);
            }

            is_file = true;
        }
    }

    disk
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    let src = parse(input);
    let mut dst = src.clone();
    let mut cursor = 0;

    for (pos, id) in src.iter().enumerate().rev() {
        if pos <= cursor {
            break;
        }

        if *id == -1 {
            continue;
        }

        dst[pos] = -1;

        while dst[cursor] != -1 {
            cursor += 1;
        }

        dst[cursor] = *id;
        cursor += 1;
    }

    let mut checksum = 0;

    for (pos, id) in dst.iter().enumerate() {
        if *id == -1 {
            break;
        }

        checksum += pos as i64 * *id as i64;
    }

    checksum
}

#[derive(Debug, Default, Clone, Copy)]
struct Block {
    size_pos: u32,
}

impl Block {
    #[inline(always)]
    fn new(size: u8, pos: u32) -> Self {
        Self {
            size_pos: ((size as u32) << 24) | pos,
        }
    }

    #[inline(always)]
    fn pos(&self) -> u32 {
        self.size_pos & 0x00_FF_FF_FF
    }

    #[inline(always)]
    fn size(&self) -> u8 {
        ((self.size_pos >> 24) & 0x00_00_00_FF) as u8
    }

    #[inline(always)]
    fn mv(&mut self, pos: u32) {
        self.size_pos &= 0xFF_00_00_00;
        self.size_pos |= pos;
    }

    #[inline(always)]
    fn set_size(&mut self, size: u8) {
        self.size_pos &= 0x00_FF_FF_FF;
        self.size_pos |= (size as u32) << 24;
    }
}

#[derive(Debug)]
struct Disk {
    blanks: [Block; 10_000],
    files: [Block; 10_000],
}

fn parse_2(input: &str) -> Disk {
    let mut disk = Disk {
        blanks: [Block { size_pos: 0 }; 10_000],
        files: [Block { size_pos: 0 }; 10_000],
    };

    let mut is_file = true;
    let mut pos = 0;
    let mut files_count = 0;
    let mut blank_count = 0;

    for size in input.bytes() {
        let size = size - b'0';

        unsafe {
            if is_file {
                *disk.files.get_unchecked_mut(files_count) = Block::new(size, pos);
                files_count += 1;
                is_file = false;
            } else {
                *disk.blanks.get_unchecked_mut(blank_count) = Block::new(size, pos);
                blank_count += 1;
                is_file = true;
            };
        }

        pos += size as u32;
    }

    disk
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut disk = parse_2(input);
    let mut starts = [0; 10];
    let mut checksum = 0;

    for (id, file) in disk.files.iter_mut().enumerate().rev() {
        let file_size = file.size();

        unsafe {
            let start = starts.get_unchecked_mut(file_size as usize);

            for (pos, blank) in &mut disk
                .blanks
                .get_unchecked_mut(*start..)
                .iter_mut()
                .enumerate()
            {
                if blank.pos() >= file.pos() {
                    break;
                }

                if blank.size() >= file_size {
                    *start += pos;
                    file.mv(blank.pos());
                    blank.set_size(blank.size() - file_size);
                    blank.mv(blank.pos() + file_size as u32);

                    break;
                }
            }
        }

        for offset in 0..file_size {
            checksum += id * (file.pos() as usize + offset as usize);
        }
    }

    checksum
}
