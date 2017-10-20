mod my_mod {
    fn private_function() {
        println!("called my_mod private function()");
    }

    pub fn function() {
        println!("called my_mod::function()");
    }

    pub fn indirect_access() {
        print!("called my_mod::indirect_access, that >");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called my_mod nested")
        }

        fn private_funcion() {
            println!("called my_mod::nested::function()");
        }

        //pub (in my_mod) fn public_function_in_my_mod() {
        // print!("called my_mod::nested::public_function_in_my_mod() that > ");
        // public_function_in_nested();
        //}

        pub (self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested");
        }

        pub (super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }
}

fn function() {
    println!("called function()");
}

pub fn test() {
    function();
    my_mod::function();
    use self::my_mod::nested;
    //nested::public_function_in_nested();
}
