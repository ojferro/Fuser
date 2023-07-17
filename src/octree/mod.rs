pub use self::octree::Octree;

mod octree{
    pub struct Octree {
        pub center: (f32, f32),
        pub shape: (f32, f32),
        pub children: Option<[Box<Octree>; 4]>,
        pub max_depth: usize,
    }
    
    impl Octree {
        pub fn new(center: (f32, f32), shape: (f32, f32), max_depth: usize) -> Self {
            Self {
                center,
                shape,
                children: None,
                max_depth,
            }
        }

        pub fn insert(&mut self, point: (f32, f32)){
            self.insert_depth_limited(point, 0);
        }
    
        fn insert_depth_limited(&mut self, point: (f32, f32), depth: usize) {
            if depth >= self.max_depth {
                return;
            }

            if self.children.is_none() {
                self.split();
            }
    
            let (x, y) = point;
            let (cx, cy) = self.center;
            let (w, h) = self.shape;
    
            let idx = if x < cx {
                if y < cy {
                    0
                } else {
                    2
                }
            } else {
                if y < cy {
                    1
                } else {
                    3
                }
            };
    
            if let Some(children) = &mut self.children{
                children[idx].insert_depth_limited(point, depth + 1);
            }
        }
    
        pub fn split(&mut self) {
            let (cx, cy) = self.center;
            let (w, h) = self.shape;
    
            let nw = Octree::new((cx - w / 4.0, cy - h / 4.0), (w / 2.0, h / 2.0), self.max_depth);
            let ne = Octree::new((cx + w / 4.0, cy - h / 4.0), (w / 2.0, h / 2.0), self.max_depth);
            let sw = Octree::new((cx - w / 4.0, cy + h / 4.0), (w / 2.0, h / 2.0), self.max_depth);
            let se = Octree::new((cx + w / 4.0, cy + h / 4.0), (w / 2.0, h / 2.0), self.max_depth);
    
            self.children = Some([Box::new(nw), Box::new(ne), Box::new(sw), Box::new(se)]);
        }
    }
}