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

pub struct myStack<T> {
    //TODO
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            //TODO
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // if self.q1.is_empty() {
        //     self.q1.enqueue(elem);
        //     if !self.q2.is_empty() {
        //         while !self.q2.is_empty() {
        //             if let Ok(item) = self.q2.dequeue() {
        //                 self.q1.enqueue(item);
        //             }
        //         }
        //     }
        // } else {
        //     self.q2.enqueue(elem);
        //     if !self.q1.is_empty() {
        //         while !self.q1.is_empty() {
        //             if let Ok(item) = self.q1.dequeue() {
        //                 self.q1.enqueue(item);
        //             }
        //         }
        //     }
        // }
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        // if !self.q1.is_empty() {
        //     self.q1.dequeue()
        // } else if !self.q2.is_empty() {
        //     self.q2.dequeue()
        // } else {
        //     Err("Stack is empty")
        // }
        if self.is_empty() {
            return Err("Stack is empty");
        }

        let (from_queue, to_queue) = if self.q1.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            (&mut self.q1, &mut self.q2)
        };

        while from_queue.size() > 1 {
            if let Ok(item) = from_queue.dequeue() {
                to_queue.enqueue(item);
            }
        }
        from_queue.dequeue()
    }
    pub fn is_empty(&self) -> bool {
        //TODO
        if self.q1.elements.is_empty() && self.q2.elements.is_empty() {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
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
