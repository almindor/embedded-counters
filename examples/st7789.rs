use embedded_counters::{DelayCounter, PinCounter, display_interface::DIC};
use embedded_graphics::{pixelcolor::*, prelude::*, primitives::*};
use st7789::*;

fn main() {
    let di = DIC::default();
    let rst = PinCounter::default();
    let mut delay = DelayCounter::default();

    let mut display = ST7789::new(di, rst, 240, 240);

    display.init(&mut delay).unwrap();
    display.set_orientation(Orientation::Landscape).unwrap();

    let circle1 =
        Circle::new(Point::new(128, 64), 64).into_styled(PrimitiveStyle::with_fill(Rgb565::RED));
    let circle2 = Circle::new(Point::new(64, 64), 64)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1));

    let blue_with_red_outline = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .stroke_color(Rgb565::RED)
        .stroke_width(1) // > 1 is not currently supported in embedded-graphics on triangles
        .build();
    let triangle = Triangle::new(
        Point::new(40, 120),
        Point::new(40, 220),
        Point::new(140, 120),
    )
    .into_styled(blue_with_red_outline);

    let line = Line::new(Point::new(180, 160), Point::new(239, 239))
        .into_styled(PrimitiveStyle::with_stroke(RgbColor::WHITE, 10));

    // draw two circles on black background
    display.clear(Rgb565::BLACK).unwrap();
    circle1.draw(&mut display).unwrap();
    circle2.draw(&mut display).unwrap();
    triangle.draw(&mut display).unwrap();
    line.draw(&mut display).unwrap();

    let (di, rst) = display.release();

    let di_json = serde_json::to_string_pretty(&di).unwrap();
    let rst_json = serde_json::to_string_pretty(&rst).unwrap();

    println!("{},\n{}", di_json, rst_json);
}
