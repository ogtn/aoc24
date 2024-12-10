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

    // println!("{:2?}", src);
    // println!("{:2?}", dst);

    checksum
}

#[derive(Debug, Default)]
struct Block {
    pos: usize,
    size: u8,
}

#[derive(Debug, Default)]
struct Disk {
    blanks: Vec<Block>,
    files: Vec<Block>,
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, file) in self.files.iter().enumerate() {
            for _ in 0..file.pos {
                write!(f, ".");
            }

            for _ in 0..file.size {
                write!(f, "{id}");
            }

            writeln!(f);
        }

        for blank in &self.blanks {
            for _ in 0..blank.pos {
                write!(f, "x");
            }

            for _ in 0..blank.size {
                write!(f, ".");
            }

            writeln!(f);
        }

        write!(f, "")
    }
}

fn parse_2(input: &str) -> Disk {
    let mut disk = Disk::default();
    let mut is_file = true;
    let mut pos = 0;

    for size in input.bytes() {
        let size = size - b'0';

        if is_file {
            disk.files.push(Block { pos, size });
            is_file = false;
        } else {
            disk.blanks.push(Block { pos, size });
            is_file = true;
        };

        pos += size as usize;
    }

    disk
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut disk = parse_2(input);

    // println!("before: {:#?}", disk);
    // println!("{disk}");

    for (id, file) in disk.files.iter_mut().enumerate().rev() {
        for blank in &mut disk.blanks {
            if blank.pos >= file.pos {
                break;
            }

            if blank.size >= file.size {
                // println!("moving file {}:{:?} to {:?}", id, file, blank);
                file.pos = blank.pos;
                blank.size -= file.size;
                blank.pos += file.size as usize;
                // println!("after move file {}:{:?}, blank: {:?}", id, file, blank);
                break;
            }
        }
    }

    // println!("after: {:#?}", disk);
    // println!("{disk}");

    let mut checksum = 0;

    for (id, file) in disk.files.iter_mut().enumerate().rev() {
        for offset in 0..file.size {
            // println!("{id} * {pos}");
            checksum += id * (file.pos + offset as usize);
        }
    }

    checksum
}
