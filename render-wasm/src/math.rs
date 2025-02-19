use skia_safe::{Matrix, Point, Vector};

pub trait VectorExt {
    fn new_points(a: &Point, b: &Point) -> Vector;
}

impl VectorExt for Vector {
    // Creates a vector from two points
    fn new_points(from: &Point, to: &Point) -> Vector {
        Vector::new(to.x - from.x, to.y - from.y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub nw: Point,
    pub ne: Point,
    pub se: Point,
    pub sw: Point,
}

fn vec_min_max(arr: &[Option<f32>]) -> Option<(f32, f32)> {
    let mut minv: Option<f32> = None;
    let mut maxv: Option<f32> = None;

    for it in arr {
        if let Some(it) = *it {
            match minv {
                None => minv = Some(it),
                Some(n) => minv = Some(f32::min(it, n)),
            }
            match maxv {
                None => maxv = Some(it),
                Some(n) => maxv = Some(f32::max(it, n)),
            }
        }
    }

    Some((minv?, maxv?))
}

impl Bounds {
    pub fn new(nw: Point, ne: Point, se: Point, sw: Point) -> Self {
        Self { nw, ne, se, sw }
    }

    pub fn horizontal_vec(&self) -> Vector {
        Vector::new_points(&self.nw, &self.ne)
    }

    pub fn vertical_vec(&self) -> Vector {
        Vector::new_points(&self.nw, &self.sw)
    }

    pub fn hv(&self, scalar: f32) -> Vector {
        let mut hv = self.horizontal_vec();
        hv.normalize();
        hv.scale(scalar);
        hv
    }

    pub fn vv(&self, scalar: f32) -> Vector {
        let mut vv = self.vertical_vec();
        vv.normalize();
        vv.scale(scalar);
        vv
    }

    pub fn width(&self) -> f32 {
        Point::distance(self.nw, self.ne)
    }

    pub fn height(&self) -> f32 {
        Point::distance(self.nw, self.sw)
    }

    pub fn transform(&self, mtx: &Matrix) -> Self {
        Self {
            nw: mtx.map_point(self.nw),
            ne: mtx.map_point(self.ne),
            se: mtx.map_point(self.se),
            sw: mtx.map_point(self.sw),
        }
    }

    pub fn transform_mut(&mut self, mtx: &Matrix) {
        self.nw = mtx.map_point(self.nw);
        self.ne = mtx.map_point(self.ne);
        self.se = mtx.map_point(self.se);
        self.sw = mtx.map_point(self.sw);
    }

    pub fn box_bounds(&self, other: &Self) -> Option<Self> {
        let hv = self.horizontal_vec();
        let vv = self.vertical_vec();

        let hr = Ray::new(self.nw, hv);
        let vr = Ray::new(self.nw, vv);

        let (min_ht, max_ht) = vec_min_max(&[
            intersect_rays_t(&hr, &Ray::new(other.nw, vv)),
            intersect_rays_t(&hr, &Ray::new(other.ne, vv)),
            intersect_rays_t(&hr, &Ray::new(other.sw, vv)),
            intersect_rays_t(&hr, &Ray::new(other.se, vv)),
        ])?;

        let (min_vt, max_vt) = vec_min_max(&[
            intersect_rays_t(&vr, &Ray::new(other.nw, hv)),
            intersect_rays_t(&vr, &Ray::new(other.ne, hv)),
            intersect_rays_t(&vr, &Ray::new(other.sw, hv)),
            intersect_rays_t(&vr, &Ray::new(other.se, hv)),
        ])?;

        let nw = intersect_rays(&Ray::new(hr.t(min_ht), vv), &Ray::new(vr.t(min_vt), hv))?;
        let ne = intersect_rays(&Ray::new(hr.t(max_ht), vv), &Ray::new(vr.t(min_vt), hv))?;
        let sw = intersect_rays(&Ray::new(hr.t(min_ht), vv), &Ray::new(vr.t(max_vt), hv))?;
        let se = intersect_rays(&Ray::new(hr.t(max_ht), vv), &Ray::new(vr.t(max_vt), hv))?;

        Some(Self { nw, ne, se, sw })
    }

    pub fn left(&self, p: Point) -> f32 {
        let hr = Ray::new(p, self.horizontal_vec());
        let vr = Ray::new(self.nw, self.vertical_vec());
        if let Some(project_point) = intersect_rays(&hr, &vr) {
            if vr.is_positive_side(&p) {
                -Point::distance(project_point, p)
            } else {
                Point::distance(project_point, p)
            }
        } else {
            // This should not happen. All points should have a proyection so the
            // intersection ray should always exist
            0.0
        }
    }

    pub fn right(&self, p: Point) -> f32 {
        let hr = Ray::new(p, self.horizontal_vec());
        let vr = Ray::new(self.ne, self.vertical_vec());
        if let Some(project_point) = intersect_rays(&hr, &vr) {
            if vr.is_positive_side(&p) {
                Point::distance(project_point, p)
            } else {
                -Point::distance(project_point, p)
            }
        } else {
            // This should not happen. All points should have a proyection so the
            // intersection ray should always exist
            0.0
        }
    }

    pub fn top(&self, p: Point) -> f32 {
        let vr = Ray::new(p, self.vertical_vec());
        let hr = Ray::new(self.nw, self.horizontal_vec());
        if let Some(project_point) = intersect_rays(&vr, &hr) {
            if hr.is_positive_side(&p) {
                Point::distance(project_point, p)
            } else {
                -Point::distance(project_point, p)
            }
        } else {
            // This should not happen. All points should have a proyection so the
            // intersection ray should always exist
            0.0
        }
    }

    pub fn bottom(&self, p: Point) -> f32 {
        let vr = Ray::new(p, self.vertical_vec());
        let hr = Ray::new(self.sw, self.horizontal_vec());
        if let Some(project_point) = intersect_rays(&vr, &hr) {
            if hr.is_positive_side(&p) {
                -Point::distance(project_point, p)
            } else {
                Point::distance(project_point, p)
            }
        } else {
            // This should not happen. All points should have a proyection so the
            // intersection ray should always exist
            0.0
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn t(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    pub fn is_positive_side(&self, p: &Point) -> bool {
        let a = self.direction.y;
        let b = -self.direction.x;
        let c = self.direction.x * self.origin.y - self.direction.y * self.origin.x;
        let v = p.x * a + p.y * b + c;
        v < 0.0
    }
}

pub fn intersect_rays_t(ray1: &Ray, ray2: &Ray) -> Option<f32> {
    let p1 = ray1.origin;
    let d1 = ray1.direction;
    let p2 = ray2.origin;
    let d2 = ray2.direction;

    // Calculate the determinant to check if the rays are parallel
    let determinant = d1.cross(d2);
    if determinant.abs() < f32::EPSILON {
        // Parallel rays, no intersection
        return None;
    }

    // Solve for t1 and t2 parameters
    let diff = p2 - p1;

    Some(diff.cross(d2) / determinant)
}

pub fn intersect_rays(ray1: &Ray, ray2: &Ray) -> Option<Point> {
    if let Some(t) = intersect_rays_t(ray1, ray2) {
        Some(ray1.t(t))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_parameter() {
        let r = Ray::new(Point::new(0.0, 0.0), Vector::new(0.5, 0.5));
        assert_eq!(r.t(1.0), Point::new(0.5, 0.5));
        assert_eq!(r.t(2.0), Point::new(1.0, 1.0));
        assert_eq!(r.t(-2.0), Point::new(-1.0, -1.0));
    }

    #[test]
    fn test_intersect() {
        // Test Cases for Ray-Ray Intersections
        // Simple Intersection at (2, 2)
        let r1 = Ray::new(Point::new(0.0, 0.0), Vector::new(1.0, 1.0));
        let r2 = Ray::new(Point::new(0.0, 4.0), Vector::new(1.0, -1.0));
        assert_eq!(intersect_rays(&r1, &r2), Some(Point::new(2.0, 2.0)));

        // Parallel Rays (No Intersection)
        let r1 = Ray::new(Point::new(0.0, 0.0), Vector::new(1.0, 1.0));
        let r2 = Ray::new(Point::new(0.0, 2.0), Vector::new(1.0, 1.0));
        assert_eq!(intersect_rays(&r1, &r2), None);

        // Coincident Rays (Infinite Intersections)
        let r1 = Ray::new(Point::new(0.0, 0.0), Vector::new(1.0, 1.0));
        let r2 = Ray::new(Point::new(1.0, 1.0), Vector::new(1.0, 1.0));
        assert_eq!(intersect_rays(&r1, &r2), None);

        let r1 = Ray::new(Point::new(1.0, 0.0), Vector::new(2.0, 1.0));
        let r2 = Ray::new(Point::new(4.0, 4.0), Vector::new(-1.0, -1.0));
        assert_eq!(intersect_rays(&r1, &r2), Some(Point::new(-1.0, -1.0)));

        let r1 = Ray::new(Point::new(1.0, 1.0), Vector::new(3.0, 2.0));
        let r2 = Ray::new(Point::new(4.0, 0.0), Vector::new(-2.0, 3.0));
        assert_eq!(
            intersect_rays(&r1, &r2),
            Some(Point::new(2.6153846, 2.0769231))
        );
    }

    #[test]
    fn test_vec_min_max() {
        assert_eq!(None, vec_min_max(&[]));
        assert_eq!(None, vec_min_max(&[None, None]));
        assert_eq!(Some((1.0, 1.0)), vec_min_max(&[None, Some(1.0)]));
        assert_eq!(
            Some((0.0, 1.0)),
            vec_min_max(&[Some(0.3), None, Some(0.0), Some(0.7), Some(1.0), Some(0.1)])
        );
    }

    #[test]
    fn test_box_bounds() {
        let b1 = Bounds::new(
            Point::new(1.0, 5.0),
            Point::new(5.0, 5.0),
            Point::new(5.0, 1.0),
            Point::new(1.0, 1.0),
        );
        let b2 = Bounds::new(
            Point::new(3.0, 4.0),
            Point::new(4.0, 3.0),
            Point::new(3.0, 2.0),
            Point::new(2.0, 3.0),
        );
        let result = b1.box_bounds(&b2);
        assert_eq!(
            Some(Bounds::new(
                Point::new(2.0, 4.0),
                Point::new(4.0, 4.0),
                Point::new(4.0, 2.0),
                Point::new(2.0, 2.0),
            )),
            result
        )
    }

    #[test]
    fn test_bounds_distances() {
        let b1 = Bounds::new(
            Point::new(1.0, 10.0),
            Point::new(8.0, 10.0),
            Point::new(8.0, 1.0),
            Point::new(1.0, 1.0),
        );
        assert_eq!(b1.left(Point::new(4.0, 8.0)), -3.0);
        assert_eq!(b1.top(Point::new(4.0, 8.0)), -2.0);
        assert_eq!(b1.right(Point::new(7.0, 6.0),), -1.0);
        assert_eq!(b1.bottom(Point::new(7.0, 6.0),), -5.0);
    }
}
