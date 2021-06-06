use crate::foo;

pub fn bar(a: usize, call_other: bool) -> usize {
    let mut ret = a;
    if call_other {
        ret = foo::foo(ret, false);
    }
    return ret + 1;
}

