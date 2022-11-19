struct type_2 {
    member: f32,
    member_1: f32,
    member_2: f32,
    member_3: f32,
}

struct type_7 {
    member: type_2,
}

var<private> global: vec4<f32>;
@group(0) @binding(0) 
var<uniform> global_1: type_7;
@group(1) @binding(0) 
var global_2: texture_2d<f32>;
@group(1) @binding(1) 
var global_3: sampler;
var<private> global_4: vec4<f32>;
var<private> global_5: i32;
var<private> global_6: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);

fn function_() {
    let _e19 = global;
    let _e23 = global_1.member.member;
    let _e28 = global_1.member.member_1;
    let _e31 = textureSampleLevel(global_2, global_3, vec2<f32>((_e19.x / _e23), (_e19.y / _e28)), 0.0);
    global_4 = _e31;
    return;
}

fn function_1() {
    let _e19 = global_5;
    global_6 = vec4<f32>(fma(2.0, f32(((_e19 << bitcast<u32>(1)) & 2)), -1.0), fma(2.0, f32((_e19 & 2)), -1.0), 0.0, 1.0);
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
