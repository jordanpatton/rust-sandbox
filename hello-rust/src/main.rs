#[allow(dead_code)]

//========================================================================================
// fn main() {
//     let vector = vec!['a', 'b', 'c', 'd', 'e'];

//     let third = &vector[2];
//     println!("The third element is {third}.");

//     let third = vector.get(2);
//     match third {
//         Some(third) => println!("The third element is {third}."),
//         None => println!("There is no third element."),
//     }

//     let third = vector.get(2);
//     if let Some(third) = third {
//         println!("The third element is {third}.");
//     } else {
//         println!("There is no third element.");
//     }
// }

//========================================================================================
// fn main() {
//     let mut vector = vec!['a', 'b', 'c', 'd', 'e'];
//     dbg!(&vector);

//     let first = &vector[0];
//     println!("The first element is {first}.");

//     vector.push('f');
//     dbg!(&vector);

//     // println!("The first element is {first}.");
// }

//========================================================================================
// fn main() {
//     let vector = vec!['a', 'b', 'c', 'd', 'e'];
//     for v in &vector {
//         println!("{v}");
//     }

//     let mut vector = vec![10, 20, 30, 40, 50];
//     for v in &mut vector {
//         *v += 5;
//     }
//     for v in &vector {
//         println!("{v}");
//     }
// }

//========================================================================================
#[derive(Debug)]
enum Types {
    Char(char),
    F64(f64),
    I32(i32),
    String(String),
}

fn main() {
    let vector_with_multiple_types = vec![
        Types::I32(3),
        Types::String(String::from("asdf")),
        Types::F64(10.12),
        Types::String(String::from("zxcv")),
        Types::I32(5),
        Types::Char('a'),
    ];
    dbg!(&vector_with_multiple_types);
}
