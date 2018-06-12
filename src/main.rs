const WIDTH: i32 = 1200;
const HEIGHT: i32 = 800;

fn main() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::create("result.ppm")?;
    let mut output_str = "P3\n".to_owned()
        + &WIDTH.to_string() + &"\n".to_owned()
        + &HEIGHT.to_string() + &"\n".to_owned()
        + "255\n";
    for _i in 0..WIDTH*HEIGHT {
        output_str += "0 255 255\n";
    }
    f.write_all(output_str.as_bytes());
    Ok(())
}
