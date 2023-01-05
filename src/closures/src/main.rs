use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Counter {
    count: u32,
    skip_value: Option<u32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
            skip_value: None,
        }
    }

    fn skip_value(&mut self, skip: u32) {
        self.skip_value = Some(skip);
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;

            if let Some(skip_value) = self.skip_value {
                if self.count == skip_value {
                    self.count += 1;

                    if self.count > 5 {
                        return None;
                    }
                }
            }

            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new().skip(1);

    // counter.skip(5);

    // assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

#[test]
fn skip_special_value() {
    let mut counter = Counter::new();
    counter.skip_value(5);

    let sum: u32 = counter.sum();
    assert_eq!(1 + 2 + 3 + 4, sum);
}

fn main() {
    let mut counter = Counter::new();
    counter.skip_value(6);

    let skiped_counter: Vec<u32> = counter.collect();

    dbg!(skiped_counter);
}
