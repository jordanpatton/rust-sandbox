// #[allow(dead_code)]

// use ferris_says::say;
// use std::io::{stdout, BufWriter};
// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width = message.chars().count();
//     let mut writer = BufWriter::new(stdout.lock());
//     say(&message, width, &mut writer).unwrap();
// }

// //========================================================================================
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//     println!("before: {}", s);
//     change(&mut s);
//     println!(" after: {}", s);
//     let a = &mut s;
//     println!("     a: {}", a);
//     let b = &mut s;
//     println!("     b: {}", b);
// }

//========================================================================================
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &v) in bytes.iter().enumerate() {
//         if v == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let a = String::from("hello");
//     let a_fw = first_word(&a);
//     println!("{} (0..{}: {})", a, a_fw, &a[0..a_fw]);
//     let b = String::from("hello, world");
//     let b_fw = first_word(&b);
//     println!("{} (0..{}: {})", b, b_fw, &b[0..b_fw]);
// }

//========================================================================================
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &v) in bytes.iter().enumerate() {
//         if v == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let a = String::from("hello");
//     let a_fw = first_word(&a);
//     println!("{} ({})", a, a_fw);
//     let b = String::from("hello, world");
//     let b_fw = first_word(&b);
//     println!("{} ({})", b, b_fw);
//     let c = "string literal";
//     let c_fw = first_word(c);
//     println!("{} ({})", c, c_fw);
// }

//========================================================================================
// fn main() {
//     let mut s = String::from("hello world");
//     let _s_slice = &s[..];
//     s.clear(); // error!
//     // println!("the slice is: {_s_slice}");
// }

//========================================================================================
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     println!("{user1:#?}");
//     dbg!(&user1);
// }

//========================================================================================
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };
//     dbg!(&rect1);
// }

//========================================================================================
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!(" width: {}", rect1.width);
//     println!("height: {}", rect1.height);
//     println!("  area: {}", rect1.area());
// }

//========================================================================================
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn is_larger_than(&self, other: &Rectangle) -> bool {
//         self.area() >= other.area()
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };
//     println!("Is rect1 larger than rect2? {}", rect1.is_larger_than(&rect2));
//     println!("Is rect1 larger than rect3? {}", rect1.is_larger_than(&rect3));
//     println!("Is rect2 larger than rect3? {}", rect2.is_larger_than(&rect3));
// }

//========================================================================================
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn new_square(side_length: u32) -> Self {
//         Self {
//             width: side_length,
//             height: side_length,
//         }
//     }
// }

// fn main() {
//     let r = Rectangle::new_square(10);
//     dbg!(&r);
// }

//========================================================================================
// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     dbg!(&home);
//     let loopback = IpAddr::V6(String::from("::1"));
//     dbg!(&loopback);
// }

//========================================================================================
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("ring! {:?}", self);
//     }
// }

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     dbg!(&m);
//     m.call();
// }

//========================================================================================
// // https://doc.rust-lang.org/std/option/enum.Option.html
// fn main() {
//     let a: Option<u8> = None;
//     dbg!(&a, a.is_none(), a.is_some());
//     if a.is_some() {
//         println!("unwrapped: {}", a.unwrap())
//     } else {
//         println!("unwrapped: None")
//     }

//     let b: Option<u8> = Some(1);
//     dbg!(&b, b.is_none(), b.is_some());
//     if b.is_some() {
//         println!("unwrapped: {}", b.unwrap())
//     } else {
//         println!("unwrapped: None")
//     }
// }

//========================================================================================
// fn main() {
//     let dice_roll = 2;
//     match dice_roll {
//         1 => println!("{dice_roll}"),
//         3 => println!("odd"),
//         5 => println!("odd"),
//         _ => println!("even"),
//     }
// }

//========================================================================================
// fn add_one(input: Option<i32>) -> Option<i32> {
//     match input {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }

// fn main() {
//     println!("{:?} -> {:?}", Some(1), add_one(Some(1)));
//     println!("{:?} -> {:?}", Some(5), add_one(Some(5)));
//     println!("{:?} -> {:?}", None::<i32>, add_one(None));
// }

//========================================================================================
// fn main() {
//     let optional = Some(3u8);

//     match optional {
//         Some(x) => println!("The value is {x}."),
//         _ => (),
//     }

//     if let Some(y) = optional {
//         println!("The value is {y}.");
//     }

//     if let Some(z) = None::<u8> {
//         println!("The value is {z}.");
//     } else {
//         println!("The value is None.");
//     }
// }

//========================================================================================
// mod a {
//     pub mod b {
//         pub fn c() {
//             println!("c");
//         }
//     }
// }

// fn main() {
//     use crate::a::b;
//     b::c(); // idiomatic for function imports

//     use crate::a::b::c;
//     c(); // not idiomatic for function imports
// }
