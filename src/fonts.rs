use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use base64::{Engine as _, engine::general_purpose::STANDARD};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontData {
    pub writer_woff2_base64: String,
    pub departure_woff2_base64: String,
    pub arabic_woff2_base64: String,
}

pub const FONT_REQUIREMENTS: &[(&str, &str)] = &[
    ("fonts/writer", ".woff2"),
    ("fonts/departure", ".woff2"),
    ("fonts/arabic", ".woff2"),
];

pub fn ensure_font_files() -> io::Result<()> {
    let mut missing = Vec::with_capacity(FONT_REQUIREMENTS.len());

    for &(directory_path, extension) in FONT_REQUIREMENTS {
        if !directory_has_extension(Path::new(directory_path), extension)? {
            missing.push(format!(
                "{directory_path} (expected at least one {extension} file)"
            ));
        }
    }

    if missing.is_empty() {
        return Ok(());
    }

    let message = format!(
        "Font preflight failed:\n - {}\nFix your fonts folder or update src/fonts.rs if the \
         structure changed.",
        missing.join("\n - ")
    );

    Err(io::Error::new(ErrorKind::NotFound, message))
}

pub fn load_font_data() -> io::Result<FontData> {
    ensure_font_files()?;

    Ok(FontData {
        writer_woff2_base64: font_base64_from_dir(Path::new("fonts/writer"), ".woff2")?,
        departure_woff2_base64: font_base64_from_dir(Path::new("fonts/departure"), ".woff2")?,
        arabic_woff2_base64: font_base64_from_dir(Path::new("fonts/arabic"), ".woff2")?,
    })
}

pub fn font_faces_css(font_data: &FontData) -> String {
    format!(
        "\n  @font-face {{\n    font-family: 'Writer';\n    src: \
         url(data:font/woff2;base64,{}) format('woff2');\n    font-display: swap;\n  }}\n  \
         @font-face {{\n    font-family: 'Departure-Mono';\n    src: \
         url(data:font/woff2;base64,{}) format('woff2');\n    font-display: swap;\n  }}\n  \
         @font-face {{\n    font-family: 'Arabic';\n    src: \
         url(data:font/woff2;base64,{}) format('woff2');\n    font-display: swap;\n  }}\n",
        font_data.writer_woff2_base64,
        font_data.departure_woff2_base64,
        font_data.arabic_woff2_base64,
    )
}

fn directory_has_extension(directory_path: &Path, extension: &str) -> io::Result<bool> {
    let lower_extension = extension.to_ascii_lowercase();

    for directory_entry in fs::read_dir(directory_path)? {
        let directory_entry = directory_entry?;
        let entry_path = directory_entry.path();
        if !entry_path.is_file() {
            continue;
        }

        let Some(file_name) = entry_path.file_name() else {
            continue;
        };
        let file_name = file_name.to_string_lossy();
        if file_name.to_ascii_lowercase().ends_with(&lower_extension) {
            return Ok(true);
        }
    }

    Ok(false)
}

fn font_base64_from_dir(directory_path: &Path, extension: &str) -> io::Result<String> {
    assert!(
        extension.starts_with('.'),
        "font extension must include the leading dot",
    );

    let font_path = first_matching_font_path(directory_path, extension)?;
    let font_bytes = fs::read(font_path)?;

    Ok(STANDARD.encode(font_bytes))
}

fn first_matching_font_path(directory_path: &Path, extension: &str) -> io::Result<PathBuf> {
    let mut matching_paths = Vec::new();
    let lower_extension = extension.to_ascii_lowercase();

    for directory_entry in fs::read_dir(directory_path)? {
        let directory_entry = directory_entry?;
        let entry_path = directory_entry.path();
        if !entry_path.is_file() {
            continue;
        }

        let Some(file_name) = entry_path.file_name() else {
            continue;
        };
        let file_name = file_name.to_string_lossy();
        if file_name.to_ascii_lowercase().ends_with(&lower_extension) {
            matching_paths.push(entry_path);
        }
    }

    matching_paths.sort();

    match matching_paths.into_iter().next() {
        Some(path) => Ok(path),
        None => Err(io::Error::new(
            ErrorKind::NotFound,
            format!(
                "No {extension} font found in {}. Check your fonts directory setup.",
                directory_path.display()
            ),
        )),
    }
}
