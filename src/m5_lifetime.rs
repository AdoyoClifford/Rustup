#[allow(dead_code, unused_variables)]
fn example_0() {
    let r: &i32;

    let x: i32 = 5;
    r = &x;

    print!("{}", r)
}

#[allow(dead_code, unused_variables)]
fn example_1() {
    let highest_age: i32;

    let alice_age: i32 = 20;
    let bob_age: i32 = 20;

    highest_age = find_largest(&alice_age, &bob_age);
    println!("Highest age is{}", highest_age);

    fn find_largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_2() {
    let highest_age: &i32;

    let alice_age: i32 = 20;
    let bob_age: i32 = 20;

    highest_age = find_largest(&alice_age, &bob_age);
    println!("Highest age is{}", highest_age);

    fn find_largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_3_generics() {
    let highest_age: &f32;

    let alice_age: Person = Person {
        name: "Alice",
        points: &20.2,
    };
    let bob_age: Person = Person {
        name: "Bob",
        points: &20.0,
    };

    highest_age = find_largest::<f32>(&alice_age.points, &bob_age.points);
    println!("Highest age is{}", highest_age);

    fn find_largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

struct Person<'p> {
    name: &'p str,
    points: &'p f32,
}
