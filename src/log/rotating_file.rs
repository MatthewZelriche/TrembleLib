use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

pub struct RotatingFile {
    file: BufWriter<File>,
}

impl RotatingFile {
    pub fn new(filename: &str, max_rotates: u32) -> io::Result<Self> {
        if fs::exists(filename)? {
            RotatingFile::rotate(filename, max_rotates)?;
        }
        Ok(Self {
            file: BufWriter::new(
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(filename)?,
            ),
        })
    }

    fn rotate(filename: &str, max_rotates: u32) -> io::Result<()> {
        for n in (1..max_rotates).rev() {
            let old_path = PathBuf::from(filename).with_extension(format!("{}.log", n));
            if old_path.exists() {
                let new_path = PathBuf::from(filename).with_extension(format!("{}.log", n + 1));
                fs::rename(old_path, new_path)?;
            }
        }

        // Clean out the rotated file that's beyond max rotates
        let rolled_over_path =
            PathBuf::from(filename).with_extension(format!("{}.log", max_rotates));
        if rolled_over_path.exists() {
            fs::remove_file(rolled_over_path)?;
        }

        // Rotate the very first file
        let new_path = PathBuf::from(filename).with_extension("1.log");
        fs::rename(filename, new_path)?;

        Ok(())
    }
}

impl Write for RotatingFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
