# MagicPdf
<div style="text-align:center">
<img style='float: center' src="./public/logoPdf.png" alt="drawing" width="200"/>
</div>
MagicPdf is a quick and easy-to-use tool to merge two PDF documents.

It's built using multiple technologies:

- **Tauri**: Provides a lightweight and secure framework for building cross-platform desktop applications using web technologies.
 - **Rust**: Acts as the backend for Tauri, handling system interactions, file management, and PDF processing with high performance and safety.
- **JavaScript**: Powers the frontend logic and interactivity of the application.
- **React**: Used to build the user interface, ensuring a responsive and modern experience.
<div style="text-align:center">
<img style='float: center' src="./public/Screenshot 2025-01-29 alle 16.45.28.png" alt="drawing" width="500"/>
</div>

## Prerequisites

Before downloading the repo, ensure you have the following installed on your system:

- Node.js (Latest LTS version recommended)

- Rust (Required for Tauri development)

- Cargo (Rust package manager, installed with Rust)

- Tauri CLI (Install with cargo install tauri-cli)

## Installation

To install MagicPdf, clone this repository and install the dependencies:
```
git clone https://github.com/AndreaSillano/MagicPdf.git
cd MagicPdf
npm install
cd src-tauri
cargo build
```
## Usage

Run the application in development mode with the following command:
```
npm run tauri dev
```
## Building Executables

To build the MagicPdf executable for your platform, run the following command:
```
npm run tauri build
```
This will generate platform-specific binaries inside the ***src-tauri/target/release*** directory.
