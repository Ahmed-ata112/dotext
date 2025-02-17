extern crate pdf_extract;

use zip::ZipArchive;

use self::pdf_extract::*;
use xml::events::Event;
use xml::name::QName;
use xml::reader::Reader;

use std::clone::Clone;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use zip::read::ZipFile;

use doc::{DocumentHandler, HasKind};
pub struct Pdf {
    path: PathBuf,
    data: Cursor<String>,
}
impl HasKind for Pdf {
    fn kind(&self) -> &'static str {
        "Pdf File"
    }

    fn ext(&self) -> &'static str {
        "pdf"
    }
}

impl DocumentHandler<Pdf> for Pdf {
    fn open<P: AsRef<Path>>(path: P) -> io::Result<Pdf> {
        let bytes = std::fs::read(path.as_ref()).unwrap();
        let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();

        Ok(Pdf {
            path: path.as_ref().to_path_buf(),
            data: Cursor::new(out),
        })
    }
}

impl Read for Pdf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.data.read(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn instantiate() {
        let _ = Pdf::open(Path::new("samples/sample.pdf"));
    }

    #[test]
    fn read() {
        let mut f = Pdf::open(Path::new("samples/sample.pdf")).unwrap();
        let mut data = String::new();
        let len = f.read_to_string(&mut data).unwrap();
        println!("len: {}, data: {}", len, data);
    }
}
