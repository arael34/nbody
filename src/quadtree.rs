#[allow(unused_imports)]
use std::mem;

#[allow(dead_code, unused_variables)]
// representing a point in 2d space
pub type Point = (f64, f64);

pub trait Position {
    fn position(&self) -> Point;
}

#[derive(Debug)]
pub struct Bound {
    pub pos: Point,
    pub x: f64,
    pub y: f64,
}

impl Bound {
    pub fn new(pos: Point, x: f64, y: f64) -> Self {
        Bound { pos, x, y }
    }
    fn contains(&self, point: &Point) -> bool {
        point.0 >= self.pos.0 &&
        point.0 < self.pos.0 + self.x &&
        point.1 >= self.pos.1 &&
        point.1 < self.pos.1 + self.y
    }
    fn intersects(&self, bound: &Bound) -> bool {
        let s_top_left = (self.pos.0, self.pos.1);
        let s_bottom_right = (self.pos.0 + self.x, self.pos.1 + self.y);
        let b_top_left = (bound.pos.0, bound.pos.1);
        let b_bottom_right = (bound.pos.0 + bound.x, bound.pos.1 + bound.y);
        if s_top_left.0 > b_bottom_right.0 || b_top_left.0 > s_bottom_right.0 {
            return false;
        }
        if s_top_left.1 < b_bottom_right.1 || b_top_left.1 < s_bottom_right.1 {
            return false;
        }
        true
    }
}

#[allow(dead_code, unused_variables)]
// main quadtree struct
#[derive(Debug)]
pub struct QuadTree<T> {
    pub bounds: Bound, // bounds of QuadTree
    // depth: usize,
    // max_depth: usize,
    items: Vec<T>, // contained items
    pub subtrees: Option<[Box<QuadTree<T>>; 4]>,
}

#[allow(dead_code, unused_variables)]
impl<T> QuadTree<T> 
where T: Position {
    const MAX_CAPACITY: usize = 1;

    pub fn new(bounds: Bound) -> Self {
        let items = Vec::<T>::new();
        QuadTree::<T>{bounds, items, subtrees: None}
    }
    pub fn insert(&mut self, item: T) -> Option<T> {
        let pos = item.position();
        if !self.bounds.contains(&pos) { return Some(item); }
        if self.items.len() < Self::MAX_CAPACITY && self.is_leaf() {
            self.items.push(item);
            return None;
        } else if self.is_leaf() {
            self.subdivide();
        }
        let mut item_option = self.subtrees.as_mut().unwrap()[0].insert(item);
        if !item_option.is_none() {
            //println!("\nnot in 1");
            item_option = self.subtrees.as_mut().unwrap()[1].insert(item_option.unwrap());
        } if !item_option.is_none() {
            //println!("not in 2");
            item_option = self.subtrees.as_mut().unwrap()[2].insert(item_option.unwrap());
        } if !item_option.is_none() {
            //println!("not in 3");
            item_option = self.subtrees.as_mut().unwrap()[3].insert(item_option.unwrap());
        } if !item_option.is_none() {
            //println!("not in 4. shouldn't be here");
        }
        item_option
    }
    pub fn insert_all(&mut self, items: Vec<T>) {
        for i in items {
            self.insert(i);
        }
    }
    pub fn query(&self, bounds: &Bound) -> Vec<&T> {
        let mut res: Vec<&T> = Vec::<&T>::new();
        if !self.bounds.intersects(bounds) { return res; }
        for p in &self.items {
            if bounds.contains(&(p.position())) {
                res.push(p);
            }
        }
        let mut res: Vec<&T> = self.items
                                .iter()
                                .filter(|i| bounds.contains(&(i.position())))
                                .collect();
        if !self.is_leaf() {
            for i in 0..=3 {
                res.extend(self.subtrees.as_ref().unwrap()[i].query(bounds));
            }
        }
        res
    }
    pub fn query_all(&self) -> Vec<&T> {
        let mut res: Vec<&T> = self.items.iter().collect();
        if !self.is_leaf() {
            for s in self.subtrees.as_ref().unwrap() {
                res.extend(s.query_all());
            } 
        }
        res
    }
    pub fn query_all_mut(&mut self) -> Vec<&mut T> {
        let mut res: Vec<&mut T> = self.items.iter_mut().collect();
        if !Option::is_none(&self.subtrees) {
            for s in self.subtrees.as_mut().unwrap() {
                res.extend(s.query_all_mut());
            }
        }
        res
    }
    pub fn clear(&mut self) -> () {
        self.items = vec![];
        if !self.is_leaf() {
            for s in self.subtrees.as_mut().unwrap() {
                s.clear()
            }
            self.subtrees = None;
        }
    }
    fn subdivide(&mut self) -> () {
        let pos = self.bounds.pos;
        let x = self.bounds.x / 2.;
        let y = self.bounds.y / 2.;
        let top_left = (pos.0, pos.1);
        let bottom_left = (pos.0, pos.1 + y);
        let bottom_right = (pos.0 + x, pos.1 + y);
        let top_right = (pos.0 + x, pos.1);
        self.subtrees = Some([
            Box::new(QuadTree::<T>::new(Bound::new(top_left, x, y))),
            Box::new(QuadTree::<T>::new(Bound::new(top_right, x, y))),
            Box::new(QuadTree::<T>::new(Bound::new(bottom_left, x, y))),
            Box::new(QuadTree::<T>::new(Bound::new(bottom_right, x, y))),
        ]);
        // let all_items = mem::replace(&mut self.items, Vec::new());
        // self.insert_all(all_items);
    }
    fn is_leaf(&self) -> bool {
        Option::is_none(&self.subtrees)
    }
    pub fn get_trees(&self) -> Vec<&QuadTree<T>> {
        let mut res: Vec<&QuadTree<T>> = vec![&self];
        if !self.is_leaf() {
            for s in self.subtrees.as_ref().unwrap() {
                    res.extend(s.get_trees());
            } 
        }
        res
    }
}

mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    #[allow(dead_code)]
    impl Point2D {
        fn new(x: f64, y: f64) -> Self {
            Point2D{ x, y }
        }
    }

    impl Position for Point2D {
        fn position(&self) -> (f64, f64) {
            (self.x, self.y)
        }
    }
    #[test]
    fn test_contains() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((0., 0.), 800., 800.));
        let p = Point2D::new(600., 600.);
        let q = Point2D::new(700., 600.);
        // let g = Point2D::new(600., 200.);
        //let b = Point2D::new(100., 100.);
        //let a = Point2D::new(100., 200.);
        qt.insert_all(vec![p, q]);
        let g = Point2D::new(600., 200.);
        assert!(qt.subtrees.as_ref().unwrap()[1].bounds.contains(&g.position()));
    }
    #[test]
    fn test_query_all() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((0., 0.), 800., 800.));
        let p = Point2D::new(600., 600.);
        let q = Point2D::new(700., 600.);
        // let g = Point2D::new(600., 200.);
        //let b = Point2D::new(100., 100.);
        //let a = Point2D::new(100., 200.);
        qt.insert_all(vec![p, q]);
        assert_eq!(qt.query_all().len(), 2);
    }
    #[test]
    fn test_subdivide() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((0., 0.), 800., 800.));
        let p = Point2D::new(600., 600.);
        let q = Point2D::new(700., 600.);
        //let g = Point2D::new(600., 200.);
        //let b = Point2D::new(100., 100.);
        //let a = Point2D::new(100., 200.);
        qt.insert_all(vec![p, q]);
        assert!(!qt.is_leaf());
    }
    #[test]
    fn test_insert() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((0., 0.), 800., 800.));
        let p = Point2D::new(600., 600.);
        let q = Point2D::new(700., 600.);
        //let g = Point2D::new(600., 200.);
        //let b = Point2D::new(100., 100.);
        //let a = Point2D::new(100., 200.);
        qt.insert_all(vec![p, q]);
        assert_eq!(qt.subtrees.as_ref().unwrap()[3].items.len(), 1);
    }
}