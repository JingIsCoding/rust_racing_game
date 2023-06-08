use ndarray::arr2;
use super::*;

pub struct BoundingBox {
    pub rect: Rect,
    pub lines: Option<Vec<Line>>,
    pub points: Option<Vec<FVec>>,
    pub rotate: f64,
    pub origin: FVec,
}

impl BoundingBox {
    pub fn new(rect: &Rect) -> Self {
        return Self::new_with_origin(rect, FVec { x: rect.w / 2.0, y: rect.h / 2.0 });
    }

    pub fn new_with_origin(rect: &Rect, origin: FVec) -> Self {
        let rect = rect.clone();
        let mut bounding_box = BoundingBox { 
            rect: rect.clone(), 
            rotate: 0.0, 
            origin,
            lines: None,
            points: None,
        };
        bounding_box.update_coordinates();
        return bounding_box;
    }

    pub fn turn_at(&mut self, delta_dregess: f64) {
        self.rotate += delta_dregess;
        self.update_coordinates();
    }

    pub fn move_at(&mut self, velocity: f64) {
        let velocity_x = (self.rotate).cos() * velocity;
        let velocity_y = (self.rotate).sin() * velocity;
        self.rect.x += velocity_x;
        self.rect.y += velocity_y;
        self.update_coordinates();
    }

    fn update_coordinates(&mut self) {
        let mut coordinates = vec![];
        let mut lines = vec![];
        let origin_x = self.origin.x;
        let origin_y = self.origin.y;
        let x = self.rect.x - origin_x;
        let y = self.rect.y - origin_y;
        let w = self.rect.w;
        let h = self.rect.h;
        let cx = x + 2.0 * origin_x;
        let cy = y + 2.0 * origin_y;
        let p = arr2(&[[ x, x + w, x + w, x], [ y, y, y+ h, y + h ]]);
        let c = arr2(&[[ cx, cx, cx, cx ], [ cy, cy, cy, cy ]]);
        let r = arr2(&[[self.rotate.cos(), -self.rotate.sin()], [self.rotate.sin(), self.rotate.cos()]]);
        let pc = &r.dot(&(&p - &c)) + &c;
        for i in 0..4 {
            coordinates.push(FVec { x: pc[[0, i]], y: pc[[1, i]] });
        }
        for i in 0..4 {
            if i < 3 {
                lines.push(Line{ start: coordinates[i].clone(), end: coordinates[i + 1].clone() });
            } else {
                lines.push(Line{ start: coordinates[i].clone(), end: coordinates[0].clone() });
            }
        }
        (self.lines, self.points) = (Some(lines), Some(coordinates));
    }

    pub fn get_coordinates(&self) -> (&Option<Vec<FVec>>, &Option<Vec<Line>>) {
        return (&self.points, &self.lines);
    }

    pub fn get_center(&self) -> Option<FVec> {
        let (points, _) = self.get_coordinates();
        if let Some(points) = points {
            let center = points.iter().fold(FVec::default(), |mut acc, point|{
                acc.x += point.x;
                acc.y += point.y;
                return acc;
            }) / FVec { x: 4.0, y: 4.0 };
            return Some(center);
        }
        None
    }

    pub fn debug_view(&self, renderer: &Renderer) {
        let (coordinates, lines) = self.get_coordinates();
        renderer.save();
        renderer.restore();
    }
}
