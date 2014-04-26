// Don't forget to add -L . (or whatever dir has .rlib) to rustc!
extern crate imagequant;

fn main() {
    // Image loading/saving is outside scope of this library
    static width:uint = 10;
    static height:uint = 10;
    let fakebitmap = ~[255, ..4*width*height];

    // http://pngquant.org/lib/

    // Configure the library
    let mut liq = imagequant::new();
    liq.set_speed(5);
    liq.set_quality(70, 99);

    // Describe the bitmap
    let mut img = liq.new_image(fakebitmap, width, height, 0.0).unwrap();

    // The magic happens in quantize()
    let mut res = match liq.quantize(img) {
        Some(res) => res,
        _ => fail!("Quantization failed — quality too low"),
    };

    // Enable dithering for subsequent remappings
    res.set_dithering_level(1.0);

    // You can reuse the result to generate several images with the same palette
    let (palette, pixels) = res.remapped(img).unwrap();

    println!("Done! Got palette {} and {} pixels with {}% quality", palette, pixels.len(), res.quantization_quality());
}