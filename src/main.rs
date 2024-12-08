use anyhow::Result;
use crypticRust::prelude::Primality;

fn main() {
    let only_primes = only_primes();
    println!("My List:\n");
    only_primes
        .into_iter()
        .for_each(|item| println!("{:?}", item));
}

fn prime_nums() -> PrimeList {
    PrimeList::new(vec![]).fill()
}

fn only_primes() -> Result<PrimeList> {
    let numbers: Vec<usize> = (0..100).collect();
    PrimeList::new(numbers).prune()
}


#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct PrimeList {
    pub list: Vec<usize>,
}

impl PrimeList {
    pub fn new(list: Vec<usize>) -> PrimeList {
        PrimeList { list }
    }

    fn fill(self) -> PrimeList {
        let vector = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 31];

        PrimeList { list: vector }
    }

    fn prune(self) -> Result<PrimeList> {
        // Return only the prime numbers within self
        let list = self.list.into_iter().filter(|i| i.primality()).collect();

        Ok(PrimeList { list })
    }
}
