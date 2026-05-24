use crate::n_body::vec2::Vec2;

#[derive(Copy, Clone)]
pub struct Node {
    pub pos: Vec2,
    pub size: f64,
}

impl Node {
    pub fn new(pos: Vec2, size: f64) -> Self {
        Node { pos, size }
    }

    pub fn contains(&self, p_pos: &Vec2) -> bool {
        let half_size = self.size / 2.0;
        p_pos.x() >= self.pos.x() - half_size
            && p_pos.x() < self.pos.x() + half_size
            && p_pos.y() >= self.pos.y() - half_size
            && p_pos.y() < self.pos.y() + half_size
    }
}

pub struct QuadTree {
    pub boundary: Node,
    pub capacity: usize,
    pub bodies: Vec<usize>, // Store indices now
    pub divided: bool,
    pub nw: Option<Box<QuadTree>>,
    pub ne: Option<Box<QuadTree>>,
    pub sw: Option<Box<QuadTree>>,
    pub se: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Node, capacity: usize) -> Self {
        QuadTree {
            boundary,
            capacity,
            bodies: Vec::new(),
            divided: false,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }

    pub fn subdivide(&mut self) {
        let half_size = self.boundary.size / 2.0;
        let quarter_size = self.boundary.size / 4.0;
        let x = self.boundary.pos.x();
        let y = self.boundary.pos.y();

        // Northwest
        let nw_pos = Vec2::new(x - quarter_size, y - quarter_size);
        self.nw = Some(Box::new(QuadTree::new(
            Node::new(nw_pos, half_size),
            self.capacity,
        )));

        // Northeast
        let ne_pos = Vec2::new(x + quarter_size, y - quarter_size);
        self.ne = Some(Box::new(QuadTree::new(
            Node::new(ne_pos, half_size),
            self.capacity,
        )));

        // Southwest
        let sw_pos = Vec2::new(x - quarter_size, y + quarter_size);
        self.sw = Some(Box::new(QuadTree::new(
            Node::new(sw_pos, half_size),
            self.capacity,
        )));

        // Southeast
        let se_pos = Vec2::new(x + quarter_size, y + quarter_size);
        self.se = Some(Box::new(QuadTree::new(
            Node::new(se_pos, half_size),
            self.capacity,
        )));

        self.divided = true;
    }

    pub fn insert(&mut self, index: usize, all_bodies: &[Vec2]) -> bool {
        if !self.boundary.contains(&all_bodies[index]) {
            return false;
        }

        if self.bodies.len() < self.capacity {
            self.bodies.push(index);
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        self.nw.as_mut().unwrap().insert(index, all_bodies)
            || self.ne.as_mut().unwrap().insert(index, all_bodies)
            || self.sw.as_mut().unwrap().insert(index, all_bodies)
            || self.se.as_mut().unwrap().insert(index, all_bodies)
    }

    pub fn query(&self, range: &Node, all_bodies: &[Vec2], found: &mut Vec<usize>) {
        if !self.intersects(range) {
            return;
        }

        for &index in &self.bodies {
            if range.contains(&all_bodies[index]) {
                found.push(index);
            }
        }

        if self.divided {
            self.nw.as_ref().unwrap().query(range, all_bodies, found);
            self.ne.as_ref().unwrap().query(range, all_bodies, found);
            self.sw.as_ref().unwrap().query(range, all_bodies, found);
            self.se.as_ref().unwrap().query(range, all_bodies, found);
        }
    }

    pub fn intersects(&self, range: &Node) -> bool {
        let half_size = self.boundary.size / 2.0;
        let range_half = range.size / 2.0;

        !(range.pos.x() - range_half > self.boundary.pos.x() + half_size
            || range.pos.x() + range_half < self.boundary.pos.x() - half_size
            || range.pos.y() - range_half > self.boundary.pos.y() + half_size
            || range.pos.y() + range_half < self.boundary.pos.y() - half_size)
    }

    /// Draw every cell boundary as a rectangle outline (for debug visualisation).
    pub fn draw(&self) {
        use macroquad::prelude::*;
        let half = self.boundary.size / 2.0;
        let x = (self.boundary.pos.x() - half) as f32;
        let y = (self.boundary.pos.y() - half) as f32;
        let s = self.boundary.size as f32;

        draw_rectangle_lines(x, y, s, s, 1.5, Color::from_rgba(0, 200, 100, 180));

        if self.divided {
            if let Some(ref nw) = self.nw { nw.draw(); }
            if let Some(ref ne) = self.ne { ne.draw(); }
            if let Some(ref sw) = self.sw { sw.draw(); }
            if let Some(ref se) = self.se { se.draw(); }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_contains() {
        let node = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let body_inside = Vec2::new(25.0, 25.0);
        let body_outside = Vec2::new(60.0, 60.0);

        assert!(node.contains(&body_inside));
        assert!(!node.contains(&body_outside));
    }

    #[test]
    fn test_quadtree_insert() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 4);

        let bodies = vec![Vec2::new(10.0, 10.0), Vec2::new(20.0, 20.0)];

        assert!(qt.insert(0, &bodies));
        assert!(qt.insert(1, &bodies));
        assert_eq!(qt.bodies.len(), 2);
    }

    #[test]
    fn test_quadtree_subdivide() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 2);

        let bodies = vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(20.0, 20.0),
            Vec2::new(-10.0, -10.0),
        ];

        // Insert 3 bodies to trigger subdivision
        qt.insert(0, &bodies);
        qt.insert(1, &bodies);
        qt.insert(2, &bodies);

        assert!(qt.divided);
    }

    #[test]
    fn test_quadtree_query() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 4);

        let bodies = vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(20.0, 20.0),
            Vec2::new(-30.0, -30.0),
        ];

        qt.insert(0, &bodies);
        qt.insert(1, &bodies);
        qt.insert(2, &bodies);

        let query_range = Node::new(Vec2::new(0.0, 0.0), 40.0);
        let mut found = Vec::new();
        qt.query(&query_range, &bodies, &mut found);

        assert_eq!(found.len(), 2);
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 4);

        let bodies = vec![Vec2::new(100.0, 100.0)];
        assert!(!qt.insert(0, &bodies));
    }
}