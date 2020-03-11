use crate::errors::Errors;

pub fn parse_date(date: &str) -> Result<(u32, u32, u32), Errors> {
    let numbers: Vec<u32> = date
        .split('-')
        .map(|s| Ok(s.parse()?))
        .collect::<Result<_, _>>()
        .map_err(|_: std::num::ParseIntError| Errors::InvalidDate)?;
    match numbers.as_slice() {
        &[y, m, d] => Ok((y, m, d)),
        _ => Err(Errors::InvalidDate),
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn date() {
        //
    }
}
