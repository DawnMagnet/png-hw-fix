use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

struct IHDR {
    _raw_data: Vec<u8>,
    header: Vec<u8>,
    width_bytes: [u8; 4],
    height_bytes: [u8; 4],
    ends: Vec<u8>,
}

impl IHDR {
    fn new(data: &[u8]) -> Self {
        let header = data[0..4].to_vec();
        let mut width_bytes = [0u8; 4];
        width_bytes.copy_from_slice(&data[4..8]);
        let mut height_bytes = [0u8; 4];
        height_bytes.copy_from_slice(&data[8..12]);
        let ends = data[12..].to_vec();
        IHDR {
            _raw_data: data.to_vec(),
            header,
            width_bytes,
            height_bytes,
            ends,
        }
    }

    fn width(&self) -> u32 {
        u32::from_be_bytes(self.width_bytes)
    }

    fn height(&self) -> u32 {
        u32::from_be_bytes(self.height_bytes)
    }

    fn get_data(&self, width: Option<u32>, height: Option<u32>) -> Vec<u8> {
        let mut data = self.header.clone();
        if let Some(w) = width {
            data.extend(&w.to_be_bytes());
        } else {
            data.extend(&self.width_bytes);
        }
        if let Some(h) = height {
            data.extend(&h.to_be_bytes());
        } else {
            data.extend(&self.height_bytes);
        }
        data.extend(&self.ends);
        data
    }

    fn crc32(&self, width: Option<u32>, height: Option<u32>) -> u32 {
        let data = self.get_data(width, height);
        crc32fast::hash(&data)
    }

    fn from_png(png_data: &[u8]) -> (IHDR, u32) {
        let ihdr_data = &png_data[12..29];
        let expected_crc32 =
            u32::from_be_bytes([png_data[29], png_data[30], png_data[31], png_data[32]]);
        (IHDR::new(ihdr_data), expected_crc32)
    }
}

fn try_fix_height(ihdr: &IHDR, expected_crc32: u32) -> Option<u32> {
    (0..65535).find(|&height| ihdr.crc32(None, Some(height)) == expected_crc32)
}

fn try_fix_width(ihdr: &IHDR, expected_crc32: u32) -> Option<u32> {
    (0..65535).find(|&width| ihdr.crc32(Some(width), None) == expected_crc32)
}

fn try_fix_both(ihdr: &IHDR, expected_crc32: u32) -> Option<(u32, u32)> {
    for height in 0..65535 {
        for width in 0..65535 {
            if ihdr.crc32(Some(width), Some(height)) == expected_crc32 {
                return Some((height, width));
            }
        }
    }
    None
}

fn save_fixed_png(
    original_data: &[u8],
    width: Option<u32>,
    height: Option<u32>,
    output_path: &Path,
) -> io::Result<()> {
    let mut new_data = original_data.to_vec();

    if let Some(w) = width {
        new_data[16..20].copy_from_slice(&w.to_be_bytes());
    }

    if let Some(h) = height {
        new_data[20..24].copy_from_slice(&h.to_be_bytes());
    }

    let new_crc = crc32fast::hash(&new_data[12..29]);
    new_data[29..33].copy_from_slice(&new_crc.to_be_bytes());

    let mut file = File::create(output_path)?;
    file.write_all(&new_data)?;
    info!("Saved fixed PNG to: {}", output_path.display());
    Ok(())
}

fn try_fix_crc32(img_data: &[u8], _png_path: &Path, output_path: &Path) {
    let (ihdr, expected_crc32) = IHDR::from_png(img_data);
    let real_crc32 = ihdr.crc32(None, None);

    if expected_crc32 == real_crc32 {
        info!("CRC32 is correct!");
        return;
    }

    // info!("IHDR data: {:?}", ihdr.get_data(None, None));
    error!("Expected CRC32: 0x{:X}", expected_crc32);
    error!("Current CRC32: 0x{:X}", real_crc32);
    info!("Current dimensions: {}x{}", ihdr.width(), ihdr.height());

    // Try fixing height
    if let Some(height) = try_fix_height(&ihdr, expected_crc32) {
        info!("Found height fix: {} (0x{:X})", height, height);
        info!("Real dimensions: {}x{}", ihdr.width(), height);
        if let Err(e) = save_fixed_png(img_data, None, Some(height), output_path) {
            error!("Error: {}", e);
        }
        return;
    }

    // Try fixing width
    if let Some(width) = try_fix_width(&ihdr, expected_crc32) {
        info!("Found width fix: {} (0x{:X})", width, width);
        info!("Real dimensions: {}x{}", width, ihdr.height());
        if let Err(e) = save_fixed_png(img_data, Some(width), None, output_path) {
            error!("Error: {}", e);
        }
        return;
    }

    // Try fixing both
    if let Some((height, width)) = try_fix_both(&ihdr, expected_crc32) {
        info!(
            "Found height/width fix: {}x{} (0x{:X}x0x{:X})",
            height, width, height, width
        );
        if let Err(e) = save_fixed_png(img_data, Some(width), Some(height), output_path) {
            error!("Error: {}", e);
        }
        return;
    }

    error!("No fix found");
}

#[derive(Parser)]
#[command(
    name = "CRC32 Break",
    version = "0.1.0",
    author = "Your Name",
    about = "Fixes PNG dimensions by brute-forcing CRC32",
    arg_required_else_help = true
)]
struct Cli {
    /// Path to the PNG file
    png_file_path: String,

    /// Output file name
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    let cli = Cli::parse();
    let png_path = Path::new(&cli.png_file_path);

    if !png_path.exists() {
        error!("File {} does not exist", png_path.display());
        process::exit(1);
    }

    if !png_path.is_file() {
        error!("{} is not a file", png_path.display());
        process::exit(1);
    }

    let output_path = if let Some(ref out) = cli.output {
        Path::new(out).to_path_buf()
    } else {
        png_path.parent().unwrap().join(format!(
            "{}_fix.png",
            png_path.file_stem().unwrap().to_string_lossy()
        ))
    };

    match File::open(png_path) {
        Ok(mut file) => {
            let mut img_data = Vec::new();
            if let Err(e) = file.read_to_end(&mut img_data) {
                error!("Error: {}", e);
                process::exit(1);
            }
            try_fix_crc32(&img_data, png_path, &output_path);
        }
        Err(e) => {
            error!("Error: {}", e);
            process::exit(1);
        }
    }
}
