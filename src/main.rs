fn my_static_fn(&(): &()) {}

fn main() {
    accepts_static(my_static_fn);

    let param = ();
    accepts_fn_with_param(my_static_fn, &param);
}

fn accepts_static<S: 'static>(_: S) {}

fn accepts_fn_with_param<P>(f: fn(P), p: P)
where
    fn(P): 'static,
{
    f(p);
    accepts_static(f);
}
