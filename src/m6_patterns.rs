#[derive(Debug)]
enum Message {
    Quit,
    Change_Color(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Change_Color(r, g, b) => {
            println!("Changing color to Red{}, Green{}, Blue{}", r, g, b)
        }
        Message::Move { x, y: new_name } => {
            println!("Moving to x: {}, y as new_name: {}", x, new_name)
        }
        Message::Write(s) => {
            println!("{}", s)
        }
    }
}

#[cfg(test)]
mod test_message {
    use super::Message;

    use super::*;

    #[test]
    fn message_match_test() {
        let quit = Message::Quit;
        let change_color = Message::Change_Color(255, 0, 0);
        let move_msg = Message::Move { x: 10, y: 20 };
        let write = Message::Write("Hello there".to_string());

        process_message(quit);
        process_message(change_color);
        process_message(move_msg);
        process_message(write);
    }

    #[test]
    fn match_pair() {
        let pair = (2, 2);
        match pair {
            (x, y) if x == y => println!("They match"),
            (x, y) if x + y == 0 => println!("They neutralize"),
            _ => println!("No match"),
        }
    }

    #[test]
    fn match_range() {
        let number = 5;
        match number {
            1..=5 => println!("Number in range"),
            _ => println!("Number out of range"),
        }
    }
    #[test]
    fn match_char() {
        let c = 'c';
        match c {
            'a'..='z' => println!("Lower case"),
            'A'..='Z' => println!("Upper case"),
            _ => println!("Something else"),
        }
    }

    struct Location {
        x: i32,
        y: i32,
    }

    #[test]
    fn match_struct() {
        let location: Location = Location { x: 0, y: 20 };

        match location {
            Location { x: 0, y: 0 } => println!("Origin"),
            Location { x: 0, y } => println!("On y axis"),
            Location { x, y: 0 } => println!("On x axis"),
            Location { x, y } => println!("On x: {}, y: {}", x, y),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_literals() {
        let number: i32 = 17;
        let res = match number {
            1 => "Awesome first ",
            2 | 3 | 5 | 7 | 11 | 13 | 17 => "Yaay primes present ",
            _ => "Something else ",
        };
        print!("{}", res);
    }

    #[test]
    fn test_match_option() {
        let some_num: Option<i32> = Some(20);

        let res = if let Some(j) = some_num {
            j
        } else {
            panic!("Nothing to look at here")
        };
        println!("{}", res)

        // let res = match some_num {
        //     Some(i) => i,
        //     None => {
        //         panic!("Something went wrong")
        //     }
        // };
        // println!("{}",res)
    }

    #[test]
    fn match_result() {
        let result: Result<i32, &str> = Ok(50);
        let err_result: Result<&str, &str> = Err("404");
        let res = match result {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };
        // println!("{}",res);

        let my_int = if let Ok(r) = result {
            r
        } else {
            panic!("There was a problem")
        };

        println!("{}", my_int)
    }
}
