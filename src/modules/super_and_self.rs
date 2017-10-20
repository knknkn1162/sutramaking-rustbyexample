fn function() {
    println!("called fucntion()");
}

mod cool {
    pub fn function() {
        println!("called cooled function()");
    }
}

mod my {
    fn function() {
        println!("called my::function()");
    }

    mod cool {
        pub fn function() {
            println!("called my::cool::function");
        }

    }

    pub fn indirect_call() {
        print!("called my::indirect_call(), that \n>");

        // called my::function()
        self::function();
        //called my::function()
        function();
        // called my::cool::function
        self::cool::function();
        // called fucntion()
        super::function();

        {
            use self::cool::function as root_function;
            // called my::cool::function
            root_function();
        }
    }
}

pub fn test() {
    my::indirect_call();
}