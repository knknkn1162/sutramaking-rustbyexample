use std::marker::PhantomData;

// use PhantomData, save computations
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct OrdinaryTuple<A, B>(A, B);

#[derive(PartialEq)]
struct PhantomStruct<A, B>{first: A, phantom: PhantomData<B>}

pub fn test() {
    let _ord_tuple1: OrdinaryTuple<char, f32> = OrdinaryTuple('Q', 3.4);
    let _ord_tuple2: OrdinaryTuple<char, f64> = OrdinaryTuple('Q', 3.4);

    // expected f32, found f64
    println!("{}", _ord_tuple1==_ord_tuple2);


    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);

    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // mismatched types
    println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
}