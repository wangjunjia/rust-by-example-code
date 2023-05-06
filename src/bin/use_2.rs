fn function() {
    println!("called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

use deeply::nested::function as nested_function;

fn main() {
    nested_function();

    println!("Entering block");

    {
        use deeply::nested::function;
        function();

        println!("leaving block");
    }

    function();
}
