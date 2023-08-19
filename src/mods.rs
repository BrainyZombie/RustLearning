mod a_00_fmt_experiments;
mod a_01_primitives;
mod a_02_custom_types;
mod a_03_cli_project;
fn noop(_: Vec<String>) {}
pub const MODS: [fn(Vec<String>); 100] = {
    let mut arr: [fn(Vec<String>); 100] = [noop; 100];
    arr[0] = a_00_fmt_experiments::a00::main;
    arr[1] = a_00_fmt_experiments::a01::main;
    arr[2] = a_01_primitives::a00::main;
    arr[3] = a_01_primitives::a01::main;
    arr[4] = a_02_custom_types::a00::main;
    arr[5] = a_02_custom_types::a01::main;
    arr[6] = a_03_cli_project::a00::main;
    arr
};
