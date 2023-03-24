use std::marker::PhantomData;

use libR_sys::SEXP;

struct GRobj<T>{
    inner: SEXP,
    sexp_type: PhantomData<T>
}

trait 