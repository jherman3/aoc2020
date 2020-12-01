use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

pub fn line_input<T: std::str::FromStr>(p: &str) -> AnyResult<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::error::Error + 'static,
{
    let f = File::open(p)?;
    let r = BufReader::new(f);
    let mut vals = Vec::new();
    for l in r.lines() {
        let n = l?.parse::<T>();
        vals.push(n?);
    }
    Ok(vals)
}
