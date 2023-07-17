use minifb::{Key, Window, WindowOptions};


use octree::Octree;
mod octree;

const WINDOW_W: usize = 1500;
const WINDOW_H: usize = 1500;
const WHITE: u32 = 0xFFFFFF;
const BLUE: u32 = 0x8888FF;

fn interpolate_gray(val: f32) -> u32 {
    let min_gray = 0.1;
    let new_val = (val+min_gray)/(1.0+min_gray);
    let gray = (f32::max(min_gray,f32::min(new_val,1.0)) * 255.0) as u32;
    (gray << 16) | (gray << 8) | gray
}

fn uv2pxl(uv: (f32,f32)) -> (usize,usize) {
    let pxl_x = (uv.0 * WINDOW_W as f32) as usize;
    let pxl_y = (uv.1 * WINDOW_H as f32) as usize;
    (pxl_x, pxl_y)
}

fn pxl2idx(pxl: (usize,usize)) -> usize {
    pxl.1 * WINDOW_W + pxl.0
}

fn draw_rect(buffer: &mut Vec<u32>, center: (f32,f32), shape: (f32,f32), color: u32) {
    
    let top_left_px = uv2pxl((center.0-shape.0/2.0,center.1-shape.1/2.0));
    let shape_px = uv2pxl(shape);

    // Draw the top and bottom sides of the square
    for x in top_left_px.0 .. top_left_px.0 + shape_px.0 {
        buffer[pxl2idx((top_left_px.1, x))] = color;
        buffer[pxl2idx((top_left_px.1 + shape_px.1 - 1, x))] = color;
    }

    // Draw the left and right sides of the square
    for y in top_left_px.1 .. top_left_px.1 + shape_px.1 {
        buffer[pxl2idx((y, top_left_px.0))] = color;
        buffer[pxl2idx((y, top_left_px.0 + shape_px.0 - 1))] = color;
    }
}

fn draw_rect_fill(buffer: &mut Vec<u32>, center: (f32,f32), shape: (f32,f32)) {
    let top_left_px = uv2pxl((center.0-shape.0/2.0,center.1-shape.1/2.0));
    let shape_px = uv2pxl(shape);

    for x in top_left_px.0 .. top_left_px.0 + shape_px.0 {
        for y in top_left_px.1 .. top_left_px.1 + shape_px.1 {
            buffer[pxl2idx((y, x))] = BLUE;
        }
    }
}

fn draw_pt(buffer: &mut Vec<u32>, pt: (f32,f32)) {
    // let pt_px = uv2pxl(pt);
    // buffer[pxl2idx(pt_px)] = BLUE;
    draw_rect_fill(buffer, pt, (0.01,0.01));
}

fn draw_octree(buffer: &mut Vec<u32>, octree: &Octree, depth: Option<usize>) {
    let current_depth = depth.unwrap_or(0);
    if let Some(children) = &octree.children {
        for child in children.iter() {
            draw_octree(buffer, child, Some(current_depth + 1));
        }
    }

    draw_rect(buffer, octree.center, octree.shape, interpolate_gray(current_depth as f32 / octree.max_depth as f32));
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WINDOW_W * WINDOW_H];

    // let square_center = (0.5,0.5);
    // let square_shape = (0.1,0.2);
    // draw_rect(&mut buffer, square_center, square_shape);

    let mut octree = Octree::new((0.5, 0.5), (1.0, 1.0), 6);

    // Insert some points into the octree
    let pt0 = (0.2, 0.2);
    octree.insert(pt0);
    draw_pt(&mut buffer,pt0);

    let pt1 = (0.8, 0.4);
    octree.insert(pt1);
    draw_pt(&mut buffer,pt1);

    let pt2 = (0.3, 0.7);
    octree.insert(pt2);
    draw_pt(&mut buffer,pt2);

    let pt3 = (0.7, 0.9);
    octree.insert(pt3);
    draw_pt(&mut buffer,pt3);

    // Draw the octree
    draw_octree(&mut buffer, &octree, None);

    // Drawing boilerplate
    let mut window = Window::new(
        "Rust Window",
        WINDOW_W,
        WINDOW_H,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WINDOW_W, WINDOW_H).unwrap();
    }
}