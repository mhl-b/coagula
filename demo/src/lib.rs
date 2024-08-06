#![allow(unused)]

use std::ptr::addr_of;

use engine::geometry;
use engine::geometry::{hex, Hex, Shape};
use engine::screen::{Layout, ORIENTATION_POINTY, Point, point, POINT0};

static mut CONTROLLER: Controller = Controller::new();

pub struct Controller {
    hexes: Vec<Hex>,
    centers: Vec<Point>,
    corners: Vec<Point>,
    view_scale: Point,
    view_origin: Point,
    layout: Layout,
}

impl Controller {
    const fn new() -> Controller {
        let origin = point(16., 16.);
        let size = point(16., 16.);
        Controller {
            hexes: vec![],
            centers: vec![],
            corners: vec![],
            view_scale: point(1., 1.),
            view_origin: point(0., 0.),
            layout: Layout::new(ORIENTATION_POINTY, size, origin),
        }
    }

    #[no_mangle]
    fn ptr() -> *const Controller {
        unsafe { addr_of!(CONTROLLER) }
    }

    #[no_mangle]
    fn get_hex(&self, off_x: f32, off_y: f32) -> (i32, i32) {
        let lp = self.view_to_layout(off_x, off_y);
        let h = self.layout.point_to_hex(lp);
        (h.q(), h.r())
    }

    fn view_to_layout(&self, off_x: f32, off_y: f32) -> Point {
        (point(off_x, off_y) + self.view_origin) / self.view_scale
    }

    #[no_mangle]
    fn init(&mut self) {
        let p = geometry::Point::new(hex(1, 1));
        let t = geometry::Tri::new(hex(3, 3));
        self.hexes = p.hex_iter().chain(t.hex_iter()).collect();
        let size = self.hexes.len();
        self.centers = vec![POINT0; size];
        self.corners = vec![POINT0; size * 6];
        self.layout.grid_centers(&self.hexes, &mut self.centers);
        self.layout.grid_corners(&self.centers, &mut self.corners);
    }

    #[no_mangle]
    fn adj_origin(&mut self, off_x: f32, off_y: f32) {
        self.view_origin = self.view_to_layout(off_x, off_y);
    }

    #[no_mangle]
    fn zoom(&mut self, dy: f32, off_x: f32, off_y: f32) {
        let s1 = self.view_scale.y();
        let s2 = s1 * dy_to_scale(dy);
        let ds = 1. / s1 - 1. / s2;
        self.view_scale = point(s2, s2);
        self.view_origin = point(off_x, off_y) * point(ds, ds) + self.view_origin;
    }

    #[no_mangle]
    fn adj_size(&mut self, scale: f32) {
        self.view_scale = self.view_scale * point(scale, scale);
    }

    #[no_mangle]
    fn get_origin(&self) -> (f32, f32) {
        (self.view_origin.x(), self.view_origin.y())
    }

    #[no_mangle]
    fn get_scale(&self) -> f32 {
        self.view_scale.y()
    }

    #[no_mangle]
    fn hex_buf(&self) -> (*const Hex, usize) {
        (self.hexes.as_ptr(), self.hexes.len())
    }

    #[no_mangle]
    fn centers_buf(&self) -> (*const Point, usize) {
        (self.centers.as_ptr(), self.centers.len())
    }

    #[no_mangle]
    fn corners_buf(&self) -> (*const Point, usize) {
        (self.corners.as_ptr(), self.corners.len())
    }
}

fn dy_to_scale(dy: f32) -> f32 {
    let d = dy * -0.01;
    if d < 0. {
        1. / d.abs()
    } else {
        d
    }
}
