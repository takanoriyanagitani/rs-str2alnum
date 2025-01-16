use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn chars2filtered<I, F>(c: I, f: F) -> impl Iterator<Item = char>
where
    I: Iterator<Item = char>,
    F: Fn(&char) -> bool,
{
    c.filter(f)
}

pub fn chars2writer<I, W>(c: I, mut w: W) -> Result<(), io::Error>
where
    I: Iterator<Item = char>,
    W: Write,
{
    for i in c {
        write!(w, "{i}")?;
    }
    Ok(())
}

pub fn string2filter2writer<F, W>(s: String, f: F, w: W) -> Result<(), io::Error>
where
    W: Write,
    F: Fn(&char) -> bool,
{
    let chars = s.chars();
    let mapd = chars2filtered(chars, f);
    chars2writer(mapd, w)
}

pub fn strings2filter2writer<I, F, W>(s: I, f: F, mut w: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
    F: Fn(&char) -> bool,
    W: Write,
{
    for i in s {
        let item: String = i?;
        string2filter2writer(item, &f, &mut w)?;
        writeln!(&mut w)?;
        w.flush()?;
    }
    Ok(())
}

pub fn reader2filter2writer<R, F, W>(r: R, f: F, w: W) -> Result<(), io::Error>
where
    R: Read,
    F: Fn(&char) -> bool,
    W: Write,
{
    let br = BufReader::new(r);
    let lines = br.lines();
    strings2filter2writer(lines, f, w)
}

pub fn stdin2filter2stdout<F>(f: F) -> Result<(), io::Error>
where
    F: Fn(&char) -> bool,
{
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let mut ol = o.lock();

    {
        let mut bw = BufWriter::new(&mut ol);
        reader2filter2writer(il, f, &mut bw)?;
    }

    ol.flush()
}

pub fn stdin2alnum2stdout() -> Result<(), io::Error> {
    stdin2filter2stdout(|c| c.is_ascii_alphanumeric())
}

pub fn stdin2alnum_us2stdout() -> Result<(), io::Error> {
    stdin2filter2stdout(|c| {
        let is_us: bool = '_'.eq(c);
        let is_alnum: bool = c.is_ascii_alphanumeric();
        is_us || is_alnum
    })
}

pub fn stdin2stdout(allow_under_score: bool) -> Result<(), io::Error> {
    match allow_under_score {
        true => stdin2alnum_us2stdout(),
        false => stdin2alnum2stdout(),
    }
}
