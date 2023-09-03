const MAX_SIZE: usize = 10_000;

struct Que {
    storage: [i32; MAX_SIZE],
    head: usize,
    tail: usize,
}

impl Que {
    fn new() -> Que {
        Que {
            storage: [0; MAX_SIZE],
            head: 0,
            tail: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        self.head == ((self.tail + 1) % MAX_SIZE)
    }

    fn enque(&mut self, x: i32) {
        if self.is_full() {
            println!("error: queue is full");
            return;
        }

        self.storage[self.tail] = x;
        self.tail += 1;

        if self.tail == MAX_SIZE {
            self.tail =0;
        }
    }
}

fn main() {
}
