use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};
mod nestedbtree;
use crate::nestedbtree::DeeplyNestedBTree;
use crate::nestedbtree::NestedBTree;
use rand::Rng;

#[inline(always)]
fn nested_btree_test(width: u16, iterations: u32, serialize: bool) -> Duration {
    let mut map: NestedBTree<u8> = NestedBTree::new();
    let mut coords: Vec<(u16, u16, u8)> = vec![];
    let mut x: u16 = 0;
    let mut y = 0;
    let mut z: u8 = 0;
    //let mut rng = rand::thread_rng();
    for i in 0..iterations {
        // x = rng.gen_range(0..width);
        // y = rng.gen_range(0..width);
        // z = rng.gen_range(0..255);
        if x >= width {
            x = 0;
            y += 1;
        }
        if z == 255 {
            z = 0;
        }
        coords.push((x, y, z));
        x += 1;
        z += 1;
    }
    let start = Instant::now();
    for x in 0..coords.len() {
        let c = coords.get(x).unwrap();
        map.insert(c.0, c.1, c.2, c.2);
    }
    let mut duration = start.elapsed();
    println!("\nnested insert finished...{:?} elapsed", duration);
    if serialize {
        let start2 = Instant::now();
        let mut data: Vec<u8> = vec![];
        data = bincode::serialize(&map).unwrap();
        let mut file = File::create("test.dat").unwrap();
        file.write_all(&data).unwrap();
        duration = start2.elapsed();
        println!("nested write finished...{:?} elapsed", duration);
        let start2 = Instant::now();
        let mut file2 = File::open("test.dat").unwrap();
        let mut data2: Vec<u8> = vec![];
        file2.read_to_end(&mut data2).unwrap();
        let mut map2: NestedBTree<u8> = NestedBTree::new();
        map2 = bincode::deserialize(&data2).unwrap();
        duration = start2.elapsed();
        println!("nested read finished...{:?} elapsed", duration);
        let mut same: bool = true;
        assert!(coords.len() == iterations as usize);
        let start2 = Instant::now();
        for c in coords {
            if *map2.get(c.0, c.1, c.2).unwrap() != c.2 {
                same = false;
                panic!();
            }
        }
        duration = start2.elapsed();
        println!("nested compare finished...{:?} elapsed", duration);
        println!("Data is the same = {}", same);
    } else {
        let mut same: bool = true;
        assert!(coords.len() == iterations as usize);
        let start2 = Instant::now();
        for c in coords {
            if *map.get(c.0, c.1, c.2).unwrap() != c.2 {
                same = false;
                panic!();
            }
        }
        duration = start2.elapsed();
        println!("nested compare finished...{:?} elapsed", duration);
        println!("Data is the same = {}", same);
    }
    let elapsed = start.elapsed();
    println!("total elapsed = {:?}, iterations = {}, dimenions = {} X {} X {}, serialize/deserialize = {}",elapsed,iterations,width,width,256,serialize);
    elapsed
}

#[inline(always)]
fn deeply_nested_btree_test(width: u16, iterations: u32, serialize: bool) -> Duration {
    let mut map: DeeplyNestedBTree<u8> = DeeplyNestedBTree::new();
    let mut coords: Vec<(u16, u16, u8)> = vec![];
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut z: u8 = 0;
    //let mut rng = rand::thread_rng();
    for i in 0..iterations {
        // x = rng.gen_range(0..width);
        // y = rng.gen_range(0..width);
        // z = rng.gen_range(0..255);
        if x >= width {
            x = 0;
            y += 1;
        }
        if z == 255 {
            z = 0;
        }
        coords.push((x, y, z));
        x += 1;
        z += 1;
    }
    let start = Instant::now();
    for x in 0..coords.len() {
        let c = coords.get(x).unwrap();
        map.insert(c.0, c.1, c.2, c.2);
    }
    let mut duration = start.elapsed();
    println!("\ndeeply nested insert finished...{:?} elapsed", duration);
    if serialize {
        let start2 = Instant::now();
        let mut data: Vec<u8> = vec![];
        data = bincode::serialize(&map).unwrap();
        let mut file = File::create("test.dat").unwrap();
        file.write_all(&data).unwrap();
        duration = start2.elapsed();
        println!("deeply nested write finished...{:?} elapsed", duration);
        let start2 = Instant::now();
        let mut file2 = File::open("test.dat").unwrap();
        let mut data2: Vec<u8> = vec![];
        file2.read_to_end(&mut data2).unwrap();
        let mut map2: DeeplyNestedBTree<u8> = DeeplyNestedBTree::new();
        map2 = bincode::deserialize(&data2).unwrap();
        duration = start2.elapsed();
        println!("deeply nested read finished...{:?} elapsed", duration);
        let mut same: bool = true;
        assert!(coords.len() == iterations as usize);
        let start2 = Instant::now();
        for c in coords {
            if *map2.get(c.0, c.1, c.2).unwrap() != c.2 {
                same = false;
                panic!();
            }
        }
        duration = start2.elapsed();
        println!("deeply nested compare finished...{:?} elapsed", duration);
        println!("Data is the same = {}", same);
    } else {
        let mut same: bool = true;
        assert!(coords.len() == iterations as usize);
        let start2 = Instant::now();
        for c in coords {
            if *map.get(c.0, c.1, c.2).unwrap() != c.2 {
                same = false;
                panic!();
            }
        }
        duration = start2.elapsed();
        println!("deeply nested compare finished...{:?} elapsed", duration);
        println!("Data is the same = {}", same);
    }
    let elapsed = start.elapsed();
    println!("total elapsed = {:?}, iterations = {}, dimenions = {} X {} X {}, serialize/deserialize = {}",elapsed,iterations,width,width,256,serialize);
    elapsed
}

