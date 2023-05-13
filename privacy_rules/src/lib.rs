mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function();
        }
        fn secret_function() {}
    }

    // Here this function has access to the inner_function because it is in the parent module of
    // the inside
    fn testing() {
        inside::inner_function();
    }
}

fn try_me() {
    fn testing() {
        outermost::middle_function();
        outermost::middle_secret_function();
        outermost::inside::inner_function();
        outermost::inside::secret_function();
    }

    testing();
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use a::series::of;

fn main() {
    of::nested_modules();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
