/// One way to implement a queue is to use a linked list; however, that requires a lot of dynamic memory manipulation to add/remove individual items.
/// A more low-level approach is to use a circular buffer: the compromise is that the capacity of the queue is then "fixed". For a background on circular buffers,
/// you can consult https://en.wikipedia.org/wiki/Circular_buffer

// A partial implementation is provided below; please finish it and add some more methods; please remember to run 'cargo fmt' and 'cargo clippy' after
// every step to get feedback from the rust compiler!

// 1) implement read()

// 2) the queue now has a fixed size; change the definition so that the data member becomes a Box<[u8]>; you can use the provided function 'make_box' to make
// boxed slices of arbitrary sizes. Make changes to your method definitions as needed (the definition of 'write' should not need changes!)

// 3) change the method 'new()' into 'new(size: usize)' that initializes a ring buffer of the given size (instead of a fixed size of 16); use the 'make_box' function.

// 4) in a queue that has size N, how many elements can be stored at one time? (test your answer experimentally)

// 5) EXTRA EXERCISES:
//  - add a method "has_room" so that "queue.has_room()" is true if and only if writing to the queue will succeed
//  - add a method "peek" so that "queue.peek()" returns the same thing as "queue.read()", but leaves the element in the queue

use std::boxed::Box;

struct RingBuffer {
    data: Box<[u8]>, // Use a boxed slice for dynamic size
    start: usize,    // Tracks the start of the buffer
    end: usize,      // Tracks the end of the buffer
    size: usize,     // Total size of the buffer
}

impl RingBuffer {
    fn new(size: usize) -> RingBuffer {
        RingBuffer {
            data: make_box(size),
            start: 0,
            end: 0,
            size,
        }
    }

    fn read(&mut self) -> Option<u8> {
        if self.start == self.end {
            None
        } else {
            let value = self.data[self.start];
            self.start = (self.start + 1) % self.size;
            Some(value)
        }
    }

    fn write(&mut self, value: u8) -> bool {
        let next_pos = (self.end + 1) % self.size;
        if next_pos == self.start {
            false
        } else {
            self.data[self.end] = value;
            self.end = next_pos;
            true
        }
    }

    fn has_room(&self) -> bool {
        (self.end + 1) % self.size != self.start
    }

    fn peek(&self) -> Option<u8> {
        if self.start == self.end {
            None
        } else {
            Some(self.data[self.start])
        }
    }
}

fn make_box(reqsize: usize) -> Box<[u8]> {
    vec![0; reqsize].into_boxed_slice()
}

impl Iterator for RingBuffer {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.read()
    }
}

fn main() {
    let mut queue = RingBuffer::new(16);

    for i in 1..=10 {
        assert!(queue.write(i));
    }

    assert_eq!(queue.peek(), Some(1));

    assert!(queue.has_room());

    for elem in queue.by_ref() {
        println!("{elem}");
    }

    assert_eq!(queue.read(), None);
}
