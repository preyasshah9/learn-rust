use std::collections::HashMap;
use rand::{Rng, prelude::IteratorRandom};

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    idx: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet{map: HashMap::new(), list: Vec::new(), idx: 0}
    }
    
    fn insert(&mut self, val: i32) -> bool {
        match self.map.get(&val) {
            Some(x) => return false,
            None => {
                self.map.insert(val, self.idx);
                self.list.push(val);
                self.idx = self.idx + 1;
                return true;
            }
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if self.map.get(&val).is_none() {
            return false;
        }
        let x = self.map[&val];
        self.map.remove(&val);

        if(x != self.idx - 1) {
            self.list.swap(x, self.idx - 1);
            self.map.insert(self.list[x], x);
        }
        self.list.pop();
        self.idx = self.idx - 1;
        return true;
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.list.iter().choose(&mut rng).unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */