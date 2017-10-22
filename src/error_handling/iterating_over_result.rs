pub fn test() {
    let strings = vec!["tofu", "93", "18"];

    let numbers: Vec<_> = strings.clone().into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    // Once and Result:Err is found, the iteration will terminate
    let numbers2: Result<Vec<_>, _> =
        strings.clone().into_iter()
            .map(|s| s.parse::<i32>()).collect();

    let (numbers3, errors3): (Vec<_>, Vec<_>) =
        strings.clone().into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);

    println!("numbers: {:?}", numbers3);
    println!("errors : {:?}", errors3);


    let (numbers4, errors4): (Vec<_>, Vec<_>) =
        strings.clone().into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);

    let numbers5: Vec<_> = numbers4.into_iter().map(Result::unwrap).collect();
    let errors5: Vec<_> = errors4.into_iter().map(Result::unwrap_err).collect();

    println!("{:?}", numbers5);
    println!("{:?}", errors5);



}