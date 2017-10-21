struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

pub fn test() {
    let _a = Droppable {name: "a"};

    {
        let _b = Droppable {name : "b"};
        {
            let _c = Droppable {name: "c"};
            let _d = Droppable {name: "d"};

            println!("Exiting block B");
        }

        //drop(_a); //use of moved value: `_a`
        println!("Exiting block A")
    }

    drop(_a); // drop manually

    println!("end of the main function");

    // _a won't dropped.
}