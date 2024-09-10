use std::u64::MAX;
const FNV_OFFSET_BASIS: u64 = 0x811C9DC5;
const FNV_PRIME: u64 = 0x100000001B3;
const MOD: u64 = MAX;


#[derive(Default, Clone)]
struct Bucket<K, V> {
    key: K,
    value: V,
    occupied: bool,
}

impl<K,V> Bucket<K,V> 
where
    K: Hashable + Eq + Clone + Default, 
    V: Clone + Default,
{ 
    fn new() -> Self {
        Bucket {
            key: Default::default(),
            value: Default::default(),
            occupied: false,
        }
    }
}


struct HashTable<K, V> {
    table: Vec<Bucket<K,V>>,
    len: usize,
    load_factor: f64,
}

trait Hashable {
    fn hash(&self) -> usize;
}


// fnv_1a
impl Hashable for [u8] {
    fn hash(&self) -> usize {
        let mut hash = FNV_OFFSET_BASIS;
        for &b in self {
            hash ^= b as u64;
            let high = (hash >> 32) as u32;
            let low = hash as u32;
            let prime_high = (FNV_PRIME >> 32) as u32;
            let prime_low = FNV_PRIME as u32;
            let product_high = (high as u64 * prime_high as u64) + (high as u64 * prime_low as u64) + (low as u64 * prime_high as u64);
            let product_low = low as u64 * prime_low as u64;
            hash = (product_high << 32) + product_low;
            hash = hash % MOD;
        }
        hash as usize 
    }
}

impl Hashable for &str {
    fn hash(&self) -> usize {
        self.as_bytes().hash() 
    }
}

impl Hashable for String {
    fn hash(&self) -> usize {
        self.as_bytes().hash() 
    }
}


// const FNV_OFFSET_BASIS: u64 = 0x811C9DC5;
// const FNV_PRIME: u64 = 0x100000001B3;

// struct Fnv1aHasher {
//     hash: u64,
// }

// impl Fnv1aHasher {
//     fn new() -> Self {
//         Fnv1aHasher {
//             hash: FNV_OFFSET_BASIS,
//         }
//     }

//     fn update(&mut self, byte: u8) {
//         self.hash = (self.hash ^ (byte as u64)) * FNV_PRIME;
//     }

//     fn finish(&self) -> u64 {
//         self.hash
//     }
// }

// impl Hashable for [u8] {
//     fn hash(&self) -> u64 {
//         let mut hasher = Fnv1aHasher::new();
//         for byte in self {
//             hasher.update(*byte);
//         }
//         hasher.finish()
//     }
// }

impl<K, V> HashTable<K,V>
where 
K: Eq + Clone + Default + Hashable,
V: Clone+ Default
{
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 127;
        Self { 
            len: 0,
            load_factor: 0.75,
            table: vec![Bucket::<K, V>::default(); INITIAL_CAPACITY], 
        }
    }

    fn hash(&self, key: &K) -> usize {
        let hash_value = key.hash();
        hash_value % self.table.len()
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket = &mut self.table[index];
        if bucket.occupied {
            bucket.value = value;
        } else {
            bucket.key = key;
            bucket.value = value;
            bucket.occupied = true;
            self.len += 1;
            if self.len as f64 / self.table.len() as f64 > self.load_factor {
                self.resize();
            }
        }

    }

    fn resize(&mut self) {
        let new_size: usize;
        if self.table.len() < 1000 {
            new_size = self.table.len() * 2;
        } else {
            new_size = (self.table.len() as f64 * 1.6) as usize;
        }
        let mut new_table = vec![Bucket::new(); new_size];
        for bucket in self.table.iter()  {
            if bucket.occupied {
                let index = self.hash(&bucket.key);
                let new_bucket = &mut new_table[index];
                new_bucket.key = bucket.key.clone();
                new_bucket.value = bucket.value.clone();
                new_bucket.occupied = true;
            }
        }
        self.table = new_table;
    }

    fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(key);
        let bucket = &self.table[index];
        if bucket.occupied && bucket.key == *key {
            Some(bucket.value.clone())
        } else {
            None
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(key);
        let bucket = &mut self.table[index];
        if bucket.occupied && bucket.key == *key {
            let removed_value = bucket.value.clone();
            bucket.key = Default::default();
            bucket.value = Default::default();
            bucket.occupied = false;
            self.len -= 1;
            Some(removed_value.clone())
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.table.iter().filter_map(|bucket| {
            if bucket.occupied {
                Some((&bucket.key, &bucket.value))
            } else {
                None
            }
        })
    }
}


fn main() {
    // println!("Hello, world!");
    // // let data = b"hello";
    // // let hash_value = data.hash();
    // // println!("Hash value: 0x{:x}", hash_value);
    // let data: &[u8] = b"HelloWorld";
    // let data1: &[u8] = b"Hello";
    // let data2: &[u8] = b"World";
    // let hash_value = data.hash();
    // let hash_value1 = data1.hash();
    // let hash_value2 = data2.hash();
    // println!("Hash value: 0x{}", hash_value);
    // println!("Hash value: 0x{}", hash_value1);
    // println!("Hash value: 0x{}", hash_value2);
    // //println!("Hash value: 0x{}", hash_value1 + hash_value2);
    let key: &str = "apple";
    let hash_value = key.hash();
    println!("Hash value for 'apple': {}", hash_value);

    let key_string = String::from("orange");
    let hash_value_string = key_string.hash();
    println!("Hash value for 'orange': {}", hash_value_string);

    let byte_slice: &[u8] = b"banana";
    let hash_value_bytes = byte_slice.hash();
    println!("Hash value for 'banana': {}", hash_value_bytes);

    let mut hashtable = HashTable::new();

    hashtable.insert("apple", 5);
    hashtable.insert("banana", 10);
    hashtable.insert("orange", 15);

    println!("Apple: {:?}", hashtable.get(&"apple")); 
    println!("Banana: {:?}", hashtable.get(&"banana")); 
    println!("Orange: {:?}", hashtable.get(&"orange")); 

    hashtable.remove(&key);
    for (key, value) in hashtable.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}
