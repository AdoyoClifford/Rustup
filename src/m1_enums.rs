#[derive(Debug)]
enum CarColor {
    RED,
    BLUE,
    GREEN,
    YELLOW,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    SUCCESS(T),
    ERROR(E),
}

#[derive(Debug)]
enum GivenOption<T> {
    NONE,
    SOME(T),
}

fn check_remainder(num: f32) -> GivenOption<f32> {
    let remainder = num % 10.0;
    if remainder != 0.0 {
        GivenOption::SOME(remainder)
    } else {
        GivenOption::NONE
    }
}

fn check_number_under_five(num: u8) -> GivenResult<u8, String> {
    if num < 5 {
        GivenResult::SUCCESS(10)
    } else {
        GivenResult::ERROR(String::from("Not in range"))
    }
}

fn check_number_under_five_built_in(num: u8) -> Result<u8, String> {
    if num < 5 {
        Ok(10)
    } else {
        Err("Not in range".to_string())
    }
}

fn create_car_color() -> CarColor {
    let car_color: CarColor = CarColor::BLUE;
    car_color
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_enum() {
        let car_color: CarColor = create_car_color();
        dbg!(car_color);

        let under_five_res = check_number_under_five_built_in(10);
        dbg!(under_five_res);

        let remainder = check_remainder(20.0);
        dbg!(remainder);
    }
}
