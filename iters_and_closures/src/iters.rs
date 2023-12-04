#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

// The `.sum()` method consumes the iterator and takes ownership of it.
// Such methods are called consuming adaptors.
#[test]
fn iteration_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // v1_iter consumed here.
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // v1_iter consumed above, so this will fail to compile if uncommented.
    // let total: i32 = v1_iter.sum();
}

// The `.map()` method produces another iterator, so it does not consume the iterator it is called on.
// Such methods are called iterator adaptors.
#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // v1_iter is not consumed here.
    // warning will be produced by this line if uncommented.
    // the closure was never called because iterators are lazy.
    // v1.iter().map(|x| x + 1);

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn filters_by_size() {
    
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

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

    let in_my_size: Vec<Shoe> = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}