#[inline(always)]
fn btree_test(width: u16, iterations: u32, zxy: bool, serialize: bool) -> Duration {
    let mut mapz: BTreeMap<(u8, u16, u16), u8> = BTreeMap::new();
    let mut mapx: BTreeMap<(u16, u16, u8), u8> = BTreeMap::new();
    let mut coords: Vec<(u16, u16, u8)> = vec![];
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut z: u8 = 0;
    //let mut rng = rand::thread_rng();
    for i in 0..iterations {
        // x = rng.gen_range(0..width) as u16;
        // y = rng.gen_range(0..width) as u16;
        // z = rng.gen_range(0..255);
        if x >= width {
            x = 0;
            y += 1;
        }
        if z == 255 {
            z = 0;
        }
        coords.push((x, y, z));
        if zxy {
            mapz.insert((z, x, y), z);
        } else {
            mapx.insert((x, y, z), z);
        }
        x += 1;
        z += 1;
    }
    let start = Instant::now();
    for x in 0..coords.len() {
        let c = coords.get(x).unwrap();
        if zxy {
            mapz.insert((c.2, c.0, c.1), c.2);
        } else {
            mapx.insert((c.0, c.1, c.2), c.2);
        }
    }
    let mut duration = start.elapsed();
    if zxy {
        println!("\nz,x,y insert finished...{:?} elapsed", duration);
    } else {
        println!("\nx,y,z insert finished...{:?} elapsed", duration);
    }
    if serialize {
        let start2 = Instant::now();
        let mut data: Vec<u8> = vec![];
        if zxy {
            data = bincode::serialize(&mapz).unwrap();
        } else {
            data = bincode::serialize(&mapx).unwrap();
        }
        let mut file = File::create("test.dat").unwrap();
        file.write_all(&data).unwrap();
        duration = start2.elapsed();
        if zxy {
            println!("z,x,y write finished...{:?} elapsed", duration);
        } else {
            println!("x,y,z write finished...{:?} elapsed", duration);
        }
        let start2 = Instant::now();
        let mut file2 = File::open("test.dat").unwrap();
        let mut data2: Vec<u8> = vec![];
        file2.read_to_end(&mut data2).unwrap();
        let mut map2z: BTreeMap<(u8, u16, u16), u8> = BTreeMap::new();
        let mut map2x: BTreeMap<(u16, u16, u8), u8> = BTreeMap::new();
        if zxy {
            map2z = bincode::deserialize(&data2).unwrap();
        } else {
            map2x = bincode::deserialize(&data2).unwrap();
        }
        duration = start2.elapsed();
        if zxy {
            println!("z,x,y read finished...{:?} elapsed", duration);
        } else {
            println!("x,y,z read finished...{:?} elapsed", duration);
        }
        let mut same: bool = true;
        assert!(coords.len() == iterations as usize);
        let start2 = Instant::now();
        if zxy {
            for c in coords {
                if map2z.get(&(c.2, c.0, c.1)).unwrap() != &c.2 {
                    same = false;
                    panic!();
                }
            }
        } else {
            for c in coords {
                if map2x.get(&c).unwrap() != &c.2 {
                    same = false;
                    panic!();
                }
            }
        }
        duration = start2.elapsed();
        if zxy {
            println!("z,x,y compare finished...{:?} elapsed", duration);
        } else {
            println!("x,y,z compare finished...{:?} elapsed", duration);
        }
        println!("Data is the same = {}", same);
    } else {
        let mut same: bool = true;
        assert!(coords.len() == iterations as usize);
        let start2 = Instant::now();
        if zxy {
            for c in coords {
                if mapz.get(&(c.2, c.0, c.1)).unwrap() != &c.2 {
                    same = false;
                    panic!();
                }
            }
        } else {
            for c in coords {
                if mapx.get(&c).unwrap() != &c.2 {
                    same = false;
                    panic!();
                }
            }
        }
        duration = start2.elapsed();
        if zxy {
            println!("z,x,y compare finished...{:?} elapsed", duration);
        } else {
            println!("x,y,z compare finished...{:?} elapsed", duration);
        }
        println!("Data is the same = {}", same);
    }
    let elapsed = start.elapsed();
    if zxy {
        println!("total elapsed = {:?}, iterations = {}, dimenions = {} X {} X {}, serialize/deserialize = {}",elapsed,iterations,256,width,width,serialize);
    } else {
        println!("total elapsed = {:?}, iterations = {}, dimenions = {} X {} X {}, serialize/deserialize = {}",elapsed,iterations,width,width,256,serialize);
    }
    elapsed
}

