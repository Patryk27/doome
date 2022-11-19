struct type_7 {
    member: f32,
    member_1: f32,
    member_2: f32,
    member_3: f32,
}

struct type_8 {
    member: type_7,
}

var<private> global: vec4<f32>;
@group(0) @binding(0) 
var<uniform> global_1: type_8;
@group(1) @binding(0) 
var global_2: texture_2d<f32>;
@group(1) @binding(1) 
var global_3: sampler;
var<private> global_4: vec4<f32>;
var<private> global_5: i32;
var<private> global_6: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);

fn function_() {
    let _e20 = global;
    let _e24 = global_1.member.member_2;
    let _e29 = global_1.member.member_3;
    let _e32 = textureSampleLevel(global_2, global_3, vec2<f32>((_e20.x / _e24), (_e20.y / _e29)), 0.0);
    global_4 = _e32;
    return;
}

fn function_1() {
    let _e20 = global_5;
    global_6 = vec4<f32>(fma(2.0, f32(((_e20 << bitcast<u32>(1)) & 2)), -1.0), fma(2.0, f32((_e20 & 2)), -1.0), 0.0, 1.0);
    return;
}

@fragment 
fn main_fs(@builtin(position) param: vec4<f32>) -> @location(0) vec4<f32> {
    global = param;
    function_();
    let _e3 = global_4;
    return _e3;
}

@vertex 
fn main_vs(@builtin(vertex_index) param_1: u32) -> @builtin(position) @invariant vec4<f32> {
    global_5 = i32(param_1);
    function_1();
    let _e5 = global_6.y;
    global_6.y = -(_e5);
    let _e7 = global_6;
    return _e7;
}
