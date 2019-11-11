use libc::c_char;
use nannou::prelude::*;
use std::ffi::CStr;
use std::ffi::CString;

fn main() {
    nannou::sketch(view);
}

#[link(name = "hello", kind = "static")]
extern "C" {
    fn hello(name: *const c_char) -> *const c_char;
}

fn view(app: &App, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    let t = app.time;
    let win = app.window_rect();
    let timer_as_index = ((t as u32) % 3) as usize;

    let entity = CString::new("Hello World").expect("Default");
    let decorated_entity: &CStr = unsafe { CStr::from_ptr(hello(entity.as_ptr())) };
    println!("{:?}", decorated_entity);

    let background_colors = [
        LIGHTBLUE,
        SADDLEBROWN,
        SLATEGREY,
        LIGHTGRAY,
        LIGHTGREEN,
        LIGHTGREY,
        DARKORANGE,
        PLUM,
        REBECCAPURPLE,
        LIGHTYELLOW,
    ];

    let foreground_colors = [
        BLACK,
        BLUE,
        SANDYBROWN,
        SILVER,
        DARKBLUE,
        BROWN,
        DARKGRAY,
        DARKGREEN,
        DARKGREY,
        DARKORANGE,
        MEDIUMPURPLE,
        DARKRED,
        YELLOWGREEN,
        GRAY,
        GREEN,
        GREY,
        ORANGE,
        PURPLE,
        RED,
        WHITE,
        YELLOW,
    ];

    let background_color = background_colors[timer_as_index];

    let foreground_color = foreground_colors[timer_as_index];

    // Clear the background to blue.
    draw.background().color(background_color);

    // Draw a purple triangle in the top left half of the window.

    // draw.tri()
    //     .points(win.bottom_left(), win.top_left(), win.top_right())
    //     .color(DARK_PURPLE);

    // Draw an ellipse to follow the mouse.
    // draw.ellipse()
    //     .x_y(app.mouse.x * t.cos(), app.mouse.y)
    //     .radius(win.w() * 0.125 * t.sin())
    //     .color(RED);

    // Draw a line!
    draw.line()
        .weight(10.0 + (t.sin() * 0.5 + 0.5) * 90.0)
        .caps_round()
        .color(foreground_color);

    // Draw a quad that follows the inverse of the ellipse.
    // draw.quad()
    //     .x_y(-app.mouse.x, app.mouse.y)
    //     .color(DARK_GREEN)
    //     .rotate(t);

    // // Draw a rect that follows a different inverse of the ellipse.
    // draw.rect()
    //     .x_y(app.mouse.y, app.mouse.x)
    //     .w(app.mouse.x * 0.25)
    //     .hsv(t, 1.0, 1.0);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
