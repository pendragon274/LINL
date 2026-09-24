use core::fmt::{Debug, Formatter};
use core::ops::{Index, IndexMut};
use crate::collections::kbox::Box;

#[allow(dead_code)]
pub trait BasicCollection<T: Sized + PartialEq + Clone>: Index<usize> + IndexMut<usize>
where Self::Output: Sized + PartialEq{
    fn insert_at(&mut self, index: usize, val: T);
    fn remove_at(&mut self, index: usize) -> Option<T>;
    fn len(&self) -> usize;

    fn append(&mut self, val: T){
        self.insert_at(self.len(), val);
    }

    fn find(&self, val: Self::Output) -> Option<usize>{
        for i in 0..self.len(){
            if self[i] == val {
                return Some(i);
            }
        }

        None
    }

    fn extend(&mut self, vals: &[T]){
        for item in vals{
            self.append(item.clone());
        }
    }
}

#[allow(dead_code)]
pub struct LinkedArray<T: Sized + PartialEq, const N: usize> {
    len: usize,
    link_count: usize,
    node: LinkedArrayNode<T, N>
}

#[allow(dead_code)]
pub struct LinkedArrayNode<T: Sized + PartialEq, const N: usize> {
    count: usize,
    collection: [Option<T>; N],
    next: Option<Box<LinkedArrayNode<T, N>>>,
}

#[allow(dead_code)]
pub struct LinkedArrayIter<'a, T: PartialEq, const N: usize> {
    collection: &'a [Option<T>],
    next: Option<&'a Box<LinkedArrayNode<T, N>>>,
}

impl<T: PartialEq + Debug + Copy, const N: usize> Debug for LinkedArray<T, N>{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "LinkedArray[")?;

        for (idx, item) in self.node.collection.iter().enumerate(){
            if idx + 1 < N{
                if item.is_some() {
                    write!(f, "{:?}, ", item)?;
                }else{
                    write!(f, "_, ")?;
                }
            }else{
                if item.is_some() {
                    write!(f, "{:?}", item)?;
                }else{
                    write!(f, "_")?;
                }
            }
        }

        write!(f, "]")
    }
}

impl<T: PartialEq, const N: usize> Index<usize> for LinkedArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.node.collection[index].as_ref().unwrap()
    }
}

impl<T: PartialEq, const N: usize> IndexMut<usize> for LinkedArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.node.collection[index].as_mut().unwrap()
    }
}

impl<T: PartialEq + Copy, const N: usize> BasicCollection<T> for LinkedArray<T, N>{
    fn insert_at(&mut self, index: usize, val: T) {
        let node_idx = index / N;
        let node_pos = index % N;

        if node_idx == 0{
            self.node.shrink_to(node_pos);
            self.node.push_up_insert(node_pos, val);
            /*match &self.node.collection[node_pos]{
                Some(_) => {
                    self.node.push_up_insert(node_pos, val);
                }, None => {
                    self.node.collection[node_pos] = Some(val);
                }
            }*/
        }else{
            todo!("Not implemented yet!");
        }

        self.len += 1;
    }

    fn remove_at(&mut self, index: usize) -> Option<T> {
        let node_idx = index / N;
        let node_pos = index % N;

        if index >= self.len{
            return None;
        }

        self.len -= 1;

        if node_idx == 0{
            self.node.shrink_from(node_pos)
        }else {
            todo!("Not implemented yet!");
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[allow(dead_code)]
impl<T: PartialEq + Copy, const N: usize> LinkedArray<T, N>{
    // ***** Public Functions *****
    pub fn iter(&self) -> LinkedArrayIter<'_, T, N> {
        LinkedArrayIter{
            collection: self.node.collection.as_slice(),
            next: self.node.next.as_ref(),
        }
    }

    // ***** Private Functions *****

    // ***** Struct Init *****
    pub fn new() -> LinkedArray<T, N>{
        LinkedArray{
            len: 0,
            link_count: 1,
            node: LinkedArrayNode {
                count: 0,
                collection: [None; N],
                next: None,
            },
        }
    }
}

impl<'a, T: PartialEq, const N: usize> Iterator for LinkedArrayIter<'a, T, N>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = 0;
        let mut found = false;
        for (idx, item) in self.collection.iter().enumerate(){
            if item.is_some(){
                next = idx;
                found = true;
                break;
            }
        }

        if found{
            self.collection = &self.collection[next..];
            self.next()
        }else{
            if self.next.is_some(){
                todo!("Can't implement this until Box is worked on.");
                //let next = self.next.unwrap().as_ref();
            }else{
                None
            }
        }
    }
}

#[allow(dead_code)]
impl<T: PartialEq + Clone, const N: usize> LinkedArrayNode<T, N>{
    pub fn shrink_to(&mut self, idx: usize){
        let mut open_idx = 0;
        let mut found_open = false;
        for i in 0..=idx{
            if !found_open{
                if self.collection[i].is_none(){
                    open_idx = i;
                    found_open = true;
                }
            }else{
                if self.collection[i].is_some() && found_open{
                    let item = self.collection[i].take();
                    self.collection[open_idx] = item;
                    open_idx = i;
                }
            }
        }
    }

    pub fn shrink_from(&mut self, idx: usize) -> Option<T>{
        let ret = self.collection[idx].take();
        let mut some_idx = idx;
        let mut some_found = false;
        for i in idx..self.collection.len(){
            if self.collection[i].is_none(){
                let next: Option<usize>;
                match some_found{
                    true => {
                        next = self.find_next_after(some_idx);
                    }, false =>{
                        next = self.find_next_after(idx);
                    }
                }

                match next{
                    Some(next_idx) => {
                        some_found = true;
                        some_idx = next_idx;
                        self.collection[i] = self.collection[next_idx].take();
                    }, None => {
                        break;
                    }
                }
            }
        }

        ret
    }

    pub fn push_up_insert(&mut self, idx: usize, val: T){
        if self.collection[idx].is_none(){
            self.collection[idx] = Some(val);
        }else{
            let mut prev_item: T = val;
            for i in idx..self.collection.len(){
                match self.collection[i].is_some(){
                    true => {
                        let new_item = self.collection[i].take().unwrap();
                        self.collection[i] = Some(prev_item);
                        prev_item = new_item;
                    }, false => {
                        self.collection[i] = Some(prev_item);
                        return;
                    }
                }
            }

            todo!("Not implemented yet!");
        }
    }

    pub fn find_next_after(&self, idx: usize) -> Option<usize>{
        for i in idx..self.collection.len(){
            if self.collection[i].is_some(){
                return Some(i);
            }
        }

        None
    }
}