use std::fmt;

#[derive(Debug)]
pub enum Query {
    City(String),
    Zip(String),
    Id(String),
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Query::City(ref city) => write!(f, "q={}", city),
            Query::Zip(ref zip) => write!(f, "zip={}", zip),
            Query::Id(ref id) => write!(f, "id={}", id),
        }
    }
}

pub fn queries() -> impl Iterator<Item = Query> {
    use std::env;

    let pairs = PairingIterator::new(env::args().skip(1));

    pairs.filter_map(|(left, right)| {
        match &*left {
            "-c" | "--city" => Some(Query::City(right)),
            "-z" | "--zip" => Some(Query::Zip(right)),
            "-i" | "--id" => Some(Query::Id(right)),

            _ => None
        }
    })
}

struct PairingIterator<T> { source: T }

impl<T: Iterator> PairingIterator<T> {
    fn new(source: T) -> Self {
        Self { source }
    }
}

impl<T: Iterator> Iterator for PairingIterator<T> {
    type Item = (T::Item, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.source.next()?, self.source.next()?))
    }
}
