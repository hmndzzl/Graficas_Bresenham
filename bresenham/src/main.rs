mod framebuffer;
mod bmp;
mod line;

use crate::bmp::WriteBmp;
use crate::framebuffer::Framebuffer;
use crate::line::draw_line;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    draw_line(100, 100, 700, 500, &mut framebuffer);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}