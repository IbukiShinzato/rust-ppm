use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::io::{self, ErrorKind};

const MAGIC_NUMBER: &str = "P3"; // マジックナンバー
const GRID_SIZE: usize = 5; // 5 * 5のグリッド
const SCALE: usize = 50; // 1マス50ピクセル
const MAX_BRIGHT: usize = 255; // 最大輝度

#[derive(Debug)]
struct FileStructure {
    pixel_data: [[(u8, u8, u8); GRID_SIZE]; GRID_SIZE],
}

impl FileStructure {
    fn new(pixel_data: [[(u8, u8, u8); GRID_SIZE]; GRID_SIZE]) -> Self {
        FileStructure { pixel_data }
    }

    fn set_pixel_data(&mut self, pixel_data: [[(u8, u8, u8); GRID_SIZE]; GRID_SIZE]) {
        self.pixel_data = pixel_data;
    }

    fn pixel_data(&self) -> &[[(u8, u8, u8); GRID_SIZE]; GRID_SIZE] {
        &self.pixel_data
    }

    fn change_grid(&mut self, w: usize, h: usize, color: (u8, u8, u8)) {
        self.pixel_data[h][w] = color;
    }

    fn create_ppm(&self, file: &mut File) -> io::Result<()> {
        let physical_width = GRID_SIZE * SCALE;
        let physical_height = GRID_SIZE * SCALE;
        writeln!(file, "{}", MAGIC_NUMBER)?;
        writeln!(file, "{} {}", physical_width, physical_height)?;
        writeln!(file, "{}", MAX_BRIGHT)?;

        let pixel_data = self.pixel_data();
        let mut image = [[(0, 0, 0); GRID_SIZE * SCALE]; GRID_SIZE * SCALE];

        // h: pixel_dataのインデックス
        // w: pixel_data[i]のインデックス
        // pixel_data[h][w]で(h, w)グリッド
        for (h, pixel) in pixel_data.iter().enumerate() {
            for (w, &p) in pixel.iter().enumerate() {
                for hi in image.iter_mut().skip(h * SCALE).take(SCALE) {
                    for hij in hi.iter_mut().skip(w * SCALE).take(SCALE) {
                        *hij = p;
                    }
                }
            }
        }

        for i in image {
            for p in i {
                let (r, g, b) = (p.0, p.1, p.2);
                write!(file, "{} {} {}  ", r, g, b)?;
            }
            writeln!(file)?;
        }

        Ok(())
    }

    #[allow(unused)]
    fn make_black(&mut self) {
        let black_pixel = [[(0, 0, 0); GRID_SIZE]; GRID_SIZE];
        self.set_pixel_data(black_pixel);
    }

    #[allow(unused)]
    fn make_white(&mut self) {
        let white_pixel = [[(255, 255, 255); GRID_SIZE]; GRID_SIZE];
        self.set_pixel_data(white_pixel);
    }
}

fn main() -> io::Result<()> {
    let mut w = vec![];
    writeln!(&mut w)?;
    writeln!(&mut w, "test")?;
    writeln!(&mut w, "formatted {} arguments", 2)?;

    println!("w: {:?}", w);

    for (i, &wi) in w.iter().enumerate() {
        let c = if wi == 10 {
            "Enter".to_string()
        } else if wi == 32 {
            "Space".to_string()
        } else {
            char::from_u32(wi as u32).unwrap().to_string()
        };
        println!("i: {i}, c: {c}");
    }

    let args: Vec<String> = env::args().collect();

    if let Some(file_index) = args.iter().position(|arg| arg == "-o") {
        let mut file = if let Some(path) = args.get(file_index + 1) {
            let path = if !path.contains(".ppm") {
                &(path.to_owned() + ".ppm")
            } else {
                path
            };

            File::create(path)?
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid to argumetns"));
        };

        let pixel_data = [[(0, 0, 0); GRID_SIZE]; GRID_SIZE];

        let mut fs = FileStructure::new(pixel_data);
        fs.make_white();

        let red = (255, 0, 0);
        let blue = (0, 0, 255);
        let green = (0, 255, 0);
        fs.change_grid(2, 2, green);
        fs.change_grid(0, 0, red);
        fs.change_grid(4, 4, blue);

        fs.create_ppm(&mut file)?;

        Ok(())
    } else {
        eprintln!("Usage: cargo run -- -o <output_file>");
        Err(Error::new(ErrorKind::InvalidInput, "invalid to argumetns"))
    }
}
