extern crate tesseract;
extern crate image;
extern crate clap;
extern crate log;
extern crate simple_logger;

use tesseract::{Tesseract, OcrEngineMode, PageSegMode};
use image::{open, DynamicImage, ImageError, ImageOutputFormat, imageops::FilterType};
use clap::{Arg, App};
use log::{info, warn, error};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::error::Error;

fn main() {
    if let Err(e) = run() {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;

    let matches = App::new("Rust OCR Application")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Performs OCR on images using Tesseract and Rust")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Sets the output file to save the OCR result")
            .takes_value(true))
        .arg(Arg::with_name("lang")
            .short('l')
            .long("lang")
            .value_name("LANGUAGE")
            .help("Sets the language for OCR")
            .takes_value(true)
            .default_value("eng"))
        .arg(Arg::with_name("resize")
            .short('r')
            .long("resize")
            .value_name("DIMENSIONS")
            .help("Sets the dimensions for image resizing, format: WIDTHxHEIGHT")
            .takes_value(true)
            .default_value("3000x3000"))
        .arg(Arg::with_name("contrast")
            .short('c')
            .long("contrast")
            .value_name("VALUE")
            .help("Sets the contrast adjustment value")
            .takes_value(true)
            .default_value("30.0"))
        .arg(Arg::with_name("oem")
            .long("oem")
            .value_name("OEM")
            .help("Sets the OCR Engine Mode (0: Original Tesseract only, 1: Neural nets LSTM only, 2: Tesseract + LSTM, 3: Default)")
            .takes_value(true)
            .default_value("1"))
        .arg(Arg::with_name("psm")
            .long("psm")
            .value_name("PSM")
            .help("Sets the Page Segmentation Mode")
            .takes_value(true)
            .default_value("3"))
        .get_matches();

    let input_path = Path::new(matches.value_of("input").unwrap());
    let output_path = matches.value_of("output");
    let language = matches.value_of("lang").unwrap();
    let resize_dimensions = matches.value_of("resize").unwrap();
    let contrast_value: f32 = matches.value_of("contrast").unwrap().parse()?;
    let oem_mode: OcrEngineMode = match matches.value_of("oem").unwrap().parse()? {
        0 => OcrEngineMode::TesseractOnly,
        1 => OcrEngineMode::LstmOnly,
        2 => OcrEngineMode::TesseractLstmCombined,
        3 => OcrEngineMode::Default,
        _ => return Err("Invalid OCR Engine Mode (OEM)".into()),
    };
    let psm_mode: PageSegMode = match matches.value_of("psm").unwrap().parse()? {
        mode @ 0..=13 => PageSegMode::from(mode),
        _ => return Err("Invalid Page Segmentation Mode (PSM)".into()),
    };

    let (width, height) = parse_dimensions(resize_dimensions)?;
    let preprocessed_path = PathBuf::from(format!("{}_preprocessed.jpg", input_path.display()));

    preprocess_image(&input_path, &preprocessed_path, width, height, contrast_value)?;

    let text = perform_ocr(&preprocessed_path, language, oem_mode, psm_mode)?;

    match output_path {
        Some(path) => {
            fs::write(path, text)?;
            info!("OCR result saved to {}", path);
        },
        None => println!("Recognized text: {}", text),
    }

    Ok(())
}

fn preprocess_image(input_path: &Path, output_path: &Path, width: u32, height: u32, contrast: f32) -> Result<(), ImageError> {
    let img = open(input_path)?;

    let resized = img.resize_exact(width, height, FilterType::CatmullRom);
    let grayscale = resized.grayscale();
    let contrast_adjusted = grayscale.adjust_contrast(contrast);
    let fout = File::create(output_path)?;
    let mut writer = BufWriter::new(fout);

    contrast_adjusted.write_to(&mut writer, ImageOutputFormat::Jpeg(90))?;

    Ok(())
}

fn perform_ocr(image_path: &Path, language: &str, oem: OcrEngineMode, psm: PageSegMode) -> Result<String, Box<dyn Error>> {
    let mut tess = Tesseract::new(None, Some(language))?;
    tess.set_oem(oem);
    tess.set_psm(psm);
    tess.set_image(image_path)?;

    tess.recognize()?;
    let text = tess.get_text()?;

    Ok(text)
}

fn parse_dimensions(dimensions: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let parts: Vec<&str> = dimensions.split('x').collect();
    if parts.len() != 2 {
        return Err("Invalid dimensions format. Use WIDTHxHEIGHT.".into());
    }
    let width = parts[0].parse()?;
    let height = parts[1].parse()?;
    Ok((width, height))
}
