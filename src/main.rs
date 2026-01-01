#![feature(hasher_prefixfree_extras)]

use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::io::{self, ErrorKind};
use std::process::Command;

const MAGIC_NUMBER: &str = "P3"; // マジックナンバー
const GRID_SIZE: usize = 5; // 5 * 5のグリッド
const SCALE: usize = 50; // 1マス50ピクセル
const MAX_BRIGHT: usize = 255; // 最大輝度

#[derive(Debug)]
struct FileStructure {
    color: (u8, u8, u8),
    is_paint: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl FileStructure {
    fn new(color: (u8, u8, u8)) -> Self {
        FileStructure {
            color,
            is_paint: [[false; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn color(&self) -> (u8, u8, u8) {
        self.color
    }

    fn is_paint(&self) -> [[bool; GRID_SIZE]; GRID_SIZE] {
        self.is_paint
    }

    fn set_is_paint(&mut self, finish: Vec<char>) {
        let finish: Vec<char> = finish.into_iter().take(15).collect();
        let mut is_paint_half = [[false; GRID_SIZE / 2 + 1]; GRID_SIZE];

        for (count, &c) in finish.iter().enumerate() {
            let h = count / 3;
            let w = count % 3;
            let ok = c.to_digit(16).expect("should convert hex") % 2 == 0;

            println!("h: {h}, w: {w}, c: {c}, ok: {ok}");
            is_paint_half[h][w] = ok;
        }

        let mut is_paint = [[false; GRID_SIZE]; GRID_SIZE];
        for (h, p) in is_paint_half.iter().enumerate() {
            for (w, &ok) in p.iter().enumerate() {
                println!("h: {h}, w: {w}, GRID_SIZE - w - 1: {}", GRID_SIZE - w - 1);
                is_paint[h][w] = ok;
                is_paint[h][GRID_SIZE - w - 1] = ok;
            }
        }

        self.is_paint = is_paint;
    }

    fn create_ppm(&self, file: &mut File) -> io::Result<()> {
        let physical_width = GRID_SIZE * SCALE;
        let physical_height = GRID_SIZE * SCALE;
        writeln!(file, "{}", MAGIC_NUMBER)?;
        writeln!(file, "{} {}", physical_width, physical_height)?;
        writeln!(file, "{}", MAX_BRIGHT)?;

        let color = self.color();
        let is_paint = self.is_paint();
        let mut image = [[(255, 255, 255); GRID_SIZE * SCALE]; GRID_SIZE * SCALE];

        // h: pixel_dataのインデックス
        // w: pixel_data[i]のインデックス
        // pixel_data[h][w]で(h, w)グリッド
        for (h, paint) in is_paint.iter().enumerate() {
            for (w, &ok) in paint.iter().enumerate() {
                if ok {
                    for hi in image.iter_mut().skip(h * SCALE).take(SCALE) {
                        for hij in hi.iter_mut().skip(w * SCALE).take(SCALE) {
                            *hij = color;
                        }
                    }
                }
            }
        }

        for i in image.iter() {
            for p in i {
                let (r, g, b) = (p.0, p.1, p.2);
                write!(file, "{} {} {}  ", r, g, b)?;
            }
            writeln!(file)?;
        }

        Ok(())
    }

    // #[allow(unused)]
    // fn make_black(&mut self) {
    //     let black_pixel = [[(0, 0, 0); GRID_SIZE]; GRID_SIZE];
    //     self.set_pixel_data(black_pixel);
    // }

    // #[allow(unused)]
    // fn make_white(&mut self) {
    //     let white_pixel = [[(255, 255, 255); GRID_SIZE]; GRID_SIZE];
    //     self.set_pixel_data(white_pixel);
    // }
}

fn main() -> io::Result<()> {
    let mut user_name = String::new();
    print!("User名を入力してください！: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut user_name)?;

    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    user_name.hash(&mut hasher);

    let hash_val = hasher.finish();

    let r = (hash_val & 0xFF) as u8; // 最初の8ビット
    let g = ((hash_val >> 8) & 0xFF) as u8; // 次の8ビット
    let b = ((hash_val >> 16) & 0xFF) as u8; // その次の8ビット
    let color = (r, g, b);

    println!("color: {:?}", color);

    let finish: Vec<char> = format!("{:x}", hasher.finish()).chars().collect();
    println!("finish len: {}", finish.len());
    println!("Hash is {:x}!", hasher.finish());

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

        let mut fs = FileStructure::new(color);

        fs.set_is_paint(finish);
        let is_paint = fs.is_paint();
        println!("is_paint: {:?}", is_paint);

        fs.create_ppm(&mut file)?;

        // convert ppm to png
        #[allow(unused)]
        let mut magick = Command::new("magick");

        Ok(())
    } else {
        eprintln!("Usage: cargo run -- -o <output_file>");
        Err(Error::new(ErrorKind::InvalidInput, "invalid to argumetns"))
    }
}
