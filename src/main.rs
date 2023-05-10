mod cvmodule;

use image::open;
use std::path::Path;
use std::time::Instant;
use cvmodule::convert::*;

fn main() {
    let rgb = open(Path::new("img/input/neko.jpg")).unwrap().into_rgb8();
    
    let start_time = Instant::now();

    let img = get_channel_image(rgb, Channel::G);

    let elapsed_time = start_time.elapsed();
    img.save(Path::new("img/output/neko_g.jpg")).unwrap();

    print!("{}マイクロ秒経過しました。", elapsed_time.as_micros());
}
