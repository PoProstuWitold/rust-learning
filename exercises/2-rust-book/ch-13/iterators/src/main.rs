// Iterators, a way of processing a series of elements
// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();
// }

// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     for val in v1_iter {
//         println!("Got: {}", val);
//     }
// }


// The Iterator Trait and the next Method
// All iterators implement a trait named Iterator that is defined in the standard library. 
// The definition of the trait looks like this:
// #![allow(unused)]
// fn main() {
//     pub trait Iterator {
//         type Item;

//         fn next(&mut self) -> Option<Self::Item>;

//         // methods with default implementations elided
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn iterator_demonstration() {
//         let v1 = vec![1, 2, 3];

//         let mut v1_iter = v1.iter();

//         assert_eq!(v1_iter.next(), Some(&1));
//         assert_eq!(v1_iter.next(), Some(&2));
//         assert_eq!(v1_iter.next(), Some(&3));
//         assert_eq!(v1_iter.next(), None);
//     }
// }


// Methods that Consume the Iterator
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn iterator_sum() {
//         let v1 = vec![1, 2, 3];

//         let v1_iter = v1.iter();

//         let total: i32 = v1_iter.sum();

//         assert_eq!(total, 6);
//     }
// }


// Methods that Produce Other Iterators
// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];

//     v1.iter().map(|x| x + 1);
// }

// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

//     assert_eq!(v2, vec![2, 3, 4]);
// }


// Using Closures that Capture Their Environment
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

