mod a_00_fmt_experiments;
mod a_01_primitives;
fn noop(_: Vec<String>) {}
pub const MODS: [fn(Vec<String>); 100] = {
    let mut arr: [fn(Vec<String>); 100] = [noop; 100];
    arr[0] = a_00_fmt_experiments::a00::main;
    arr[1] = a_00_fmt_experiments::a01::main;
    arr[2] = a_01_primitives::a00::main;
    arr[3] = a_01_primitives::a01::main;
    arr
};
