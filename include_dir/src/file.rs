use std::{
    fmt::{self, Debug, Formatter},
    fs, io,
    path::Path,
    str,
};

/// A file with its contents stored in a `&'static [u8]`.
#[derive(Copy, Clone, PartialEq)]
pub struct File<'a> {
    #[doc(hidden)]
    pub path: &'a str,
    #[doc(hidden)]
    pub contents: &'a [u8],
}

impl<'a> File<'a> {
    /// The file's path, relative to the directory included with
    /// `include_dir!()`.
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// The file's raw contents.
    pub fn contents(&self) -> &'a [u8] {
        self.contents
    }

    /// The file's contents interpreted as a string.
    pub fn contents_utf8(&self) -> Option<&'a str> {
        str::from_utf8(self.contents()).ok()
    }

    /// Writes the file's contents to a location on disk.
    pub fn write_to<S: AsRef<Path>>(&self, path: S) -> io::Result<()> {
        fs::write(path, self.contents())
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("contents", &format!("<{} bytes>", self.contents.len()))
            .finish()
    }
}
