pub fn test() {
    let mut _mutable_integer = 7i32;

    {
        let _large_integer = &_mutable_integer;

        // mutable_integer is frozen in this scope
        //_mutable_integer=50;
    }

    _mutable_integer = 3;
}