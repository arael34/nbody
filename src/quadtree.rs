#[allow(dead_code, unused_variables)]
// representing a point in 2d space
pub type Point = (f64, f64);

pub trait Position {
    fn position(&self) -> Point;
}

#[derive(Debug)]
pub struct Bound {
    pos: Point,
    half_x: f64,
    half_y: f64,
}

impl Bound {
    pub fn new(pos: Point, half_x: f64, half_y: f64) -> Self {
        Bound { pos, half_x, half_y }
    }
    fn contains(&self, point: &Point) -> bool {
        point.0 >= self.pos.0 - self.half_x &&
        point.0 < self.pos.0 + self.half_x &&
        point.1 >= self.pos.1 - self.half_y &&
        point.1 < self.pos.1 + self.half_y
    }
    fn intersects(&self, bound: &Bound) -> bool {
        let s_top_left = (self.pos.0 - self.half_x, self.pos.1 + self.half_y);
        let s_bottom_right = (self.pos.0 + self.half_x, self.pos.1 - self.half_x);
        let b_top_left = (bound.pos.0 - bound.half_x, bound.pos.1 + bound.half_y);
        let b_bottom_right = (bound.pos.0 + bound.half_x, bound.pos.1 - bound.half_x);
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
    bounds: Bound, // bounds of QuadTree
    // depth: usize,
    // max_depth: usize,
    items: Vec<T>, // contained items
    subtrees: Option<[Box<QuadTree<T>>; 4]>,
}

#[allow(dead_code, unused_variables)]
impl<T> QuadTree<T> 
where T: Position {
    const MAX_CAPACITY: usize = 4;

    pub fn new(bounds: Bound) -> Self {
        let items = Vec::<T>::new();
        QuadTree::<T>{bounds, items, subtrees: None}
    }
    pub fn insert(&mut self, item: T) -> Option<T> {
        let pos = item.position();
        if !self.bounds.contains(&pos) { return Some(item); }
        else if self.items.len() < Self::MAX_CAPACITY {
            self.items.push(item);
            return None;
        } else if self.is_leaf() {
            self.subdivide();
        }
        let mut item_option = self.subtrees.as_mut().unwrap()[0].insert(item);
        if !item_option.is_none() {
            item_option = self.subtrees.as_mut().unwrap()[1].insert(item_option.unwrap());
        } else if !item_option.is_none() {
            item_option = self.subtrees.as_mut().unwrap()[2].insert(item_option.unwrap());
        } else if !item_option.is_none() {
            item_option = self.subtrees.as_mut().unwrap()[3].insert(item_option.unwrap());
        } // maybe take out last else if and change it to else
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
            for s in self.subtrees.as_ref().unwrap() {
                res.extend(s.query(bounds));
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
        let half_x = self.bounds.half_x / 2.;
        let half_y = self.bounds.half_y / 2.;
        self.subtrees = Some([
            Box::new(QuadTree::<T>::new(Bound::new((pos.0 + half_x, pos.1 + half_y), half_x, half_y))),
            Box::new(QuadTree::<T>::new(Bound::new((pos.0 + half_x, pos.1 - half_y), half_x, half_y))),
            Box::new(QuadTree::<T>::new(Bound::new((pos.0 - half_x, pos.1 - half_y), half_x, half_y))),
            Box::new(QuadTree::<T>::new(Bound::new((pos.0 - half_x, pos.1 + half_y), half_x, half_y))),
        ]);
    }
    fn is_leaf(&self) -> bool {
        Option::is_none(&self.subtrees)
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
    fn test_insert() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((400., 400.), 400., 400.));
        let p = Point2D::new(400., 400.);
        qt.insert(p);
        let p = Point2D::new(400., 400.);
        assert_eq!(qt.items[0], p);
    }
    #[test]
    fn test_query() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((400., 400.), 400., 400.));
        let p = Point2D::new(400., 400.);
        qt.insert(p);
        let p = Point2D::new(400., 400.);
        assert_eq!(qt.query(&Bound::new((400., 400.), 200., 200.)), vec![&p]);
    }
    #[test]
    fn test_query_all() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((400., 400.), 400., 400.));
        let p = Point2D::new(400., 400.);
        qt.insert(p);
        let p = Point2D::new(400., 400.);
        assert_eq!(qt.query_all(), vec![&p]);
    }
    #[test]
    fn test_insert_all() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((400., 400.), 400., 400.));
        let p = Point2D::new(400., 400.);
        let q = Point2D::new(300., 500.);
        let g = Point2D::new(200., 200.);
        qt.insert_all(vec![p, q, g]);
        let p = Point2D::new(400., 400.);
        let q = Point2D::new(300., 500.);
        let g = Point2D::new(200., 200.);
        assert_eq!(qt.query_all(), vec![&p, &q, &g]);
    }
    #[test]
    fn test_subdivide() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((400., 400.), 400., 400.));
        let p = Point2D::new(400., 400.);
        let q = Point2D::new(300., 500.);
        let g = Point2D::new(200., 200.);
        let b = Point2D::new(100., 100.);
        let a = Point2D::new(100., 200.);
        qt.insert_all(vec![p, q, g, b, a]);
        assert!(!qt.is_leaf());
    }
    #[test]
    fn test_default() {
        let qt = QuadTree::<Point2D>::new(Bound::new((400., 400.), 400., 400.));
        assert!(qt.is_leaf())
    }
    #[test]
    fn test_clear() {
        let mut qt = QuadTree::<Point2D>::new(Bound::new((400., 400.), 400., 400.));
        let p = Point2D::new(400., 400.);
        let q = Point2D::new(300., 500.);
        let g = Point2D::new(200., 200.);
        let b = Point2D::new(100., 100.);
        let a = Point2D::new(100., 200.);
        qt.insert_all(vec![p, q, g, b, a]);
        qt.clear();
        assert_eq!(Option::is_none(&qt.subtrees), true);
    }
}