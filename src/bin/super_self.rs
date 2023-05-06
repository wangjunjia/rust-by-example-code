fn function() {
    println!("called function()");
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

mod my {
    fn function() {
        println!("called my::function");
    }

    mod cool {
        pub fn function() {
            println!("called my::cool::function()");
        }
    }

    pub fn indirect_call() {
        println!("called my::indirect_call()");

        self::function();
        function();

        self::cool::function();

        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
