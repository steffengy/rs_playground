#![feature(plugin, custom_attribute)]
#![plugin(testplugin_test)]

#[test_plug]
fn hello_world() -> u64 {
   6
}

/*tests\main.rs:1:1: 1:1 error: mismatched types:
 expected `()`,
    found `u64`
(expected (),
    found u64) [E0308]
*/
