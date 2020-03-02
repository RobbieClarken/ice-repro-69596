mod mod_a {
    pub(crate) use macros::xyz;
}

mod mod_b {
    mod xyz {}

    use crate::mod_a::*;

    mod mod_c {
        #[allow(dead_code,unused_imports)]
        fn foobar() {
            use super::xyz;
        }
    }
}
