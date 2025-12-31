use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::io::{self, ErrorKind};

#[derive(Debug)]
struct FileStructure {
    pub magic_number: &'static str,
    height: usize,
    width: usize,
    max_bright: usize,
    pixel_data: Vec<Vec<(u8, u8, u8)>>,
}

impl FileStructure {
    fn new(
        height: usize,
        width: usize,
        max_bright: usize,
        pixel_data: Vec<Vec<(u8, u8, u8)>>,
    ) -> Self {
        FileStructure {
            magic_number: "P3",
            height,
            width,
            max_bright,
            pixel_data,
        }
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn max_bright(&self) -> usize {
        self.max_bright
    }

    fn pixel_data(&self) -> &Vec<Vec<(u8, u8, u8)>> {
        &self.pixel_data
    }

    fn create_ppm(&self, file: &mut File) -> io::Result<()> {
        let (width, height) = (self.width(), self.height());
        writeln!(file, "{}", self.magic_number)?;
        writeln!(file, "{} {}", width, height)?;
        writeln!(file, "{}", self.max_bright())?;

        let pixel_data = self.pixel_data();
        for pixel in pixel_data {
            for &data in pixel.iter() {
                write!(file, "{} {} {}  ", data.0, data.1, data.2)?;
            }
            writeln!(file)?;
        }

        Ok(())
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

        let height = 2;
        let width = 2;
        let max_bright = 255;
        // サンプルデータ
        let pixel_data = vec![vec![(255, 0, 0), (0, 0, 0)], vec![(0, 0, 0), (0, 0, 255)]];
        let fs = FileStructure::new(height, width, max_bright, pixel_data);
        println!("FileStructure: {:#?}", fs);

        fs.create_ppm(&mut file)?;

        Ok(())
    } else {
        eprintln!("Usage: cargo run -- -o <output_file>");
        Err(Error::new(ErrorKind::InvalidInput, "invalid to argumetns"))
    }
}
