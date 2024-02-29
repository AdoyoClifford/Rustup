mod m1_enums;
mod m2_stucts;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetime;
mod m6_patterns;

#[allow(dead_code, unused_variables)]
const OUR_COURSE: &str = "Rust with AutoGpt";

fn main() {
    // println!("Welcome to this course on {}", OUR_COURSE);

    // let y: i32 = 4;

    // for i in 0..= y {
    //     if i != 4 {
    //         print!("{}", i);
    //     } else {
    //         println!("{}", i)
    //     }
    // }

    // let is_zero_remainder = 10 % 4 != 0;
    // println!("{}",is_zero_remainder);

    // let char_emoji = '😎';
    // println!("Hello there {}",char_emoji);

    // let my_ints: [f32; 10] = [0.0;10];
    // println!("{:?}", my_ints);

    // let my_floats_new: [f32; 10] = my_ints.map(|n:f32| n + 2.0);
    // println!("{:?}", my_floats_new);

    // let dynamic_name = "Cliff".to_string();
    // println!("{}",dynamic_name);

    // let dynamic_name = &dynamic_name[0..3];
    // println!("sliced name is {}",dynamic_name);
    // let mut chars: Vec<char> = Vec::new();
    // chars.insert(0,'h');
    // chars.insert(1, 'e');
    // chars.push('l');
    // chars.push('l');
    // chars.push('o');

    // println!("{:?}",chars);

    //chars.iter().for_each(|c| println!("char is {}",c))

    // let chars_again: Vec<char> = vec!['h','e','l','l','o',];
    // dbg!(&chars_again);

    // let collected: String = chars_again.iter().collect();
    // dbg!(&collected);

    // for c in chars_again {
    //     print!("{}",c);
    //     if c == 'o' {
    //         print!("world!")
    //     }
    // }

    //closures
    // let num = 5;
    // let add_num = |x: i32| x + num;
    // let new_num = add_num(7);
    // dbg!(new_num);

    //Number literals
    // println!("Hex number is {}",0xff);
    // println!("Octal number is {}",0o77);
    // println!("Bytes for 'A' is {}",b'A');

    //Logic Gates
    let a: u8 = 0b_10101011;
    let b: u8 = 0b_010010010;
    // println!("AND {:08b}",a & b);
    // println!("OR {:08b}",a | b);
    // println!("XOR {:08b}",a ^ b);
    // println!("NOT {:08b}",!a);
    println!("a << 1 {:08b}", a >> 1);
}
