use euclid::Point2D;

pub struct Pixel;

pub type SPoint = Point2D<f32, Pixel>;

pub struct Quadrilater {
    pub p1: SPoint,
    pub p2: SPoint,
    pub p3: SPoint,
    pub p4: SPoint,
}
