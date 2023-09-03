const MAX_SIZE: usize = 10_000;

struct Stack {
    storage: [i32; MAX_SIZE],
    top: usize,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            storage: [0; MAX_SIZE],
            top: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn is_full(&self) -> bool {
        self.top == MAX_SIZE
    }

    fn push(&mut self, x: i32) {
        if self.is_full() {
            println!("error: stack is full");
            return;
        }
        self.storage[self.top] = x;
        self.top += 1;
    }

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            println!("error: stack is empty");
            return -1;
        }
        self.top -= 1;
        self.storage[self.top]
    }

    fn show(&self) {
        for i in 0..self.top {
            print!("{} ", self.storage[i]);
        }
        println!("");
    }
}

fn main() {
    let mut stack_0 = Stack::new();
    stack_0.push(3); //{3}
    stack_0.push(5); //{3,5}
    stack_0.push(7); //{3,5,7}
    stack_0.show();

    println!("{}", stack_0.pop()); // {3,5}
    stack_0.show();
    println!("{}", stack_0.pop()); // {3}
    stack_0.show();
    stack_0.push(9);
    stack_0.show();
}
