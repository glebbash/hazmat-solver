#[macro_export]
macro_rules! get {
    ($a: expr, $b: expr, $c: expr) => {
        unsafe { *$a.get_unchecked($b).get_unchecked($c) }
    };
}

#[macro_export]
macro_rules! set {
    ($a: expr, $b: expr, $c: expr, $d: expr) => {
        unsafe { *$a.get_unchecked_mut($b).get_unchecked_mut($c) = $d }
    };
}