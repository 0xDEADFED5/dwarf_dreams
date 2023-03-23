use serde::{Serialize, Deserialize};
use std::collections::{BTreeMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct NestedBTree<T> {
    x65535y65535: BTreeMap<(u8, u8), BTreeMap<(u8, u8, u8), T>>,
    x65535y255: BTreeMap<(u8, u8), BTreeMap<(u8, u8, u8), T>>,
    x255y65535: BTreeMap<(u8, u8), BTreeMap<(u8, u8, u8), T>>,
    x255y255: BTreeMap<(u8, u8), BTreeMap<(u8, u8, u8), T>>,
}

impl<T> NestedBTree<T> {
    pub fn new() -> NestedBTree<T> {
        Self {
            x255y255: BTreeMap::new(),
            x255y65535: BTreeMap::new(),
            x65535y255: BTreeMap::new(),
            x65535y65535: BTreeMap::new(),
        }
    }
    #[inline(always)]
    pub fn get (&self, x:u16, y:u16, z: u8) -> Result<&T, &'static str> {
        if x > 255 && y > 255 {
            let child = match self.x65535y65535.get(&((x/256) as u8, (y/256) as u8)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            let child = match child.get(&((x%256) as u8, (y%256) as u8, z)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            return Ok(&child);
        }
        else if x > 255 {
            let child = match self.x65535y255.get(&((x/256) as u8, y as u8)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            let child = match child.get(&((x%256) as u8, y as u8, z)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            return Ok(&child);
        }
        else if y > 255 {
            let child = match self.x255y65535.get(&(x as u8, (y/256) as u8)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            let child = match child.get(&(x as u8, (y%256) as u8, z)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            return Ok(&child);
        }
        else {
            let child = match self.x255y255.get(&(x as u8, y as u8)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            let child = match child.get(&(x as u8, y as u8, z)) {
                None => return Err("Not found"),
                Some(s) => s,
            };
            return Ok(&child);
        }
    }
    #[inline(always)]
    pub fn insert(&mut self, x:u16, y:u16, z: u8, value: T) {
        if x > 255 && y > 255 {
            if !self.x65535y65535.contains_key(&((x/256) as u8, (y/256) as u8)){
                self.x65535y65535.insert(((x/256) as u8, (y/256) as u8), BTreeMap::new());
            }
            let child = self.x65535y65535.get_mut(&((x/256) as u8, (y/256) as u8)).unwrap();
            child.insert(((x%256) as u8, (y%256) as u8, z), value);
        }
        else if x > 255 {
            if !self.x65535y255.contains_key(&((x/256) as u8, y as u8)){
                self.x65535y255.insert(((x/256) as u8, y as u8), BTreeMap::new());
            }
            let child = self.x65535y255.get_mut(&((x/256) as u8, y as u8)).unwrap();
            child.insert(((x%256) as u8, y as u8, z), value);
        }
        else if y > 255 {
            if !self.x255y65535.contains_key(&(x as u8, (y/256) as u8)){
                self.x255y65535.insert((x as u8, (y/256) as u8), BTreeMap::new());
            }
            let child = self.x255y65535.get_mut(&(x as u8, (y/256) as u8)).unwrap();
            child.insert((x as u8, (y%256) as u8, z), value);
        }
        else {
            if !self.x255y255.contains_key(&(x as u8, y as u8)){
                self.x255y255.insert((x as u8, y as u8), BTreeMap::new());
            }
            let child = self.x255y255.get_mut(&(x as u8, y as u8)).unwrap();
            child.insert((x as u8, y as u8, z), value);
        }
    }
}

impl<T> DeeplyNestedBTree<T> {
    pub fn new() -> DeeplyNestedBTree<T> {
        Self {
            buf: BTreeMap::new(),
        }
    }
    #[inline(always)]
    pub fn get(&self, x: u16, y:u16, z: u8) -> Result<&T, &'static str> {
        let x2a = (x / 256) as u8;
        let x2b = (x % 256) as u8;
        let y2a = (y / 256) as u8;
        let y2b = (y % 256) as u8;
        let child = match self.buf.get(&x2a) {
            None => return Err("Not found"),
            Some(s) => s,
        };
        let child = match child.buf.get(&x2b) {
            None => return Err("Not found"),
            Some(s) => s,
        };
        let child = match child.buf.get(&y2a) {
            None => return Err("Not found"),
            Some(s) => s,
        };
        let child = match child.buf.get(&y2b) {
            None => return Err("Not found"),
            Some(s) => s,
        };
        let child = match child.buf.get(&z) {
            None => return Err("Not found"),
            Some(s) => s,
        };
        Ok(child)
    }
    #[inline(always)]
    pub fn insert(&mut self, x:u16, y:u16, z:u8, value: T) {
        let x2a = (x / 256) as u8;
        let x2b = (x % 256) as u8;
        let y2a = (y / 256) as u8;
        let y2b = (y % 256) as u8;
        if !self.buf.contains_key(&x2a) {
            self.buf.insert(x2a, Branch4::new());
        }
        let child = self.buf.get_mut(&x2a).unwrap();
        if !child.buf.contains_key(&x2b) {
            child.buf.insert(x2b, Branch3::new());
        }
        let child = child.buf.get_mut(&x2b).unwrap();
        if !child.buf.contains_key(&y2a) {
            child.buf.insert(y2a, Branch2::new());
        }
        let child = child.buf.get_mut(&y2a).unwrap();
        if !child.buf.contains_key(&y2b) {
            child.buf.insert(y2b, Branch::new());
        }
        let child = child.buf.get_mut(&y2b).unwrap();
        child.buf.insert(z, value);

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeeplyNestedBTree<T> {
    buf: BTreeMap<u8, Branch4<T>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Branch<T> {
    pub buf: BTreeMap<u8, T>
}
#[derive(Serialize, Deserialize, Debug)]
struct Branch2<T> {
    pub buf: BTreeMap<u8, Branch<T>>
}
#[derive(Serialize, Deserialize, Debug)]
struct Branch3<T> {
    pub buf: BTreeMap<u8, Branch2<T>>
}
#[derive(Serialize, Deserialize, Debug)]
struct Branch4<T> {
    pub buf: BTreeMap<u8, Branch3<T>>
}

impl<T> Branch<T> {
    fn new() -> Branch<T> {
        Self {
            buf: BTreeMap::new(),
        }
    }
}

impl<T> Branch2<T> {
    fn new() -> Branch2<T> {
        Self {
            buf: BTreeMap::new(),
        }
    }
}

impl<T> Branch3<T> {
    fn new() -> Branch3<T> {
        Self {
            buf: BTreeMap::new(),
        }
    }
}

impl<T> Branch4<T> {
    fn new() -> Branch4<T> {
        Self {
            buf: BTreeMap::new(),
        }
    }
}
