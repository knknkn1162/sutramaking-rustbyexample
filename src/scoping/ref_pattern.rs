#[derive(Clone, Copy)]
struct Point {x: i32, y: i32}

pub fn test() {
    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point {x: 0, y: 0};

    let _copy_of_x = {
        let Point {x: ref ref_to_x, y: _} = point;

        // return copy of the x, namely, i32
        *ref_to_x
    };

    let mut mutable_point = point;

    {
        let Point {x: _, y: ref mut mut_ref_to_y} = mutable_point;

        *mut_ref_to_y = 1;
    }

    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}