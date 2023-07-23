mod a01;
fn noop(_: Vec<String>) {}
pub const MODS: [fn(Vec<String>); 100] = {
    let mut arr: [fn(Vec<String>); 100] = [noop; 100];
    arr[0] = a01::main;
    arr
};