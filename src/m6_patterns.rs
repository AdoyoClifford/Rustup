
#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn match_literals() {
        let number: i32 = 17;
        let res = match  number{
            1 => "Awesome first ",
            2 |3 |5 |7 |11 |13 |17 => "Yaay primes present ",
            _ => "Something else "
        };
        print!("{}",res);
    }

    #[test]
    fn test_match_option() {
        let some_num: Option<i32> = Some(20);

        let res = if let Some(j) = some_num {
             j
        } else {
            panic!("Nothing to look at here")
        };
        println!("{}",res)
        

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
        let result: Result<i32, &str> = Err("There was an error");
        let err_result: Result<&str, &str> = Err("404");
        let res = match result {
            Ok(val ) => val,
            Err(e) => panic!("{}",e)
        };
        println!("{}",res)
    }
}
