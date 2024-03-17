\# Rust OCR Application

This Rust application leverages the power of Tesseract OCR to recognize and extract text from images. It provides command-line options to customize the OCR process, including language selection, image resizing, contrast adjustment, and more.

\## Features

- \*\*Tesseract OCR Integration\*\*: Uses the Tesseract library for OCR.
- \*\*Image Preprocessing\*\*: Supports resizing and contrast adjustment of images before OCR.
- \*\*Language Support\*\*: Allows specifying the language for OCR.
- \*\*Flexible Output Options\*\*: Outputs OCR result to a file or prints to stdout.
- \*\*Customizable OCR Engine and Page Segmentation Modes\*\*: Configurable OCR engine mode (OEM) and page segmentation mode (PSM).

\## Prerequisites

To run this application, you need Rust and Cargo installed on your system. Additionally, ensure you have the Tesseract OCR engine and its dependencies installed.

\## Installation

Clone the repository and navigate to the application directory:

\```sh
git clone <repository-url>
cd rust-ocr-application
\```

Build the project using Cargo:

\```sh
cargo build --release
\```

\## Usage

Run the application with the required \`--input\` option and any other optional arguments:

\```sh
cargo run --release -- --input <path/to/image.jpg> [options]
\```

\### Options

- \`-i\`, \`--input FILE\`: Sets the input file to use (required).
- \`-o\`, \`--output FILE\`: Sets the output file to save the OCR result.
- \`-l\`, \`--lang LANGUAGE\`: Sets the language for OCR (default: "eng").
- \`-r\`, \`--resize DIMENSIONS\`: Sets the dimensions for image resizing, format: WIDTHxHEIGHT (default: "3000x3000").
- \`-c\`, \`--contrast VALUE\`: Sets the contrast adjustment value (default: "30.0").
- \`--oem OEM\`: Sets the OCR Engine Mode (default: "1").
- \`--psm PSM\`: Sets the Page Segmentation Mode (default: "3").

\## Contributing

Contributions to improve the application or extend its functionalities are welcome. Please follow the standard Github pull request process to submit your changes.

\## License

Specify the license under which your application is released, or state if it's proprietary software.

\## Author

Ryan Decker ryan@allaspectscomputers.com  
