use crate::bar;

pub fn foo(a: usize, call_other: bool) -> usize {
    let mut ret = a;
    if call_other {
        ret = bar::bar(ret, false);
    }
    return ret + 1;
}
