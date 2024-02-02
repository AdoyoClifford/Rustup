#[derive(Debug)]
struct User {
    name: String,
    email: String,
    is_active: bool,
    count: i8,
}

fn change_username(user: &mut User, new_name: &str) {
    user.name = String::from(new_name)
}

impl User {
    fn increment_count(&mut self) {
        self.count += 1
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = new_email.to_string()
    }

    fn change_name(&mut self, new_name: &str) {
        self.email = new_name.to_string()
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_struct() {
        let mut user = User {
            name: String::from("someone"),
            email: String::from("someone@example.com"),
            is_active: true,
            count: 10,
        };

        let mut user2 = User {
            name: String::from("someone2"),
            email: String::from("someone2@example.com"),
            is_active: true,
            count: 0,
        };

        let new_name = change_username(&mut user, "somenewuser");

        user2.increment_count();

        dbg!(user2);
    }
}
