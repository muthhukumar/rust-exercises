struct CustomSmartPointers {
    data: String,
}

impl Drop for CustomSmartPointers {
    fn drop(&mut self) {
        println!(
            "smart pointer is going out of scope and its value is {}",
            self.data
        );
    }
}

fn main() {
    println!("Before the block");
    {
        let pointer = CustomSmartPointers {
            data: "Hello world".to_string(),
        };

        drop(pointer);

        println!("the pointer is dropped early")
    }

    println!("After the block");
}
