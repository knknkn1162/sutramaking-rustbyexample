struct Point {x: i32, y: i32, z: i32}

pub fn test() {
    let mut point = Point {x: 0, y: 0, z: 0};

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );

        //let mutable_borrow = &mut point;
    }

    {
        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        //let y = &point.y;

        //println!("Point Z coodinate is {}", point.z);

        println!(
            "Point has corrdinates : ({}, {}, {})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );
    }

    let borrowed_point = &point;

    println!(
        "Point now has coodinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );
}