enum TestType {
    FlatBtreeXYZ,
    FlatBtreeZXY,
    DeeplyNestedBTree,
    NestedBTree,
}
struct Test {
    pub num_tests: u32,
    pub iterations: u32,
    pub width: u16,
    pub serialize: bool,
    pub test_type: TestType,
}
impl Test {
    fn new(
        num_tests: u32,
        iterations: u32,
        width: u16,
        serialize: bool,
        test_type: TestType,
    ) -> Test {
        Self {
            num_tests: num_tests,
            iterations: iterations,
            width: width,
            serialize: serialize,
            test_type: test_type,
        }
    }
}

fn nested_btree_random_test(iterations: u32) {
    let mut coords: Vec<(u16,u16,u8,u32)> = Vec::new();
    let mut map: NestedBTree<u32> = NestedBTree::new();
    let mut rng = rand::thread_rng();
    for i in 0..iterations {
        let x = rng.gen_range(0..64000);
        let y = rng.gen_range(0..64000);
        let z = rng.gen_range(0..255);
        let val = rng.gen_range(0..u32::MAX);
        coords.push((x,y,z,val));
        map.insert(x, y, z, val);
    }
    println!("insert done");
    let mut same: bool = true;
    for i in 0..coords.len() {
        let c = coords.get(i).unwrap();
        if !map.get(c.0, c.1, c.2).unwrap() == c.3 {
            println!("data mismatch");
            same = false;
            break;
        }
    }
    println!("data matches = {}", same);
    println!("\nPress any key to continue...");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();

}
fn main() {
    //nested_btree_random_test(200_000_000);
    let mut tests: Vec<Test> = Vec::new();
    let mut results: Vec<String> = Vec::new();
    tests.push(Test::new(1, 100_000_000, 64000, true, TestType::FlatBtreeXYZ));
    tests.push(Test::new(1, 100_000_000, 64000, true, TestType::FlatBtreeZXY));
    tests.push(Test::new(
        1,
        100_000_000,
        64000,
        true,
        TestType::DeeplyNestedBTree,
    ));
    tests.push(Test::new(1, 100_000_000, 64000, true, TestType::NestedBTree));

    for test in tests {
        let mut min: Duration = Duration::MAX;
        let mut max: Duration = Duration::ZERO;
        let mut total = Duration::ZERO;
        for _i in 0..test.num_tests {
            let mut val = Duration::MAX;
            match test.test_type {
                TestType::DeeplyNestedBTree => {
                    val = deeply_nested_btree_test(test.width, test.iterations, test.serialize);
                }
                TestType::NestedBTree => {
                    val = nested_btree_test(test.width, test.iterations, test.serialize);
                }
                TestType::FlatBtreeXYZ => {
                    val = btree_test(test.width, test.iterations, false, test.serialize);
                }
                TestType::FlatBtreeZXY => {
                    val = btree_test(test.width, test.iterations, true, test.serialize);
                }
            };
            if val < min {
                min = val;
            }
            if val > max {
                max = val;
            }
            total += val;
        }
        let avg = total / test.num_tests;
        let mut name = "";
        match test.test_type {
            TestType::DeeplyNestedBTree => {
                name = "Results for deeply nested btree";
            }
            TestType::NestedBTree => {
                name = "Results for nested btree";
            }
            TestType::FlatBtreeXYZ => {
                name = "Results for flat b-tree as XYZ";
            }
            TestType::FlatBtreeZXY => {
                name = "Results for flat b-tree as ZXY";
            }
        };
        results.push(format!(
            "{}: # of tests: {}, iterations = {}, width = {}, serialize={}\r",
            name, test.num_tests,test.iterations, test.width, test.serialize
        ));
        if test.serialize {
            let mib: f64 = (File::metadata(&File::open("test.dat").unwrap())
                .unwrap()
                .len()) as f64
                / (1024.0 * 1024.0);
            results.push(format!(
                "minimum time = {:?}, maximum time = {:?}, mean time = {:?}, file size = {:.2} MiB",
                min, max, avg, mib
            ));
            fs::remove_file("test.dat").unwrap();
        } else {
            results.push(format!(
                "minimum time = {:?}, maximum time = {:?}, mean time = {:?}",
                min, max, avg
            ));
        }
    }
    println!();
    for r in results {
        println!("{r}");
    }

    println!("\nPress any key to continue...");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
