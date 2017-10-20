use self::deeply::nested::function as other_function;

fn function() {
    println!("called fucntion()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

pub fn test() {
    other_function();

    println!("entering block");

    {
        use self::deeply::nested::function;
        function();
        println!("leaving block");
    }

    function();
}