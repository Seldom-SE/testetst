macro_rules! what_happens_if_i_do_this {
    ($car:ident, $($cdr:ident,)*) => {
        what_happens_if_i_do_this!($($cdr,)*);
        what_happens_if_i_do_this!($($cdr,)*);
        what_happens_if_i_do_this!($($cdr,)*);
        what_happens_if_i_do_this!($($cdr,)*);
        what_happens_if_i_do_this!($($cdr,)*);
        what_happens_if_i_do_this!($($cdr,)*);
        what_happens_if_i_do_this!($($cdr,)*);
        what_happens_if_i_do_this!($($cdr,)*);
    };
    () => {};
}

what_happens_if_i_do_this!(a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a,);

fn main() {}
