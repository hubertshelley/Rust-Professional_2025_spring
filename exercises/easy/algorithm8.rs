/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    // 利用队列的先进先出的元素呼唤实现栈的先进后出特性
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        if self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            // 先将元素放入q2，再将q1中的元素全部放入q2，再将q2中的元素全部放入q1 实现先进后出特性
            self.q2.enqueue(elem);
            for _ in 0..self.q1.size() {
                self.q2.enqueue(self.q1.dequeue().unwrap());
            }
            for _ in 0..self.q2.size() {
                self.q1.enqueue(self.q2.dequeue().unwrap());
            }
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if !self.q1.is_empty() {
            Ok(self.q1.dequeue().map_err(|_| "Queue is empty")?)
        } else {
            Err("Stack is empty")
        }
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
