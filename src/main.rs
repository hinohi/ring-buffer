use std::time::Instant;

const K_COUNT: usize = 500000;

pub struct RingBuffer0 {
    buffer: Vec<i32>,
    read_idx: usize,
    write_idx: usize,
}

impl RingBuffer0 {
    pub fn new(size: usize) -> RingBuffer0 {
        RingBuffer0 {
            buffer: vec![0; size],
            read_idx: 0,
            write_idx: 0,
        }
    }

    pub fn enqueue(&mut self, item: i32) -> bool {
        if self.write_idx - self.read_idx == self.buffer.len() {
            false
        } else {
            let i = self.write_idx % self.buffer.len();
            self.buffer[i] = item;
            self.write_idx += 1;
            true
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.write_idx == self.read_idx {
            None
        } else {
            let item = self.buffer[self.read_idx % self.buffer.len()];
            self.read_idx += 1;
            Some(item)
        }
    }
}

fn benchmark_single(rb: &mut RingBuffer0) {
    let start = Instant::now();
    for _ in 0..K_COUNT {
        for j in 0..1000 {
            rb.enqueue(j);
        }
        for j in 0..1000 {
            assert_eq!(rb.dequeue(), Some(j));
        }
    }
    let duration = start.elapsed().as_millis();
    let count = K_COUNT * (1000 + 1000);
    println!("{} ops in {} ms", count, duration);
    println!("{} ops/ms", count as u128 / duration);
}

fn main() {
    let mut rb0 = RingBuffer0::new(2 * 1024 * 1024);
    benchmark_single(&mut rb0);
}
