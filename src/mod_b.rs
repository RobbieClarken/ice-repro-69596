mod xyz {}

use crate::mod_a::*;

mod mod_c {
    #[allow(dead_code,unused_imports)]
    fn foobar() {
        use super::xyz;
    }
}
