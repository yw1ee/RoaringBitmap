extern crate hwloc;

use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use hwloc::Bitmap;

#[derive(Clone)]
pub struct ArrayContainer {
    inner: Vec<u32>,
}

impl Deref for ArrayContainer {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ArrayContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<&BitmapContainer> for ArrayContainer {
    fn from(bitmap: &BitmapContainer) -> Self {
        let mut array = vec![];
        for i in bitmap.first()..bitmap.last() {
            if bitmap.is_set(i as u32) {
                array.push(i as u32);
            }
        }
        ArrayContainer { inner: array }
    }
}

#[derive(Clone)]
pub struct BitmapContainer {
    inner: Bitmap,
}

impl Deref for BitmapContainer {
    type Target = Bitmap;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for BitmapContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<&ArrayContainer> for BitmapContainer {
    fn from(array: &ArrayContainer) -> Self {
        let mut bitmap = Bitmap::new();
        for v in array.iter() {
            bitmap.set(*v);
        }
        BitmapContainer { inner: bitmap }
    }
}

impl BitmapContainer {
    fn or(lhs: &BitmapContainer, rhs: &BitmapContainer) -> Self {
        todo!()
    }

    fn and(lhs: &BitmapContainer, rhs: &BitmapContainer) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub enum Container {
    Array(ArrayContainer),
    Bitmap(BitmapContainer),
}

impl Container {
    pub fn new() -> Self {
        Self::Array(ArrayContainer { inner: vec![] })
    }

    pub fn insert(&mut self, value: u32) {
        match self {
            Container::Array(array) => {
                array.push(value);
                if array.len() > 4096 {
                    *self = Container::Bitmap(BitmapContainer::from(&*array));
                }
            }
            Container::Bitmap(bitmap) => bitmap.set(value),
        }
    }

    pub fn remove(&mut self, value: u32) {
        match self {
            Container::Array(array) => {
                array.retain(|&x| x != value);
            }
            Container::Bitmap(bitmap) => {
                bitmap.unset(value);
                if bitmap.weight() <= 4096 {
                    *self = Container::Array(ArrayContainer::from(&*bitmap));
                }
            }
        }
    }

    pub fn union(lhs: Option<&Container>, rhs: Option<&Container>) -> Self {
        match (lhs, rhs) {
            (Some(c1), Some(c2)) => match (c1, c2) {
                (Container::Array(a1), Container::Array(a2)) => {
                    if a1.len() + a2.len() <= 4096 {
                        let array = HashSet::<&u32>::from_iter(a1.iter().chain(a2.iter()))
                            .into_iter()
                            .cloned()
                            .collect();
                        Container::Array(ArrayContainer { inner: array })
                    } else {
                        let b1 = BitmapContainer::from(a1);
                        let b2 = BitmapContainer::from(a2);
                        let bitmap = BitmapContainer::or(&b1, &b2);
                        if bitmap.weight() <= 4096 {
                            Container::Array(ArrayContainer::from(&bitmap))
                        } else {
                            Container::Bitmap(bitmap)
                        }
                    }
                }
                (Container::Array(a), Container::Bitmap(b))
                | (Container::Bitmap(b), Container::Array(a)) => {
                    let mut bitmap = b.clone();
                    for v in a.iter() {
                        bitmap.set(*v);
                    }
                    Container::Bitmap(bitmap)
                }
                (Container::Bitmap(b1), Container::Bitmap(b2)) => {
                    let bitmap = BitmapContainer::or(b1, b2);
                    Container::Bitmap(bitmap)
                }
            },
            (Some(c), None) | (None, Some(c)) => c.clone(),
            (None, None) => unreachable!(),
        }
    }

    pub fn intersection(lhs: Option<&Container>, rhs: Option<&Container>) -> Self {
        match (lhs, rhs) {
            (Some(c1), Some(c2)) => match (c1, c2) {
                (Container::Array(a1), Container::Array(a2)) => {
                    let mut array = a1.clone();
                    array.retain(|v| a2.contains(v));
                    Container::Array(array)
                }
                (Container::Array(a), Container::Bitmap(b))
                | (Container::Bitmap(b), Container::Array(a)) => {
                    let mut array = a.clone();
                    array.retain(|v| b.is_set(*v));
                    Container::Array(array)
                }
                (Container::Bitmap(b1), Container::Bitmap(b2)) => {
                    let bitmap = BitmapContainer::and(b1, b2);
                    if bitmap.weight() <= 4096 {
                        Container::Array(ArrayContainer::from(&bitmap))
                    } else {
                        Container::Bitmap(bitmap)
                    }
                }
            },
            (Some(c), None) | (None, Some(c)) => c.clone(),
            (None, None) => unreachable!(),
        }
    }

    pub fn difference(lhs: Option<&Container>, rhs: Option<&Container>) -> Self {
        todo!()
    }

    pub fn symmetric_difference(lhs: Option<&Container>, rhs: Option<&Container>) -> Self {
        todo!()
    }
}
