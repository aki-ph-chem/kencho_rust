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

    fn deque(&mut self) -> i32 {
        if self.is_empty() {
            println!("error: queue is empty");
            return -1;
        }
        let res = self.storage[self.head];
        self.head += 1;
        if self.head == MAX_SIZE {
            self.head = 0;
        }
        return res;
    }

    fn show(&self) {
        if self.is_empty() {
            println!("error: queue is empty");
        }else{
            for i in self.head..self.tail {
                print!("{} ", self.storage[i]);
            }
            println!("");
        }
    }
}

fn main() {
    let mut que_0 = Que::new(); 
    que_0.enque(3);
    que_0.enque(5);
    que_0.enque(7);
    que_0.show();

    println!("{}", que_0.deque());
    println!("{}", que_0.deque());
    println!("{}", que_0.deque());

    // エラー: empty
    println!("{}", que_0.deque());
    que_0.show();

    que_0.enque(9);
    que_0.enque(11);
    que_0.show();
}
