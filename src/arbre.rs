pub struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct Avl {
    root: Option<Box<Node>>,
    size: u32,
}

impl Avl {
    pub fn new() -> Avl {
        Avl {
            root: None,
            size: 0,
        }
    }
    pub fn insert(&mut self, value: u32) -> bool {
        match &self.root {
            Some(_) => {
                let ret = self.root.as_mut().unwrap().insert(value);
                if ret {
                    self.size += 1;
                }
                return ret;
            }
            None => {
                self.root = Node::new(value);
                self.size += 1;
                return true;
            }
        }
    }

    pub fn display(&self) {
        match self.root {
            Some(_) => self.root.as_ref().unwrap().display(),
            None => {}
        }
    }
}

impl Node {
    fn new(value: u32) -> Option<Box<Node>> {
        let n = Node {
            value,
            left: None,
            right: None,
        };
        Some(Box::new(n))
    }

    fn insert(&mut self, value: u32) -> bool {
        if value > self.value {
            match self.right {
                Some(_) => {
                    return self.right.as_mut().unwrap().insert(value);
                }
                None => {
                    self.right = Node::new(value);
                    return true;
                }
            }
        } else if value < self.value {
            match self.left {
                Some(_) => {
                    return self.left.as_mut().unwrap().insert(value);
                }
                None => {
                    self.left = Node::new(value);
                    return true;
                }
            }
        }

        false
    }

    fn display(&self) {
        match self.left {
            Some(_) => self.left.as_ref().unwrap().display(),
            None => {}
        }
        println!("{} ", self.value);

        match self.right {
            Some(_) => self.right.as_ref().unwrap().display(),
            None => {}
        }
    }

    // fn search(&self, value: u32) -> bool {
    //     if value == self.value {
    //         return true;
    //     }

    //     if value > self.value && self.right != None {
    //         self.right.search(value);
    //     } else if value < self.value && self.left != None {
    //         self.left.search(value);
    //     }
    //     false
    // }
}
