use rand::Rng;
struct MyHashSet {
    size: usize,
    capacity: usize,
    arr: Vec<Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        let default_capacity = 8;
        return MyHashSet { 
            size: 0, 
            capacity: 8, 
            arr: vec![Vec::new();default_capacity]
        }
    }
    
    fn add(&mut self, key: i32) {
        if self.size > self.capacity * 2 {
            self.resize(true);
        }
        let idx = self.arr_idx(&key);

        let bucket = &self.arr[idx];

        for b in bucket{
            if *b == key{
                return;
            }
        }

        self.arr[idx].push(key);       
        self.size += 1;
    }

    fn resize(&mut self, up: bool){


        if up {
            self.capacity *= 2;
        } else {
            self.capacity /= 2;
        }

        let mut new_arr: Vec<Vec<i32>> = vec![Vec::new(); self.capacity];

        for bucket in self.arr.iter() {
            for val in bucket{
                let idx = self.arr_idx(val);
                new_arr[idx].push(*val)
            }
        }

        self.arr = new_arr;
    }

    fn arr_idx(&self, el: &i32) -> usize{
        MyHashSet::hash(el) % self.capacity
    }

    fn hash(el: &i32) -> usize{
        *el as usize
    }

    fn remove(&mut self, key: i32) {
        if (self.size < self.capacity / 4) && (self.capacity > 8){
            self.resize(false)
        }

        let idx = self.arr_idx(&key);
        let bucket = &mut self.arr[idx];

        let remove_idx = match bucket.iter().position(|x| *x == key) {
            Some(i) => i,
            None => {
                return;
            }
        };

        bucket.remove(remove_idx);
        self.size -= 1;

    }
    
    fn contains(&self, key: i32) -> bool {
        let idx = self.arr_idx(&key);
        let bucket = &self.arr[idx];
        for el in bucket{
            if *el == key {
                return true;
            }
        }
        return false;

    }

}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */



fn main() {
    let mut a = MyHashSet::new();

    let mut nums = Vec::new();
    for _ in 0..1000{
        let r = rand::thread_rng().gen_range(0..1000);
        nums.push(r);
        a.add(r);
    }
    
    println!("{:?}", a.arr);

    println!("{}", a.contains(3));

    for n in nums {
        a.remove(n);
    }

    println!("{:?}", a.arr);

}
