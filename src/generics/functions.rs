struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

// A has not been specified as a generic type parameter.
fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

pub fn test() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic(SGen("C"));
}