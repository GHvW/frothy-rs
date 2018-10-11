// use std::collections::HashMap;
use std::io;
use std::io::{Write, Read};
use std::fmt::Debug;

fn main() {
    
    let mut stack: StackImpl<String> = StackImpl::new();
    stack.push("1".to_string());
    
    repl(&mut stack);
}

// enum Lex {
//     Plus = "+"
// }

// struct Node<T> {
//     val: T,
//     next: Option<Box<Node>>
// }

trait Stack<T> {
    // fn cons(&mut self) -> (T, Stack<T>);
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, val: T) -> ();
    fn peek(&self) -> Option<&T>;
    fn clear(&mut self) -> ();
    // fn print(&self) -> ();
}

#[derive(Debug)]
struct StackImpl<T: Debug> {
    stack: Vec<T>
}

impl<T: Debug> StackImpl<T> {
    fn new() -> Self {
        StackImpl { stack: Vec::new() }
    }
}

impl<T: Debug> Stack<T> for StackImpl<T> {
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    
    fn push(&mut self, val: T) -> () {
        self.stack.push(val);
    }
    
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
    
    fn clear(&mut self) -> () {
        self.stack.clear();
    }

    // fn print(&self) -> () {
    //     println!("{:?}", self);
    // }
}

// fn get_input() -> io::Result<String> {
//     let mut buffer = String::new();
//     match io::stdin().read_to_string(&mut buffer)?;
    
// }

fn repl(stack: &mut impl Stack<String>) -> () {
    println!("current stack: {:?}", *stack);
    print!("> ");
    io::stdout().flush().unwrap(); 
    
    let mut buffer = String::new(); //should i make a pre-allocated buffer so I'm not creating a new one each time?
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            eval(buffer, stack); //buffer consumed here
        },
        Err(e) => {
            println!("Couldn't read input, try again. Error: {}", e);
            repl(stack);
        }
    }
}

fn eval(buffer: String, stack: &mut impl Stack<String>) -> &impl Stack<String> {
    //implement
    //evaluate head, if operator call fn, if not repl
    for data in buffer.split_whitespace() {
        stack.push(data.to_string());
    }
    stack
}