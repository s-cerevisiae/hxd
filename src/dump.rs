use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use crate::cli::DumpArgs;

enum ReadResult {
    Eof(usize),
    Ok,
    Err(io::Error),
}

fn read_till_full<R: Read>(mut reader: R, buf: &mut [u8]) -> ReadResult {
    let total_len = buf.len();
    let mut remaining_buf = buf;
    while !remaining_buf.is_empty() {
        match reader.read(remaining_buf) {
            Ok(0) => return ReadResult::Eof(total_len - remaining_buf.len()),
            Ok(n) => remaining_buf = &mut remaining_buf[n..],
            Err(e) => return ReadResult::Err(e),
        }
    }
    ReadResult::Ok
}

struct CountingWriter<W> {
    writer: W,
    count: usize,
}

impl<W> CountingWriter<W> {
    fn new(writer: W) -> Self {
        Self { writer, count: 0 }
    }

    fn count(&self) -> usize {
        self.count
    }
}

impl<W: Write> Write for CountingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf).inspect(|n| self.count += n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

struct Printer {
    octets_per_group: usize,
    output_width: usize,
    current_offset: usize,
}

impl Printer {
    fn new(octets_per_group: usize) -> Self {
        Self {
            octets_per_group,
            output_width: 0,
            current_offset: 0,
        }
    }

    fn write_line<W: Write>(&mut self, out: W, buf: &[u8]) -> io::Result<()> {
        let mut out = CountingWriter::new(out);
        write!(out, "{:08x}: ", self.current_offset)?;
        for (i, b) in buf.iter().enumerate() {
            if i != 0 && i % self.octets_per_group == 0 {
                write!(out, " ")?;
            }
            write!(out, "{b:02x}")?;
        }

        self.output_width = self.output_width.max(out.count());
        let padding = self.output_width - out.count() + 2;
        write!(out, "{:1$}", " ", padding)?;

        for &b in buf {
            let c = if b.is_ascii_graphic() || b == b' ' {
                b.into()
            } else {
                '.'
            };
            write!(out, "{c}")?;
        }
        writeln!(out)?;

        self.current_offset += buf.len();

        Ok(())
    }
}

pub fn dump(options: DumpArgs) -> io::Result<()> {
    let writer = io::stdout().lock();
    if let Some(path) = &options.input {
        let reader = BufReader::new(File::open(path)?);
        dump_impl(options, reader, writer)
    } else {
        let reader = io::stdin().lock();
        dump_impl(options, reader, writer)
    }
}

pub(crate) fn dump_impl<R: Read, W: Write>(
    options: DumpArgs,
    mut reader: R,
    mut writer: W,
) -> Result<(), io::Error> {
    let octets_per_line = options.columns;

    let mut printer = Printer::new(options.groupsize);
    let mut buf = vec![0; octets_per_line];
    loop {
        match read_till_full(&mut reader, &mut buf) {
            ReadResult::Eof(0) => break,
            ReadResult::Eof(n) => {
                printer.write_line(&mut writer, &buf[..n])?;
                break;
            }
            ReadResult::Ok => printer.write_line(&mut writer, &buf)?,
            ReadResult::Err(e) => return Err(e),
        }
    }

    Ok(())
}
