struct type_7 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
}

struct type_9 {
    member: array<type_7,1365u>,
}

struct type_11 {
    member: array<vec4<f32>,4096u>,
}

struct type_13 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_14 {
    member: array<type_7,128u>,
    member_1: type_13,
}

struct type_17 {
    member: array<vec4<f32>,2047u>,
    member_1: array<vec4<f32>,192u>,
}

struct type_18 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<f32>,
}

struct type_20 {
    member: array<type_7,64u>,
    member_1: type_13,
}

struct type_21 {
    member: vec4<f32>,
    member_1: vec4<f32>,
}

struct type_23 {
    member: array<type_21,64u>,
}

struct type_27 {
    member: type_9,
}

struct type_29 {
    member: type_11,
}

struct type_31 {
    member: type_14,
}

struct type_33 {
    member: type_17,
}

struct type_35 {
    member: type_18,
}

struct type_37 {
    member: type_20,
}

struct type_39 {
    member: type_23,
}

struct type_42 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_46 {
    member: u32,
    member_1: bool,
}

struct type_47 {
    member: type_42,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
    member_3: vec2<f32>,
    member_4: f32,
    member_5: type_46,
    member_6: u32,
}

struct type_50 {
    member: vec2<f32>,
    member_1: vec2<f32>,
    member_2: vec2<f32>,
}

var<private> global: i32;
var<private> global_1: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_2: vec4<f32>;
@group(0) @binding(0) 
var<uniform> global_3: type_27;
@group(1) @binding(0) 
var<uniform> global_4: type_29;
@group(1) @binding(1) 
var<uniform> global_5: type_31;
@group(1) @binding(2) 
var<uniform> global_6: type_33;
@group(2) @binding(0) 
var<uniform> global_7: type_35;
@group(2) @binding(1) 
var<uniform> global_8: type_37;
@group(2) @binding(2) 
var<uniform> global_9: type_39;
@group(3) @binding(0) 
var global_10: texture_2d<f32>;
@group(3) @binding(1) 
var global_11: sampler;
var<private> global_12: vec4<f32>;

fn function_() {
    let _e64 = global;
    global_1 = vec4<f32>(fma(2.0, f32(((_e64 << bitcast<u32>(1)) & 2)), -1.0), fma(2.0, f32((_e64 & 2)), -1.0), 0.0, 1.0);
    return;
}

fn function_1() {
    var phi_359_: u32;
    var phi_362_: type_47;
    var phi_680_: u32;
    var phi_683_: type_47;
    var phi_713_: bool;
    var phi_718_: bool;
    var phi_10460_: bool;
    var phi_1071_: u32;
    var phi_10118_: bool;
    var phi_10128_: bool;
    var phi_10157_: bool;
    var phi_10218_: type_47;
    var phi_10221_: type_47;
    var phi_10228_: type_47;
    var phi_10235_: type_47;
    var phi_867_: type_50;
    var phi_1015_: bool;
    var phi_1025_: type_47;
    var phi_1026_: type_47;
    var phi_681_: u32;
    var phi_684_: type_47;
    var phi_1323_: type_50;
    var phi_1213_: type_50;
    var phi_1325_: type_50;
    var phi_1377_: vec3<f32>;
    var phi_1381_: bool;
    var phi_1384_: bool;
    var phi_1386_: vec3<f32>;
    var phi_1388_: bool;
    var phi_1390_: bool;
    var phi_1392_: vec3<f32>;
    var phi_1394_: bool;
    var phi_1396_: bool;
    var phi_1398_: u32;
    var phi_1400_: vec3<f32>;
    var phi_1402_: vec3<f32>;
    var phi_1404_: bool;
    var phi_3222_: bool;
    var phi_3225_: bool;
    var phi_3227_: u32;
    var phi_3229_: type_47;
    var phi_3231_: vec3<f32>;
    var phi_3233_: vec3<f32>;
    var phi_3235_: bool;
    var phi_3553_: bool;
    var phi_3556_: bool;
    var phi_3558_: u32;
    var phi_3560_: type_47;
    var phi_3562_: vec3<f32>;
    var phi_3564_: vec3<f32>;
    var phi_3566_: bool;
    var phi_3594_: bool;
    var phi_3599_: bool;
    var phi_12994_: bool;
    var phi_3952_: u32;
    var phi_12652_: bool;
    var phi_12662_: bool;
    var phi_12691_: bool;
    var phi_12752_: type_47;
    var phi_12755_: type_47;
    var phi_12762_: type_47;
    var phi_12769_: type_47;
    var phi_3748_: type_50;
    var phi_3896_: bool;
    var phi_3906_: type_47;
    var phi_3907_: type_47;
    var phi_3559_: u32;
    var phi_3561_: type_47;
    var phi_4204_: type_50;
    var phi_4094_: type_50;
    var phi_4206_: type_50;
    var phi_4258_: vec3<f32>;
    var phi_4262_: bool;
    var phi_4265_: bool;
    var phi_4267_: u32;
    var phi_4269_: vec3<f32>;
    var phi_4271_: vec3<f32>;
    var phi_4273_: bool;
    var phi_5213_: u32;
    var phi_5216_: bool;
    var phi_5218_: vec3<f32>;
    var phi_5220_: bool;
    var phi_5544_: u32;
    var phi_5547_: bool;
    var phi_5549_: bool;
    var phi_5551_: bool;
    var phi_5553_: bool;
    var phi_5581_: bool;
    var phi_5586_: bool;
    var phi_14674_: bool;
    var phi_5945_: u32;
    var phi_14347_: bool;
    var phi_14357_: bool;
    var phi_14386_: bool;
    var phi_14447_: type_47;
    var phi_14450_: type_47;
    var phi_14457_: type_47;
    var phi_14464_: type_47;
    var phi_5736_: type_50;
    var phi_5884_: bool;
    var phi_5891_: bool;
    var phi_5892_: bool;
    var phi_5893_: bool;
    var phi_5905_: u32;
    var phi_5545_: u32;
    var phi_5946_: bool;
    var phi_5947_: bool;
    var phi_5948_: bool;
    var phi_5548_: bool;
    var phi_5959_: bool;
    var phi_5960_: bool;
    var phi_13992_: bool;
    var phi_14002_: bool;
    var phi_14031_: bool;
    var phi_14092_: type_47;
    var phi_14095_: type_47;
    var phi_14102_: type_47;
    var phi_14109_: type_47;
    var phi_5479_: type_50;
    var phi_5517_: bool;
    var phi_5524_: bool;
    var phi_5525_: bool;
    var phi_5526_: bool;
    var phi_5531_: u32;
    var phi_5214_: u32;
    var phi_5217_: bool;
    var phi_5977_: bool;
    var phi_5978_: bool;
    var phi_5979_: bool;
    var phi_6005_: f32;
    var phi_5219_: vec3<f32>;
    var phi_4384_: f32;
    var phi_4390_: u32;
    var phi_4393_: bool;
    var phi_4395_: vec3<f32>;
    var phi_4397_: bool;
    var phi_4721_: u32;
    var phi_4724_: bool;
    var phi_4726_: bool;
    var phi_4728_: bool;
    var phi_4730_: bool;
    var phi_4758_: bool;
    var phi_4763_: bool;
    var phi_13887_: bool;
    var phi_5122_: u32;
    var phi_13560_: bool;
    var phi_13570_: bool;
    var phi_13599_: bool;
    var phi_13660_: type_47;
    var phi_13663_: type_47;
    var phi_13670_: type_47;
    var phi_13677_: type_47;
    var phi_4913_: type_50;
    var phi_5061_: bool;
    var phi_5068_: bool;
    var phi_5069_: bool;
    var phi_5070_: bool;
    var phi_5082_: u32;
    var phi_4722_: u32;
    var phi_5123_: bool;
    var phi_5124_: bool;
    var phi_5125_: bool;
    var phi_4725_: bool;
    var phi_5136_: bool;
    var phi_5137_: bool;
    var phi_13205_: bool;
    var phi_13215_: bool;
    var phi_13244_: bool;
    var phi_13305_: type_47;
    var phi_13308_: type_47;
    var phi_13315_: type_47;
    var phi_13322_: type_47;
    var phi_4656_: type_50;
    var phi_4694_: bool;
    var phi_4701_: bool;
    var phi_4702_: bool;
    var phi_4703_: bool;
    var phi_4708_: u32;
    var phi_4391_: u32;
    var phi_4394_: bool;
    var phi_5154_: bool;
    var phi_5155_: bool;
    var phi_5156_: bool;
    var phi_5182_: f32;
    var phi_4396_: vec3<f32>;
    var phi_6032_: bool;
    var phi_6033_: bool;
    var phi_6034_: vec3<f32>;
    var phi_4263_: bool;
    var phi_4266_: bool;
    var phi_4268_: u32;
    var phi_4270_: vec3<f32>;
    var phi_6051_: bool;
    var phi_6052_: bool;
    var phi_6053_: vec3<f32>;
    var phi_3554_: bool;
    var phi_3557_: bool;
    var phi_3563_: vec3<f32>;
    var phi_3565_: vec3<f32>;
    var phi_12282_: bool;
    var phi_12292_: bool;
    var phi_12321_: bool;
    var phi_12382_: type_47;
    var phi_12385_: type_47;
    var phi_12392_: type_47;
    var phi_12399_: type_47;
    var phi_3493_: type_50;
    var phi_3531_: bool;
    var phi_3541_: type_47;
    var phi_3542_: type_47;
    var phi_3223_: bool;
    var phi_3226_: bool;
    var phi_3228_: u32;
    var phi_3230_: type_47;
    var phi_3232_: vec3<f32>;
    var phi_3234_: vec3<f32>;
    var phi_6087_: bool;
    var phi_6088_: bool;
    var phi_6089_: vec3<f32>;
    var phi_6090_: vec3<f32>;
    var phi_6131_: type_7;
    var phi_6153_: bool;
    var phi_6156_: bool;
    var phi_6158_: u32;
    var phi_6160_: type_47;
    var phi_6162_: vec3<f32>;
    var phi_6164_: vec3<f32>;
    var phi_6166_: bool;
    var phi_6484_: bool;
    var phi_6487_: bool;
    var phi_6489_: u32;
    var phi_6491_: type_47;
    var phi_6493_: vec3<f32>;
    var phi_6495_: vec3<f32>;
    var phi_6497_: bool;
    var phi_6525_: bool;
    var phi_6530_: bool;
    var phi_15544_: bool;
    var phi_6883_: u32;
    var phi_15202_: bool;
    var phi_15212_: bool;
    var phi_15241_: bool;
    var phi_15302_: type_47;
    var phi_15305_: type_47;
    var phi_15312_: type_47;
    var phi_15319_: type_47;
    var phi_6679_: type_50;
    var phi_6827_: bool;
    var phi_6837_: type_47;
    var phi_6838_: type_47;
    var phi_6490_: u32;
    var phi_6492_: type_47;
    var phi_7135_: type_50;
    var phi_7025_: type_50;
    var phi_7137_: type_50;
    var phi_7189_: vec3<f32>;
    var phi_7193_: bool;
    var phi_7196_: bool;
    var phi_7198_: u32;
    var phi_7200_: vec3<f32>;
    var phi_7202_: vec3<f32>;
    var phi_7204_: bool;
    var phi_8144_: u32;
    var phi_8147_: bool;
    var phi_8149_: vec3<f32>;
    var phi_8151_: bool;
    var phi_8475_: u32;
    var phi_8478_: bool;
    var phi_8480_: bool;
    var phi_8482_: bool;
    var phi_8484_: bool;
    var phi_8512_: bool;
    var phi_8517_: bool;
    var phi_17224_: bool;
    var phi_8876_: u32;
    var phi_16897_: bool;
    var phi_16907_: bool;
    var phi_16936_: bool;
    var phi_16997_: type_47;
    var phi_17000_: type_47;
    var phi_17007_: type_47;
    var phi_17014_: type_47;
    var phi_8667_: type_50;
    var phi_8815_: bool;
    var phi_8822_: bool;
    var phi_8823_: bool;
    var phi_8824_: bool;
    var phi_8836_: u32;
    var phi_8476_: u32;
    var phi_8877_: bool;
    var phi_8878_: bool;
    var phi_8879_: bool;
    var phi_8479_: bool;
    var phi_8890_: bool;
    var phi_8891_: bool;
    var phi_16542_: bool;
    var phi_16552_: bool;
    var phi_16581_: bool;
    var phi_16642_: type_47;
    var phi_16645_: type_47;
    var phi_16652_: type_47;
    var phi_16659_: type_47;
    var phi_8410_: type_50;
    var phi_8448_: bool;
    var phi_8455_: bool;
    var phi_8456_: bool;
    var phi_8457_: bool;
    var phi_8462_: u32;
    var phi_8145_: u32;
    var phi_8148_: bool;
    var phi_8908_: bool;
    var phi_8909_: bool;
    var phi_8910_: bool;
    var phi_8936_: f32;
    var phi_8150_: vec3<f32>;
    var phi_7315_: f32;
    var phi_7321_: u32;
    var phi_7324_: bool;
    var phi_7326_: vec3<f32>;
    var phi_7328_: bool;
    var phi_7652_: u32;
    var phi_7655_: bool;
    var phi_7657_: bool;
    var phi_7659_: bool;
    var phi_7661_: bool;
    var phi_7689_: bool;
    var phi_7694_: bool;
    var phi_16437_: bool;
    var phi_8053_: u32;
    var phi_16110_: bool;
    var phi_16120_: bool;
    var phi_16149_: bool;
    var phi_16210_: type_47;
    var phi_16213_: type_47;
    var phi_16220_: type_47;
    var phi_16227_: type_47;
    var phi_7844_: type_50;
    var phi_7992_: bool;
    var phi_7999_: bool;
    var phi_8000_: bool;
    var phi_8001_: bool;
    var phi_8013_: u32;
    var phi_7653_: u32;
    var phi_8054_: bool;
    var phi_8055_: bool;
    var phi_8056_: bool;
    var phi_7656_: bool;
    var phi_8067_: bool;
    var phi_8068_: bool;
    var phi_15755_: bool;
    var phi_15765_: bool;
    var phi_15794_: bool;
    var phi_15855_: type_47;
    var phi_15858_: type_47;
    var phi_15865_: type_47;
    var phi_15872_: type_47;
    var phi_7587_: type_50;
    var phi_7625_: bool;
    var phi_7632_: bool;
    var phi_7633_: bool;
    var phi_7634_: bool;
    var phi_7639_: u32;
    var phi_7322_: u32;
    var phi_7325_: bool;
    var phi_8085_: bool;
    var phi_8086_: bool;
    var phi_8087_: bool;
    var phi_8113_: f32;
    var phi_7327_: vec3<f32>;
    var phi_8963_: bool;
    var phi_8964_: bool;
    var phi_8965_: vec3<f32>;
    var phi_7194_: bool;
    var phi_7197_: bool;
    var phi_7199_: u32;
    var phi_7201_: vec3<f32>;
    var phi_8982_: bool;
    var phi_8983_: bool;
    var phi_8984_: vec3<f32>;
    var phi_6485_: bool;
    var phi_6488_: bool;
    var phi_6494_: vec3<f32>;
    var phi_6496_: vec3<f32>;
    var phi_14832_: bool;
    var phi_14842_: bool;
    var phi_14871_: bool;
    var phi_14932_: type_47;
    var phi_14935_: type_47;
    var phi_14942_: type_47;
    var phi_14949_: type_47;
    var phi_6424_: type_50;
    var phi_6462_: bool;
    var phi_6472_: type_47;
    var phi_6473_: type_47;
    var phi_6154_: bool;
    var phi_6157_: bool;
    var phi_6159_: u32;
    var phi_6161_: type_47;
    var phi_6163_: vec3<f32>;
    var phi_6165_: vec3<f32>;
    var phi_9019_: bool;
    var phi_9020_: bool;
    var phi_9021_: vec3<f32>;
    var phi_9022_: vec3<f32>;
    var phi_2344_: u32;
    var phi_2347_: bool;
    var phi_2349_: vec3<f32>;
    var phi_2351_: bool;
    var phi_2675_: u32;
    var phi_2678_: bool;
    var phi_2680_: bool;
    var phi_2682_: bool;
    var phi_2684_: bool;
    var phi_2712_: bool;
    var phi_2717_: bool;
    var phi_12140_: bool;
    var phi_3076_: u32;
    var phi_11813_: bool;
    var phi_11823_: bool;
    var phi_11852_: bool;
    var phi_11913_: type_47;
    var phi_11916_: type_47;
    var phi_11923_: type_47;
    var phi_11930_: type_47;
    var phi_2867_: type_50;
    var phi_3015_: bool;
    var phi_3022_: bool;
    var phi_3023_: bool;
    var phi_3024_: bool;
    var phi_3036_: u32;
    var phi_2676_: u32;
    var phi_3077_: bool;
    var phi_3078_: bool;
    var phi_3079_: bool;
    var phi_2679_: bool;
    var phi_3090_: bool;
    var phi_3091_: bool;
    var phi_11458_: bool;
    var phi_11468_: bool;
    var phi_11497_: bool;
    var phi_11558_: type_47;
    var phi_11561_: type_47;
    var phi_11568_: type_47;
    var phi_11575_: type_47;
    var phi_2610_: type_50;
    var phi_2648_: bool;
    var phi_2655_: bool;
    var phi_2656_: bool;
    var phi_2657_: bool;
    var phi_2662_: u32;
    var phi_2345_: u32;
    var phi_2348_: bool;
    var phi_3108_: bool;
    var phi_3109_: bool;
    var phi_3110_: bool;
    var phi_3136_: f32;
    var phi_2350_: vec3<f32>;
    var phi_1515_: f32;
    var phi_1521_: u32;
    var phi_1524_: bool;
    var phi_1526_: vec3<f32>;
    var phi_1528_: bool;
    var phi_1852_: u32;
    var phi_1855_: bool;
    var phi_1857_: bool;
    var phi_1859_: bool;
    var phi_1861_: bool;
    var phi_1889_: bool;
    var phi_1894_: bool;
    var phi_11353_: bool;
    var phi_2253_: u32;
    var phi_11026_: bool;
    var phi_11036_: bool;
    var phi_11065_: bool;
    var phi_11126_: type_47;
    var phi_11129_: type_47;
    var phi_11136_: type_47;
    var phi_11143_: type_47;
    var phi_2044_: type_50;
    var phi_2192_: bool;
    var phi_2199_: bool;
    var phi_2200_: bool;
    var phi_2201_: bool;
    var phi_2213_: u32;
    var phi_1853_: u32;
    var phi_2254_: bool;
    var phi_2255_: bool;
    var phi_2256_: bool;
    var phi_1856_: bool;
    var phi_2267_: bool;
    var phi_2268_: bool;
    var phi_10671_: bool;
    var phi_10681_: bool;
    var phi_10710_: bool;
    var phi_10771_: type_47;
    var phi_10774_: type_47;
    var phi_10781_: type_47;
    var phi_10788_: type_47;
    var phi_1787_: type_50;
    var phi_1825_: bool;
    var phi_1832_: bool;
    var phi_1833_: bool;
    var phi_1834_: bool;
    var phi_1839_: u32;
    var phi_1522_: u32;
    var phi_1525_: bool;
    var phi_2285_: bool;
    var phi_2286_: bool;
    var phi_2287_: bool;
    var phi_2313_: f32;
    var phi_1527_: vec3<f32>;
    var phi_3163_: bool;
    var phi_3164_: bool;
    var phi_3165_: vec3<f32>;
    var phi_1382_: bool;
    var phi_1385_: bool;
    var phi_1387_: vec3<f32>;
    var phi_1389_: bool;
    var phi_1391_: bool;
    var phi_1393_: vec3<f32>;
    var phi_1395_: bool;
    var phi_1397_: bool;
    var phi_1399_: u32;
    var phi_1401_: vec3<f32>;
    var phi_1403_: vec3<f32>;
    var phi_9031_: vec3<f32>;
    var phi_17890_: bool;
    var phi_9747_: bool;
    var phi_9757_: bool;
    var phi_9786_: bool;
    var phi_9847_: type_47;
    var phi_9850_: type_47;
    var phi_9857_: type_47;
    var phi_9864_: type_47;
    var phi_622_: type_50;
    var phi_660_: bool;
    var phi_670_: type_47;
    var phi_363_: type_47;
    var phi_17913_: bool;
    var local: u32;
    var local_1: type_47;
    var local_2: u32;
    var local_3: type_47;
    var local_4: type_50;
    var local_5: type_50;
    var local_6: u32;
    var local_7: type_47;
    var local_8: type_50;
    var local_9: type_50;
    var local_10: type_50;
    var local_11: type_50;
    var local_12: bool;
    var local_13: bool;
    var local_14: vec3<f32>;
    var local_15: bool;
    var local_16: bool;
    var local_17: vec3<f32>;
    var local_18: bool;
    var local_19: bool;
    var local_20: u32;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var local_23: bool;
    var local_24: bool;
    var local_25: bool;
    var local_26: u32;
    var local_27: type_47;
    var local_28: vec3<f32>;
    var local_29: vec3<f32>;
    var local_30: bool;
    var local_31: bool;
    var local_32: bool;
    var local_33: u32;
    var local_34: type_47;
    var local_35: vec3<f32>;
    var local_36: vec3<f32>;
    var local_37: bool;
    var local_38: type_50;
    var local_39: type_50;
    var local_40: u32;
    var local_41: type_47;
    var local_42: type_50;
    var local_43: type_50;
    var local_44: type_50;
    var local_45: type_50;
    var local_46: bool;
    var local_47: bool;
    var local_48: u32;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: bool;
    var local_52: u32;
    var local_53: bool;
    var local_54: vec3<f32>;
    var local_55: bool;
    var local_56: u32;
    var local_57: bool;
    var local_58: bool;
    var local_59: bool;
    var local_60: bool;
    var local_61: type_50;
    var local_62: type_50;
    var local_63: u32;
    var local_64: bool;
    var local_65: bool;
    var local_66: bool;
    var local_67: type_50;
    var local_68: type_50;
    var local_69: u32;
    var local_70: bool;
    var local_71: bool;
    var local_72: bool;
    var local_73: bool;
    var local_74: bool;
    var local_75: bool;
    var local_76: u32;
    var local_77: bool;
    var local_78: vec3<f32>;
    var local_79: bool;
    var local_80: u32;
    var local_81: bool;
    var local_82: bool;
    var local_83: bool;
    var local_84: bool;
    var local_85: type_50;
    var local_86: type_50;
    var local_87: u32;
    var local_88: bool;
    var local_89: bool;
    var local_90: bool;
    var local_91: type_50;
    var local_92: type_50;
    var local_93: u32;
    var local_94: bool;
    var local_95: bool;
    var local_96: bool;
    var local_97: bool;
    var local_98: bool;
    var local_99: bool;
    var local_100: bool;
    var local_101: bool;
    var local_102: vec3<f32>;
    var local_103: vec3<f32>;
    var local_104: bool;
    var local_105: bool;
    var local_106: u32;
    var local_107: vec3<f32>;
    var local_108: bool;
    var local_109: bool;
    var local_110: vec3<f32>;
    var local_111: type_50;
    var local_112: type_50;
    var local_113: bool;
    var local_114: bool;
    var local_115: u32;
    var local_116: type_47;
    var local_117: type_47;
    var local_118: vec3<f32>;
    var local_119: vec3<f32>;
    var local_120: bool;
    var local_121: bool;
    var local_122: vec3<f32>;
    var local_123: vec3<f32>;
    var local_124: bool;
    var local_125: bool;
    var local_126: u32;
    var local_127: type_47;
    var local_128: vec3<f32>;
    var local_129: vec3<f32>;
    var local_130: bool;
    var local_131: bool;
    var local_132: bool;
    var local_133: u32;
    var local_134: type_47;
    var local_135: vec3<f32>;
    var local_136: vec3<f32>;
    var local_137: bool;
    var local_138: type_50;
    var local_139: type_50;
    var local_140: u32;
    var local_141: type_47;
    var local_142: type_50;
    var local_143: type_50;
    var local_144: type_50;
    var local_145: type_50;
    var local_146: bool;
    var local_147: bool;
    var local_148: u32;
    var local_149: vec3<f32>;
    var local_150: vec3<f32>;
    var local_151: bool;
    var local_152: u32;
    var local_153: bool;
    var local_154: vec3<f32>;
    var local_155: bool;
    var local_156: u32;
    var local_157: bool;
    var local_158: bool;
    var local_159: bool;
    var local_160: bool;
    var local_161: type_50;
    var local_162: type_50;
    var local_163: u32;
    var local_164: bool;
    var local_165: bool;
    var local_166: bool;
    var local_167: type_50;
    var local_168: type_50;
    var local_169: u32;
    var local_170: bool;
    var local_171: bool;
    var local_172: bool;
    var local_173: bool;
    var local_174: bool;
    var local_175: bool;
    var local_176: u32;
    var local_177: bool;
    var local_178: vec3<f32>;
    var local_179: bool;
    var local_180: u32;
    var local_181: bool;
    var local_182: bool;
    var local_183: bool;
    var local_184: bool;
    var local_185: type_50;
    var local_186: type_50;
    var local_187: u32;
    var local_188: bool;
    var local_189: bool;
    var local_190: bool;
    var local_191: type_50;
    var local_192: type_50;
    var local_193: u32;
    var local_194: bool;
    var local_195: bool;
    var local_196: bool;
    var local_197: bool;
    var local_198: bool;
    var local_199: bool;
    var local_200: bool;
    var local_201: bool;
    var local_202: vec3<f32>;
    var local_203: vec3<f32>;
    var local_204: bool;
    var local_205: bool;
    var local_206: u32;
    var local_207: vec3<f32>;
    var local_208: bool;
    var local_209: bool;
    var local_210: vec3<f32>;
    var local_211: type_50;
    var local_212: type_50;
    var local_213: bool;
    var local_214: bool;
    var local_215: u32;
    var local_216: type_47;
    var local_217: type_47;
    var local_218: vec3<f32>;
    var local_219: vec3<f32>;
    var local_220: bool;
    var local_221: bool;
    var local_222: vec3<f32>;
    var local_223: vec3<f32>;
    var local_224: u32;
    var local_225: bool;
    var local_226: vec3<f32>;
    var local_227: bool;
    var local_228: u32;
    var local_229: bool;
    var local_230: bool;
    var local_231: bool;
    var local_232: bool;
    var local_233: type_50;
    var local_234: type_50;
    var local_235: u32;
    var local_236: bool;
    var local_237: bool;
    var local_238: bool;
    var local_239: type_50;
    var local_240: type_50;
    var local_241: u32;
    var local_242: bool;
    var local_243: bool;
    var local_244: bool;
    var local_245: bool;
    var local_246: bool;
    var local_247: bool;
    var local_248: u32;
    var local_249: bool;
    var local_250: vec3<f32>;
    var local_251: bool;
    var local_252: u32;
    var local_253: bool;
    var local_254: bool;
    var local_255: bool;
    var local_256: bool;
    var local_257: type_50;
    var local_258: type_50;
    var local_259: u32;
    var local_260: bool;
    var local_261: bool;
    var local_262: bool;
    var local_263: type_50;
    var local_264: type_50;
    var local_265: u32;
    var local_266: bool;
    var local_267: bool;
    var local_268: bool;
    var local_269: bool;
    var local_270: bool;
    var local_271: bool;
    var local_272: bool;
    var local_273: bool;
    var local_274: vec3<f32>;
    var local_275: vec3<f32>;
    var local_276: bool;
    var local_277: bool;
    var local_278: u32;
    var local_279: vec3<f32>;
    var local_280: vec3<f32>;
    var local_281: type_50;
    var local_282: type_50;

    switch bitcast<i32>(0u) {
        default: {
            let _e65 = global_2;
            let _e70 = global_7.member.member;
            let _e74 = vec3<f32>(_e70.x, _e70.y, _e70.z);
            let _e80 = global_7.member.member_3[1u];
            let _e84 = global_7.member.member_3[0u];
            let _e89 = global_7.member.member_3[2u];
            let _e90 = global_7.member.member_3;
            let _e99 = tan((_e89 * 0.5));
            let _e100 = ((fma(2.0, (_e65.x / _e90.x), -1.0) / (_e80 / _e84)) * _e99);
            let _e101 = (fma(2.0, (_e65.y / _e90.y), -1.0) * _e99);
            let _e104 = global_7.member.member_4;
            let _e107 = global_7.member.member_5;
            let _e110 = global_7.member.member_6;
            let _e114 = global_7.member.member[3u];
            let _e115 = -(_e114);
            let _e131 = fma(_e110.x, _e115, fma(_e104.x, _e100, (_e107.x * _e101)));
            let _e132 = fma(_e110.y, _e115, fma(_e104.y, _e100, (_e107.y * _e101)));
            let _e133 = fma(_e110.z, _e115, fma(_e104.z, _e100, (_e107.z * _e101)));
            let _e140 = (vec3<f32>(_e131, _e132, _e133) * (1.0 / sqrt(fma(_e133, _e133, fma(_e131, _e131, (_e132 * _e132))))));
            let _e141 = type_42(_e74, _e140);
            phi_359_ = 0u;
            phi_362_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
            loop {
                let _e143 = phi_359_;
                let _e145 = phi_362_;
                let _e149 = global_5.member.member_1.member;
                if (_e143 < _e149) {
                    if (_e143 < 128u) {
                        let _e6200 = global_5.member.member[_e143];
                        let _e6206 = (_e6200.member_1.x - _e6200.member.x);
                        let _e6209 = (_e6200.member_1.y - _e6200.member.y);
                        let _e6212 = (_e6200.member_1.z - _e6200.member.z);
                        let _e6214 = (_e6200.member_2.x - _e6200.member.x);
                        let _e6216 = (_e6200.member_2.y - _e6200.member.y);
                        let _e6218 = (_e6200.member_2.z - _e6200.member.z);
                        let _e6223 = fma(_e140.y, _e6218, -((_e6216 * _e140.z)));
                        let _e6227 = fma(_e140.z, _e6214, -((_e6218 * _e140.x)));
                        let _e6230 = fma(_e140.x, _e6216, -((_e6214 * _e140.y)));
                        let _e6233 = fma(_e6212, _e6230, fma(_e6206, _e6223, (_e6209 * _e6227)));
                        if (abs(_e6233) < 1.1920928955078125e-7) {
                            phi_9864_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                        } else {
                            let _e6236 = (1.0 / _e6233);
                            let _e6237 = (_e70.x - _e6200.member.x);
                            let _e6238 = (_e70.y - _e6200.member.y);
                            let _e6239 = (_e70.z - _e6200.member.z);
                            let _e6243 = (fma(_e6239, _e6230, fma(_e6237, _e6223, (_e6238 * _e6227))) * _e6236);
                            if (_e6243 > -9.999999747378752e-5) {
                                phi_9747_ = (_e6243 < 0.0);
                            } else {
                                phi_9747_ = false;
                            }
                            let _e6247 = phi_9747_;
                            let _e6248 = select(_e6243, 1.1920928955078125e-7, _e6247);
                            if (_e6248 < 0.0) {
                                phi_9757_ = true;
                            } else {
                                phi_9757_ = (_e6248 > 1.0);
                            }
                            let _e6252 = phi_9757_;
                            if _e6252 {
                                phi_9857_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                            } else {
                                let _e6255 = fma(_e6238, _e6212, -((_e6209 * _e6239)));
                                let _e6258 = fma(_e6239, _e6206, -((_e6212 * _e6237)));
                                let _e6261 = fma(_e6237, _e6209, -((_e6206 * _e6238)));
                                let _e6264 = fma(_e140.z, _e6261, fma(_e140.x, _e6255, (_e140.y * _e6258)));
                                let _e6265 = (_e6264 * _e6236);
                                if (_e6265 < 0.0) {
                                    phi_9786_ = true;
                                } else {
                                    phi_9786_ = (fma(_e6264, _e6236, _e6248) > 1.0);
                                }
                                let _e6270 = phi_9786_;
                                if _e6270 {
                                    phi_9850_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                } else {
                                    let _e6273 = fma(_e6218, _e6261, fma(_e6214, _e6255, (_e6216 * _e6258)));
                                    let _e6274 = (_e6273 * _e6236);
                                    if (_e6274 < 0.0) {
                                        phi_9847_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                    } else {
                                        let _e6277 = fma(_e6273, _e6236, -0.009999999776482582);
                                        let _e6285 = fma(_e6209, _e6218, -((_e6216 * _e6212)));
                                        let _e6288 = fma(_e6212, _e6214, -((_e6218 * _e6206)));
                                        let _e6291 = fma(_e6206, _e6216, -((_e6214 * _e6209)));
                                        let _e6296 = (1.0 / sqrt(fma(_e6291, _e6291, fma(_e6285, _e6285, (_e6288 * _e6288)))));
                                        phi_9847_ = type_47(_e141, (_e74 + vec3<f32>((_e140.x * _e6277), (_e140.y * _e6277), (_e140.z * _e6277))), vec3<f32>((_e6285 * _e6296), (_e6288 * _e6296), (_e6291 * _e6296)), vec2<f32>(_e6248, _e6265), _e6274, type_46(0u, false), select(select(u32(_e6200.member.w), 0u, (_e6200.member.w < 0.0)), 4294967295u, (_e6200.member.w > 4294967040.0)));
                                    }
                                    let _e6309 = phi_9847_;
                                    phi_9850_ = _e6309;
                                }
                                let _e6311 = phi_9850_;
                                phi_9857_ = _e6311;
                            }
                            let _e6313 = phi_9857_;
                            phi_9864_ = _e6313;
                        }
                        let _e6315 = phi_9864_;
                        if (_e6315.member_4 < _e145.member_4) {
                            if (_e6200.member_2.w == 1.0) {
                                if ((_e143 % 2u) == 0u) {
                                    let _e6350 = (3u * (_e143 / 2u));
                                    let _e6351 = (_e6350 < 192u);
                                    if _e6351 {
                                        let _e6355 = global_6.member.member_1[_e6350];
                                        if _e6351 {
                                            let _e6362 = (_e6350 + 1u);
                                            if (_e6362 < 192u) {
                                            } else {
                                                phi_17913_ = false;
                                                break;
                                            }
                                            let _e6367 = global_6.member.member_1[_e6362];
                                            local_281 = type_50(vec2<f32>(_e6355.x, _e6355.y), vec2<f32>(_e6355.z, _e6355.w), vec2<f32>(_e6367.x, _e6367.y));
                                        } else {
                                            phi_17913_ = false;
                                            break;
                                        }
                                    } else {
                                        phi_17913_ = false;
                                        break;
                                    }
                                    let _e7513 = local_281;
                                    phi_622_ = _e7513;
                                } else {
                                    let _e6326 = (3u * ((_e143 - 1u) / 2u));
                                    let _e6327 = (_e6326 + 1u);
                                    if (_e6327 < 192u) {
                                        let _e6332 = global_6.member.member_1[_e6327];
                                        let _e6336 = (_e6326 + 2u);
                                        let _e6337 = (_e6336 < 192u);
                                        if _e6337 {
                                            let _e6341 = global_6.member.member_1[_e6336];
                                            if _e6337 {
                                            } else {
                                                phi_17913_ = false;
                                                break;
                                            }
                                            local_282 = type_50(vec2<f32>(_e6332.z, _e6332.w), vec2<f32>(_e6341.x, _e6341.y), vec2<f32>(_e6341.z, _e6341.w));
                                        } else {
                                            phi_17913_ = false;
                                            break;
                                        }
                                    } else {
                                        phi_17913_ = false;
                                        break;
                                    }
                                    let _e7515 = local_282;
                                    phi_622_ = _e7515;
                                }
                                let _e6373 = phi_622_;
                                let _e6394 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e6373.member_2.x - _e6373.member.x), _e145.member_3.y, fma((_e6373.member_1.x - _e6373.member.x), _e145.member_3.x, _e6373.member.x)), fma((_e6373.member_2.y - _e6373.member.y), _e145.member_3.y, fma((_e6373.member_1.y - _e6373.member.y), _e145.member_3.x, _e6373.member.y))), 0.0);
                                phi_660_ = (_e6394.w == 1.0);
                            } else {
                                phi_660_ = true;
                            }
                            let _e6398 = phi_660_;
                            if _e6398 {
                                _ = _e6315.member_5;
                                phi_670_ = type_47(_e6315.member, _e6315.member_1, _e6315.member_2, _e6315.member_3, _e6315.member_4, type_46(_e143, true), _e6315.member_6);
                            } else {
                                phi_670_ = _e145;
                            }
                            let _e6409 = phi_670_;
                            phi_363_ = _e6409;
                        } else {
                            phi_363_ = _e145;
                        }
                        let _e6411 = phi_363_;
                        local = (_e143 + 1u);
                        local_1 = _e6411;
                    } else {
                        phi_17913_ = false;
                        break;
                    }
                } else {
                    phi_680_ = 0u;
                    phi_683_ = _e145;
                    loop {
                        let _e152 = phi_680_;
                        let _e154 = phi_683_;
                        if (_e152 < 4096u) {
                            let _e159 = global_4.member.member[_e152];
                            let _e160 = (_e152 + 1u);
                            if (_e160 < 4096u) {
                                let _e165 = global_4.member.member[_e160];
                                if (_e159.x == _e165.x) {
                                    phi_713_ = (_e159.y == _e165.y);
                                } else {
                                    phi_713_ = false;
                                }
                                let _e175 = phi_713_;
                                if _e175 {
                                    phi_718_ = (_e159.z == _e165.z);
                                } else {
                                    phi_718_ = false;
                                }
                                let _e178 = phi_718_;
                                if _e178 {
                                    let _e230 = select(select(u32(_e159.w), 0u, (_e159.w < 0.0)), 4294967295u, (_e159.w > 4294967040.0));
                                    if (_e230 < 1365u) {
                                        let _e235 = global_3.member.member[_e230];
                                        let _e241 = (_e235.member_1.x - _e235.member.x);
                                        let _e244 = (_e235.member_1.y - _e235.member.y);
                                        let _e247 = (_e235.member_1.z - _e235.member.z);
                                        let _e249 = (_e235.member_2.x - _e235.member.x);
                                        let _e251 = (_e235.member_2.y - _e235.member.y);
                                        let _e253 = (_e235.member_2.z - _e235.member.z);
                                        let _e258 = fma(_e140.y, _e253, -((_e251 * _e140.z)));
                                        let _e262 = fma(_e140.z, _e249, -((_e253 * _e140.x)));
                                        let _e265 = fma(_e140.x, _e251, -((_e249 * _e140.y)));
                                        let _e268 = fma(_e247, _e265, fma(_e241, _e258, (_e244 * _e262)));
                                        if (abs(_e268) < 1.1920928955078125e-7) {
                                            phi_10235_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                        } else {
                                            let _e271 = (1.0 / _e268);
                                            let _e272 = (_e70.x - _e235.member.x);
                                            let _e273 = (_e70.y - _e235.member.y);
                                            let _e274 = (_e70.z - _e235.member.z);
                                            let _e278 = (fma(_e274, _e265, fma(_e272, _e258, (_e273 * _e262))) * _e271);
                                            if (_e278 > -9.999999747378752e-5) {
                                                phi_10118_ = (_e278 < 0.0);
                                            } else {
                                                phi_10118_ = false;
                                            }
                                            let _e282 = phi_10118_;
                                            let _e283 = select(_e278, 1.1920928955078125e-7, _e282);
                                            if (_e283 < 0.0) {
                                                phi_10128_ = true;
                                            } else {
                                                phi_10128_ = (_e283 > 1.0);
                                            }
                                            let _e287 = phi_10128_;
                                            if _e287 {
                                                phi_10228_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                            } else {
                                                let _e290 = fma(_e273, _e247, -((_e244 * _e274)));
                                                let _e293 = fma(_e274, _e241, -((_e247 * _e272)));
                                                let _e296 = fma(_e272, _e244, -((_e241 * _e273)));
                                                let _e299 = fma(_e140.z, _e296, fma(_e140.x, _e290, (_e140.y * _e293)));
                                                let _e300 = (_e299 * _e271);
                                                if (_e300 < 0.0) {
                                                    phi_10157_ = true;
                                                } else {
                                                    phi_10157_ = (fma(_e299, _e271, _e283) > 1.0);
                                                }
                                                let _e305 = phi_10157_;
                                                if _e305 {
                                                    phi_10221_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                } else {
                                                    let _e308 = fma(_e253, _e296, fma(_e249, _e290, (_e251 * _e293)));
                                                    let _e309 = (_e308 * _e271);
                                                    if (_e309 < 0.0) {
                                                        phi_10218_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                    } else {
                                                        let _e312 = fma(_e308, _e271, -0.009999999776482582);
                                                        let _e320 = fma(_e244, _e253, -((_e251 * _e247)));
                                                        let _e323 = fma(_e247, _e249, -((_e253 * _e241)));
                                                        let _e326 = fma(_e241, _e251, -((_e249 * _e244)));
                                                        let _e331 = (1.0 / sqrt(fma(_e326, _e326, fma(_e320, _e320, (_e323 * _e323)))));
                                                        phi_10218_ = type_47(_e141, (_e74 + vec3<f32>((_e140.x * _e312), (_e140.y * _e312), (_e140.z * _e312))), vec3<f32>((_e320 * _e331), (_e323 * _e331), (_e326 * _e331)), vec2<f32>(_e283, _e300), _e309, type_46(0u, false), select(select(u32(_e235.member.w), 0u, (_e235.member.w < 0.0)), 4294967295u, (_e235.member.w > 4294967040.0)));
                                                    }
                                                    let _e344 = phi_10218_;
                                                    phi_10221_ = _e344;
                                                }
                                                let _e346 = phi_10221_;
                                                phi_10228_ = _e346;
                                            }
                                            let _e348 = phi_10228_;
                                            phi_10235_ = _e348;
                                        }
                                        let _e350 = phi_10235_;
                                        if (_e350.member_4 < _e154.member_4) {
                                            if (_e235.member_2.w == 1.0) {
                                                if ((_e230 % 2u) == 0u) {
                                                    let _e385 = (3u * (_e230 / 2u));
                                                    let _e386 = (_e385 < 2047u);
                                                    if _e386 {
                                                        let _e390 = global_6.member.member[_e385];
                                                        if _e386 {
                                                            let _e397 = (_e385 + 1u);
                                                            if (_e397 < 2047u) {
                                                            } else {
                                                                phi_17890_ = false;
                                                                break;
                                                            }
                                                            let _e402 = global_6.member.member[_e397];
                                                            local_4 = type_50(vec2<f32>(_e390.x, _e390.y), vec2<f32>(_e390.z, _e390.w), vec2<f32>(_e402.x, _e402.y));
                                                        } else {
                                                            phi_17890_ = false;
                                                            break;
                                                        }
                                                    } else {
                                                        phi_17890_ = false;
                                                        break;
                                                    }
                                                    let _e6440 = local_4;
                                                    phi_867_ = _e6440;
                                                } else {
                                                    let _e361 = (3u * ((_e230 - 1u) / 2u));
                                                    let _e362 = (_e361 + 1u);
                                                    if (_e362 < 2047u) {
                                                        let _e367 = global_6.member.member[_e362];
                                                        let _e371 = (_e361 + 2u);
                                                        let _e372 = (_e371 < 2047u);
                                                        if _e372 {
                                                            let _e376 = global_6.member.member[_e371];
                                                            if _e372 {
                                                            } else {
                                                                phi_17890_ = false;
                                                                break;
                                                            }
                                                            local_5 = type_50(vec2<f32>(_e367.z, _e367.w), vec2<f32>(_e376.x, _e376.y), vec2<f32>(_e376.z, _e376.w));
                                                        } else {
                                                            phi_17890_ = false;
                                                            break;
                                                        }
                                                    } else {
                                                        phi_17890_ = false;
                                                        break;
                                                    }
                                                    let _e6442 = local_5;
                                                    phi_867_ = _e6442;
                                                }
                                                let _e408 = phi_867_;
                                                let _e429 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e408.member_2.x - _e408.member.x), _e154.member_3.y, fma((_e408.member_1.x - _e408.member.x), _e154.member_3.x, _e408.member.x)), fma((_e408.member_2.y - _e408.member.y), _e154.member_3.y, fma((_e408.member_1.y - _e408.member.y), _e154.member_3.x, _e408.member.y))), 0.0);
                                                phi_1015_ = (_e429.w == 1.0);
                                            } else {
                                                phi_1015_ = true;
                                            }
                                            let _e433 = phi_1015_;
                                            if _e433 {
                                                _ = _e350.member_5;
                                                phi_1025_ = type_47(_e350.member, _e350.member_1, _e350.member_2, _e350.member_3, _e350.member_4, type_46(_e230, false), _e350.member_6);
                                            } else {
                                                phi_1025_ = _e154;
                                            }
                                            let _e444 = phi_1025_;
                                            phi_1026_ = _e444;
                                        } else {
                                            phi_1026_ = _e154;
                                        }
                                        let _e446 = phi_1026_;
                                        local_6 = select(select(u32(_e165.w), 0u, (_e165.w < 0.0)), 4294967295u, (_e165.w > 4294967040.0));
                                        local_7 = _e446;
                                    } else {
                                        phi_17890_ = false;
                                        break;
                                    }
                                    let _e6448 = local_6;
                                    phi_681_ = _e6448;
                                    let _e6451 = local_7;
                                    phi_684_ = _e6451;
                                } else {
                                    let _e183 = ((_e159.x - _e70.x) / _e140.x);
                                    let _e185 = ((_e159.y - _e70.y) / _e140.y);
                                    let _e187 = ((_e159.z - _e70.z) / _e140.z);
                                    let _e191 = ((_e165.x - _e70.x) / _e140.x);
                                    let _e192 = ((_e165.y - _e70.y) / _e140.y);
                                    let _e193 = ((_e165.z - _e70.z) / _e140.z);
                                    let _e201 = max(max(min(_e183, _e191), min(_e185, _e192)), min(_e187, _e193));
                                    let _e203 = min(min(max(_e183, _e191), max(_e185, _e192)), max(_e187, _e193));
                                    if (_e201 <= _e203) {
                                        phi_10460_ = (_e203 > 0.0);
                                    } else {
                                        phi_10460_ = false;
                                    }
                                    let _e207 = phi_10460_;
                                    if (select(3.4028234663852886e38, _e201, _e207) < _e154.member_4) {
                                        phi_1071_ = select(select(u32(_e159.w), 0u, (_e159.w < 0.0)), 4294967295u, (_e159.w > 4294967040.0));
                                    } else {
                                        phi_1071_ = select(select(u32(_e165.w), 0u, (_e165.w < 0.0)), 4294967295u, (_e165.w > 4294967040.0));
                                    }
                                    let _e224 = phi_1071_;
                                    phi_681_ = _e224;
                                    phi_684_ = _e154;
                                }
                                let _e454 = phi_681_;
                                let _e456 = phi_684_;
                                local_2 = _e454;
                                local_3 = _e456;
                                if (_e454 == 0u) {
                                    if (_e456.member_4 < 1000.0) {
                                        if (_e456.member_6 < 64u) {
                                            let _e465 = global_9.member.member[_e456.member_6];
                                            if (_e465.member.w == 1.0) {
                                                switch select(0, 1, _e456.member_5.member_1) {
                                                    case 0: {
                                                        if ((_e456.member_5.member % 2u) == 0u) {
                                                            let _e560 = (3u * (_e456.member_5.member / 2u));
                                                            let _e561 = (_e560 < 2047u);
                                                            if _e561 {
                                                                let _e565 = global_6.member.member[_e560];
                                                                if _e561 {
                                                                    let _e572 = (_e560 + 1u);
                                                                    if (_e572 < 2047u) {
                                                                    } else {
                                                                        phi_17890_ = false;
                                                                        break;
                                                                    }
                                                                    let _e577 = global_6.member.member[_e572];
                                                                    local_10 = type_50(vec2<f32>(_e565.x, _e565.y), vec2<f32>(_e565.z, _e565.w), vec2<f32>(_e577.x, _e577.y));
                                                                } else {
                                                                    phi_17890_ = false;
                                                                    break;
                                                                }
                                                            } else {
                                                                phi_17890_ = false;
                                                                break;
                                                            }
                                                            let _e6459 = local_10;
                                                            phi_1213_ = _e6459;
                                                        } else {
                                                            let _e536 = (3u * ((_e456.member_5.member - 1u) / 2u));
                                                            let _e537 = (_e536 + 1u);
                                                            if (_e537 < 2047u) {
                                                                let _e542 = global_6.member.member[_e537];
                                                                let _e546 = (_e536 + 2u);
                                                                let _e547 = (_e546 < 2047u);
                                                                if _e547 {
                                                                    let _e551 = global_6.member.member[_e546];
                                                                    if _e547 {
                                                                    } else {
                                                                        phi_17890_ = false;
                                                                        break;
                                                                    }
                                                                    local_11 = type_50(vec2<f32>(_e542.z, _e542.w), vec2<f32>(_e551.x, _e551.y), vec2<f32>(_e551.z, _e551.w));
                                                                } else {
                                                                    phi_17890_ = false;
                                                                    break;
                                                                }
                                                            } else {
                                                                phi_17890_ = false;
                                                                break;
                                                            }
                                                            let _e6461 = local_11;
                                                            phi_1213_ = _e6461;
                                                        }
                                                        let _e583 = phi_1213_;
                                                        phi_1325_ = _e583;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if ((_e456.member_5.member % 2u) == 0u) {
                                                            let _e508 = (3u * (_e456.member_5.member / 2u));
                                                            let _e509 = (_e508 < 192u);
                                                            if _e509 {
                                                                let _e513 = global_6.member.member_1[_e508];
                                                                if _e509 {
                                                                    let _e520 = (_e508 + 1u);
                                                                    if (_e520 < 192u) {
                                                                    } else {
                                                                        phi_17890_ = false;
                                                                        break;
                                                                    }
                                                                    let _e525 = global_6.member.member_1[_e520];
                                                                    local_8 = type_50(vec2<f32>(_e513.x, _e513.y), vec2<f32>(_e513.z, _e513.w), vec2<f32>(_e525.x, _e525.y));
                                                                } else {
                                                                    phi_17890_ = false;
                                                                    break;
                                                                }
                                                            } else {
                                                                phi_17890_ = false;
                                                                break;
                                                            }
                                                            let _e6454 = local_8;
                                                            phi_1323_ = _e6454;
                                                        } else {
                                                            let _e484 = (3u * ((_e456.member_5.member - 1u) / 2u));
                                                            let _e485 = (_e484 + 1u);
                                                            if (_e485 < 192u) {
                                                                let _e490 = global_6.member.member_1[_e485];
                                                                let _e494 = (_e484 + 2u);
                                                                let _e495 = (_e494 < 192u);
                                                                if _e495 {
                                                                    let _e499 = global_6.member.member_1[_e494];
                                                                    if _e495 {
                                                                    } else {
                                                                        phi_17890_ = false;
                                                                        break;
                                                                    }
                                                                    local_9 = type_50(vec2<f32>(_e490.z, _e490.w), vec2<f32>(_e499.x, _e499.y), vec2<f32>(_e499.z, _e499.w));
                                                                } else {
                                                                    phi_17890_ = false;
                                                                    break;
                                                                }
                                                            } else {
                                                                phi_17890_ = false;
                                                                break;
                                                            }
                                                            let _e6456 = local_9;
                                                            phi_1323_ = _e6456;
                                                        }
                                                        let _e531 = phi_1323_;
                                                        phi_1325_ = _e531;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_17890_ = false;
                                                        break;
                                                    }
                                                }
                                                let _e585 = phi_1325_;
                                                let _e606 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e585.member_2.x - _e585.member.x), _e456.member_3.y, fma((_e585.member_1.x - _e585.member.x), _e456.member_3.x, _e585.member.x)), fma((_e585.member_2.y - _e585.member.y), _e456.member_3.y, fma((_e585.member_1.y - _e585.member.y), _e456.member_3.x, _e585.member.y))), 0.0);
                                                phi_1377_ = vec3<f32>((_e606.x * _e465.member.x), (_e606.y * _e465.member.y), (_e606.z * _e465.member.z));
                                            } else {
                                                phi_1377_ = vec3<f32>(_e465.member.x, _e465.member.y, _e465.member.z);
                                            }
                                            let _e618 = phi_1377_;
                                            phi_1381_ = false;
                                            phi_1384_ = false;
                                            phi_1386_ = vec3<f32>(0.0, 0.0, 0.0);
                                            phi_1388_ = false;
                                            phi_1390_ = false;
                                            phi_1392_ = vec3<f32>(0.0, 0.0, 0.0);
                                            phi_1394_ = false;
                                            phi_1396_ = false;
                                            phi_1398_ = 0u;
                                            phi_1400_ = vec3<f32>(0.0, 0.0, 0.0);
                                            phi_1402_ = vec3<f32>(0.0, 0.0, 0.0);
                                            phi_1404_ = true;
                                            loop {
                                                let _e620 = phi_1381_;
                                                let _e622 = phi_1384_;
                                                let _e624 = phi_1386_;
                                                let _e626 = phi_1388_;
                                                let _e628 = phi_1390_;
                                                let _e630 = phi_1392_;
                                                let _e632 = phi_1394_;
                                                let _e634 = phi_1396_;
                                                let _e636 = phi_1398_;
                                                let _e638 = phi_1400_;
                                                let _e640 = phi_1402_;
                                                let _e642 = phi_1404_;
                                                local_280 = _e640;
                                                if _e642 {
                                                    let _e646 = global_8.member.member_1.member;
                                                    let _e647 = (_e636 < _e646);
                                                    if _e647 {
                                                        if (_e636 < 64u) {
                                                            let _e4882 = global_8.member.member[_e636];
                                                            let _e4891 = (vec3<f32>(_e4882.member.x, _e4882.member.y, _e4882.member.z) - _e456.member_1);
                                                            let _e4900 = (_e4891 * (1.0 / sqrt(fma(_e4891.z, _e4891.z, fma(_e4891.x, _e4891.x, (_e4891.y * _e4891.y))))));
                                                            let _e4901 = type_42(_e456.member_1, _e4900);
                                                            let _e4903 = (_e4882.member.x - _e456.member_1.x);
                                                            let _e4905 = (_e4882.member.y - _e456.member_1.y);
                                                            let _e4907 = (_e4882.member.z - _e456.member_1.z);
                                                            let _e4911 = sqrt(fma(_e4907, _e4907, fma(_e4903, _e4903, (_e4905 * _e4905))));
                                                            if (_e4882.member.w == 1.0) {
                                                                let _e5512 = (_e456.member_1.x - _e4882.member.x);
                                                                let _e5513 = (_e456.member_1.y - _e4882.member.y);
                                                                let _e5514 = (_e456.member_1.z - _e4882.member.z);
                                                                let _e5518 = (_e4882.member_1.x - _e4882.member.x);
                                                                let _e5519 = (_e4882.member_1.y - _e4882.member.y);
                                                                let _e5520 = (_e4882.member_1.z - _e4882.member.z);
                                                                let _e5532 = (fma(_e5520, _e5514, fma(_e5518, _e5512, (_e5519 * _e5513))) / sqrt((fma(_e5520, _e5520, fma(_e5518, _e5518, (_e5519 * _e5519))) * fma(_e5514, _e5514, fma(_e5512, _e5512, (_e5513 * _e5513))))));
                                                                let _e5534 = abs(_e5532);
                                                                let _e5535 = (1.0 - _e5534);
                                                                let _e5538 = sqrt(select(_e5535, 0.0, (_e5535 < 0.0)));
                                                                let _e5545 = fma(fma(fma(fma(fma(fma(fma(-0.0012624911032617092, _e5534, 0.006670089904218912), _e5534, -0.01708812639117241), _e5534, 0.03089188039302826), _e5534, -0.050174303352832794), _e5534, 0.08897899091243744), _e5534, -0.21459880471229553), _e5534, 1.570796251296997);
                                                                if (_e5532 >= 0.0) {
                                                                    phi_1515_ = (_e5545 * _e5538);
                                                                } else {
                                                                    phi_1515_ = fma(-(_e5545), _e5538, 3.1415927410125732);
                                                                }
                                                                let _e5550 = phi_1515_;
                                                                let _e5554 = (1.0 - pow((_e5550 / _e4882.member_1.w), 2.0));
                                                                let _e5556 = select(_e5554, 0.0, (_e5554 < 0.0));
                                                                phi_1521_ = 0u;
                                                                phi_1524_ = _e634;
                                                                phi_1526_ = _e638;
                                                                phi_1528_ = true;
                                                                loop {
                                                                    let _e5560 = phi_1521_;
                                                                    let _e5562 = phi_1524_;
                                                                    let _e5564 = phi_1526_;
                                                                    let _e5566 = phi_1528_;
                                                                    local_273 = _e5562;
                                                                    local_274 = _e5564;
                                                                    if _e5566 {
                                                                        if (_e5560 < _e149) {
                                                                            if (_e5560 < 128u) {
                                                                                let _e5897 = global_5.member.member[_e5560];
                                                                                let _e5903 = (_e5897.member_1.x - _e5897.member.x);
                                                                                let _e5906 = (_e5897.member_1.y - _e5897.member.y);
                                                                                let _e5909 = (_e5897.member_1.z - _e5897.member.z);
                                                                                let _e5911 = (_e5897.member_2.x - _e5897.member.x);
                                                                                let _e5913 = (_e5897.member_2.y - _e5897.member.y);
                                                                                let _e5915 = (_e5897.member_2.z - _e5897.member.z);
                                                                                let _e5920 = fma(_e4900.y, _e5915, -((_e5913 * _e4900.z)));
                                                                                let _e5924 = fma(_e4900.z, _e5911, -((_e5915 * _e4900.x)));
                                                                                let _e5927 = fma(_e4900.x, _e5913, -((_e5911 * _e4900.y)));
                                                                                let _e5930 = fma(_e5909, _e5927, fma(_e5903, _e5920, (_e5906 * _e5924)));
                                                                                if (abs(_e5930) < 1.1920928955078125e-7) {
                                                                                    phi_10788_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                } else {
                                                                                    let _e5933 = (1.0 / _e5930);
                                                                                    let _e5934 = (_e456.member_1.x - _e5897.member.x);
                                                                                    let _e5935 = (_e456.member_1.y - _e5897.member.y);
                                                                                    let _e5936 = (_e456.member_1.z - _e5897.member.z);
                                                                                    let _e5940 = (fma(_e5936, _e5927, fma(_e5934, _e5920, (_e5935 * _e5924))) * _e5933);
                                                                                    if (_e5940 > -9.999999747378752e-5) {
                                                                                        phi_10671_ = (_e5940 < 0.0);
                                                                                    } else {
                                                                                        phi_10671_ = false;
                                                                                    }
                                                                                    let _e5944 = phi_10671_;
                                                                                    let _e5945 = select(_e5940, 1.1920928955078125e-7, _e5944);
                                                                                    if (_e5945 < 0.0) {
                                                                                        phi_10681_ = true;
                                                                                    } else {
                                                                                        phi_10681_ = (_e5945 > 1.0);
                                                                                    }
                                                                                    let _e5949 = phi_10681_;
                                                                                    if _e5949 {
                                                                                        phi_10781_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                    } else {
                                                                                        let _e5952 = fma(_e5935, _e5909, -((_e5906 * _e5936)));
                                                                                        let _e5955 = fma(_e5936, _e5903, -((_e5909 * _e5934)));
                                                                                        let _e5958 = fma(_e5934, _e5906, -((_e5903 * _e5935)));
                                                                                        let _e5961 = fma(_e4900.z, _e5958, fma(_e4900.x, _e5952, (_e4900.y * _e5955)));
                                                                                        let _e5962 = (_e5961 * _e5933);
                                                                                        if (_e5962 < 0.0) {
                                                                                            phi_10710_ = true;
                                                                                        } else {
                                                                                            phi_10710_ = (fma(_e5961, _e5933, _e5945) > 1.0);
                                                                                        }
                                                                                        let _e5967 = phi_10710_;
                                                                                        if _e5967 {
                                                                                            phi_10774_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                        } else {
                                                                                            let _e5970 = fma(_e5915, _e5958, fma(_e5911, _e5952, (_e5913 * _e5955)));
                                                                                            let _e5971 = (_e5970 * _e5933);
                                                                                            if (_e5971 < 0.0) {
                                                                                                phi_10771_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                            } else {
                                                                                                let _e5974 = fma(_e5970, _e5933, -0.009999999776482582);
                                                                                                let _e5982 = fma(_e5906, _e5915, -((_e5913 * _e5909)));
                                                                                                let _e5985 = fma(_e5909, _e5911, -((_e5915 * _e5903)));
                                                                                                let _e5988 = fma(_e5903, _e5913, -((_e5911 * _e5906)));
                                                                                                let _e5993 = (1.0 / sqrt(fma(_e5988, _e5988, fma(_e5982, _e5982, (_e5985 * _e5985)))));
                                                                                                phi_10771_ = type_47(_e4901, (_e456.member_1 + vec3<f32>((_e4900.x * _e5974), (_e4900.y * _e5974), (_e4900.z * _e5974))), vec3<f32>((_e5982 * _e5993), (_e5985 * _e5993), (_e5988 * _e5993)), vec2<f32>(_e5945, _e5962), _e5971, type_46(0u, false), select(select(u32(_e5897.member.w), 0u, (_e5897.member.w < 0.0)), 4294967295u, (_e5897.member.w > 4294967040.0)));
                                                                                            }
                                                                                            let _e6006 = phi_10771_;
                                                                                            phi_10774_ = _e6006;
                                                                                        }
                                                                                        let _e6008 = phi_10774_;
                                                                                        phi_10781_ = _e6008;
                                                                                    }
                                                                                    let _e6010 = phi_10781_;
                                                                                    phi_10788_ = _e6010;
                                                                                }
                                                                                let _e6012 = phi_10788_;
                                                                                if (_e6012.member_4 < _e4911) {
                                                                                    if (_e5897.member_2.w == 1.0) {
                                                                                        if ((_e5560 % 2u) == 0u) {
                                                                                            let _e6046 = (3u * (_e5560 / 2u));
                                                                                            let _e6047 = (_e6046 < 192u);
                                                                                            if _e6047 {
                                                                                                let _e6051 = global_6.member.member_1[_e6046];
                                                                                                if _e6047 {
                                                                                                    let _e6058 = (_e6046 + 1u);
                                                                                                    if (_e6058 < 192u) {
                                                                                                    } else {
                                                                                                        break;
                                                                                                    }
                                                                                                    let _e6063 = global_6.member.member_1[_e6058];
                                                                                                    local_263 = type_50(vec2<f32>(_e6051.x, _e6051.y), vec2<f32>(_e6051.z, _e6051.w), vec2<f32>(_e6063.x, _e6063.y));
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                            let _e7441 = local_263;
                                                                                            phi_1787_ = _e7441;
                                                                                        } else {
                                                                                            let _e6022 = (3u * ((_e5560 - 1u) / 2u));
                                                                                            let _e6023 = (_e6022 + 1u);
                                                                                            if (_e6023 < 192u) {
                                                                                                let _e6028 = global_6.member.member_1[_e6023];
                                                                                                let _e6032 = (_e6022 + 2u);
                                                                                                let _e6033 = (_e6032 < 192u);
                                                                                                if _e6033 {
                                                                                                    let _e6037 = global_6.member.member_1[_e6032];
                                                                                                    if _e6033 {
                                                                                                    } else {
                                                                                                        break;
                                                                                                    }
                                                                                                    local_264 = type_50(vec2<f32>(_e6028.z, _e6028.w), vec2<f32>(_e6037.x, _e6037.y), vec2<f32>(_e6037.z, _e6037.w));
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                            let _e7443 = local_264;
                                                                                            phi_1787_ = _e7443;
                                                                                        }
                                                                                        let _e6069 = phi_1787_;
                                                                                        let _e6090 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e6069.member_2.x - _e6069.member.x), _e6012.member_3.y, fma((_e6069.member_1.x - _e6069.member.x), _e6012.member_3.x, _e6069.member.x)), fma((_e6069.member_2.y - _e6069.member.y), _e6012.member_3.y, fma((_e6069.member_1.y - _e6069.member.y), _e6012.member_3.x, _e6069.member.y))), 0.0);
                                                                                        phi_1825_ = (_e6090.w == 1.0);
                                                                                    } else {
                                                                                        phi_1825_ = true;
                                                                                    }
                                                                                    let _e6094 = phi_1825_;
                                                                                    phi_1832_ = select(_e5562, true, _e6094);
                                                                                    phi_1833_ = select(false, true, _e6094);
                                                                                    phi_1834_ = select(true, false, _e6094);
                                                                                } else {
                                                                                    phi_1832_ = _e5562;
                                                                                    phi_1833_ = false;
                                                                                    phi_1834_ = true;
                                                                                }
                                                                                let _e6099 = phi_1832_;
                                                                                let _e6101 = phi_1833_;
                                                                                let _e6103 = phi_1834_;
                                                                                local_266 = _e6099;
                                                                                if _e6103 {
                                                                                    phi_1839_ = (_e5560 + 1u);
                                                                                } else {
                                                                                    phi_1839_ = _e5560;
                                                                                }
                                                                                let _e6106 = phi_1839_;
                                                                                local_265 = _e6106;
                                                                                local_268 = select(false, true, _e6103);
                                                                                local_269 = select(_e6101, false, _e6103);
                                                                            } else {
                                                                                break;
                                                                            }
                                                                            let _e7451 = local_265;
                                                                            phi_1522_ = _e7451;
                                                                            let _e7454 = local_266;
                                                                            phi_1525_ = _e7454;
                                                                            let _e7459 = local_268;
                                                                            phi_2285_ = _e7459;
                                                                            let _e7462 = local_269;
                                                                            phi_2286_ = _e7462;
                                                                            phi_2287_ = false;
                                                                        } else {
                                                                            phi_1852_ = 0u;
                                                                            phi_1855_ = _e5562;
                                                                            phi_1857_ = false;
                                                                            phi_1859_ = false;
                                                                            phi_1861_ = true;
                                                                            loop {
                                                                                let _e5569 = phi_1852_;
                                                                                let _e5571 = phi_1855_;
                                                                                let _e5573 = phi_1857_;
                                                                                let _e5575 = phi_1859_;
                                                                                let _e5577 = phi_1861_;
                                                                                local_267 = _e5571;
                                                                                local_270 = _e5575;
                                                                                local_271 = _e5573;
                                                                                if _e5577 {
                                                                                    if (_e5569 < 4096u) {
                                                                                        let _e5582 = global_4.member.member[_e5569];
                                                                                        let _e5583 = (_e5569 + 1u);
                                                                                        if (_e5583 < 4096u) {
                                                                                            let _e5588 = global_4.member.member[_e5583];
                                                                                            if (_e5582.x == _e5588.x) {
                                                                                                phi_1889_ = (_e5582.y == _e5588.y);
                                                                                            } else {
                                                                                                phi_1889_ = false;
                                                                                            }
                                                                                            let _e5598 = phi_1889_;
                                                                                            if _e5598 {
                                                                                                phi_1894_ = (_e5582.z == _e5588.z);
                                                                                            } else {
                                                                                                phi_1894_ = false;
                                                                                            }
                                                                                            let _e5601 = phi_1894_;
                                                                                            if _e5601 {
                                                                                                let _e5652 = select(select(u32(_e5582.w), 0u, (_e5582.w < 0.0)), 4294967295u, (_e5582.w > 4294967040.0));
                                                                                                if (_e5652 < 1365u) {
                                                                                                    let _e5657 = global_3.member.member[_e5652];
                                                                                                    let _e5663 = (_e5657.member_1.x - _e5657.member.x);
                                                                                                    let _e5666 = (_e5657.member_1.y - _e5657.member.y);
                                                                                                    let _e5669 = (_e5657.member_1.z - _e5657.member.z);
                                                                                                    let _e5671 = (_e5657.member_2.x - _e5657.member.x);
                                                                                                    let _e5673 = (_e5657.member_2.y - _e5657.member.y);
                                                                                                    let _e5675 = (_e5657.member_2.z - _e5657.member.z);
                                                                                                    let _e5680 = fma(_e4900.y, _e5675, -((_e5673 * _e4900.z)));
                                                                                                    let _e5684 = fma(_e4900.z, _e5671, -((_e5675 * _e4900.x)));
                                                                                                    let _e5687 = fma(_e4900.x, _e5673, -((_e5671 * _e4900.y)));
                                                                                                    let _e5690 = fma(_e5669, _e5687, fma(_e5663, _e5680, (_e5666 * _e5684)));
                                                                                                    if (abs(_e5690) < 1.1920928955078125e-7) {
                                                                                                        phi_11143_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                    } else {
                                                                                                        let _e5693 = (1.0 / _e5690);
                                                                                                        let _e5694 = (_e456.member_1.x - _e5657.member.x);
                                                                                                        let _e5695 = (_e456.member_1.y - _e5657.member.y);
                                                                                                        let _e5696 = (_e456.member_1.z - _e5657.member.z);
                                                                                                        let _e5700 = (fma(_e5696, _e5687, fma(_e5694, _e5680, (_e5695 * _e5684))) * _e5693);
                                                                                                        if (_e5700 > -9.999999747378752e-5) {
                                                                                                            phi_11026_ = (_e5700 < 0.0);
                                                                                                        } else {
                                                                                                            phi_11026_ = false;
                                                                                                        }
                                                                                                        let _e5704 = phi_11026_;
                                                                                                        let _e5705 = select(_e5700, 1.1920928955078125e-7, _e5704);
                                                                                                        if (_e5705 < 0.0) {
                                                                                                            phi_11036_ = true;
                                                                                                        } else {
                                                                                                            phi_11036_ = (_e5705 > 1.0);
                                                                                                        }
                                                                                                        let _e5709 = phi_11036_;
                                                                                                        if _e5709 {
                                                                                                            phi_11136_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                        } else {
                                                                                                            let _e5712 = fma(_e5695, _e5669, -((_e5666 * _e5696)));
                                                                                                            let _e5715 = fma(_e5696, _e5663, -((_e5669 * _e5694)));
                                                                                                            let _e5718 = fma(_e5694, _e5666, -((_e5663 * _e5695)));
                                                                                                            let _e5721 = fma(_e4900.z, _e5718, fma(_e4900.x, _e5712, (_e4900.y * _e5715)));
                                                                                                            let _e5722 = (_e5721 * _e5693);
                                                                                                            if (_e5722 < 0.0) {
                                                                                                                phi_11065_ = true;
                                                                                                            } else {
                                                                                                                phi_11065_ = (fma(_e5721, _e5693, _e5705) > 1.0);
                                                                                                            }
                                                                                                            let _e5727 = phi_11065_;
                                                                                                            if _e5727 {
                                                                                                                phi_11129_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                            } else {
                                                                                                                let _e5730 = fma(_e5675, _e5718, fma(_e5671, _e5712, (_e5673 * _e5715)));
                                                                                                                let _e5731 = (_e5730 * _e5693);
                                                                                                                if (_e5731 < 0.0) {
                                                                                                                    phi_11126_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                } else {
                                                                                                                    let _e5734 = fma(_e5730, _e5693, -0.009999999776482582);
                                                                                                                    let _e5742 = fma(_e5666, _e5675, -((_e5673 * _e5669)));
                                                                                                                    let _e5745 = fma(_e5669, _e5671, -((_e5675 * _e5663)));
                                                                                                                    let _e5748 = fma(_e5663, _e5673, -((_e5671 * _e5666)));
                                                                                                                    let _e5753 = (1.0 / sqrt(fma(_e5748, _e5748, fma(_e5742, _e5742, (_e5745 * _e5745)))));
                                                                                                                    phi_11126_ = type_47(_e4901, (_e456.member_1 + vec3<f32>((_e4900.x * _e5734), (_e4900.y * _e5734), (_e4900.z * _e5734))), vec3<f32>((_e5742 * _e5753), (_e5745 * _e5753), (_e5748 * _e5753)), vec2<f32>(_e5705, _e5722), _e5731, type_46(0u, false), select(select(u32(_e5657.member.w), 0u, (_e5657.member.w < 0.0)), 4294967295u, (_e5657.member.w > 4294967040.0)));
                                                                                                                }
                                                                                                                let _e5766 = phi_11126_;
                                                                                                                phi_11129_ = _e5766;
                                                                                                            }
                                                                                                            let _e5768 = phi_11129_;
                                                                                                            phi_11136_ = _e5768;
                                                                                                        }
                                                                                                        let _e5770 = phi_11136_;
                                                                                                        phi_11143_ = _e5770;
                                                                                                    }
                                                                                                    let _e5772 = phi_11143_;
                                                                                                    if (_e5772.member_4 < _e4911) {
                                                                                                        if (_e5657.member_2.w == 1.0) {
                                                                                                            if ((_e5652 % 2u) == 0u) {
                                                                                                                let _e5806 = (3u * (_e5652 / 2u));
                                                                                                                let _e5807 = (_e5806 < 2047u);
                                                                                                                if _e5807 {
                                                                                                                    let _e5811 = global_6.member.member[_e5806];
                                                                                                                    if _e5807 {
                                                                                                                        let _e5818 = (_e5806 + 1u);
                                                                                                                        if (_e5818 < 2047u) {
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                        let _e5823 = global_6.member.member[_e5818];
                                                                                                                        local_257 = type_50(vec2<f32>(_e5811.x, _e5811.y), vec2<f32>(_e5811.z, _e5811.w), vec2<f32>(_e5823.x, _e5823.y));
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                                let _e7409 = local_257;
                                                                                                                phi_2044_ = _e7409;
                                                                                                            } else {
                                                                                                                let _e5782 = (3u * ((_e5652 - 1u) / 2u));
                                                                                                                let _e5783 = (_e5782 + 1u);
                                                                                                                if (_e5783 < 2047u) {
                                                                                                                    let _e5788 = global_6.member.member[_e5783];
                                                                                                                    let _e5792 = (_e5782 + 2u);
                                                                                                                    let _e5793 = (_e5792 < 2047u);
                                                                                                                    if _e5793 {
                                                                                                                        let _e5797 = global_6.member.member[_e5792];
                                                                                                                        if _e5793 {
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                        local_258 = type_50(vec2<f32>(_e5788.z, _e5788.w), vec2<f32>(_e5797.x, _e5797.y), vec2<f32>(_e5797.z, _e5797.w));
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                                let _e7411 = local_258;
                                                                                                                phi_2044_ = _e7411;
                                                                                                            }
                                                                                                            let _e5829 = phi_2044_;
                                                                                                            let _e5850 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e5829.member_2.x - _e5829.member.x), _e5772.member_3.y, fma((_e5829.member_1.x - _e5829.member.x), _e5772.member_3.x, _e5829.member.x)), fma((_e5829.member_2.y - _e5829.member.y), _e5772.member_3.y, fma((_e5829.member_1.y - _e5829.member.y), _e5772.member_3.x, _e5829.member.y))), 0.0);
                                                                                                            phi_2192_ = (_e5850.w == 1.0);
                                                                                                        } else {
                                                                                                            phi_2192_ = true;
                                                                                                        }
                                                                                                        let _e5854 = phi_2192_;
                                                                                                        phi_2199_ = select(_e5571, true, _e5854);
                                                                                                        phi_2200_ = select(false, true, _e5854);
                                                                                                        phi_2201_ = select(true, false, _e5854);
                                                                                                    } else {
                                                                                                        phi_2199_ = _e5571;
                                                                                                        phi_2200_ = false;
                                                                                                        phi_2201_ = true;
                                                                                                    }
                                                                                                    let _e5859 = phi_2199_;
                                                                                                    let _e5861 = phi_2200_;
                                                                                                    let _e5863 = phi_2201_;
                                                                                                    local_260 = _e5859;
                                                                                                    if _e5863 {
                                                                                                        phi_2213_ = select(select(u32(_e5588.w), 0u, (_e5588.w < 0.0)), 4294967295u, (_e5588.w > 4294967040.0));
                                                                                                    } else {
                                                                                                        phi_2213_ = _e5569;
                                                                                                    }
                                                                                                    let _e5871 = phi_2213_;
                                                                                                    local_259 = _e5871;
                                                                                                    local_261 = select(false, true, _e5863);
                                                                                                    local_262 = select(_e5861, false, _e5863);
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                let _e7419 = local_259;
                                                                                                phi_1853_ = _e7419;
                                                                                                let _e7422 = local_260;
                                                                                                phi_2254_ = _e7422;
                                                                                                let _e7425 = local_261;
                                                                                                phi_2255_ = _e7425;
                                                                                                let _e7428 = local_262;
                                                                                                phi_2256_ = _e7428;
                                                                                            } else {
                                                                                                let _e5606 = ((_e5582.x - _e456.member_1.x) / _e4900.x);
                                                                                                let _e5608 = ((_e5582.y - _e456.member_1.y) / _e4900.y);
                                                                                                let _e5610 = ((_e5582.z - _e456.member_1.z) / _e4900.z);
                                                                                                let _e5614 = ((_e5588.x - _e456.member_1.x) / _e4900.x);
                                                                                                let _e5615 = ((_e5588.y - _e456.member_1.y) / _e4900.y);
                                                                                                let _e5616 = ((_e5588.z - _e456.member_1.z) / _e4900.z);
                                                                                                let _e5624 = max(max(min(_e5606, _e5614), min(_e5608, _e5615)), min(_e5610, _e5616));
                                                                                                let _e5626 = min(min(max(_e5606, _e5614), max(_e5608, _e5615)), max(_e5610, _e5616));
                                                                                                if (_e5624 <= _e5626) {
                                                                                                    phi_11353_ = (_e5626 > 0.0);
                                                                                                } else {
                                                                                                    phi_11353_ = false;
                                                                                                }
                                                                                                let _e5630 = phi_11353_;
                                                                                                if (select(3.4028234663852886e38, _e5624, _e5630) < _e4911) {
                                                                                                    phi_2253_ = select(select(u32(_e5582.w), 0u, (_e5582.w < 0.0)), 4294967295u, (_e5582.w > 4294967040.0));
                                                                                                } else {
                                                                                                    phi_2253_ = select(select(u32(_e5588.w), 0u, (_e5588.w < 0.0)), 4294967295u, (_e5588.w > 4294967040.0));
                                                                                                }
                                                                                                let _e5646 = phi_2253_;
                                                                                                phi_1853_ = _e5646;
                                                                                                phi_2254_ = _e5571;
                                                                                                phi_2255_ = true;
                                                                                                phi_2256_ = false;
                                                                                            }
                                                                                            let _e5875 = phi_1853_;
                                                                                            let _e5877 = phi_2254_;
                                                                                            let _e5879 = phi_2255_;
                                                                                            let _e5881 = phi_2256_;
                                                                                            local_252 = _e5875;
                                                                                            if _e5879 {
                                                                                                let _e5882 = (_e5875 == 0u);
                                                                                                phi_1856_ = select(_e5877, false, _e5882);
                                                                                                phi_2267_ = select(false, true, _e5882);
                                                                                                phi_2268_ = select(true, false, _e5882);
                                                                                            } else {
                                                                                                phi_1856_ = _e5877;
                                                                                                phi_2267_ = false;
                                                                                                phi_2268_ = false;
                                                                                            }
                                                                                            let _e5887 = phi_1856_;
                                                                                            let _e5889 = phi_2267_;
                                                                                            let _e5891 = phi_2268_;
                                                                                            local_253 = _e5887;
                                                                                            local_254 = _e5889;
                                                                                            local_255 = select(_e5881, false, _e5879);
                                                                                            local_256 = _e5891;
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                    } else {
                                                                                        break;
                                                                                    }
                                                                                    continue;
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                                continuing {
                                                                                    let _e7383 = local_252;
                                                                                    phi_1852_ = _e7383;
                                                                                    let _e7386 = local_253;
                                                                                    phi_1855_ = _e7386;
                                                                                    let _e7389 = local_254;
                                                                                    phi_1857_ = _e7389;
                                                                                    let _e7392 = local_255;
                                                                                    phi_1859_ = _e7392;
                                                                                    let _e7395 = local_256;
                                                                                    phi_1861_ = _e7395;
                                                                                }
                                                                            }
                                                                            phi_1522_ = _e5560;
                                                                            let _e7456 = local_267;
                                                                            phi_1525_ = _e7456;
                                                                            phi_2285_ = false;
                                                                            let _e7464 = local_270;
                                                                            phi_2286_ = _e7464;
                                                                            let _e7467 = local_271;
                                                                            phi_2287_ = _e7467;
                                                                        }
                                                                        let _e6110 = phi_1522_;
                                                                        let _e6112 = phi_1525_;
                                                                        let _e6114 = phi_2285_;
                                                                        let _e6116 = phi_2286_;
                                                                        let _e6118 = phi_2287_;
                                                                        let _e6119 = select(_e6118, true, _e6116);
                                                                        local_248 = _e6110;
                                                                        local_249 = _e6112;
                                                                        if _e6119 {
                                                                            if _e6112 {
                                                                                phi_2313_ = 0.0;
                                                                            } else {
                                                                                phi_2313_ = max(fma(_e4900.z, _e456.member_2.z, fma(_e4900.x, _e456.member_2.x, (_e4900.y * _e456.member_2.y))), 0.0);
                                                                            }
                                                                            let _e6133 = phi_2313_;
                                                                            let _e6134 = (select(_e5556, 1.0, (_e5556 > 1.0)) * _e6133);
                                                                            phi_1527_ = vec3<f32>(fma(((_e6134 * _e4882.member_2.x) * _e4882.member_2.w), _e618.x, _e5564.x), fma(((_e6134 * _e4882.member_2.y) * _e4882.member_2.w), _e618.y, _e5564.y), fma(((_e6134 * _e4882.member_2.z) * _e4882.member_2.w), _e618.z, _e5564.z));
                                                                        } else {
                                                                            phi_1527_ = _e5564;
                                                                        }
                                                                        let _e6156 = phi_1527_;
                                                                        local_250 = _e6156;
                                                                        local_251 = select(select(_e6114, false, _e6116), false, _e6119);
                                                                        continue;
                                                                    } else {
                                                                        break;
                                                                    }
                                                                    continuing {
                                                                        let _e7371 = local_248;
                                                                        phi_1521_ = _e7371;
                                                                        let _e7374 = local_249;
                                                                        phi_1524_ = _e7374;
                                                                        let _e7377 = local_250;
                                                                        phi_1526_ = _e7377;
                                                                        let _e7380 = local_251;
                                                                        phi_1528_ = _e7380;
                                                                    }
                                                                }
                                                                phi_3163_ = _e632;
                                                                let _e7475 = local_273;
                                                                phi_3164_ = _e7475;
                                                                let _e7478 = local_274;
                                                                phi_3165_ = _e7478;
                                                            } else {
                                                                phi_2344_ = 0u;
                                                                phi_2347_ = _e632;
                                                                phi_2349_ = _e638;
                                                                phi_2351_ = true;
                                                                loop {
                                                                    let _e4915 = phi_2344_;
                                                                    let _e4917 = phi_2347_;
                                                                    let _e4919 = phi_2349_;
                                                                    let _e4921 = phi_2351_;
                                                                    local_272 = _e4917;
                                                                    local_275 = _e4919;
                                                                    if _e4921 {
                                                                        if (_e4915 < _e149) {
                                                                            if (_e4915 < 128u) {
                                                                                let _e5252 = global_5.member.member[_e4915];
                                                                                let _e5258 = (_e5252.member_1.x - _e5252.member.x);
                                                                                let _e5261 = (_e5252.member_1.y - _e5252.member.y);
                                                                                let _e5264 = (_e5252.member_1.z - _e5252.member.z);
                                                                                let _e5266 = (_e5252.member_2.x - _e5252.member.x);
                                                                                let _e5268 = (_e5252.member_2.y - _e5252.member.y);
                                                                                let _e5270 = (_e5252.member_2.z - _e5252.member.z);
                                                                                let _e5275 = fma(_e4900.y, _e5270, -((_e5268 * _e4900.z)));
                                                                                let _e5279 = fma(_e4900.z, _e5266, -((_e5270 * _e4900.x)));
                                                                                let _e5282 = fma(_e4900.x, _e5268, -((_e5266 * _e4900.y)));
                                                                                let _e5285 = fma(_e5264, _e5282, fma(_e5258, _e5275, (_e5261 * _e5279)));
                                                                                if (abs(_e5285) < 1.1920928955078125e-7) {
                                                                                    phi_11575_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                } else {
                                                                                    let _e5288 = (1.0 / _e5285);
                                                                                    let _e5289 = (_e456.member_1.x - _e5252.member.x);
                                                                                    let _e5290 = (_e456.member_1.y - _e5252.member.y);
                                                                                    let _e5291 = (_e456.member_1.z - _e5252.member.z);
                                                                                    let _e5295 = (fma(_e5291, _e5282, fma(_e5289, _e5275, (_e5290 * _e5279))) * _e5288);
                                                                                    if (_e5295 > -9.999999747378752e-5) {
                                                                                        phi_11458_ = (_e5295 < 0.0);
                                                                                    } else {
                                                                                        phi_11458_ = false;
                                                                                    }
                                                                                    let _e5299 = phi_11458_;
                                                                                    let _e5300 = select(_e5295, 1.1920928955078125e-7, _e5299);
                                                                                    if (_e5300 < 0.0) {
                                                                                        phi_11468_ = true;
                                                                                    } else {
                                                                                        phi_11468_ = (_e5300 > 1.0);
                                                                                    }
                                                                                    let _e5304 = phi_11468_;
                                                                                    if _e5304 {
                                                                                        phi_11568_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                    } else {
                                                                                        let _e5307 = fma(_e5290, _e5264, -((_e5261 * _e5291)));
                                                                                        let _e5310 = fma(_e5291, _e5258, -((_e5264 * _e5289)));
                                                                                        let _e5313 = fma(_e5289, _e5261, -((_e5258 * _e5290)));
                                                                                        let _e5316 = fma(_e4900.z, _e5313, fma(_e4900.x, _e5307, (_e4900.y * _e5310)));
                                                                                        let _e5317 = (_e5316 * _e5288);
                                                                                        if (_e5317 < 0.0) {
                                                                                            phi_11497_ = true;
                                                                                        } else {
                                                                                            phi_11497_ = (fma(_e5316, _e5288, _e5300) > 1.0);
                                                                                        }
                                                                                        let _e5322 = phi_11497_;
                                                                                        if _e5322 {
                                                                                            phi_11561_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                        } else {
                                                                                            let _e5325 = fma(_e5270, _e5313, fma(_e5266, _e5307, (_e5268 * _e5310)));
                                                                                            let _e5326 = (_e5325 * _e5288);
                                                                                            if (_e5326 < 0.0) {
                                                                                                phi_11558_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                            } else {
                                                                                                let _e5329 = fma(_e5325, _e5288, -0.009999999776482582);
                                                                                                let _e5337 = fma(_e5261, _e5270, -((_e5268 * _e5264)));
                                                                                                let _e5340 = fma(_e5264, _e5266, -((_e5270 * _e5258)));
                                                                                                let _e5343 = fma(_e5258, _e5268, -((_e5266 * _e5261)));
                                                                                                let _e5348 = (1.0 / sqrt(fma(_e5343, _e5343, fma(_e5337, _e5337, (_e5340 * _e5340)))));
                                                                                                phi_11558_ = type_47(_e4901, (_e456.member_1 + vec3<f32>((_e4900.x * _e5329), (_e4900.y * _e5329), (_e4900.z * _e5329))), vec3<f32>((_e5337 * _e5348), (_e5340 * _e5348), (_e5343 * _e5348)), vec2<f32>(_e5300, _e5317), _e5326, type_46(0u, false), select(select(u32(_e5252.member.w), 0u, (_e5252.member.w < 0.0)), 4294967295u, (_e5252.member.w > 4294967040.0)));
                                                                                            }
                                                                                            let _e5361 = phi_11558_;
                                                                                            phi_11561_ = _e5361;
                                                                                        }
                                                                                        let _e5363 = phi_11561_;
                                                                                        phi_11568_ = _e5363;
                                                                                    }
                                                                                    let _e5365 = phi_11568_;
                                                                                    phi_11575_ = _e5365;
                                                                                }
                                                                                let _e5367 = phi_11575_;
                                                                                if (_e5367.member_4 < _e4911) {
                                                                                    if (_e5252.member_2.w == 1.0) {
                                                                                        if ((_e4915 % 2u) == 0u) {
                                                                                            let _e5401 = (3u * (_e4915 / 2u));
                                                                                            let _e5402 = (_e5401 < 192u);
                                                                                            if _e5402 {
                                                                                                let _e5406 = global_6.member.member_1[_e5401];
                                                                                                if _e5402 {
                                                                                                    let _e5413 = (_e5401 + 1u);
                                                                                                    if (_e5413 < 192u) {
                                                                                                    } else {
                                                                                                        break;
                                                                                                    }
                                                                                                    let _e5418 = global_6.member.member_1[_e5413];
                                                                                                    local_239 = type_50(vec2<f32>(_e5406.x, _e5406.y), vec2<f32>(_e5406.z, _e5406.w), vec2<f32>(_e5418.x, _e5418.y));
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                            let _e7339 = local_239;
                                                                                            phi_2610_ = _e7339;
                                                                                        } else {
                                                                                            let _e5377 = (3u * ((_e4915 - 1u) / 2u));
                                                                                            let _e5378 = (_e5377 + 1u);
                                                                                            if (_e5378 < 192u) {
                                                                                                let _e5383 = global_6.member.member_1[_e5378];
                                                                                                let _e5387 = (_e5377 + 2u);
                                                                                                let _e5388 = (_e5387 < 192u);
                                                                                                if _e5388 {
                                                                                                    let _e5392 = global_6.member.member_1[_e5387];
                                                                                                    if _e5388 {
                                                                                                    } else {
                                                                                                        break;
                                                                                                    }
                                                                                                    local_240 = type_50(vec2<f32>(_e5383.z, _e5383.w), vec2<f32>(_e5392.x, _e5392.y), vec2<f32>(_e5392.z, _e5392.w));
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                            let _e7341 = local_240;
                                                                                            phi_2610_ = _e7341;
                                                                                        }
                                                                                        let _e5424 = phi_2610_;
                                                                                        let _e5445 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e5424.member_2.x - _e5424.member.x), _e5367.member_3.y, fma((_e5424.member_1.x - _e5424.member.x), _e5367.member_3.x, _e5424.member.x)), fma((_e5424.member_2.y - _e5424.member.y), _e5367.member_3.y, fma((_e5424.member_1.y - _e5424.member.y), _e5367.member_3.x, _e5424.member.y))), 0.0);
                                                                                        phi_2648_ = (_e5445.w == 1.0);
                                                                                    } else {
                                                                                        phi_2648_ = true;
                                                                                    }
                                                                                    let _e5449 = phi_2648_;
                                                                                    phi_2655_ = select(_e4917, true, _e5449);
                                                                                    phi_2656_ = select(false, true, _e5449);
                                                                                    phi_2657_ = select(true, false, _e5449);
                                                                                } else {
                                                                                    phi_2655_ = _e4917;
                                                                                    phi_2656_ = false;
                                                                                    phi_2657_ = true;
                                                                                }
                                                                                let _e5454 = phi_2655_;
                                                                                let _e5456 = phi_2656_;
                                                                                let _e5458 = phi_2657_;
                                                                                local_242 = _e5454;
                                                                                if _e5458 {
                                                                                    phi_2662_ = (_e4915 + 1u);
                                                                                } else {
                                                                                    phi_2662_ = _e4915;
                                                                                }
                                                                                let _e5461 = phi_2662_;
                                                                                local_241 = _e5461;
                                                                                local_244 = select(false, true, _e5458);
                                                                                local_245 = select(_e5456, false, _e5458);
                                                                            } else {
                                                                                break;
                                                                            }
                                                                            let _e7349 = local_241;
                                                                            phi_2345_ = _e7349;
                                                                            let _e7352 = local_242;
                                                                            phi_2348_ = _e7352;
                                                                            let _e7357 = local_244;
                                                                            phi_3108_ = _e7357;
                                                                            let _e7360 = local_245;
                                                                            phi_3109_ = _e7360;
                                                                            phi_3110_ = false;
                                                                        } else {
                                                                            phi_2675_ = 0u;
                                                                            phi_2678_ = _e4917;
                                                                            phi_2680_ = false;
                                                                            phi_2682_ = false;
                                                                            phi_2684_ = true;
                                                                            loop {
                                                                                let _e4924 = phi_2675_;
                                                                                let _e4926 = phi_2678_;
                                                                                let _e4928 = phi_2680_;
                                                                                let _e4930 = phi_2682_;
                                                                                let _e4932 = phi_2684_;
                                                                                local_243 = _e4926;
                                                                                local_246 = _e4930;
                                                                                local_247 = _e4928;
                                                                                if _e4932 {
                                                                                    if (_e4924 < 4096u) {
                                                                                        let _e4937 = global_4.member.member[_e4924];
                                                                                        let _e4938 = (_e4924 + 1u);
                                                                                        if (_e4938 < 4096u) {
                                                                                            let _e4943 = global_4.member.member[_e4938];
                                                                                            if (_e4937.x == _e4943.x) {
                                                                                                phi_2712_ = (_e4937.y == _e4943.y);
                                                                                            } else {
                                                                                                phi_2712_ = false;
                                                                                            }
                                                                                            let _e4953 = phi_2712_;
                                                                                            if _e4953 {
                                                                                                phi_2717_ = (_e4937.z == _e4943.z);
                                                                                            } else {
                                                                                                phi_2717_ = false;
                                                                                            }
                                                                                            let _e4956 = phi_2717_;
                                                                                            if _e4956 {
                                                                                                let _e5007 = select(select(u32(_e4937.w), 0u, (_e4937.w < 0.0)), 4294967295u, (_e4937.w > 4294967040.0));
                                                                                                if (_e5007 < 1365u) {
                                                                                                    let _e5012 = global_3.member.member[_e5007];
                                                                                                    let _e5018 = (_e5012.member_1.x - _e5012.member.x);
                                                                                                    let _e5021 = (_e5012.member_1.y - _e5012.member.y);
                                                                                                    let _e5024 = (_e5012.member_1.z - _e5012.member.z);
                                                                                                    let _e5026 = (_e5012.member_2.x - _e5012.member.x);
                                                                                                    let _e5028 = (_e5012.member_2.y - _e5012.member.y);
                                                                                                    let _e5030 = (_e5012.member_2.z - _e5012.member.z);
                                                                                                    let _e5035 = fma(_e4900.y, _e5030, -((_e5028 * _e4900.z)));
                                                                                                    let _e5039 = fma(_e4900.z, _e5026, -((_e5030 * _e4900.x)));
                                                                                                    let _e5042 = fma(_e4900.x, _e5028, -((_e5026 * _e4900.y)));
                                                                                                    let _e5045 = fma(_e5024, _e5042, fma(_e5018, _e5035, (_e5021 * _e5039)));
                                                                                                    if (abs(_e5045) < 1.1920928955078125e-7) {
                                                                                                        phi_11930_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                    } else {
                                                                                                        let _e5048 = (1.0 / _e5045);
                                                                                                        let _e5049 = (_e456.member_1.x - _e5012.member.x);
                                                                                                        let _e5050 = (_e456.member_1.y - _e5012.member.y);
                                                                                                        let _e5051 = (_e456.member_1.z - _e5012.member.z);
                                                                                                        let _e5055 = (fma(_e5051, _e5042, fma(_e5049, _e5035, (_e5050 * _e5039))) * _e5048);
                                                                                                        if (_e5055 > -9.999999747378752e-5) {
                                                                                                            phi_11813_ = (_e5055 < 0.0);
                                                                                                        } else {
                                                                                                            phi_11813_ = false;
                                                                                                        }
                                                                                                        let _e5059 = phi_11813_;
                                                                                                        let _e5060 = select(_e5055, 1.1920928955078125e-7, _e5059);
                                                                                                        if (_e5060 < 0.0) {
                                                                                                            phi_11823_ = true;
                                                                                                        } else {
                                                                                                            phi_11823_ = (_e5060 > 1.0);
                                                                                                        }
                                                                                                        let _e5064 = phi_11823_;
                                                                                                        if _e5064 {
                                                                                                            phi_11923_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                        } else {
                                                                                                            let _e5067 = fma(_e5050, _e5024, -((_e5021 * _e5051)));
                                                                                                            let _e5070 = fma(_e5051, _e5018, -((_e5024 * _e5049)));
                                                                                                            let _e5073 = fma(_e5049, _e5021, -((_e5018 * _e5050)));
                                                                                                            let _e5076 = fma(_e4900.z, _e5073, fma(_e4900.x, _e5067, (_e4900.y * _e5070)));
                                                                                                            let _e5077 = (_e5076 * _e5048);
                                                                                                            if (_e5077 < 0.0) {
                                                                                                                phi_11852_ = true;
                                                                                                            } else {
                                                                                                                phi_11852_ = (fma(_e5076, _e5048, _e5060) > 1.0);
                                                                                                            }
                                                                                                            let _e5082 = phi_11852_;
                                                                                                            if _e5082 {
                                                                                                                phi_11916_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                            } else {
                                                                                                                let _e5085 = fma(_e5030, _e5073, fma(_e5026, _e5067, (_e5028 * _e5070)));
                                                                                                                let _e5086 = (_e5085 * _e5048);
                                                                                                                if (_e5086 < 0.0) {
                                                                                                                    phi_11913_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                } else {
                                                                                                                    let _e5089 = fma(_e5085, _e5048, -0.009999999776482582);
                                                                                                                    let _e5097 = fma(_e5021, _e5030, -((_e5028 * _e5024)));
                                                                                                                    let _e5100 = fma(_e5024, _e5026, -((_e5030 * _e5018)));
                                                                                                                    let _e5103 = fma(_e5018, _e5028, -((_e5026 * _e5021)));
                                                                                                                    let _e5108 = (1.0 / sqrt(fma(_e5103, _e5103, fma(_e5097, _e5097, (_e5100 * _e5100)))));
                                                                                                                    phi_11913_ = type_47(_e4901, (_e456.member_1 + vec3<f32>((_e4900.x * _e5089), (_e4900.y * _e5089), (_e4900.z * _e5089))), vec3<f32>((_e5097 * _e5108), (_e5100 * _e5108), (_e5103 * _e5108)), vec2<f32>(_e5060, _e5077), _e5086, type_46(0u, false), select(select(u32(_e5012.member.w), 0u, (_e5012.member.w < 0.0)), 4294967295u, (_e5012.member.w > 4294967040.0)));
                                                                                                                }
                                                                                                                let _e5121 = phi_11913_;
                                                                                                                phi_11916_ = _e5121;
                                                                                                            }
                                                                                                            let _e5123 = phi_11916_;
                                                                                                            phi_11923_ = _e5123;
                                                                                                        }
                                                                                                        let _e5125 = phi_11923_;
                                                                                                        phi_11930_ = _e5125;
                                                                                                    }
                                                                                                    let _e5127 = phi_11930_;
                                                                                                    if (_e5127.member_4 < _e4911) {
                                                                                                        if (_e5012.member_2.w == 1.0) {
                                                                                                            if ((_e5007 % 2u) == 0u) {
                                                                                                                let _e5161 = (3u * (_e5007 / 2u));
                                                                                                                let _e5162 = (_e5161 < 2047u);
                                                                                                                if _e5162 {
                                                                                                                    let _e5166 = global_6.member.member[_e5161];
                                                                                                                    if _e5162 {
                                                                                                                        let _e5173 = (_e5161 + 1u);
                                                                                                                        if (_e5173 < 2047u) {
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                        let _e5178 = global_6.member.member[_e5173];
                                                                                                                        local_233 = type_50(vec2<f32>(_e5166.x, _e5166.y), vec2<f32>(_e5166.z, _e5166.w), vec2<f32>(_e5178.x, _e5178.y));
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                                let _e7307 = local_233;
                                                                                                                phi_2867_ = _e7307;
                                                                                                            } else {
                                                                                                                let _e5137 = (3u * ((_e5007 - 1u) / 2u));
                                                                                                                let _e5138 = (_e5137 + 1u);
                                                                                                                if (_e5138 < 2047u) {
                                                                                                                    let _e5143 = global_6.member.member[_e5138];
                                                                                                                    let _e5147 = (_e5137 + 2u);
                                                                                                                    let _e5148 = (_e5147 < 2047u);
                                                                                                                    if _e5148 {
                                                                                                                        let _e5152 = global_6.member.member[_e5147];
                                                                                                                        if _e5148 {
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                        local_234 = type_50(vec2<f32>(_e5143.z, _e5143.w), vec2<f32>(_e5152.x, _e5152.y), vec2<f32>(_e5152.z, _e5152.w));
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                                let _e7309 = local_234;
                                                                                                                phi_2867_ = _e7309;
                                                                                                            }
                                                                                                            let _e5184 = phi_2867_;
                                                                                                            let _e5205 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e5184.member_2.x - _e5184.member.x), _e5127.member_3.y, fma((_e5184.member_1.x - _e5184.member.x), _e5127.member_3.x, _e5184.member.x)), fma((_e5184.member_2.y - _e5184.member.y), _e5127.member_3.y, fma((_e5184.member_1.y - _e5184.member.y), _e5127.member_3.x, _e5184.member.y))), 0.0);
                                                                                                            phi_3015_ = (_e5205.w == 1.0);
                                                                                                        } else {
                                                                                                            phi_3015_ = true;
                                                                                                        }
                                                                                                        let _e5209 = phi_3015_;
                                                                                                        phi_3022_ = select(_e4926, true, _e5209);
                                                                                                        phi_3023_ = select(false, true, _e5209);
                                                                                                        phi_3024_ = select(true, false, _e5209);
                                                                                                    } else {
                                                                                                        phi_3022_ = _e4926;
                                                                                                        phi_3023_ = false;
                                                                                                        phi_3024_ = true;
                                                                                                    }
                                                                                                    let _e5214 = phi_3022_;
                                                                                                    let _e5216 = phi_3023_;
                                                                                                    let _e5218 = phi_3024_;
                                                                                                    local_236 = _e5214;
                                                                                                    if _e5218 {
                                                                                                        phi_3036_ = select(select(u32(_e4943.w), 0u, (_e4943.w < 0.0)), 4294967295u, (_e4943.w > 4294967040.0));
                                                                                                    } else {
                                                                                                        phi_3036_ = _e4924;
                                                                                                    }
                                                                                                    let _e5226 = phi_3036_;
                                                                                                    local_235 = _e5226;
                                                                                                    local_237 = select(false, true, _e5218);
                                                                                                    local_238 = select(_e5216, false, _e5218);
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                let _e7317 = local_235;
                                                                                                phi_2676_ = _e7317;
                                                                                                let _e7320 = local_236;
                                                                                                phi_3077_ = _e7320;
                                                                                                let _e7323 = local_237;
                                                                                                phi_3078_ = _e7323;
                                                                                                let _e7326 = local_238;
                                                                                                phi_3079_ = _e7326;
                                                                                            } else {
                                                                                                let _e4961 = ((_e4937.x - _e456.member_1.x) / _e4900.x);
                                                                                                let _e4963 = ((_e4937.y - _e456.member_1.y) / _e4900.y);
                                                                                                let _e4965 = ((_e4937.z - _e456.member_1.z) / _e4900.z);
                                                                                                let _e4969 = ((_e4943.x - _e456.member_1.x) / _e4900.x);
                                                                                                let _e4970 = ((_e4943.y - _e456.member_1.y) / _e4900.y);
                                                                                                let _e4971 = ((_e4943.z - _e456.member_1.z) / _e4900.z);
                                                                                                let _e4979 = max(max(min(_e4961, _e4969), min(_e4963, _e4970)), min(_e4965, _e4971));
                                                                                                let _e4981 = min(min(max(_e4961, _e4969), max(_e4963, _e4970)), max(_e4965, _e4971));
                                                                                                if (_e4979 <= _e4981) {
                                                                                                    phi_12140_ = (_e4981 > 0.0);
                                                                                                } else {
                                                                                                    phi_12140_ = false;
                                                                                                }
                                                                                                let _e4985 = phi_12140_;
                                                                                                if (select(3.4028234663852886e38, _e4979, _e4985) < _e4911) {
                                                                                                    phi_3076_ = select(select(u32(_e4937.w), 0u, (_e4937.w < 0.0)), 4294967295u, (_e4937.w > 4294967040.0));
                                                                                                } else {
                                                                                                    phi_3076_ = select(select(u32(_e4943.w), 0u, (_e4943.w < 0.0)), 4294967295u, (_e4943.w > 4294967040.0));
                                                                                                }
                                                                                                let _e5001 = phi_3076_;
                                                                                                phi_2676_ = _e5001;
                                                                                                phi_3077_ = _e4926;
                                                                                                phi_3078_ = true;
                                                                                                phi_3079_ = false;
                                                                                            }
                                                                                            let _e5230 = phi_2676_;
                                                                                            let _e5232 = phi_3077_;
                                                                                            let _e5234 = phi_3078_;
                                                                                            let _e5236 = phi_3079_;
                                                                                            local_228 = _e5230;
                                                                                            if _e5234 {
                                                                                                let _e5237 = (_e5230 == 0u);
                                                                                                phi_2679_ = select(_e5232, false, _e5237);
                                                                                                phi_3090_ = select(false, true, _e5237);
                                                                                                phi_3091_ = select(true, false, _e5237);
                                                                                            } else {
                                                                                                phi_2679_ = _e5232;
                                                                                                phi_3090_ = false;
                                                                                                phi_3091_ = false;
                                                                                            }
                                                                                            let _e5242 = phi_2679_;
                                                                                            let _e5244 = phi_3090_;
                                                                                            let _e5246 = phi_3091_;
                                                                                            local_229 = _e5242;
                                                                                            local_230 = _e5244;
                                                                                            local_231 = select(_e5236, false, _e5234);
                                                                                            local_232 = _e5246;
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                    } else {
                                                                                        break;
                                                                                    }
                                                                                    continue;
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                                continuing {
                                                                                    let _e7281 = local_228;
                                                                                    phi_2675_ = _e7281;
                                                                                    let _e7284 = local_229;
                                                                                    phi_2678_ = _e7284;
                                                                                    let _e7287 = local_230;
                                                                                    phi_2680_ = _e7287;
                                                                                    let _e7290 = local_231;
                                                                                    phi_2682_ = _e7290;
                                                                                    let _e7293 = local_232;
                                                                                    phi_2684_ = _e7293;
                                                                                }
                                                                            }
                                                                            phi_2345_ = _e4915;
                                                                            let _e7354 = local_243;
                                                                            phi_2348_ = _e7354;
                                                                            phi_3108_ = false;
                                                                            let _e7362 = local_246;
                                                                            phi_3109_ = _e7362;
                                                                            let _e7365 = local_247;
                                                                            phi_3110_ = _e7365;
                                                                        }
                                                                        let _e5465 = phi_2345_;
                                                                        let _e5467 = phi_2348_;
                                                                        let _e5469 = phi_3108_;
                                                                        let _e5471 = phi_3109_;
                                                                        let _e5473 = phi_3110_;
                                                                        let _e5474 = select(_e5473, true, _e5471);
                                                                        local_224 = _e5465;
                                                                        local_225 = _e5467;
                                                                        if _e5474 {
                                                                            if _e5467 {
                                                                                phi_3136_ = 0.0;
                                                                            } else {
                                                                                phi_3136_ = max(fma(_e4900.z, _e456.member_2.z, fma(_e4900.x, _e456.member_2.x, (_e4900.y * _e456.member_2.y))), 0.0);
                                                                            }
                                                                            let _e5488 = phi_3136_;
                                                                            phi_2350_ = vec3<f32>(fma(((_e5488 * _e4882.member_2.x) * _e4882.member_2.w), _e618.x, _e4919.x), fma(((_e5488 * _e4882.member_2.y) * _e4882.member_2.w), _e618.y, _e4919.y), fma(((_e5488 * _e4882.member_2.z) * _e4882.member_2.w), _e618.z, _e4919.z));
                                                                        } else {
                                                                            phi_2350_ = _e4919;
                                                                        }
                                                                        let _e5510 = phi_2350_;
                                                                        local_226 = _e5510;
                                                                        local_227 = select(select(_e5469, false, _e5471), false, _e5474);
                                                                        continue;
                                                                    } else {
                                                                        break;
                                                                    }
                                                                    continuing {
                                                                        let _e7269 = local_224;
                                                                        phi_2344_ = _e7269;
                                                                        let _e7272 = local_225;
                                                                        phi_2347_ = _e7272;
                                                                        let _e7275 = local_226;
                                                                        phi_2349_ = _e7275;
                                                                        let _e7278 = local_227;
                                                                        phi_2351_ = _e7278;
                                                                    }
                                                                }
                                                                let _e7472 = local_272;
                                                                phi_3163_ = _e7472;
                                                                phi_3164_ = _e634;
                                                                let _e7480 = local_275;
                                                                phi_3165_ = _e7480;
                                                            }
                                                            let _e6159 = phi_3163_;
                                                            let _e6161 = phi_3164_;
                                                            let _e6163 = phi_3165_;
                                                            local_276 = _e6159;
                                                            local_277 = _e6161;
                                                            local_278 = (_e636 + 1u);
                                                            local_279 = _e6163;
                                                        } else {
                                                            break;
                                                        }
                                                        phi_1382_ = _e620;
                                                        phi_1385_ = _e622;
                                                        phi_1387_ = _e624;
                                                        phi_1389_ = _e626;
                                                        phi_1391_ = _e628;
                                                        phi_1393_ = _e630;
                                                        let _e7489 = local_276;
                                                        phi_1395_ = _e7489;
                                                        let _e7492 = local_277;
                                                        phi_1397_ = _e7492;
                                                        let _e7495 = local_278;
                                                        phi_1399_ = _e7495;
                                                        let _e7498 = local_279;
                                                        phi_1401_ = _e7498;
                                                        phi_1403_ = _e640;
                                                    } else {
                                                        if (_e465.member_1.w > 0.0) {
                                                            let _e667 = fma(_e456.member_2.z, -(_e456.member.member_1.z), fma(_e456.member_2.x, -(_e456.member.member_1.x), (_e456.member_2.y * -(_e456.member.member_1.y))));
                                                            let _e671 = fma((_e456.member_2.x * _e667), 2.0, _e456.member.member_1.x);
                                                            let _e672 = fma((_e456.member_2.y * _e667), 2.0, _e456.member.member_1.y);
                                                            let _e673 = fma((_e456.member_2.z * _e667), 2.0, _e456.member.member_1.z);
                                                            let _e681 = (vec3<f32>(_e671, _e672, _e673) * (1.0 / sqrt(fma(_e673, _e673, fma(_e671, _e671, (_e672 * _e672))))));
                                                            let _e682 = type_42(_e456.member_1, _e681);
                                                            phi_3222_ = _e626;
                                                            phi_3225_ = _e628;
                                                            phi_3227_ = 0u;
                                                            phi_3229_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                            phi_3231_ = _e630;
                                                            phi_3233_ = _e638;
                                                            phi_3235_ = true;
                                                            loop {
                                                                let _e684 = phi_3222_;
                                                                let _e686 = phi_3225_;
                                                                let _e688 = phi_3227_;
                                                                let _e690 = phi_3229_;
                                                                let _e692 = phi_3231_;
                                                                let _e694 = phi_3233_;
                                                                let _e696 = phi_3235_;
                                                                local_120 = _e684;
                                                                local_121 = _e686;
                                                                local_122 = _e692;
                                                                local_123 = _e694;
                                                                if _e696 {
                                                                    let _e697 = (_e688 < _e149);
                                                                    if _e697 {
                                                                        if (_e688 < 128u) {
                                                                            let _e2524 = global_5.member.member[_e688];
                                                                            let _e2530 = (_e2524.member_1.x - _e2524.member.x);
                                                                            let _e2533 = (_e2524.member_1.y - _e2524.member.y);
                                                                            let _e2536 = (_e2524.member_1.z - _e2524.member.z);
                                                                            let _e2538 = (_e2524.member_2.x - _e2524.member.x);
                                                                            let _e2540 = (_e2524.member_2.y - _e2524.member.y);
                                                                            let _e2542 = (_e2524.member_2.z - _e2524.member.z);
                                                                            let _e2547 = fma(_e681.y, _e2542, -((_e2540 * _e681.z)));
                                                                            let _e2551 = fma(_e681.z, _e2538, -((_e2542 * _e681.x)));
                                                                            let _e2554 = fma(_e681.x, _e2540, -((_e2538 * _e681.y)));
                                                                            let _e2557 = fma(_e2536, _e2554, fma(_e2530, _e2547, (_e2533 * _e2551)));
                                                                            if (abs(_e2557) < 1.1920928955078125e-7) {
                                                                                phi_12399_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                            } else {
                                                                                let _e2560 = (1.0 / _e2557);
                                                                                let _e2562 = (_e456.member_1.x - _e2524.member.x);
                                                                                let _e2564 = (_e456.member_1.y - _e2524.member.y);
                                                                                let _e2566 = (_e456.member_1.z - _e2524.member.z);
                                                                                let _e2570 = (fma(_e2566, _e2554, fma(_e2562, _e2547, (_e2564 * _e2551))) * _e2560);
                                                                                if (_e2570 > -9.999999747378752e-5) {
                                                                                    phi_12282_ = (_e2570 < 0.0);
                                                                                } else {
                                                                                    phi_12282_ = false;
                                                                                }
                                                                                let _e2574 = phi_12282_;
                                                                                let _e2575 = select(_e2570, 1.1920928955078125e-7, _e2574);
                                                                                if (_e2575 < 0.0) {
                                                                                    phi_12292_ = true;
                                                                                } else {
                                                                                    phi_12292_ = (_e2575 > 1.0);
                                                                                }
                                                                                let _e2579 = phi_12292_;
                                                                                if _e2579 {
                                                                                    phi_12392_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                } else {
                                                                                    let _e2582 = fma(_e2564, _e2536, -((_e2533 * _e2566)));
                                                                                    let _e2585 = fma(_e2566, _e2530, -((_e2536 * _e2562)));
                                                                                    let _e2588 = fma(_e2562, _e2533, -((_e2530 * _e2564)));
                                                                                    let _e2591 = fma(_e681.z, _e2588, fma(_e681.x, _e2582, (_e681.y * _e2585)));
                                                                                    let _e2592 = (_e2591 * _e2560);
                                                                                    if (_e2592 < 0.0) {
                                                                                        phi_12321_ = true;
                                                                                    } else {
                                                                                        phi_12321_ = (fma(_e2591, _e2560, _e2575) > 1.0);
                                                                                    }
                                                                                    let _e2597 = phi_12321_;
                                                                                    if _e2597 {
                                                                                        phi_12385_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                    } else {
                                                                                        let _e2600 = fma(_e2542, _e2588, fma(_e2538, _e2582, (_e2540 * _e2585)));
                                                                                        let _e2601 = (_e2600 * _e2560);
                                                                                        if (_e2601 < 0.0) {
                                                                                            phi_12382_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                        } else {
                                                                                            let _e2604 = fma(_e2600, _e2560, -0.009999999776482582);
                                                                                            let _e2612 = fma(_e2533, _e2542, -((_e2540 * _e2536)));
                                                                                            let _e2615 = fma(_e2536, _e2538, -((_e2542 * _e2530)));
                                                                                            let _e2618 = fma(_e2530, _e2540, -((_e2538 * _e2533)));
                                                                                            let _e2623 = (1.0 / sqrt(fma(_e2618, _e2618, fma(_e2612, _e2612, (_e2615 * _e2615)))));
                                                                                            phi_12382_ = type_47(_e682, (_e456.member_1 + vec3<f32>((_e681.x * _e2604), (_e681.y * _e2604), (_e681.z * _e2604))), vec3<f32>((_e2612 * _e2623), (_e2615 * _e2623), (_e2618 * _e2623)), vec2<f32>(_e2575, _e2592), _e2601, type_46(0u, false), select(select(u32(_e2524.member.w), 0u, (_e2524.member.w < 0.0)), 4294967295u, (_e2524.member.w > 4294967040.0)));
                                                                                        }
                                                                                        let _e2636 = phi_12382_;
                                                                                        phi_12385_ = _e2636;
                                                                                    }
                                                                                    let _e2638 = phi_12385_;
                                                                                    phi_12392_ = _e2638;
                                                                                }
                                                                                let _e2640 = phi_12392_;
                                                                                phi_12399_ = _e2640;
                                                                            }
                                                                            let _e2642 = phi_12399_;
                                                                            if (_e2642.member_4 < _e690.member_4) {
                                                                                if (_e2524.member_2.w == 1.0) {
                                                                                    if ((_e688 % 2u) == 0u) {
                                                                                        let _e2677 = (3u * (_e688 / 2u));
                                                                                        let _e2678 = (_e2677 < 192u);
                                                                                        if _e2678 {
                                                                                            let _e2682 = global_6.member.member_1[_e2677];
                                                                                            if _e2678 {
                                                                                                let _e2689 = (_e2677 + 1u);
                                                                                                if (_e2689 < 192u) {
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                let _e2694 = global_6.member.member_1[_e2689];
                                                                                                local_111 = type_50(vec2<f32>(_e2682.x, _e2682.y), vec2<f32>(_e2682.z, _e2682.w), vec2<f32>(_e2694.x, _e2694.y));
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                        let _e6845 = local_111;
                                                                                        phi_3493_ = _e6845;
                                                                                    } else {
                                                                                        let _e2653 = (3u * ((_e688 - 1u) / 2u));
                                                                                        let _e2654 = (_e2653 + 1u);
                                                                                        if (_e2654 < 192u) {
                                                                                            let _e2659 = global_6.member.member_1[_e2654];
                                                                                            let _e2663 = (_e2653 + 2u);
                                                                                            let _e2664 = (_e2663 < 192u);
                                                                                            if _e2664 {
                                                                                                let _e2668 = global_6.member.member_1[_e2663];
                                                                                                if _e2664 {
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                local_112 = type_50(vec2<f32>(_e2659.z, _e2659.w), vec2<f32>(_e2668.x, _e2668.y), vec2<f32>(_e2668.z, _e2668.w));
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                        let _e6847 = local_112;
                                                                                        phi_3493_ = _e6847;
                                                                                    }
                                                                                    let _e2700 = phi_3493_;
                                                                                    let _e2721 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e2700.member_2.x - _e2700.member.x), _e690.member_3.y, fma((_e2700.member_1.x - _e2700.member.x), _e690.member_3.x, _e2700.member.x)), fma((_e2700.member_2.y - _e2700.member.y), _e690.member_3.y, fma((_e2700.member_1.y - _e2700.member.y), _e690.member_3.x, _e2700.member.y))), 0.0);
                                                                                    phi_3531_ = (_e2721.w == 1.0);
                                                                                } else {
                                                                                    phi_3531_ = true;
                                                                                }
                                                                                let _e2725 = phi_3531_;
                                                                                if _e2725 {
                                                                                    _ = _e2642.member_5;
                                                                                    phi_3541_ = type_47(_e2642.member, _e2642.member_1, _e2642.member_2, _e2642.member_3, _e2642.member_4, type_46(_e688, true), _e2642.member_6);
                                                                                } else {
                                                                                    phi_3541_ = _e690;
                                                                                }
                                                                                let _e2736 = phi_3541_;
                                                                                phi_3542_ = _e2736;
                                                                            } else {
                                                                                phi_3542_ = _e690;
                                                                            }
                                                                            let _e2738 = phi_3542_;
                                                                            local_115 = (_e688 + 1u);
                                                                            local_116 = _e2738;
                                                                        } else {
                                                                            break;
                                                                        }
                                                                        phi_3223_ = _e684;
                                                                        phi_3226_ = _e686;
                                                                        let _e6859 = local_115;
                                                                        phi_3228_ = _e6859;
                                                                        let _e6862 = local_116;
                                                                        phi_3230_ = _e6862;
                                                                        phi_3232_ = _e692;
                                                                        phi_3234_ = _e694;
                                                                    } else {
                                                                        phi_3553_ = _e684;
                                                                        phi_3556_ = _e686;
                                                                        phi_3558_ = 0u;
                                                                        phi_3560_ = _e690;
                                                                        phi_3562_ = _e692;
                                                                        phi_3564_ = _e694;
                                                                        phi_3566_ = true;
                                                                        loop {
                                                                            let _e699 = phi_3553_;
                                                                            let _e701 = phi_3556_;
                                                                            let _e703 = phi_3558_;
                                                                            let _e705 = phi_3560_;
                                                                            let _e707 = phi_3562_;
                                                                            let _e709 = phi_3564_;
                                                                            let _e711 = phi_3566_;
                                                                            local_113 = _e699;
                                                                            local_114 = _e701;
                                                                            local_117 = _e705;
                                                                            local_118 = _e707;
                                                                            local_119 = _e709;
                                                                            if _e711 {
                                                                                if (_e703 < 4096u) {
                                                                                    let _e716 = global_4.member.member[_e703];
                                                                                    let _e717 = (_e703 + 1u);
                                                                                    if (_e717 < 4096u) {
                                                                                        let _e722 = global_4.member.member[_e717];
                                                                                        if (_e716.x == _e722.x) {
                                                                                            phi_3594_ = (_e716.y == _e722.y);
                                                                                        } else {
                                                                                            phi_3594_ = false;
                                                                                        }
                                                                                        let _e732 = phi_3594_;
                                                                                        if _e732 {
                                                                                            phi_3599_ = (_e716.z == _e722.z);
                                                                                        } else {
                                                                                            phi_3599_ = false;
                                                                                        }
                                                                                        let _e735 = phi_3599_;
                                                                                        if _e735 {
                                                                                            let _e790 = select(select(u32(_e716.w), 0u, (_e716.w < 0.0)), 4294967295u, (_e716.w > 4294967040.0));
                                                                                            if (_e790 < 1365u) {
                                                                                                let _e795 = global_3.member.member[_e790];
                                                                                                let _e801 = (_e795.member_1.x - _e795.member.x);
                                                                                                let _e804 = (_e795.member_1.y - _e795.member.y);
                                                                                                let _e807 = (_e795.member_1.z - _e795.member.z);
                                                                                                let _e809 = (_e795.member_2.x - _e795.member.x);
                                                                                                let _e811 = (_e795.member_2.y - _e795.member.y);
                                                                                                let _e813 = (_e795.member_2.z - _e795.member.z);
                                                                                                let _e818 = fma(_e681.y, _e813, -((_e811 * _e681.z)));
                                                                                                let _e822 = fma(_e681.z, _e809, -((_e813 * _e681.x)));
                                                                                                let _e825 = fma(_e681.x, _e811, -((_e809 * _e681.y)));
                                                                                                let _e828 = fma(_e807, _e825, fma(_e801, _e818, (_e804 * _e822)));
                                                                                                if (abs(_e828) < 1.1920928955078125e-7) {
                                                                                                    phi_12769_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                } else {
                                                                                                    let _e831 = (1.0 / _e828);
                                                                                                    let _e833 = (_e456.member_1.x - _e795.member.x);
                                                                                                    let _e835 = (_e456.member_1.y - _e795.member.y);
                                                                                                    let _e837 = (_e456.member_1.z - _e795.member.z);
                                                                                                    let _e841 = (fma(_e837, _e825, fma(_e833, _e818, (_e835 * _e822))) * _e831);
                                                                                                    if (_e841 > -9.999999747378752e-5) {
                                                                                                        phi_12652_ = (_e841 < 0.0);
                                                                                                    } else {
                                                                                                        phi_12652_ = false;
                                                                                                    }
                                                                                                    let _e845 = phi_12652_;
                                                                                                    let _e846 = select(_e841, 1.1920928955078125e-7, _e845);
                                                                                                    if (_e846 < 0.0) {
                                                                                                        phi_12662_ = true;
                                                                                                    } else {
                                                                                                        phi_12662_ = (_e846 > 1.0);
                                                                                                    }
                                                                                                    let _e850 = phi_12662_;
                                                                                                    if _e850 {
                                                                                                        phi_12762_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                    } else {
                                                                                                        let _e853 = fma(_e835, _e807, -((_e804 * _e837)));
                                                                                                        let _e856 = fma(_e837, _e801, -((_e807 * _e833)));
                                                                                                        let _e859 = fma(_e833, _e804, -((_e801 * _e835)));
                                                                                                        let _e862 = fma(_e681.z, _e859, fma(_e681.x, _e853, (_e681.y * _e856)));
                                                                                                        let _e863 = (_e862 * _e831);
                                                                                                        if (_e863 < 0.0) {
                                                                                                            phi_12691_ = true;
                                                                                                        } else {
                                                                                                            phi_12691_ = (fma(_e862, _e831, _e846) > 1.0);
                                                                                                        }
                                                                                                        let _e868 = phi_12691_;
                                                                                                        if _e868 {
                                                                                                            phi_12755_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                        } else {
                                                                                                            let _e871 = fma(_e813, _e859, fma(_e809, _e853, (_e811 * _e856)));
                                                                                                            let _e872 = (_e871 * _e831);
                                                                                                            if (_e872 < 0.0) {
                                                                                                                phi_12752_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                            } else {
                                                                                                                let _e875 = fma(_e871, _e831, -0.009999999776482582);
                                                                                                                let _e883 = fma(_e804, _e813, -((_e811 * _e807)));
                                                                                                                let _e886 = fma(_e807, _e809, -((_e813 * _e801)));
                                                                                                                let _e889 = fma(_e801, _e811, -((_e809 * _e804)));
                                                                                                                let _e894 = (1.0 / sqrt(fma(_e889, _e889, fma(_e883, _e883, (_e886 * _e886)))));
                                                                                                                phi_12752_ = type_47(_e682, (_e456.member_1 + vec3<f32>((_e681.x * _e875), (_e681.y * _e875), (_e681.z * _e875))), vec3<f32>((_e883 * _e894), (_e886 * _e894), (_e889 * _e894)), vec2<f32>(_e846, _e863), _e872, type_46(0u, false), select(select(u32(_e795.member.w), 0u, (_e795.member.w < 0.0)), 4294967295u, (_e795.member.w > 4294967040.0)));
                                                                                                            }
                                                                                                            let _e907 = phi_12752_;
                                                                                                            phi_12755_ = _e907;
                                                                                                        }
                                                                                                        let _e909 = phi_12755_;
                                                                                                        phi_12762_ = _e909;
                                                                                                    }
                                                                                                    let _e911 = phi_12762_;
                                                                                                    phi_12769_ = _e911;
                                                                                                }
                                                                                                let _e913 = phi_12769_;
                                                                                                if (_e913.member_4 < _e705.member_4) {
                                                                                                    if (_e795.member_2.w == 1.0) {
                                                                                                        if ((_e790 % 2u) == 0u) {
                                                                                                            let _e948 = (3u * (_e790 / 2u));
                                                                                                            let _e949 = (_e948 < 2047u);
                                                                                                            if _e949 {
                                                                                                                let _e953 = global_6.member.member[_e948];
                                                                                                                if _e949 {
                                                                                                                    let _e960 = (_e948 + 1u);
                                                                                                                    if (_e960 < 2047u) {
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e965 = global_6.member.member[_e960];
                                                                                                                    local_38 = type_50(vec2<f32>(_e953.x, _e953.y), vec2<f32>(_e953.z, _e953.w), vec2<f32>(_e965.x, _e965.y));
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                            } else {
                                                                                                                break;
                                                                                                            }
                                                                                                            let _e6555 = local_38;
                                                                                                            phi_3748_ = _e6555;
                                                                                                        } else {
                                                                                                            let _e924 = (3u * ((_e790 - 1u) / 2u));
                                                                                                            let _e925 = (_e924 + 1u);
                                                                                                            if (_e925 < 2047u) {
                                                                                                                let _e930 = global_6.member.member[_e925];
                                                                                                                let _e934 = (_e924 + 2u);
                                                                                                                let _e935 = (_e934 < 2047u);
                                                                                                                if _e935 {
                                                                                                                    let _e939 = global_6.member.member[_e934];
                                                                                                                    if _e935 {
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    local_39 = type_50(vec2<f32>(_e930.z, _e930.w), vec2<f32>(_e939.x, _e939.y), vec2<f32>(_e939.z, _e939.w));
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                            } else {
                                                                                                                break;
                                                                                                            }
                                                                                                            let _e6557 = local_39;
                                                                                                            phi_3748_ = _e6557;
                                                                                                        }
                                                                                                        let _e971 = phi_3748_;
                                                                                                        let _e992 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e971.member_2.x - _e971.member.x), _e705.member_3.y, fma((_e971.member_1.x - _e971.member.x), _e705.member_3.x, _e971.member.x)), fma((_e971.member_2.y - _e971.member.y), _e705.member_3.y, fma((_e971.member_1.y - _e971.member.y), _e705.member_3.x, _e971.member.y))), 0.0);
                                                                                                        phi_3896_ = (_e992.w == 1.0);
                                                                                                    } else {
                                                                                                        phi_3896_ = true;
                                                                                                    }
                                                                                                    let _e996 = phi_3896_;
                                                                                                    if _e996 {
                                                                                                        _ = _e913.member_5;
                                                                                                        phi_3906_ = type_47(_e913.member, _e913.member_1, _e913.member_2, _e913.member_3, _e913.member_4, type_46(_e790, false), _e913.member_6);
                                                                                                    } else {
                                                                                                        phi_3906_ = _e705;
                                                                                                    }
                                                                                                    let _e1007 = phi_3906_;
                                                                                                    phi_3907_ = _e1007;
                                                                                                } else {
                                                                                                    phi_3907_ = _e705;
                                                                                                }
                                                                                                let _e1009 = phi_3907_;
                                                                                                local_40 = select(select(u32(_e722.w), 0u, (_e722.w < 0.0)), 4294967295u, (_e722.w > 4294967040.0));
                                                                                                local_41 = _e1009;
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                            let _e6563 = local_40;
                                                                                            phi_3559_ = _e6563;
                                                                                            let _e6566 = local_41;
                                                                                            phi_3561_ = _e6566;
                                                                                        } else {
                                                                                            let _e743 = ((_e716.x - _e456.member_1.x) / _e681.x);
                                                                                            let _e745 = ((_e716.y - _e456.member_1.y) / _e681.y);
                                                                                            let _e747 = ((_e716.z - _e456.member_1.z) / _e681.z);
                                                                                            let _e751 = ((_e722.x - _e456.member_1.x) / _e681.x);
                                                                                            let _e752 = ((_e722.y - _e456.member_1.y) / _e681.y);
                                                                                            let _e753 = ((_e722.z - _e456.member_1.z) / _e681.z);
                                                                                            let _e761 = max(max(min(_e743, _e751), min(_e745, _e752)), min(_e747, _e753));
                                                                                            let _e763 = min(min(max(_e743, _e751), max(_e745, _e752)), max(_e747, _e753));
                                                                                            if (_e761 <= _e763) {
                                                                                                phi_12994_ = (_e763 > 0.0);
                                                                                            } else {
                                                                                                phi_12994_ = false;
                                                                                            }
                                                                                            let _e767 = phi_12994_;
                                                                                            if (select(3.4028234663852886e38, _e761, _e767) < _e705.member_4) {
                                                                                                phi_3952_ = select(select(u32(_e716.w), 0u, (_e716.w < 0.0)), 4294967295u, (_e716.w > 4294967040.0));
                                                                                            } else {
                                                                                                phi_3952_ = select(select(u32(_e722.w), 0u, (_e722.w < 0.0)), 4294967295u, (_e722.w > 4294967040.0));
                                                                                            }
                                                                                            let _e784 = phi_3952_;
                                                                                            phi_3559_ = _e784;
                                                                                            phi_3561_ = _e705;
                                                                                        }
                                                                                        let _e1017 = phi_3559_;
                                                                                        let _e1019 = phi_3561_;
                                                                                        let _e1020 = (_e1017 == 0u);
                                                                                        local_33 = _e1017;
                                                                                        local_34 = _e1019;
                                                                                        if _e1020 {
                                                                                            if (_e1019.member_4 < 1000.0) {
                                                                                                if (_e1019.member_6 < 64u) {
                                                                                                    let _e1029 = global_9.member.member[_e1019.member_6].member;
                                                                                                    if (_e1029.w == 1.0) {
                                                                                                        switch select(0, 1, _e1019.member_5.member_1) {
                                                                                                            case 0: {
                                                                                                                if ((_e1019.member_5.member % 2u) == 0u) {
                                                                                                                    let _e1122 = (3u * (_e1019.member_5.member / 2u));
                                                                                                                    let _e1123 = (_e1122 < 2047u);
                                                                                                                    if _e1123 {
                                                                                                                        let _e1127 = global_6.member.member[_e1122];
                                                                                                                        if _e1123 {
                                                                                                                            let _e1134 = (_e1122 + 1u);
                                                                                                                            if (_e1134 < 2047u) {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            let _e1139 = global_6.member.member[_e1134];
                                                                                                                            local_44 = type_50(vec2<f32>(_e1127.x, _e1127.y), vec2<f32>(_e1127.z, _e1127.w), vec2<f32>(_e1139.x, _e1139.y));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6574 = local_44;
                                                                                                                    phi_4094_ = _e6574;
                                                                                                                } else {
                                                                                                                    let _e1098 = (3u * ((_e1019.member_5.member - 1u) / 2u));
                                                                                                                    let _e1099 = (_e1098 + 1u);
                                                                                                                    if (_e1099 < 2047u) {
                                                                                                                        let _e1104 = global_6.member.member[_e1099];
                                                                                                                        let _e1108 = (_e1098 + 2u);
                                                                                                                        let _e1109 = (_e1108 < 2047u);
                                                                                                                        if _e1109 {
                                                                                                                            let _e1113 = global_6.member.member[_e1108];
                                                                                                                            if _e1109 {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            local_45 = type_50(vec2<f32>(_e1104.z, _e1104.w), vec2<f32>(_e1113.x, _e1113.y), vec2<f32>(_e1113.z, _e1113.w));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6576 = local_45;
                                                                                                                    phi_4094_ = _e6576;
                                                                                                                }
                                                                                                                let _e1145 = phi_4094_;
                                                                                                                phi_4206_ = _e1145;
                                                                                                                break;
                                                                                                            }
                                                                                                            case 1: {
                                                                                                                if ((_e1019.member_5.member % 2u) == 0u) {
                                                                                                                    let _e1070 = (3u * (_e1019.member_5.member / 2u));
                                                                                                                    let _e1071 = (_e1070 < 192u);
                                                                                                                    if _e1071 {
                                                                                                                        let _e1075 = global_6.member.member_1[_e1070];
                                                                                                                        if _e1071 {
                                                                                                                            let _e1082 = (_e1070 + 1u);
                                                                                                                            if (_e1082 < 192u) {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            let _e1087 = global_6.member.member_1[_e1082];
                                                                                                                            local_42 = type_50(vec2<f32>(_e1075.x, _e1075.y), vec2<f32>(_e1075.z, _e1075.w), vec2<f32>(_e1087.x, _e1087.y));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6569 = local_42;
                                                                                                                    phi_4204_ = _e6569;
                                                                                                                } else {
                                                                                                                    let _e1046 = (3u * ((_e1019.member_5.member - 1u) / 2u));
                                                                                                                    let _e1047 = (_e1046 + 1u);
                                                                                                                    if (_e1047 < 192u) {
                                                                                                                        let _e1052 = global_6.member.member_1[_e1047];
                                                                                                                        let _e1056 = (_e1046 + 2u);
                                                                                                                        let _e1057 = (_e1056 < 192u);
                                                                                                                        if _e1057 {
                                                                                                                            let _e1061 = global_6.member.member_1[_e1056];
                                                                                                                            if _e1057 {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            local_43 = type_50(vec2<f32>(_e1052.z, _e1052.w), vec2<f32>(_e1061.x, _e1061.y), vec2<f32>(_e1061.z, _e1061.w));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6571 = local_43;
                                                                                                                    phi_4204_ = _e6571;
                                                                                                                }
                                                                                                                let _e1093 = phi_4204_;
                                                                                                                phi_4206_ = _e1093;
                                                                                                                break;
                                                                                                            }
                                                                                                            default: {
                                                                                                                break;
                                                                                                            }
                                                                                                        }
                                                                                                        let _e1147 = phi_4206_;
                                                                                                        let _e1168 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e1147.member_2.x - _e1147.member.x), _e1019.member_3.y, fma((_e1147.member_1.x - _e1147.member.x), _e1019.member_3.x, _e1147.member.x)), fma((_e1147.member_2.y - _e1147.member.y), _e1019.member_3.y, fma((_e1147.member_1.y - _e1147.member.y), _e1019.member_3.x, _e1147.member.y))), 0.0);
                                                                                                        phi_4258_ = vec3<f32>((_e1168.x * _e1029.x), (_e1168.y * _e1029.y), (_e1168.z * _e1029.z));
                                                                                                    } else {
                                                                                                        phi_4258_ = vec3<f32>(_e1029.x, _e1029.y, _e1029.z);
                                                                                                    }
                                                                                                    let _e1180 = phi_4258_;
                                                                                                    phi_4262_ = _e699;
                                                                                                    phi_4265_ = _e701;
                                                                                                    phi_4267_ = 0u;
                                                                                                    phi_4269_ = vec3<f32>(0.0, 0.0, 0.0);
                                                                                                    phi_4271_ = _e707;
                                                                                                    phi_4273_ = true;
                                                                                                    loop {
                                                                                                        let _e1182 = phi_4262_;
                                                                                                        let _e1184 = phi_4265_;
                                                                                                        let _e1186 = phi_4267_;
                                                                                                        let _e1188 = phi_4269_;
                                                                                                        let _e1190 = phi_4271_;
                                                                                                        let _e1192 = phi_4273_;
                                                                                                        local_108 = _e1182;
                                                                                                        local_109 = _e1184;
                                                                                                        local_110 = _e1190;
                                                                                                        if _e1192 {
                                                                                                            let _e1193 = (_e1186 < _e646);
                                                                                                            if _e1193 {
                                                                                                                if (_e1186 < 64u) {
                                                                                                                    let _e1198 = global_8.member.member[_e1186];
                                                                                                                    let _e1207 = (vec3<f32>(_e1198.member.x, _e1198.member.y, _e1198.member.z) - _e1019.member_1);
                                                                                                                    let _e1216 = (_e1207 * (1.0 / sqrt(fma(_e1207.z, _e1207.z, fma(_e1207.x, _e1207.x, (_e1207.y * _e1207.y))))));
                                                                                                                    let _e1217 = type_42(_e1019.member_1, _e1216);
                                                                                                                    let _e1219 = (_e1198.member.x - _e1019.member_1.x);
                                                                                                                    let _e1221 = (_e1198.member.y - _e1019.member_1.y);
                                                                                                                    let _e1223 = (_e1198.member.z - _e1019.member_1.z);
                                                                                                                    let _e1227 = sqrt(fma(_e1223, _e1223, fma(_e1219, _e1219, (_e1221 * _e1221))));
                                                                                                                    if (_e1198.member.w == 1.0) {
                                                                                                                        let _e1828 = (_e1019.member_1.x - _e1198.member.x);
                                                                                                                        let _e1829 = (_e1019.member_1.y - _e1198.member.y);
                                                                                                                        let _e1830 = (_e1019.member_1.z - _e1198.member.z);
                                                                                                                        let _e1834 = (_e1198.member_1.x - _e1198.member.x);
                                                                                                                        let _e1835 = (_e1198.member_1.y - _e1198.member.y);
                                                                                                                        let _e1836 = (_e1198.member_1.z - _e1198.member.z);
                                                                                                                        let _e1848 = (fma(_e1836, _e1830, fma(_e1834, _e1828, (_e1835 * _e1829))) / sqrt((fma(_e1836, _e1836, fma(_e1834, _e1834, (_e1835 * _e1835))) * fma(_e1830, _e1830, fma(_e1828, _e1828, (_e1829 * _e1829))))));
                                                                                                                        let _e1850 = abs(_e1848);
                                                                                                                        let _e1851 = (1.0 - _e1850);
                                                                                                                        let _e1854 = sqrt(select(_e1851, 0.0, (_e1851 < 0.0)));
                                                                                                                        let _e1861 = fma(fma(fma(fma(fma(fma(fma(-0.0012624911032617092, _e1850, 0.006670089904218912), _e1850, -0.01708812639117241), _e1850, 0.03089188039302826), _e1850, -0.050174303352832794), _e1850, 0.08897899091243744), _e1850, -0.21459880471229553), _e1850, 1.570796251296997);
                                                                                                                        if (_e1848 >= 0.0) {
                                                                                                                            phi_4384_ = (_e1861 * _e1854);
                                                                                                                        } else {
                                                                                                                            phi_4384_ = fma(-(_e1861), _e1854, 3.1415927410125732);
                                                                                                                        }
                                                                                                                        let _e1866 = phi_4384_;
                                                                                                                        let _e1870 = (1.0 - pow((_e1866 / _e1198.member_1.w), 2.0));
                                                                                                                        let _e1872 = select(_e1870, 0.0, (_e1870 < 0.0));
                                                                                                                        phi_4390_ = 0u;
                                                                                                                        phi_4393_ = _e1184;
                                                                                                                        phi_4395_ = _e1188;
                                                                                                                        phi_4397_ = true;
                                                                                                                        loop {
                                                                                                                            let _e1876 = phi_4390_;
                                                                                                                            let _e1878 = phi_4393_;
                                                                                                                            let _e1880 = phi_4395_;
                                                                                                                            let _e1882 = phi_4397_;
                                                                                                                            local_101 = _e1878;
                                                                                                                            local_102 = _e1880;
                                                                                                                            if _e1882 {
                                                                                                                                if (_e1876 < _e149) {
                                                                                                                                    if (_e1876 < 128u) {
                                                                                                                                        let _e2213 = global_5.member.member[_e1876];
                                                                                                                                        let _e2219 = (_e2213.member_1.x - _e2213.member.x);
                                                                                                                                        let _e2222 = (_e2213.member_1.y - _e2213.member.y);
                                                                                                                                        let _e2225 = (_e2213.member_1.z - _e2213.member.z);
                                                                                                                                        let _e2227 = (_e2213.member_2.x - _e2213.member.x);
                                                                                                                                        let _e2229 = (_e2213.member_2.y - _e2213.member.y);
                                                                                                                                        let _e2231 = (_e2213.member_2.z - _e2213.member.z);
                                                                                                                                        let _e2236 = fma(_e1216.y, _e2231, -((_e2229 * _e1216.z)));
                                                                                                                                        let _e2240 = fma(_e1216.z, _e2227, -((_e2231 * _e1216.x)));
                                                                                                                                        let _e2243 = fma(_e1216.x, _e2229, -((_e2227 * _e1216.y)));
                                                                                                                                        let _e2246 = fma(_e2225, _e2243, fma(_e2219, _e2236, (_e2222 * _e2240)));
                                                                                                                                        if (abs(_e2246) < 1.1920928955078125e-7) {
                                                                                                                                            phi_13322_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                        } else {
                                                                                                                                            let _e2249 = (1.0 / _e2246);
                                                                                                                                            let _e2250 = (_e1019.member_1.x - _e2213.member.x);
                                                                                                                                            let _e2251 = (_e1019.member_1.y - _e2213.member.y);
                                                                                                                                            let _e2252 = (_e1019.member_1.z - _e2213.member.z);
                                                                                                                                            let _e2256 = (fma(_e2252, _e2243, fma(_e2250, _e2236, (_e2251 * _e2240))) * _e2249);
                                                                                                                                            if (_e2256 > -9.999999747378752e-5) {
                                                                                                                                                phi_13205_ = (_e2256 < 0.0);
                                                                                                                                            } else {
                                                                                                                                                phi_13205_ = false;
                                                                                                                                            }
                                                                                                                                            let _e2260 = phi_13205_;
                                                                                                                                            let _e2261 = select(_e2256, 1.1920928955078125e-7, _e2260);
                                                                                                                                            if (_e2261 < 0.0) {
                                                                                                                                                phi_13215_ = true;
                                                                                                                                            } else {
                                                                                                                                                phi_13215_ = (_e2261 > 1.0);
                                                                                                                                            }
                                                                                                                                            let _e2265 = phi_13215_;
                                                                                                                                            if _e2265 {
                                                                                                                                                phi_13315_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                            } else {
                                                                                                                                                let _e2268 = fma(_e2251, _e2225, -((_e2222 * _e2252)));
                                                                                                                                                let _e2271 = fma(_e2252, _e2219, -((_e2225 * _e2250)));
                                                                                                                                                let _e2274 = fma(_e2250, _e2222, -((_e2219 * _e2251)));
                                                                                                                                                let _e2277 = fma(_e1216.z, _e2274, fma(_e1216.x, _e2268, (_e1216.y * _e2271)));
                                                                                                                                                let _e2278 = (_e2277 * _e2249);
                                                                                                                                                if (_e2278 < 0.0) {
                                                                                                                                                    phi_13244_ = true;
                                                                                                                                                } else {
                                                                                                                                                    phi_13244_ = (fma(_e2277, _e2249, _e2261) > 1.0);
                                                                                                                                                }
                                                                                                                                                let _e2283 = phi_13244_;
                                                                                                                                                if _e2283 {
                                                                                                                                                    phi_13308_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                } else {
                                                                                                                                                    let _e2286 = fma(_e2231, _e2274, fma(_e2227, _e2268, (_e2229 * _e2271)));
                                                                                                                                                    let _e2287 = (_e2286 * _e2249);
                                                                                                                                                    if (_e2287 < 0.0) {
                                                                                                                                                        phi_13305_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                    } else {
                                                                                                                                                        let _e2290 = fma(_e2286, _e2249, -0.009999999776482582);
                                                                                                                                                        let _e2298 = fma(_e2222, _e2231, -((_e2229 * _e2225)));
                                                                                                                                                        let _e2301 = fma(_e2225, _e2227, -((_e2231 * _e2219)));
                                                                                                                                                        let _e2304 = fma(_e2219, _e2229, -((_e2227 * _e2222)));
                                                                                                                                                        let _e2309 = (1.0 / sqrt(fma(_e2304, _e2304, fma(_e2298, _e2298, (_e2301 * _e2301)))));
                                                                                                                                                        phi_13305_ = type_47(_e1217, (_e1019.member_1 + vec3<f32>((_e1216.x * _e2290), (_e1216.y * _e2290), (_e1216.z * _e2290))), vec3<f32>((_e2298 * _e2309), (_e2301 * _e2309), (_e2304 * _e2309)), vec2<f32>(_e2261, _e2278), _e2287, type_46(0u, false), select(select(u32(_e2213.member.w), 0u, (_e2213.member.w < 0.0)), 4294967295u, (_e2213.member.w > 4294967040.0)));
                                                                                                                                                    }
                                                                                                                                                    let _e2322 = phi_13305_;
                                                                                                                                                    phi_13308_ = _e2322;
                                                                                                                                                }
                                                                                                                                                let _e2324 = phi_13308_;
                                                                                                                                                phi_13315_ = _e2324;
                                                                                                                                            }
                                                                                                                                            let _e2326 = phi_13315_;
                                                                                                                                            phi_13322_ = _e2326;
                                                                                                                                        }
                                                                                                                                        let _e2328 = phi_13322_;
                                                                                                                                        if (_e2328.member_4 < _e1227) {
                                                                                                                                            if (_e2213.member_2.w == 1.0) {
                                                                                                                                                if ((_e1876 % 2u) == 0u) {
                                                                                                                                                    let _e2362 = (3u * (_e1876 / 2u));
                                                                                                                                                    let _e2363 = (_e2362 < 192u);
                                                                                                                                                    if _e2363 {
                                                                                                                                                        let _e2367 = global_6.member.member_1[_e2362];
                                                                                                                                                        if _e2363 {
                                                                                                                                                            let _e2374 = (_e2362 + 1u);
                                                                                                                                                            if (_e2374 < 192u) {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            let _e2379 = global_6.member.member_1[_e2374];
                                                                                                                                                            local_91 = type_50(vec2<f32>(_e2367.x, _e2367.y), vec2<f32>(_e2367.z, _e2367.w), vec2<f32>(_e2379.x, _e2379.y));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e6771 = local_91;
                                                                                                                                                    phi_4656_ = _e6771;
                                                                                                                                                } else {
                                                                                                                                                    let _e2338 = (3u * ((_e1876 - 1u) / 2u));
                                                                                                                                                    let _e2339 = (_e2338 + 1u);
                                                                                                                                                    if (_e2339 < 192u) {
                                                                                                                                                        let _e2344 = global_6.member.member_1[_e2339];
                                                                                                                                                        let _e2348 = (_e2338 + 2u);
                                                                                                                                                        let _e2349 = (_e2348 < 192u);
                                                                                                                                                        if _e2349 {
                                                                                                                                                            let _e2353 = global_6.member.member_1[_e2348];
                                                                                                                                                            if _e2349 {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            local_92 = type_50(vec2<f32>(_e2344.z, _e2344.w), vec2<f32>(_e2353.x, _e2353.y), vec2<f32>(_e2353.z, _e2353.w));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e6773 = local_92;
                                                                                                                                                    phi_4656_ = _e6773;
                                                                                                                                                }
                                                                                                                                                let _e2385 = phi_4656_;
                                                                                                                                                let _e2406 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e2385.member_2.x - _e2385.member.x), _e2328.member_3.y, fma((_e2385.member_1.x - _e2385.member.x), _e2328.member_3.x, _e2385.member.x)), fma((_e2385.member_2.y - _e2385.member.y), _e2328.member_3.y, fma((_e2385.member_1.y - _e2385.member.y), _e2328.member_3.x, _e2385.member.y))), 0.0);
                                                                                                                                                phi_4694_ = (_e2406.w == 1.0);
                                                                                                                                            } else {
                                                                                                                                                phi_4694_ = true;
                                                                                                                                            }
                                                                                                                                            let _e2410 = phi_4694_;
                                                                                                                                            phi_4701_ = select(_e1878, true, _e2410);
                                                                                                                                            phi_4702_ = select(false, true, _e2410);
                                                                                                                                            phi_4703_ = select(true, false, _e2410);
                                                                                                                                        } else {
                                                                                                                                            phi_4701_ = _e1878;
                                                                                                                                            phi_4702_ = false;
                                                                                                                                            phi_4703_ = true;
                                                                                                                                        }
                                                                                                                                        let _e2415 = phi_4701_;
                                                                                                                                        let _e2417 = phi_4702_;
                                                                                                                                        let _e2419 = phi_4703_;
                                                                                                                                        local_94 = _e2415;
                                                                                                                                        if _e2419 {
                                                                                                                                            phi_4708_ = (_e1876 + 1u);
                                                                                                                                        } else {
                                                                                                                                            phi_4708_ = _e1876;
                                                                                                                                        }
                                                                                                                                        let _e2422 = phi_4708_;
                                                                                                                                        local_93 = _e2422;
                                                                                                                                        local_96 = select(false, true, _e2419);
                                                                                                                                        local_97 = select(_e2417, false, _e2419);
                                                                                                                                    } else {
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    let _e6781 = local_93;
                                                                                                                                    phi_4391_ = _e6781;
                                                                                                                                    let _e6784 = local_94;
                                                                                                                                    phi_4394_ = _e6784;
                                                                                                                                    let _e6789 = local_96;
                                                                                                                                    phi_5154_ = _e6789;
                                                                                                                                    let _e6792 = local_97;
                                                                                                                                    phi_5155_ = _e6792;
                                                                                                                                    phi_5156_ = false;
                                                                                                                                } else {
                                                                                                                                    phi_4721_ = 0u;
                                                                                                                                    phi_4724_ = _e1878;
                                                                                                                                    phi_4726_ = false;
                                                                                                                                    phi_4728_ = false;
                                                                                                                                    phi_4730_ = true;
                                                                                                                                    loop {
                                                                                                                                        let _e1885 = phi_4721_;
                                                                                                                                        let _e1887 = phi_4724_;
                                                                                                                                        let _e1889 = phi_4726_;
                                                                                                                                        let _e1891 = phi_4728_;
                                                                                                                                        let _e1893 = phi_4730_;
                                                                                                                                        local_95 = _e1887;
                                                                                                                                        local_98 = _e1891;
                                                                                                                                        local_99 = _e1889;
                                                                                                                                        if _e1893 {
                                                                                                                                            if (_e1885 < 4096u) {
                                                                                                                                                let _e1898 = global_4.member.member[_e1885];
                                                                                                                                                let _e1899 = (_e1885 + 1u);
                                                                                                                                                if (_e1899 < 4096u) {
                                                                                                                                                    let _e1904 = global_4.member.member[_e1899];
                                                                                                                                                    if (_e1898.x == _e1904.x) {
                                                                                                                                                        phi_4758_ = (_e1898.y == _e1904.y);
                                                                                                                                                    } else {
                                                                                                                                                        phi_4758_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e1914 = phi_4758_;
                                                                                                                                                    if _e1914 {
                                                                                                                                                        phi_4763_ = (_e1898.z == _e1904.z);
                                                                                                                                                    } else {
                                                                                                                                                        phi_4763_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e1917 = phi_4763_;
                                                                                                                                                    if _e1917 {
                                                                                                                                                        let _e1968 = select(select(u32(_e1898.w), 0u, (_e1898.w < 0.0)), 4294967295u, (_e1898.w > 4294967040.0));
                                                                                                                                                        if (_e1968 < 1365u) {
                                                                                                                                                            let _e1973 = global_3.member.member[_e1968];
                                                                                                                                                            let _e1979 = (_e1973.member_1.x - _e1973.member.x);
                                                                                                                                                            let _e1982 = (_e1973.member_1.y - _e1973.member.y);
                                                                                                                                                            let _e1985 = (_e1973.member_1.z - _e1973.member.z);
                                                                                                                                                            let _e1987 = (_e1973.member_2.x - _e1973.member.x);
                                                                                                                                                            let _e1989 = (_e1973.member_2.y - _e1973.member.y);
                                                                                                                                                            let _e1991 = (_e1973.member_2.z - _e1973.member.z);
                                                                                                                                                            let _e1996 = fma(_e1216.y, _e1991, -((_e1989 * _e1216.z)));
                                                                                                                                                            let _e2000 = fma(_e1216.z, _e1987, -((_e1991 * _e1216.x)));
                                                                                                                                                            let _e2003 = fma(_e1216.x, _e1989, -((_e1987 * _e1216.y)));
                                                                                                                                                            let _e2006 = fma(_e1985, _e2003, fma(_e1979, _e1996, (_e1982 * _e2000)));
                                                                                                                                                            if (abs(_e2006) < 1.1920928955078125e-7) {
                                                                                                                                                                phi_13677_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                            } else {
                                                                                                                                                                let _e2009 = (1.0 / _e2006);
                                                                                                                                                                let _e2010 = (_e1019.member_1.x - _e1973.member.x);
                                                                                                                                                                let _e2011 = (_e1019.member_1.y - _e1973.member.y);
                                                                                                                                                                let _e2012 = (_e1019.member_1.z - _e1973.member.z);
                                                                                                                                                                let _e2016 = (fma(_e2012, _e2003, fma(_e2010, _e1996, (_e2011 * _e2000))) * _e2009);
                                                                                                                                                                if (_e2016 > -9.999999747378752e-5) {
                                                                                                                                                                    phi_13560_ = (_e2016 < 0.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_13560_ = false;
                                                                                                                                                                }
                                                                                                                                                                let _e2020 = phi_13560_;
                                                                                                                                                                let _e2021 = select(_e2016, 1.1920928955078125e-7, _e2020);
                                                                                                                                                                if (_e2021 < 0.0) {
                                                                                                                                                                    phi_13570_ = true;
                                                                                                                                                                } else {
                                                                                                                                                                    phi_13570_ = (_e2021 > 1.0);
                                                                                                                                                                }
                                                                                                                                                                let _e2025 = phi_13570_;
                                                                                                                                                                if _e2025 {
                                                                                                                                                                    phi_13670_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                } else {
                                                                                                                                                                    let _e2028 = fma(_e2011, _e1985, -((_e1982 * _e2012)));
                                                                                                                                                                    let _e2031 = fma(_e2012, _e1979, -((_e1985 * _e2010)));
                                                                                                                                                                    let _e2034 = fma(_e2010, _e1982, -((_e1979 * _e2011)));
                                                                                                                                                                    let _e2037 = fma(_e1216.z, _e2034, fma(_e1216.x, _e2028, (_e1216.y * _e2031)));
                                                                                                                                                                    let _e2038 = (_e2037 * _e2009);
                                                                                                                                                                    if (_e2038 < 0.0) {
                                                                                                                                                                        phi_13599_ = true;
                                                                                                                                                                    } else {
                                                                                                                                                                        phi_13599_ = (fma(_e2037, _e2009, _e2021) > 1.0);
                                                                                                                                                                    }
                                                                                                                                                                    let _e2043 = phi_13599_;
                                                                                                                                                                    if _e2043 {
                                                                                                                                                                        phi_13663_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e2046 = fma(_e1991, _e2034, fma(_e1987, _e2028, (_e1989 * _e2031)));
                                                                                                                                                                        let _e2047 = (_e2046 * _e2009);
                                                                                                                                                                        if (_e2047 < 0.0) {
                                                                                                                                                                            phi_13660_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                        } else {
                                                                                                                                                                            let _e2050 = fma(_e2046, _e2009, -0.009999999776482582);
                                                                                                                                                                            let _e2058 = fma(_e1982, _e1991, -((_e1989 * _e1985)));
                                                                                                                                                                            let _e2061 = fma(_e1985, _e1987, -((_e1991 * _e1979)));
                                                                                                                                                                            let _e2064 = fma(_e1979, _e1989, -((_e1987 * _e1982)));
                                                                                                                                                                            let _e2069 = (1.0 / sqrt(fma(_e2064, _e2064, fma(_e2058, _e2058, (_e2061 * _e2061)))));
                                                                                                                                                                            phi_13660_ = type_47(_e1217, (_e1019.member_1 + vec3<f32>((_e1216.x * _e2050), (_e1216.y * _e2050), (_e1216.z * _e2050))), vec3<f32>((_e2058 * _e2069), (_e2061 * _e2069), (_e2064 * _e2069)), vec2<f32>(_e2021, _e2038), _e2047, type_46(0u, false), select(select(u32(_e1973.member.w), 0u, (_e1973.member.w < 0.0)), 4294967295u, (_e1973.member.w > 4294967040.0)));
                                                                                                                                                                        }
                                                                                                                                                                        let _e2082 = phi_13660_;
                                                                                                                                                                        phi_13663_ = _e2082;
                                                                                                                                                                    }
                                                                                                                                                                    let _e2084 = phi_13663_;
                                                                                                                                                                    phi_13670_ = _e2084;
                                                                                                                                                                }
                                                                                                                                                                let _e2086 = phi_13670_;
                                                                                                                                                                phi_13677_ = _e2086;
                                                                                                                                                            }
                                                                                                                                                            let _e2088 = phi_13677_;
                                                                                                                                                            if (_e2088.member_4 < _e1227) {
                                                                                                                                                                if (_e1973.member_2.w == 1.0) {
                                                                                                                                                                    if ((_e1968 % 2u) == 0u) {
                                                                                                                                                                        let _e2122 = (3u * (_e1968 / 2u));
                                                                                                                                                                        let _e2123 = (_e2122 < 2047u);
                                                                                                                                                                        if _e2123 {
                                                                                                                                                                            let _e2127 = global_6.member.member[_e2122];
                                                                                                                                                                            if _e2123 {
                                                                                                                                                                                let _e2134 = (_e2122 + 1u);
                                                                                                                                                                                if (_e2134 < 2047u) {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                let _e2139 = global_6.member.member[_e2134];
                                                                                                                                                                                local_85 = type_50(vec2<f32>(_e2127.x, _e2127.y), vec2<f32>(_e2127.z, _e2127.w), vec2<f32>(_e2139.x, _e2139.y));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e6739 = local_85;
                                                                                                                                                                        phi_4913_ = _e6739;
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e2098 = (3u * ((_e1968 - 1u) / 2u));
                                                                                                                                                                        let _e2099 = (_e2098 + 1u);
                                                                                                                                                                        if (_e2099 < 2047u) {
                                                                                                                                                                            let _e2104 = global_6.member.member[_e2099];
                                                                                                                                                                            let _e2108 = (_e2098 + 2u);
                                                                                                                                                                            let _e2109 = (_e2108 < 2047u);
                                                                                                                                                                            if _e2109 {
                                                                                                                                                                                let _e2113 = global_6.member.member[_e2108];
                                                                                                                                                                                if _e2109 {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                local_86 = type_50(vec2<f32>(_e2104.z, _e2104.w), vec2<f32>(_e2113.x, _e2113.y), vec2<f32>(_e2113.z, _e2113.w));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e6741 = local_86;
                                                                                                                                                                        phi_4913_ = _e6741;
                                                                                                                                                                    }
                                                                                                                                                                    let _e2145 = phi_4913_;
                                                                                                                                                                    let _e2166 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e2145.member_2.x - _e2145.member.x), _e2088.member_3.y, fma((_e2145.member_1.x - _e2145.member.x), _e2088.member_3.x, _e2145.member.x)), fma((_e2145.member_2.y - _e2145.member.y), _e2088.member_3.y, fma((_e2145.member_1.y - _e2145.member.y), _e2088.member_3.x, _e2145.member.y))), 0.0);
                                                                                                                                                                    phi_5061_ = (_e2166.w == 1.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_5061_ = true;
                                                                                                                                                                }
                                                                                                                                                                let _e2170 = phi_5061_;
                                                                                                                                                                phi_5068_ = select(_e1887, true, _e2170);
                                                                                                                                                                phi_5069_ = select(false, true, _e2170);
                                                                                                                                                                phi_5070_ = select(true, false, _e2170);
                                                                                                                                                            } else {
                                                                                                                                                                phi_5068_ = _e1887;
                                                                                                                                                                phi_5069_ = false;
                                                                                                                                                                phi_5070_ = true;
                                                                                                                                                            }
                                                                                                                                                            let _e2175 = phi_5068_;
                                                                                                                                                            let _e2177 = phi_5069_;
                                                                                                                                                            let _e2179 = phi_5070_;
                                                                                                                                                            local_88 = _e2175;
                                                                                                                                                            if _e2179 {
                                                                                                                                                                phi_5082_ = select(select(u32(_e1904.w), 0u, (_e1904.w < 0.0)), 4294967295u, (_e1904.w > 4294967040.0));
                                                                                                                                                            } else {
                                                                                                                                                                phi_5082_ = _e1885;
                                                                                                                                                            }
                                                                                                                                                            let _e2187 = phi_5082_;
                                                                                                                                                            local_87 = _e2187;
                                                                                                                                                            local_89 = select(false, true, _e2179);
                                                                                                                                                            local_90 = select(_e2177, false, _e2179);
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        let _e6749 = local_87;
                                                                                                                                                        phi_4722_ = _e6749;
                                                                                                                                                        let _e6752 = local_88;
                                                                                                                                                        phi_5123_ = _e6752;
                                                                                                                                                        let _e6755 = local_89;
                                                                                                                                                        phi_5124_ = _e6755;
                                                                                                                                                        let _e6758 = local_90;
                                                                                                                                                        phi_5125_ = _e6758;
                                                                                                                                                    } else {
                                                                                                                                                        let _e1922 = ((_e1898.x - _e1019.member_1.x) / _e1216.x);
                                                                                                                                                        let _e1924 = ((_e1898.y - _e1019.member_1.y) / _e1216.y);
                                                                                                                                                        let _e1926 = ((_e1898.z - _e1019.member_1.z) / _e1216.z);
                                                                                                                                                        let _e1930 = ((_e1904.x - _e1019.member_1.x) / _e1216.x);
                                                                                                                                                        let _e1931 = ((_e1904.y - _e1019.member_1.y) / _e1216.y);
                                                                                                                                                        let _e1932 = ((_e1904.z - _e1019.member_1.z) / _e1216.z);
                                                                                                                                                        let _e1940 = max(max(min(_e1922, _e1930), min(_e1924, _e1931)), min(_e1926, _e1932));
                                                                                                                                                        let _e1942 = min(min(max(_e1922, _e1930), max(_e1924, _e1931)), max(_e1926, _e1932));
                                                                                                                                                        if (_e1940 <= _e1942) {
                                                                                                                                                            phi_13887_ = (_e1942 > 0.0);
                                                                                                                                                        } else {
                                                                                                                                                            phi_13887_ = false;
                                                                                                                                                        }
                                                                                                                                                        let _e1946 = phi_13887_;
                                                                                                                                                        if (select(3.4028234663852886e38, _e1940, _e1946) < _e1227) {
                                                                                                                                                            phi_5122_ = select(select(u32(_e1898.w), 0u, (_e1898.w < 0.0)), 4294967295u, (_e1898.w > 4294967040.0));
                                                                                                                                                        } else {
                                                                                                                                                            phi_5122_ = select(select(u32(_e1904.w), 0u, (_e1904.w < 0.0)), 4294967295u, (_e1904.w > 4294967040.0));
                                                                                                                                                        }
                                                                                                                                                        let _e1962 = phi_5122_;
                                                                                                                                                        phi_4722_ = _e1962;
                                                                                                                                                        phi_5123_ = _e1887;
                                                                                                                                                        phi_5124_ = true;
                                                                                                                                                        phi_5125_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e2191 = phi_4722_;
                                                                                                                                                    let _e2193 = phi_5123_;
                                                                                                                                                    let _e2195 = phi_5124_;
                                                                                                                                                    let _e2197 = phi_5125_;
                                                                                                                                                    local_80 = _e2191;
                                                                                                                                                    if _e2195 {
                                                                                                                                                        let _e2198 = (_e2191 == 0u);
                                                                                                                                                        phi_4725_ = select(_e2193, false, _e2198);
                                                                                                                                                        phi_5136_ = select(false, true, _e2198);
                                                                                                                                                        phi_5137_ = select(true, false, _e2198);
                                                                                                                                                    } else {
                                                                                                                                                        phi_4725_ = _e2193;
                                                                                                                                                        phi_5136_ = false;
                                                                                                                                                        phi_5137_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e2203 = phi_4725_;
                                                                                                                                                    let _e2205 = phi_5136_;
                                                                                                                                                    let _e2207 = phi_5137_;
                                                                                                                                                    local_81 = _e2203;
                                                                                                                                                    local_82 = _e2205;
                                                                                                                                                    local_83 = select(_e2197, false, _e2195);
                                                                                                                                                    local_84 = _e2207;
                                                                                                                                                } else {
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            continue;
                                                                                                                                        } else {
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                        continuing {
                                                                                                                                            let _e6713 = local_80;
                                                                                                                                            phi_4721_ = _e6713;
                                                                                                                                            let _e6716 = local_81;
                                                                                                                                            phi_4724_ = _e6716;
                                                                                                                                            let _e6719 = local_82;
                                                                                                                                            phi_4726_ = _e6719;
                                                                                                                                            let _e6722 = local_83;
                                                                                                                                            phi_4728_ = _e6722;
                                                                                                                                            let _e6725 = local_84;
                                                                                                                                            phi_4730_ = _e6725;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    phi_4391_ = _e1876;
                                                                                                                                    let _e6786 = local_95;
                                                                                                                                    phi_4394_ = _e6786;
                                                                                                                                    phi_5154_ = false;
                                                                                                                                    let _e6794 = local_98;
                                                                                                                                    phi_5155_ = _e6794;
                                                                                                                                    let _e6797 = local_99;
                                                                                                                                    phi_5156_ = _e6797;
                                                                                                                                }
                                                                                                                                let _e2426 = phi_4391_;
                                                                                                                                let _e2428 = phi_4394_;
                                                                                                                                let _e2430 = phi_5154_;
                                                                                                                                let _e2432 = phi_5155_;
                                                                                                                                let _e2434 = phi_5156_;
                                                                                                                                let _e2435 = select(_e2434, true, _e2432);
                                                                                                                                local_76 = _e2426;
                                                                                                                                local_77 = _e2428;
                                                                                                                                if _e2435 {
                                                                                                                                    if _e2428 {
                                                                                                                                        phi_5182_ = 0.0;
                                                                                                                                    } else {
                                                                                                                                        phi_5182_ = max(fma(_e1216.z, _e1019.member_2.z, fma(_e1216.x, _e1019.member_2.x, (_e1216.y * _e1019.member_2.y))), 0.0);
                                                                                                                                    }
                                                                                                                                    let _e2449 = phi_5182_;
                                                                                                                                    let _e2450 = (select(_e1872, 1.0, (_e1872 > 1.0)) * _e2449);
                                                                                                                                    phi_4396_ = vec3<f32>(fma(((_e2450 * _e1198.member_2.x) * _e1198.member_2.w), _e1180.x, _e1880.x), fma(((_e2450 * _e1198.member_2.y) * _e1198.member_2.w), _e1180.y, _e1880.y), fma(((_e2450 * _e1198.member_2.z) * _e1198.member_2.w), _e1180.z, _e1880.z));
                                                                                                                                } else {
                                                                                                                                    phi_4396_ = _e1880;
                                                                                                                                }
                                                                                                                                let _e2472 = phi_4396_;
                                                                                                                                local_78 = _e2472;
                                                                                                                                local_79 = select(select(_e2430, false, _e2432), false, _e2435);
                                                                                                                                continue;
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            continuing {
                                                                                                                                let _e6701 = local_76;
                                                                                                                                phi_4390_ = _e6701;
                                                                                                                                let _e6704 = local_77;
                                                                                                                                phi_4393_ = _e6704;
                                                                                                                                let _e6707 = local_78;
                                                                                                                                phi_4395_ = _e6707;
                                                                                                                                let _e6710 = local_79;
                                                                                                                                phi_4397_ = _e6710;
                                                                                                                            }
                                                                                                                        }
                                                                                                                        phi_6032_ = _e1182;
                                                                                                                        let _e6805 = local_101;
                                                                                                                        phi_6033_ = _e6805;
                                                                                                                        let _e6808 = local_102;
                                                                                                                        phi_6034_ = _e6808;
                                                                                                                    } else {
                                                                                                                        phi_5213_ = 0u;
                                                                                                                        phi_5216_ = _e1182;
                                                                                                                        phi_5218_ = _e1188;
                                                                                                                        phi_5220_ = true;
                                                                                                                        loop {
                                                                                                                            let _e1231 = phi_5213_;
                                                                                                                            let _e1233 = phi_5216_;
                                                                                                                            let _e1235 = phi_5218_;
                                                                                                                            let _e1237 = phi_5220_;
                                                                                                                            local_100 = _e1233;
                                                                                                                            local_103 = _e1235;
                                                                                                                            if _e1237 {
                                                                                                                                if (_e1231 < _e149) {
                                                                                                                                    if (_e1231 < 128u) {
                                                                                                                                        let _e1568 = global_5.member.member[_e1231];
                                                                                                                                        let _e1574 = (_e1568.member_1.x - _e1568.member.x);
                                                                                                                                        let _e1577 = (_e1568.member_1.y - _e1568.member.y);
                                                                                                                                        let _e1580 = (_e1568.member_1.z - _e1568.member.z);
                                                                                                                                        let _e1582 = (_e1568.member_2.x - _e1568.member.x);
                                                                                                                                        let _e1584 = (_e1568.member_2.y - _e1568.member.y);
                                                                                                                                        let _e1586 = (_e1568.member_2.z - _e1568.member.z);
                                                                                                                                        let _e1591 = fma(_e1216.y, _e1586, -((_e1584 * _e1216.z)));
                                                                                                                                        let _e1595 = fma(_e1216.z, _e1582, -((_e1586 * _e1216.x)));
                                                                                                                                        let _e1598 = fma(_e1216.x, _e1584, -((_e1582 * _e1216.y)));
                                                                                                                                        let _e1601 = fma(_e1580, _e1598, fma(_e1574, _e1591, (_e1577 * _e1595)));
                                                                                                                                        if (abs(_e1601) < 1.1920928955078125e-7) {
                                                                                                                                            phi_14109_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                        } else {
                                                                                                                                            let _e1604 = (1.0 / _e1601);
                                                                                                                                            let _e1605 = (_e1019.member_1.x - _e1568.member.x);
                                                                                                                                            let _e1606 = (_e1019.member_1.y - _e1568.member.y);
                                                                                                                                            let _e1607 = (_e1019.member_1.z - _e1568.member.z);
                                                                                                                                            let _e1611 = (fma(_e1607, _e1598, fma(_e1605, _e1591, (_e1606 * _e1595))) * _e1604);
                                                                                                                                            if (_e1611 > -9.999999747378752e-5) {
                                                                                                                                                phi_13992_ = (_e1611 < 0.0);
                                                                                                                                            } else {
                                                                                                                                                phi_13992_ = false;
                                                                                                                                            }
                                                                                                                                            let _e1615 = phi_13992_;
                                                                                                                                            let _e1616 = select(_e1611, 1.1920928955078125e-7, _e1615);
                                                                                                                                            if (_e1616 < 0.0) {
                                                                                                                                                phi_14002_ = true;
                                                                                                                                            } else {
                                                                                                                                                phi_14002_ = (_e1616 > 1.0);
                                                                                                                                            }
                                                                                                                                            let _e1620 = phi_14002_;
                                                                                                                                            if _e1620 {
                                                                                                                                                phi_14102_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                            } else {
                                                                                                                                                let _e1623 = fma(_e1606, _e1580, -((_e1577 * _e1607)));
                                                                                                                                                let _e1626 = fma(_e1607, _e1574, -((_e1580 * _e1605)));
                                                                                                                                                let _e1629 = fma(_e1605, _e1577, -((_e1574 * _e1606)));
                                                                                                                                                let _e1632 = fma(_e1216.z, _e1629, fma(_e1216.x, _e1623, (_e1216.y * _e1626)));
                                                                                                                                                let _e1633 = (_e1632 * _e1604);
                                                                                                                                                if (_e1633 < 0.0) {
                                                                                                                                                    phi_14031_ = true;
                                                                                                                                                } else {
                                                                                                                                                    phi_14031_ = (fma(_e1632, _e1604, _e1616) > 1.0);
                                                                                                                                                }
                                                                                                                                                let _e1638 = phi_14031_;
                                                                                                                                                if _e1638 {
                                                                                                                                                    phi_14095_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                } else {
                                                                                                                                                    let _e1641 = fma(_e1586, _e1629, fma(_e1582, _e1623, (_e1584 * _e1626)));
                                                                                                                                                    let _e1642 = (_e1641 * _e1604);
                                                                                                                                                    if (_e1642 < 0.0) {
                                                                                                                                                        phi_14092_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                    } else {
                                                                                                                                                        let _e1645 = fma(_e1641, _e1604, -0.009999999776482582);
                                                                                                                                                        let _e1653 = fma(_e1577, _e1586, -((_e1584 * _e1580)));
                                                                                                                                                        let _e1656 = fma(_e1580, _e1582, -((_e1586 * _e1574)));
                                                                                                                                                        let _e1659 = fma(_e1574, _e1584, -((_e1582 * _e1577)));
                                                                                                                                                        let _e1664 = (1.0 / sqrt(fma(_e1659, _e1659, fma(_e1653, _e1653, (_e1656 * _e1656)))));
                                                                                                                                                        phi_14092_ = type_47(_e1217, (_e1019.member_1 + vec3<f32>((_e1216.x * _e1645), (_e1216.y * _e1645), (_e1216.z * _e1645))), vec3<f32>((_e1653 * _e1664), (_e1656 * _e1664), (_e1659 * _e1664)), vec2<f32>(_e1616, _e1633), _e1642, type_46(0u, false), select(select(u32(_e1568.member.w), 0u, (_e1568.member.w < 0.0)), 4294967295u, (_e1568.member.w > 4294967040.0)));
                                                                                                                                                    }
                                                                                                                                                    let _e1677 = phi_14092_;
                                                                                                                                                    phi_14095_ = _e1677;
                                                                                                                                                }
                                                                                                                                                let _e1679 = phi_14095_;
                                                                                                                                                phi_14102_ = _e1679;
                                                                                                                                            }
                                                                                                                                            let _e1681 = phi_14102_;
                                                                                                                                            phi_14109_ = _e1681;
                                                                                                                                        }
                                                                                                                                        let _e1683 = phi_14109_;
                                                                                                                                        if (_e1683.member_4 < _e1227) {
                                                                                                                                            if (_e1568.member_2.w == 1.0) {
                                                                                                                                                if ((_e1231 % 2u) == 0u) {
                                                                                                                                                    let _e1717 = (3u * (_e1231 / 2u));
                                                                                                                                                    let _e1718 = (_e1717 < 192u);
                                                                                                                                                    if _e1718 {
                                                                                                                                                        let _e1722 = global_6.member.member_1[_e1717];
                                                                                                                                                        if _e1718 {
                                                                                                                                                            let _e1729 = (_e1717 + 1u);
                                                                                                                                                            if (_e1729 < 192u) {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            let _e1734 = global_6.member.member_1[_e1729];
                                                                                                                                                            local_67 = type_50(vec2<f32>(_e1722.x, _e1722.y), vec2<f32>(_e1722.z, _e1722.w), vec2<f32>(_e1734.x, _e1734.y));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e6669 = local_67;
                                                                                                                                                    phi_5479_ = _e6669;
                                                                                                                                                } else {
                                                                                                                                                    let _e1693 = (3u * ((_e1231 - 1u) / 2u));
                                                                                                                                                    let _e1694 = (_e1693 + 1u);
                                                                                                                                                    if (_e1694 < 192u) {
                                                                                                                                                        let _e1699 = global_6.member.member_1[_e1694];
                                                                                                                                                        let _e1703 = (_e1693 + 2u);
                                                                                                                                                        let _e1704 = (_e1703 < 192u);
                                                                                                                                                        if _e1704 {
                                                                                                                                                            let _e1708 = global_6.member.member_1[_e1703];
                                                                                                                                                            if _e1704 {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            local_68 = type_50(vec2<f32>(_e1699.z, _e1699.w), vec2<f32>(_e1708.x, _e1708.y), vec2<f32>(_e1708.z, _e1708.w));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e6671 = local_68;
                                                                                                                                                    phi_5479_ = _e6671;
                                                                                                                                                }
                                                                                                                                                let _e1740 = phi_5479_;
                                                                                                                                                let _e1761 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e1740.member_2.x - _e1740.member.x), _e1683.member_3.y, fma((_e1740.member_1.x - _e1740.member.x), _e1683.member_3.x, _e1740.member.x)), fma((_e1740.member_2.y - _e1740.member.y), _e1683.member_3.y, fma((_e1740.member_1.y - _e1740.member.y), _e1683.member_3.x, _e1740.member.y))), 0.0);
                                                                                                                                                phi_5517_ = (_e1761.w == 1.0);
                                                                                                                                            } else {
                                                                                                                                                phi_5517_ = true;
                                                                                                                                            }
                                                                                                                                            let _e1765 = phi_5517_;
                                                                                                                                            phi_5524_ = select(_e1233, true, _e1765);
                                                                                                                                            phi_5525_ = select(false, true, _e1765);
                                                                                                                                            phi_5526_ = select(true, false, _e1765);
                                                                                                                                        } else {
                                                                                                                                            phi_5524_ = _e1233;
                                                                                                                                            phi_5525_ = false;
                                                                                                                                            phi_5526_ = true;
                                                                                                                                        }
                                                                                                                                        let _e1770 = phi_5524_;
                                                                                                                                        let _e1772 = phi_5525_;
                                                                                                                                        let _e1774 = phi_5526_;
                                                                                                                                        local_70 = _e1770;
                                                                                                                                        if _e1774 {
                                                                                                                                            phi_5531_ = (_e1231 + 1u);
                                                                                                                                        } else {
                                                                                                                                            phi_5531_ = _e1231;
                                                                                                                                        }
                                                                                                                                        let _e1777 = phi_5531_;
                                                                                                                                        local_69 = _e1777;
                                                                                                                                        local_72 = select(false, true, _e1774);
                                                                                                                                        local_73 = select(_e1772, false, _e1774);
                                                                                                                                    } else {
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    let _e6679 = local_69;
                                                                                                                                    phi_5214_ = _e6679;
                                                                                                                                    let _e6682 = local_70;
                                                                                                                                    phi_5217_ = _e6682;
                                                                                                                                    let _e6687 = local_72;
                                                                                                                                    phi_5977_ = _e6687;
                                                                                                                                    let _e6690 = local_73;
                                                                                                                                    phi_5978_ = _e6690;
                                                                                                                                    phi_5979_ = false;
                                                                                                                                } else {
                                                                                                                                    phi_5544_ = 0u;
                                                                                                                                    phi_5547_ = _e1233;
                                                                                                                                    phi_5549_ = false;
                                                                                                                                    phi_5551_ = false;
                                                                                                                                    phi_5553_ = true;
                                                                                                                                    loop {
                                                                                                                                        let _e1240 = phi_5544_;
                                                                                                                                        let _e1242 = phi_5547_;
                                                                                                                                        let _e1244 = phi_5549_;
                                                                                                                                        let _e1246 = phi_5551_;
                                                                                                                                        let _e1248 = phi_5553_;
                                                                                                                                        local_71 = _e1242;
                                                                                                                                        local_74 = _e1246;
                                                                                                                                        local_75 = _e1244;
                                                                                                                                        if _e1248 {
                                                                                                                                            if (_e1240 < 4096u) {
                                                                                                                                                let _e1253 = global_4.member.member[_e1240];
                                                                                                                                                let _e1254 = (_e1240 + 1u);
                                                                                                                                                if (_e1254 < 4096u) {
                                                                                                                                                    let _e1259 = global_4.member.member[_e1254];
                                                                                                                                                    if (_e1253.x == _e1259.x) {
                                                                                                                                                        phi_5581_ = (_e1253.y == _e1259.y);
                                                                                                                                                    } else {
                                                                                                                                                        phi_5581_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e1269 = phi_5581_;
                                                                                                                                                    if _e1269 {
                                                                                                                                                        phi_5586_ = (_e1253.z == _e1259.z);
                                                                                                                                                    } else {
                                                                                                                                                        phi_5586_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e1272 = phi_5586_;
                                                                                                                                                    if _e1272 {
                                                                                                                                                        let _e1323 = select(select(u32(_e1253.w), 0u, (_e1253.w < 0.0)), 4294967295u, (_e1253.w > 4294967040.0));
                                                                                                                                                        if (_e1323 < 1365u) {
                                                                                                                                                            let _e1328 = global_3.member.member[_e1323];
                                                                                                                                                            let _e1334 = (_e1328.member_1.x - _e1328.member.x);
                                                                                                                                                            let _e1337 = (_e1328.member_1.y - _e1328.member.y);
                                                                                                                                                            let _e1340 = (_e1328.member_1.z - _e1328.member.z);
                                                                                                                                                            let _e1342 = (_e1328.member_2.x - _e1328.member.x);
                                                                                                                                                            let _e1344 = (_e1328.member_2.y - _e1328.member.y);
                                                                                                                                                            let _e1346 = (_e1328.member_2.z - _e1328.member.z);
                                                                                                                                                            let _e1351 = fma(_e1216.y, _e1346, -((_e1344 * _e1216.z)));
                                                                                                                                                            let _e1355 = fma(_e1216.z, _e1342, -((_e1346 * _e1216.x)));
                                                                                                                                                            let _e1358 = fma(_e1216.x, _e1344, -((_e1342 * _e1216.y)));
                                                                                                                                                            let _e1361 = fma(_e1340, _e1358, fma(_e1334, _e1351, (_e1337 * _e1355)));
                                                                                                                                                            if (abs(_e1361) < 1.1920928955078125e-7) {
                                                                                                                                                                phi_14464_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                            } else {
                                                                                                                                                                let _e1364 = (1.0 / _e1361);
                                                                                                                                                                let _e1365 = (_e1019.member_1.x - _e1328.member.x);
                                                                                                                                                                let _e1366 = (_e1019.member_1.y - _e1328.member.y);
                                                                                                                                                                let _e1367 = (_e1019.member_1.z - _e1328.member.z);
                                                                                                                                                                let _e1371 = (fma(_e1367, _e1358, fma(_e1365, _e1351, (_e1366 * _e1355))) * _e1364);
                                                                                                                                                                if (_e1371 > -9.999999747378752e-5) {
                                                                                                                                                                    phi_14347_ = (_e1371 < 0.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_14347_ = false;
                                                                                                                                                                }
                                                                                                                                                                let _e1375 = phi_14347_;
                                                                                                                                                                let _e1376 = select(_e1371, 1.1920928955078125e-7, _e1375);
                                                                                                                                                                if (_e1376 < 0.0) {
                                                                                                                                                                    phi_14357_ = true;
                                                                                                                                                                } else {
                                                                                                                                                                    phi_14357_ = (_e1376 > 1.0);
                                                                                                                                                                }
                                                                                                                                                                let _e1380 = phi_14357_;
                                                                                                                                                                if _e1380 {
                                                                                                                                                                    phi_14457_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                } else {
                                                                                                                                                                    let _e1383 = fma(_e1366, _e1340, -((_e1337 * _e1367)));
                                                                                                                                                                    let _e1386 = fma(_e1367, _e1334, -((_e1340 * _e1365)));
                                                                                                                                                                    let _e1389 = fma(_e1365, _e1337, -((_e1334 * _e1366)));
                                                                                                                                                                    let _e1392 = fma(_e1216.z, _e1389, fma(_e1216.x, _e1383, (_e1216.y * _e1386)));
                                                                                                                                                                    let _e1393 = (_e1392 * _e1364);
                                                                                                                                                                    if (_e1393 < 0.0) {
                                                                                                                                                                        phi_14386_ = true;
                                                                                                                                                                    } else {
                                                                                                                                                                        phi_14386_ = (fma(_e1392, _e1364, _e1376) > 1.0);
                                                                                                                                                                    }
                                                                                                                                                                    let _e1398 = phi_14386_;
                                                                                                                                                                    if _e1398 {
                                                                                                                                                                        phi_14450_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e1401 = fma(_e1346, _e1389, fma(_e1342, _e1383, (_e1344 * _e1386)));
                                                                                                                                                                        let _e1402 = (_e1401 * _e1364);
                                                                                                                                                                        if (_e1402 < 0.0) {
                                                                                                                                                                            phi_14447_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                        } else {
                                                                                                                                                                            let _e1405 = fma(_e1401, _e1364, -0.009999999776482582);
                                                                                                                                                                            let _e1413 = fma(_e1337, _e1346, -((_e1344 * _e1340)));
                                                                                                                                                                            let _e1416 = fma(_e1340, _e1342, -((_e1346 * _e1334)));
                                                                                                                                                                            let _e1419 = fma(_e1334, _e1344, -((_e1342 * _e1337)));
                                                                                                                                                                            let _e1424 = (1.0 / sqrt(fma(_e1419, _e1419, fma(_e1413, _e1413, (_e1416 * _e1416)))));
                                                                                                                                                                            phi_14447_ = type_47(_e1217, (_e1019.member_1 + vec3<f32>((_e1216.x * _e1405), (_e1216.y * _e1405), (_e1216.z * _e1405))), vec3<f32>((_e1413 * _e1424), (_e1416 * _e1424), (_e1419 * _e1424)), vec2<f32>(_e1376, _e1393), _e1402, type_46(0u, false), select(select(u32(_e1328.member.w), 0u, (_e1328.member.w < 0.0)), 4294967295u, (_e1328.member.w > 4294967040.0)));
                                                                                                                                                                        }
                                                                                                                                                                        let _e1437 = phi_14447_;
                                                                                                                                                                        phi_14450_ = _e1437;
                                                                                                                                                                    }
                                                                                                                                                                    let _e1439 = phi_14450_;
                                                                                                                                                                    phi_14457_ = _e1439;
                                                                                                                                                                }
                                                                                                                                                                let _e1441 = phi_14457_;
                                                                                                                                                                phi_14464_ = _e1441;
                                                                                                                                                            }
                                                                                                                                                            let _e1443 = phi_14464_;
                                                                                                                                                            if (_e1443.member_4 < _e1227) {
                                                                                                                                                                if (_e1328.member_2.w == 1.0) {
                                                                                                                                                                    if ((_e1323 % 2u) == 0u) {
                                                                                                                                                                        let _e1477 = (3u * (_e1323 / 2u));
                                                                                                                                                                        let _e1478 = (_e1477 < 2047u);
                                                                                                                                                                        if _e1478 {
                                                                                                                                                                            let _e1482 = global_6.member.member[_e1477];
                                                                                                                                                                            if _e1478 {
                                                                                                                                                                                let _e1489 = (_e1477 + 1u);
                                                                                                                                                                                if (_e1489 < 2047u) {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                let _e1494 = global_6.member.member[_e1489];
                                                                                                                                                                                local_61 = type_50(vec2<f32>(_e1482.x, _e1482.y), vec2<f32>(_e1482.z, _e1482.w), vec2<f32>(_e1494.x, _e1494.y));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e6637 = local_61;
                                                                                                                                                                        phi_5736_ = _e6637;
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e1453 = (3u * ((_e1323 - 1u) / 2u));
                                                                                                                                                                        let _e1454 = (_e1453 + 1u);
                                                                                                                                                                        if (_e1454 < 2047u) {
                                                                                                                                                                            let _e1459 = global_6.member.member[_e1454];
                                                                                                                                                                            let _e1463 = (_e1453 + 2u);
                                                                                                                                                                            let _e1464 = (_e1463 < 2047u);
                                                                                                                                                                            if _e1464 {
                                                                                                                                                                                let _e1468 = global_6.member.member[_e1463];
                                                                                                                                                                                if _e1464 {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                local_62 = type_50(vec2<f32>(_e1459.z, _e1459.w), vec2<f32>(_e1468.x, _e1468.y), vec2<f32>(_e1468.z, _e1468.w));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e6639 = local_62;
                                                                                                                                                                        phi_5736_ = _e6639;
                                                                                                                                                                    }
                                                                                                                                                                    let _e1500 = phi_5736_;
                                                                                                                                                                    let _e1521 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e1500.member_2.x - _e1500.member.x), _e1443.member_3.y, fma((_e1500.member_1.x - _e1500.member.x), _e1443.member_3.x, _e1500.member.x)), fma((_e1500.member_2.y - _e1500.member.y), _e1443.member_3.y, fma((_e1500.member_1.y - _e1500.member.y), _e1443.member_3.x, _e1500.member.y))), 0.0);
                                                                                                                                                                    phi_5884_ = (_e1521.w == 1.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_5884_ = true;
                                                                                                                                                                }
                                                                                                                                                                let _e1525 = phi_5884_;
                                                                                                                                                                phi_5891_ = select(_e1242, true, _e1525);
                                                                                                                                                                phi_5892_ = select(false, true, _e1525);
                                                                                                                                                                phi_5893_ = select(true, false, _e1525);
                                                                                                                                                            } else {
                                                                                                                                                                phi_5891_ = _e1242;
                                                                                                                                                                phi_5892_ = false;
                                                                                                                                                                phi_5893_ = true;
                                                                                                                                                            }
                                                                                                                                                            let _e1530 = phi_5891_;
                                                                                                                                                            let _e1532 = phi_5892_;
                                                                                                                                                            let _e1534 = phi_5893_;
                                                                                                                                                            local_64 = _e1530;
                                                                                                                                                            if _e1534 {
                                                                                                                                                                phi_5905_ = select(select(u32(_e1259.w), 0u, (_e1259.w < 0.0)), 4294967295u, (_e1259.w > 4294967040.0));
                                                                                                                                                            } else {
                                                                                                                                                                phi_5905_ = _e1240;
                                                                                                                                                            }
                                                                                                                                                            let _e1542 = phi_5905_;
                                                                                                                                                            local_63 = _e1542;
                                                                                                                                                            local_65 = select(false, true, _e1534);
                                                                                                                                                            local_66 = select(_e1532, false, _e1534);
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        let _e6647 = local_63;
                                                                                                                                                        phi_5545_ = _e6647;
                                                                                                                                                        let _e6650 = local_64;
                                                                                                                                                        phi_5946_ = _e6650;
                                                                                                                                                        let _e6653 = local_65;
                                                                                                                                                        phi_5947_ = _e6653;
                                                                                                                                                        let _e6656 = local_66;
                                                                                                                                                        phi_5948_ = _e6656;
                                                                                                                                                    } else {
                                                                                                                                                        let _e1277 = ((_e1253.x - _e1019.member_1.x) / _e1216.x);
                                                                                                                                                        let _e1279 = ((_e1253.y - _e1019.member_1.y) / _e1216.y);
                                                                                                                                                        let _e1281 = ((_e1253.z - _e1019.member_1.z) / _e1216.z);
                                                                                                                                                        let _e1285 = ((_e1259.x - _e1019.member_1.x) / _e1216.x);
                                                                                                                                                        let _e1286 = ((_e1259.y - _e1019.member_1.y) / _e1216.y);
                                                                                                                                                        let _e1287 = ((_e1259.z - _e1019.member_1.z) / _e1216.z);
                                                                                                                                                        let _e1295 = max(max(min(_e1277, _e1285), min(_e1279, _e1286)), min(_e1281, _e1287));
                                                                                                                                                        let _e1297 = min(min(max(_e1277, _e1285), max(_e1279, _e1286)), max(_e1281, _e1287));
                                                                                                                                                        if (_e1295 <= _e1297) {
                                                                                                                                                            phi_14674_ = (_e1297 > 0.0);
                                                                                                                                                        } else {
                                                                                                                                                            phi_14674_ = false;
                                                                                                                                                        }
                                                                                                                                                        let _e1301 = phi_14674_;
                                                                                                                                                        if (select(3.4028234663852886e38, _e1295, _e1301) < _e1227) {
                                                                                                                                                            phi_5945_ = select(select(u32(_e1253.w), 0u, (_e1253.w < 0.0)), 4294967295u, (_e1253.w > 4294967040.0));
                                                                                                                                                        } else {
                                                                                                                                                            phi_5945_ = select(select(u32(_e1259.w), 0u, (_e1259.w < 0.0)), 4294967295u, (_e1259.w > 4294967040.0));
                                                                                                                                                        }
                                                                                                                                                        let _e1317 = phi_5945_;
                                                                                                                                                        phi_5545_ = _e1317;
                                                                                                                                                        phi_5946_ = _e1242;
                                                                                                                                                        phi_5947_ = true;
                                                                                                                                                        phi_5948_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e1546 = phi_5545_;
                                                                                                                                                    let _e1548 = phi_5946_;
                                                                                                                                                    let _e1550 = phi_5947_;
                                                                                                                                                    let _e1552 = phi_5948_;
                                                                                                                                                    local_56 = _e1546;
                                                                                                                                                    if _e1550 {
                                                                                                                                                        let _e1553 = (_e1546 == 0u);
                                                                                                                                                        phi_5548_ = select(_e1548, false, _e1553);
                                                                                                                                                        phi_5959_ = select(false, true, _e1553);
                                                                                                                                                        phi_5960_ = select(true, false, _e1553);
                                                                                                                                                    } else {
                                                                                                                                                        phi_5548_ = _e1548;
                                                                                                                                                        phi_5959_ = false;
                                                                                                                                                        phi_5960_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e1558 = phi_5548_;
                                                                                                                                                    let _e1560 = phi_5959_;
                                                                                                                                                    let _e1562 = phi_5960_;
                                                                                                                                                    local_57 = _e1558;
                                                                                                                                                    local_58 = _e1560;
                                                                                                                                                    local_59 = select(_e1552, false, _e1550);
                                                                                                                                                    local_60 = _e1562;
                                                                                                                                                } else {
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            continue;
                                                                                                                                        } else {
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                        continuing {
                                                                                                                                            let _e6611 = local_56;
                                                                                                                                            phi_5544_ = _e6611;
                                                                                                                                            let _e6614 = local_57;
                                                                                                                                            phi_5547_ = _e6614;
                                                                                                                                            let _e6617 = local_58;
                                                                                                                                            phi_5549_ = _e6617;
                                                                                                                                            let _e6620 = local_59;
                                                                                                                                            phi_5551_ = _e6620;
                                                                                                                                            let _e6623 = local_60;
                                                                                                                                            phi_5553_ = _e6623;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    phi_5214_ = _e1231;
                                                                                                                                    let _e6684 = local_71;
                                                                                                                                    phi_5217_ = _e6684;
                                                                                                                                    phi_5977_ = false;
                                                                                                                                    let _e6692 = local_74;
                                                                                                                                    phi_5978_ = _e6692;
                                                                                                                                    let _e6695 = local_75;
                                                                                                                                    phi_5979_ = _e6695;
                                                                                                                                }
                                                                                                                                let _e1781 = phi_5214_;
                                                                                                                                let _e1783 = phi_5217_;
                                                                                                                                let _e1785 = phi_5977_;
                                                                                                                                let _e1787 = phi_5978_;
                                                                                                                                let _e1789 = phi_5979_;
                                                                                                                                let _e1790 = select(_e1789, true, _e1787);
                                                                                                                                local_52 = _e1781;
                                                                                                                                local_53 = _e1783;
                                                                                                                                if _e1790 {
                                                                                                                                    if _e1783 {
                                                                                                                                        phi_6005_ = 0.0;
                                                                                                                                    } else {
                                                                                                                                        phi_6005_ = max(fma(_e1216.z, _e1019.member_2.z, fma(_e1216.x, _e1019.member_2.x, (_e1216.y * _e1019.member_2.y))), 0.0);
                                                                                                                                    }
                                                                                                                                    let _e1804 = phi_6005_;
                                                                                                                                    phi_5219_ = vec3<f32>(fma(((_e1804 * _e1198.member_2.x) * _e1198.member_2.w), _e1180.x, _e1235.x), fma(((_e1804 * _e1198.member_2.y) * _e1198.member_2.w), _e1180.y, _e1235.y), fma(((_e1804 * _e1198.member_2.z) * _e1198.member_2.w), _e1180.z, _e1235.z));
                                                                                                                                } else {
                                                                                                                                    phi_5219_ = _e1235;
                                                                                                                                }
                                                                                                                                let _e1826 = phi_5219_;
                                                                                                                                local_54 = _e1826;
                                                                                                                                local_55 = select(select(_e1785, false, _e1787), false, _e1790);
                                                                                                                                continue;
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            continuing {
                                                                                                                                let _e6599 = local_52;
                                                                                                                                phi_5213_ = _e6599;
                                                                                                                                let _e6602 = local_53;
                                                                                                                                phi_5216_ = _e6602;
                                                                                                                                let _e6605 = local_54;
                                                                                                                                phi_5218_ = _e6605;
                                                                                                                                let _e6608 = local_55;
                                                                                                                                phi_5220_ = _e6608;
                                                                                                                            }
                                                                                                                        }
                                                                                                                        let _e6802 = local_100;
                                                                                                                        phi_6032_ = _e6802;
                                                                                                                        phi_6033_ = _e1184;
                                                                                                                        let _e6810 = local_103;
                                                                                                                        phi_6034_ = _e6810;
                                                                                                                    }
                                                                                                                    let _e2475 = phi_6032_;
                                                                                                                    let _e2477 = phi_6033_;
                                                                                                                    let _e2479 = phi_6034_;
                                                                                                                    local_104 = _e2475;
                                                                                                                    local_105 = _e2477;
                                                                                                                    local_106 = (_e1186 + 1u);
                                                                                                                    local_107 = _e2479;
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                                let _e6813 = local_104;
                                                                                                                phi_4263_ = _e6813;
                                                                                                                let _e6816 = local_105;
                                                                                                                phi_4266_ = _e6816;
                                                                                                                let _e6819 = local_106;
                                                                                                                phi_4268_ = _e6819;
                                                                                                                let _e6822 = local_107;
                                                                                                                phi_4270_ = _e6822;
                                                                                                            } else {
                                                                                                                phi_4263_ = _e1182;
                                                                                                                phi_4266_ = _e1184;
                                                                                                                phi_4268_ = _e1186;
                                                                                                                phi_4270_ = _e1188;
                                                                                                            }
                                                                                                            let _e2482 = phi_4263_;
                                                                                                            let _e2484 = phi_4266_;
                                                                                                            let _e2486 = phi_4268_;
                                                                                                            let _e2488 = phi_4270_;
                                                                                                            local_46 = _e2482;
                                                                                                            local_47 = _e2484;
                                                                                                            local_48 = _e2486;
                                                                                                            local_49 = _e2488;
                                                                                                            local_50 = select(_e1188, _e1190, vec3<bool>(_e1193));
                                                                                                            local_51 = select(false, true, _e1193);
                                                                                                            continue;
                                                                                                        } else {
                                                                                                            break;
                                                                                                        }
                                                                                                        continuing {
                                                                                                            let _e6581 = local_46;
                                                                                                            phi_4262_ = _e6581;
                                                                                                            let _e6584 = local_47;
                                                                                                            phi_4265_ = _e6584;
                                                                                                            let _e6587 = local_48;
                                                                                                            phi_4267_ = _e6587;
                                                                                                            let _e6590 = local_49;
                                                                                                            phi_4269_ = _e6590;
                                                                                                            let _e6593 = local_50;
                                                                                                            phi_4271_ = _e6593;
                                                                                                            let _e6596 = local_51;
                                                                                                            phi_4273_ = _e6596;
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                let _e6825 = local_108;
                                                                                                phi_6051_ = _e6825;
                                                                                                let _e6828 = local_109;
                                                                                                phi_6052_ = _e6828;
                                                                                                let _e6831 = local_110;
                                                                                                phi_6053_ = _e6831;
                                                                                            } else {
                                                                                                phi_6051_ = _e699;
                                                                                                phi_6052_ = _e701;
                                                                                                phi_6053_ = vec3<f32>(0.0, 0.0, 0.0);
                                                                                            }
                                                                                            let _e2493 = phi_6051_;
                                                                                            let _e2495 = phi_6052_;
                                                                                            let _e2497 = phi_6053_;
                                                                                            phi_3554_ = _e2493;
                                                                                            phi_3557_ = _e2495;
                                                                                            phi_3563_ = _e2497;
                                                                                            phi_3565_ = vec3<f32>(fma((_e2497.x * _e465.member_1.x), _e465.member_1.w, _e709.x), fma((_e2497.y * _e465.member_1.y), _e465.member_1.w, _e709.y), fma((_e2497.z * _e465.member_1.z), _e465.member_1.w, _e709.z));
                                                                                        } else {
                                                                                            phi_3554_ = _e699;
                                                                                            phi_3557_ = _e701;
                                                                                            phi_3563_ = _e707;
                                                                                            phi_3565_ = _e709;
                                                                                        }
                                                                                        let _e2512 = phi_3554_;
                                                                                        let _e2514 = phi_3557_;
                                                                                        let _e2516 = phi_3563_;
                                                                                        let _e2518 = phi_3565_;
                                                                                        local_31 = _e2512;
                                                                                        local_32 = _e2514;
                                                                                        local_35 = _e2516;
                                                                                        local_36 = _e2518;
                                                                                        local_37 = select(true, false, _e1020);
                                                                                    } else {
                                                                                        break;
                                                                                    }
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                                continue;
                                                                            } else {
                                                                                break;
                                                                            }
                                                                            continuing {
                                                                                let _e6523 = local_31;
                                                                                phi_3553_ = _e6523;
                                                                                let _e6526 = local_32;
                                                                                phi_3556_ = _e6526;
                                                                                let _e6529 = local_33;
                                                                                phi_3558_ = _e6529;
                                                                                let _e6532 = local_34;
                                                                                phi_3560_ = _e6532;
                                                                                let _e6535 = local_35;
                                                                                phi_3562_ = _e6535;
                                                                                let _e6538 = local_36;
                                                                                phi_3564_ = _e6538;
                                                                                let _e6541 = local_37;
                                                                                phi_3566_ = _e6541;
                                                                            }
                                                                        }
                                                                        let _e6853 = local_113;
                                                                        phi_3223_ = _e6853;
                                                                        let _e6856 = local_114;
                                                                        phi_3226_ = _e6856;
                                                                        phi_3228_ = _e688;
                                                                        let _e6864 = local_117;
                                                                        phi_3230_ = _e6864;
                                                                        let _e6867 = local_118;
                                                                        phi_3232_ = _e6867;
                                                                        let _e6870 = local_119;
                                                                        phi_3234_ = _e6870;
                                                                    }
                                                                    let _e2741 = phi_3223_;
                                                                    let _e2743 = phi_3226_;
                                                                    let _e2745 = phi_3228_;
                                                                    let _e2747 = phi_3230_;
                                                                    let _e2749 = phi_3232_;
                                                                    let _e2751 = phi_3234_;
                                                                    local_24 = _e2741;
                                                                    local_25 = _e2743;
                                                                    local_26 = _e2745;
                                                                    local_27 = _e2747;
                                                                    local_28 = _e2749;
                                                                    local_29 = _e2751;
                                                                    local_30 = select(false, true, _e697);
                                                                    continue;
                                                                } else {
                                                                    break;
                                                                }
                                                                continuing {
                                                                    let _e6502 = local_24;
                                                                    phi_3222_ = _e6502;
                                                                    let _e6505 = local_25;
                                                                    phi_3225_ = _e6505;
                                                                    let _e6508 = local_26;
                                                                    phi_3227_ = _e6508;
                                                                    let _e6511 = local_27;
                                                                    phi_3229_ = _e6511;
                                                                    let _e6514 = local_28;
                                                                    phi_3231_ = _e6514;
                                                                    let _e6517 = local_29;
                                                                    phi_3233_ = _e6517;
                                                                    let _e6520 = local_30;
                                                                    phi_3235_ = _e6520;
                                                                }
                                                            }
                                                            let _e6873 = local_120;
                                                            phi_6087_ = _e6873;
                                                            let _e6876 = local_121;
                                                            phi_6088_ = _e6876;
                                                            let _e6879 = local_122;
                                                            phi_6089_ = _e6879;
                                                            let _e6882 = local_123;
                                                            phi_6090_ = _e6882;
                                                        } else {
                                                            phi_6087_ = _e626;
                                                            phi_6088_ = _e628;
                                                            phi_6089_ = _e630;
                                                            phi_6090_ = _e638;
                                                        }
                                                        let _e2754 = phi_6087_;
                                                        let _e2756 = phi_6088_;
                                                        let _e2758 = phi_6089_;
                                                        let _e2760 = phi_6090_;
                                                        switch select(0, 1, _e456.member_5.member_1) {
                                                            case 0: {
                                                                if (_e456.member_5.member < 1365u) {
                                                                } else {
                                                                    break;
                                                                }
                                                                let _e2775 = global_3.member.member[_e456.member_5.member];
                                                                phi_6131_ = _e2775;
                                                                break;
                                                            }
                                                            case 1: {
                                                                if (_e456.member_5.member < 128u) {
                                                                } else {
                                                                    break;
                                                                }
                                                                let _e2770 = global_5.member.member[_e456.member_5.member];
                                                                phi_6131_ = _e2770;
                                                                break;
                                                            }
                                                            default: {
                                                                break;
                                                            }
                                                        }
                                                        let _e2777 = phi_6131_;
                                                        if (_e2777.member_1.w < 1.0) {
                                                            let _e2791 = (_e456.member_1 + vec3<f32>((0.10000000149011612 * _e456.member.member_1.x), (0.10000000149011612 * _e456.member.member_1.y), (0.10000000149011612 * _e456.member.member_1.z)));
                                                            let _e2797 = (_e456.member.member_1 * (1.0 / sqrt(fma(_e456.member.member_1.z, _e456.member.member_1.z, fma(_e456.member.member_1.x, _e456.member.member_1.x, (_e456.member.member_1.y * _e456.member.member_1.y))))));
                                                            let _e2798 = type_42(_e2791, _e2797);
                                                            phi_6153_ = _e620;
                                                            phi_6156_ = _e622;
                                                            phi_6158_ = 0u;
                                                            phi_6160_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                            phi_6162_ = _e624;
                                                            phi_6164_ = _e2760;
                                                            phi_6166_ = true;
                                                            loop {
                                                                let _e2800 = phi_6153_;
                                                                let _e2802 = phi_6156_;
                                                                let _e2804 = phi_6158_;
                                                                let _e2806 = phi_6160_;
                                                                let _e2808 = phi_6162_;
                                                                let _e2810 = phi_6164_;
                                                                let _e2812 = phi_6166_;
                                                                local_220 = _e2800;
                                                                local_221 = _e2802;
                                                                local_222 = _e2808;
                                                                local_223 = _e2810;
                                                                if _e2812 {
                                                                    let _e2813 = (_e2804 < _e149);
                                                                    if _e2813 {
                                                                        if (_e2804 < 128u) {
                                                                            let _e4641 = global_5.member.member[_e2804];
                                                                            let _e4647 = (_e4641.member_1.x - _e4641.member.x);
                                                                            let _e4650 = (_e4641.member_1.y - _e4641.member.y);
                                                                            let _e4653 = (_e4641.member_1.z - _e4641.member.z);
                                                                            let _e4655 = (_e4641.member_2.x - _e4641.member.x);
                                                                            let _e4657 = (_e4641.member_2.y - _e4641.member.y);
                                                                            let _e4659 = (_e4641.member_2.z - _e4641.member.z);
                                                                            let _e4664 = fma(_e2797.y, _e4659, -((_e4657 * _e2797.z)));
                                                                            let _e4668 = fma(_e2797.z, _e4655, -((_e4659 * _e2797.x)));
                                                                            let _e4671 = fma(_e2797.x, _e4657, -((_e4655 * _e2797.y)));
                                                                            let _e4674 = fma(_e4653, _e4671, fma(_e4647, _e4664, (_e4650 * _e4668)));
                                                                            if (abs(_e4674) < 1.1920928955078125e-7) {
                                                                                phi_14949_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                            } else {
                                                                                let _e4677 = (1.0 / _e4674);
                                                                                let _e4679 = (_e2791.x - _e4641.member.x);
                                                                                let _e4681 = (_e2791.y - _e4641.member.y);
                                                                                let _e4683 = (_e2791.z - _e4641.member.z);
                                                                                let _e4687 = (fma(_e4683, _e4671, fma(_e4679, _e4664, (_e4681 * _e4668))) * _e4677);
                                                                                if (_e4687 > -9.999999747378752e-5) {
                                                                                    phi_14832_ = (_e4687 < 0.0);
                                                                                } else {
                                                                                    phi_14832_ = false;
                                                                                }
                                                                                let _e4691 = phi_14832_;
                                                                                let _e4692 = select(_e4687, 1.1920928955078125e-7, _e4691);
                                                                                if (_e4692 < 0.0) {
                                                                                    phi_14842_ = true;
                                                                                } else {
                                                                                    phi_14842_ = (_e4692 > 1.0);
                                                                                }
                                                                                let _e4696 = phi_14842_;
                                                                                if _e4696 {
                                                                                    phi_14942_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                } else {
                                                                                    let _e4699 = fma(_e4681, _e4653, -((_e4650 * _e4683)));
                                                                                    let _e4702 = fma(_e4683, _e4647, -((_e4653 * _e4679)));
                                                                                    let _e4705 = fma(_e4679, _e4650, -((_e4647 * _e4681)));
                                                                                    let _e4708 = fma(_e2797.z, _e4705, fma(_e2797.x, _e4699, (_e2797.y * _e4702)));
                                                                                    let _e4709 = (_e4708 * _e4677);
                                                                                    if (_e4709 < 0.0) {
                                                                                        phi_14871_ = true;
                                                                                    } else {
                                                                                        phi_14871_ = (fma(_e4708, _e4677, _e4692) > 1.0);
                                                                                    }
                                                                                    let _e4714 = phi_14871_;
                                                                                    if _e4714 {
                                                                                        phi_14935_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                    } else {
                                                                                        let _e4717 = fma(_e4659, _e4705, fma(_e4655, _e4699, (_e4657 * _e4702)));
                                                                                        let _e4718 = (_e4717 * _e4677);
                                                                                        if (_e4718 < 0.0) {
                                                                                            phi_14932_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                        } else {
                                                                                            let _e4721 = fma(_e4717, _e4677, -0.009999999776482582);
                                                                                            let _e4729 = fma(_e4650, _e4659, -((_e4657 * _e4653)));
                                                                                            let _e4732 = fma(_e4653, _e4655, -((_e4659 * _e4647)));
                                                                                            let _e4735 = fma(_e4647, _e4657, -((_e4655 * _e4650)));
                                                                                            let _e4740 = (1.0 / sqrt(fma(_e4735, _e4735, fma(_e4729, _e4729, (_e4732 * _e4732)))));
                                                                                            phi_14932_ = type_47(_e2798, (_e2791 + vec3<f32>((_e2797.x * _e4721), (_e2797.y * _e4721), (_e2797.z * _e4721))), vec3<f32>((_e4729 * _e4740), (_e4732 * _e4740), (_e4735 * _e4740)), vec2<f32>(_e4692, _e4709), _e4718, type_46(0u, false), select(select(u32(_e4641.member.w), 0u, (_e4641.member.w < 0.0)), 4294967295u, (_e4641.member.w > 4294967040.0)));
                                                                                        }
                                                                                        let _e4753 = phi_14932_;
                                                                                        phi_14935_ = _e4753;
                                                                                    }
                                                                                    let _e4755 = phi_14935_;
                                                                                    phi_14942_ = _e4755;
                                                                                }
                                                                                let _e4757 = phi_14942_;
                                                                                phi_14949_ = _e4757;
                                                                            }
                                                                            let _e4759 = phi_14949_;
                                                                            if (_e4759.member_4 < _e2806.member_4) {
                                                                                if (_e4641.member_2.w == 1.0) {
                                                                                    if ((_e2804 % 2u) == 0u) {
                                                                                        let _e4794 = (3u * (_e2804 / 2u));
                                                                                        let _e4795 = (_e4794 < 192u);
                                                                                        if _e4795 {
                                                                                            let _e4799 = global_6.member.member_1[_e4794];
                                                                                            if _e4795 {
                                                                                                let _e4806 = (_e4794 + 1u);
                                                                                                if (_e4806 < 192u) {
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                let _e4811 = global_6.member.member_1[_e4806];
                                                                                                local_211 = type_50(vec2<f32>(_e4799.x, _e4799.y), vec2<f32>(_e4799.z, _e4799.w), vec2<f32>(_e4811.x, _e4811.y));
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                        let _e7229 = local_211;
                                                                                        phi_6424_ = _e7229;
                                                                                    } else {
                                                                                        let _e4770 = (3u * ((_e2804 - 1u) / 2u));
                                                                                        let _e4771 = (_e4770 + 1u);
                                                                                        if (_e4771 < 192u) {
                                                                                            let _e4776 = global_6.member.member_1[_e4771];
                                                                                            let _e4780 = (_e4770 + 2u);
                                                                                            let _e4781 = (_e4780 < 192u);
                                                                                            if _e4781 {
                                                                                                let _e4785 = global_6.member.member_1[_e4780];
                                                                                                if _e4781 {
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                local_212 = type_50(vec2<f32>(_e4776.z, _e4776.w), vec2<f32>(_e4785.x, _e4785.y), vec2<f32>(_e4785.z, _e4785.w));
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                        let _e7231 = local_212;
                                                                                        phi_6424_ = _e7231;
                                                                                    }
                                                                                    let _e4817 = phi_6424_;
                                                                                    let _e4838 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e4817.member_2.x - _e4817.member.x), _e2806.member_3.y, fma((_e4817.member_1.x - _e4817.member.x), _e2806.member_3.x, _e4817.member.x)), fma((_e4817.member_2.y - _e4817.member.y), _e2806.member_3.y, fma((_e4817.member_1.y - _e4817.member.y), _e2806.member_3.x, _e4817.member.y))), 0.0);
                                                                                    phi_6462_ = (_e4838.w == 1.0);
                                                                                } else {
                                                                                    phi_6462_ = true;
                                                                                }
                                                                                let _e4842 = phi_6462_;
                                                                                if _e4842 {
                                                                                    _ = _e4759.member_5;
                                                                                    phi_6472_ = type_47(_e4759.member, _e4759.member_1, _e4759.member_2, _e4759.member_3, _e4759.member_4, type_46(_e2804, true), _e4759.member_6);
                                                                                } else {
                                                                                    phi_6472_ = _e2806;
                                                                                }
                                                                                let _e4853 = phi_6472_;
                                                                                phi_6473_ = _e4853;
                                                                            } else {
                                                                                phi_6473_ = _e2806;
                                                                            }
                                                                            let _e4855 = phi_6473_;
                                                                            local_215 = (_e2804 + 1u);
                                                                            local_216 = _e4855;
                                                                        } else {
                                                                            break;
                                                                        }
                                                                        phi_6154_ = _e2800;
                                                                        phi_6157_ = _e2802;
                                                                        let _e7243 = local_215;
                                                                        phi_6159_ = _e7243;
                                                                        let _e7246 = local_216;
                                                                        phi_6161_ = _e7246;
                                                                        phi_6163_ = _e2808;
                                                                        phi_6165_ = _e2810;
                                                                    } else {
                                                                        phi_6484_ = _e2800;
                                                                        phi_6487_ = _e2802;
                                                                        phi_6489_ = 0u;
                                                                        phi_6491_ = _e2806;
                                                                        phi_6493_ = _e2808;
                                                                        phi_6495_ = _e2810;
                                                                        phi_6497_ = true;
                                                                        loop {
                                                                            let _e2815 = phi_6484_;
                                                                            let _e2817 = phi_6487_;
                                                                            let _e2819 = phi_6489_;
                                                                            let _e2821 = phi_6491_;
                                                                            let _e2823 = phi_6493_;
                                                                            let _e2825 = phi_6495_;
                                                                            let _e2827 = phi_6497_;
                                                                            local_213 = _e2815;
                                                                            local_214 = _e2817;
                                                                            local_217 = _e2821;
                                                                            local_218 = _e2823;
                                                                            local_219 = _e2825;
                                                                            if _e2827 {
                                                                                if (_e2819 < 4096u) {
                                                                                    let _e2832 = global_4.member.member[_e2819];
                                                                                    let _e2833 = (_e2819 + 1u);
                                                                                    if (_e2833 < 4096u) {
                                                                                        let _e2838 = global_4.member.member[_e2833];
                                                                                        if (_e2832.x == _e2838.x) {
                                                                                            phi_6525_ = (_e2832.y == _e2838.y);
                                                                                        } else {
                                                                                            phi_6525_ = false;
                                                                                        }
                                                                                        let _e2848 = phi_6525_;
                                                                                        if _e2848 {
                                                                                            phi_6530_ = (_e2832.z == _e2838.z);
                                                                                        } else {
                                                                                            phi_6530_ = false;
                                                                                        }
                                                                                        let _e2851 = phi_6530_;
                                                                                        if _e2851 {
                                                                                            let _e2906 = select(select(u32(_e2832.w), 0u, (_e2832.w < 0.0)), 4294967295u, (_e2832.w > 4294967040.0));
                                                                                            if (_e2906 < 1365u) {
                                                                                                let _e2911 = global_3.member.member[_e2906];
                                                                                                let _e2917 = (_e2911.member_1.x - _e2911.member.x);
                                                                                                let _e2920 = (_e2911.member_1.y - _e2911.member.y);
                                                                                                let _e2923 = (_e2911.member_1.z - _e2911.member.z);
                                                                                                let _e2925 = (_e2911.member_2.x - _e2911.member.x);
                                                                                                let _e2927 = (_e2911.member_2.y - _e2911.member.y);
                                                                                                let _e2929 = (_e2911.member_2.z - _e2911.member.z);
                                                                                                let _e2934 = fma(_e2797.y, _e2929, -((_e2927 * _e2797.z)));
                                                                                                let _e2938 = fma(_e2797.z, _e2925, -((_e2929 * _e2797.x)));
                                                                                                let _e2941 = fma(_e2797.x, _e2927, -((_e2925 * _e2797.y)));
                                                                                                let _e2944 = fma(_e2923, _e2941, fma(_e2917, _e2934, (_e2920 * _e2938)));
                                                                                                if (abs(_e2944) < 1.1920928955078125e-7) {
                                                                                                    phi_15319_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                } else {
                                                                                                    let _e2947 = (1.0 / _e2944);
                                                                                                    let _e2949 = (_e2791.x - _e2911.member.x);
                                                                                                    let _e2951 = (_e2791.y - _e2911.member.y);
                                                                                                    let _e2953 = (_e2791.z - _e2911.member.z);
                                                                                                    let _e2957 = (fma(_e2953, _e2941, fma(_e2949, _e2934, (_e2951 * _e2938))) * _e2947);
                                                                                                    if (_e2957 > -9.999999747378752e-5) {
                                                                                                        phi_15202_ = (_e2957 < 0.0);
                                                                                                    } else {
                                                                                                        phi_15202_ = false;
                                                                                                    }
                                                                                                    let _e2961 = phi_15202_;
                                                                                                    let _e2962 = select(_e2957, 1.1920928955078125e-7, _e2961);
                                                                                                    if (_e2962 < 0.0) {
                                                                                                        phi_15212_ = true;
                                                                                                    } else {
                                                                                                        phi_15212_ = (_e2962 > 1.0);
                                                                                                    }
                                                                                                    let _e2966 = phi_15212_;
                                                                                                    if _e2966 {
                                                                                                        phi_15312_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                    } else {
                                                                                                        let _e2969 = fma(_e2951, _e2923, -((_e2920 * _e2953)));
                                                                                                        let _e2972 = fma(_e2953, _e2917, -((_e2923 * _e2949)));
                                                                                                        let _e2975 = fma(_e2949, _e2920, -((_e2917 * _e2951)));
                                                                                                        let _e2978 = fma(_e2797.z, _e2975, fma(_e2797.x, _e2969, (_e2797.y * _e2972)));
                                                                                                        let _e2979 = (_e2978 * _e2947);
                                                                                                        if (_e2979 < 0.0) {
                                                                                                            phi_15241_ = true;
                                                                                                        } else {
                                                                                                            phi_15241_ = (fma(_e2978, _e2947, _e2962) > 1.0);
                                                                                                        }
                                                                                                        let _e2984 = phi_15241_;
                                                                                                        if _e2984 {
                                                                                                            phi_15305_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                        } else {
                                                                                                            let _e2987 = fma(_e2929, _e2975, fma(_e2925, _e2969, (_e2927 * _e2972)));
                                                                                                            let _e2988 = (_e2987 * _e2947);
                                                                                                            if (_e2988 < 0.0) {
                                                                                                                phi_15302_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                            } else {
                                                                                                                let _e2991 = fma(_e2987, _e2947, -0.009999999776482582);
                                                                                                                let _e2999 = fma(_e2920, _e2929, -((_e2927 * _e2923)));
                                                                                                                let _e3002 = fma(_e2923, _e2925, -((_e2929 * _e2917)));
                                                                                                                let _e3005 = fma(_e2917, _e2927, -((_e2925 * _e2920)));
                                                                                                                let _e3010 = (1.0 / sqrt(fma(_e3005, _e3005, fma(_e2999, _e2999, (_e3002 * _e3002)))));
                                                                                                                phi_15302_ = type_47(_e2798, (_e2791 + vec3<f32>((_e2797.x * _e2991), (_e2797.y * _e2991), (_e2797.z * _e2991))), vec3<f32>((_e2999 * _e3010), (_e3002 * _e3010), (_e3005 * _e3010)), vec2<f32>(_e2962, _e2979), _e2988, type_46(0u, false), select(select(u32(_e2911.member.w), 0u, (_e2911.member.w < 0.0)), 4294967295u, (_e2911.member.w > 4294967040.0)));
                                                                                                            }
                                                                                                            let _e3023 = phi_15302_;
                                                                                                            phi_15305_ = _e3023;
                                                                                                        }
                                                                                                        let _e3025 = phi_15305_;
                                                                                                        phi_15312_ = _e3025;
                                                                                                    }
                                                                                                    let _e3027 = phi_15312_;
                                                                                                    phi_15319_ = _e3027;
                                                                                                }
                                                                                                let _e3029 = phi_15319_;
                                                                                                if (_e3029.member_4 < _e2821.member_4) {
                                                                                                    if (_e2911.member_2.w == 1.0) {
                                                                                                        if ((_e2906 % 2u) == 0u) {
                                                                                                            let _e3064 = (3u * (_e2906 / 2u));
                                                                                                            let _e3065 = (_e3064 < 2047u);
                                                                                                            if _e3065 {
                                                                                                                let _e3069 = global_6.member.member[_e3064];
                                                                                                                if _e3065 {
                                                                                                                    let _e3076 = (_e3064 + 1u);
                                                                                                                    if (_e3076 < 2047u) {
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e3081 = global_6.member.member[_e3076];
                                                                                                                    local_138 = type_50(vec2<f32>(_e3069.x, _e3069.y), vec2<f32>(_e3069.z, _e3069.w), vec2<f32>(_e3081.x, _e3081.y));
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                            } else {
                                                                                                                break;
                                                                                                            }
                                                                                                            let _e6939 = local_138;
                                                                                                            phi_6679_ = _e6939;
                                                                                                        } else {
                                                                                                            let _e3040 = (3u * ((_e2906 - 1u) / 2u));
                                                                                                            let _e3041 = (_e3040 + 1u);
                                                                                                            if (_e3041 < 2047u) {
                                                                                                                let _e3046 = global_6.member.member[_e3041];
                                                                                                                let _e3050 = (_e3040 + 2u);
                                                                                                                let _e3051 = (_e3050 < 2047u);
                                                                                                                if _e3051 {
                                                                                                                    let _e3055 = global_6.member.member[_e3050];
                                                                                                                    if _e3051 {
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    local_139 = type_50(vec2<f32>(_e3046.z, _e3046.w), vec2<f32>(_e3055.x, _e3055.y), vec2<f32>(_e3055.z, _e3055.w));
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                            } else {
                                                                                                                break;
                                                                                                            }
                                                                                                            let _e6941 = local_139;
                                                                                                            phi_6679_ = _e6941;
                                                                                                        }
                                                                                                        let _e3087 = phi_6679_;
                                                                                                        let _e3108 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e3087.member_2.x - _e3087.member.x), _e2821.member_3.y, fma((_e3087.member_1.x - _e3087.member.x), _e2821.member_3.x, _e3087.member.x)), fma((_e3087.member_2.y - _e3087.member.y), _e2821.member_3.y, fma((_e3087.member_1.y - _e3087.member.y), _e2821.member_3.x, _e3087.member.y))), 0.0);
                                                                                                        phi_6827_ = (_e3108.w == 1.0);
                                                                                                    } else {
                                                                                                        phi_6827_ = true;
                                                                                                    }
                                                                                                    let _e3112 = phi_6827_;
                                                                                                    if _e3112 {
                                                                                                        _ = _e3029.member_5;
                                                                                                        phi_6837_ = type_47(_e3029.member, _e3029.member_1, _e3029.member_2, _e3029.member_3, _e3029.member_4, type_46(_e2906, false), _e3029.member_6);
                                                                                                    } else {
                                                                                                        phi_6837_ = _e2821;
                                                                                                    }
                                                                                                    let _e3123 = phi_6837_;
                                                                                                    phi_6838_ = _e3123;
                                                                                                } else {
                                                                                                    phi_6838_ = _e2821;
                                                                                                }
                                                                                                let _e3125 = phi_6838_;
                                                                                                local_140 = select(select(u32(_e2838.w), 0u, (_e2838.w < 0.0)), 4294967295u, (_e2838.w > 4294967040.0));
                                                                                                local_141 = _e3125;
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                            let _e6947 = local_140;
                                                                                            phi_6490_ = _e6947;
                                                                                            let _e6950 = local_141;
                                                                                            phi_6492_ = _e6950;
                                                                                        } else {
                                                                                            let _e2859 = ((_e2832.x - _e2791.x) / _e2797.x);
                                                                                            let _e2861 = ((_e2832.y - _e2791.y) / _e2797.y);
                                                                                            let _e2863 = ((_e2832.z - _e2791.z) / _e2797.z);
                                                                                            let _e2867 = ((_e2838.x - _e2791.x) / _e2797.x);
                                                                                            let _e2868 = ((_e2838.y - _e2791.y) / _e2797.y);
                                                                                            let _e2869 = ((_e2838.z - _e2791.z) / _e2797.z);
                                                                                            let _e2877 = max(max(min(_e2859, _e2867), min(_e2861, _e2868)), min(_e2863, _e2869));
                                                                                            let _e2879 = min(min(max(_e2859, _e2867), max(_e2861, _e2868)), max(_e2863, _e2869));
                                                                                            if (_e2877 <= _e2879) {
                                                                                                phi_15544_ = (_e2879 > 0.0);
                                                                                            } else {
                                                                                                phi_15544_ = false;
                                                                                            }
                                                                                            let _e2883 = phi_15544_;
                                                                                            if (select(3.4028234663852886e38, _e2877, _e2883) < _e2821.member_4) {
                                                                                                phi_6883_ = select(select(u32(_e2832.w), 0u, (_e2832.w < 0.0)), 4294967295u, (_e2832.w > 4294967040.0));
                                                                                            } else {
                                                                                                phi_6883_ = select(select(u32(_e2838.w), 0u, (_e2838.w < 0.0)), 4294967295u, (_e2838.w > 4294967040.0));
                                                                                            }
                                                                                            let _e2900 = phi_6883_;
                                                                                            phi_6490_ = _e2900;
                                                                                            phi_6492_ = _e2821;
                                                                                        }
                                                                                        let _e3133 = phi_6490_;
                                                                                        let _e3135 = phi_6492_;
                                                                                        let _e3136 = (_e3133 == 0u);
                                                                                        local_133 = _e3133;
                                                                                        local_134 = _e3135;
                                                                                        if _e3136 {
                                                                                            if (_e3135.member_4 < 1000.0) {
                                                                                                if (_e3135.member_6 < 64u) {
                                                                                                    let _e3145 = global_9.member.member[_e3135.member_6].member;
                                                                                                    if (_e3145.w == 1.0) {
                                                                                                        switch select(0, 1, _e3135.member_5.member_1) {
                                                                                                            case 0: {
                                                                                                                if ((_e3135.member_5.member % 2u) == 0u) {
                                                                                                                    let _e3238 = (3u * (_e3135.member_5.member / 2u));
                                                                                                                    let _e3239 = (_e3238 < 2047u);
                                                                                                                    if _e3239 {
                                                                                                                        let _e3243 = global_6.member.member[_e3238];
                                                                                                                        if _e3239 {
                                                                                                                            let _e3250 = (_e3238 + 1u);
                                                                                                                            if (_e3250 < 2047u) {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            let _e3255 = global_6.member.member[_e3250];
                                                                                                                            local_144 = type_50(vec2<f32>(_e3243.x, _e3243.y), vec2<f32>(_e3243.z, _e3243.w), vec2<f32>(_e3255.x, _e3255.y));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6958 = local_144;
                                                                                                                    phi_7025_ = _e6958;
                                                                                                                } else {
                                                                                                                    let _e3214 = (3u * ((_e3135.member_5.member - 1u) / 2u));
                                                                                                                    let _e3215 = (_e3214 + 1u);
                                                                                                                    if (_e3215 < 2047u) {
                                                                                                                        let _e3220 = global_6.member.member[_e3215];
                                                                                                                        let _e3224 = (_e3214 + 2u);
                                                                                                                        let _e3225 = (_e3224 < 2047u);
                                                                                                                        if _e3225 {
                                                                                                                            let _e3229 = global_6.member.member[_e3224];
                                                                                                                            if _e3225 {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            local_145 = type_50(vec2<f32>(_e3220.z, _e3220.w), vec2<f32>(_e3229.x, _e3229.y), vec2<f32>(_e3229.z, _e3229.w));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6960 = local_145;
                                                                                                                    phi_7025_ = _e6960;
                                                                                                                }
                                                                                                                let _e3261 = phi_7025_;
                                                                                                                phi_7137_ = _e3261;
                                                                                                                break;
                                                                                                            }
                                                                                                            case 1: {
                                                                                                                if ((_e3135.member_5.member % 2u) == 0u) {
                                                                                                                    let _e3186 = (3u * (_e3135.member_5.member / 2u));
                                                                                                                    let _e3187 = (_e3186 < 192u);
                                                                                                                    if _e3187 {
                                                                                                                        let _e3191 = global_6.member.member_1[_e3186];
                                                                                                                        if _e3187 {
                                                                                                                            let _e3198 = (_e3186 + 1u);
                                                                                                                            if (_e3198 < 192u) {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            let _e3203 = global_6.member.member_1[_e3198];
                                                                                                                            local_142 = type_50(vec2<f32>(_e3191.x, _e3191.y), vec2<f32>(_e3191.z, _e3191.w), vec2<f32>(_e3203.x, _e3203.y));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6953 = local_142;
                                                                                                                    phi_7135_ = _e6953;
                                                                                                                } else {
                                                                                                                    let _e3162 = (3u * ((_e3135.member_5.member - 1u) / 2u));
                                                                                                                    let _e3163 = (_e3162 + 1u);
                                                                                                                    if (_e3163 < 192u) {
                                                                                                                        let _e3168 = global_6.member.member_1[_e3163];
                                                                                                                        let _e3172 = (_e3162 + 2u);
                                                                                                                        let _e3173 = (_e3172 < 192u);
                                                                                                                        if _e3173 {
                                                                                                                            let _e3177 = global_6.member.member_1[_e3172];
                                                                                                                            if _e3173 {
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            local_143 = type_50(vec2<f32>(_e3168.z, _e3168.w), vec2<f32>(_e3177.x, _e3177.y), vec2<f32>(_e3177.z, _e3177.w));
                                                                                                                        } else {
                                                                                                                            break;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    let _e6955 = local_143;
                                                                                                                    phi_7135_ = _e6955;
                                                                                                                }
                                                                                                                let _e3209 = phi_7135_;
                                                                                                                phi_7137_ = _e3209;
                                                                                                                break;
                                                                                                            }
                                                                                                            default: {
                                                                                                                break;
                                                                                                            }
                                                                                                        }
                                                                                                        let _e3263 = phi_7137_;
                                                                                                        let _e3284 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e3263.member_2.x - _e3263.member.x), _e3135.member_3.y, fma((_e3263.member_1.x - _e3263.member.x), _e3135.member_3.x, _e3263.member.x)), fma((_e3263.member_2.y - _e3263.member.y), _e3135.member_3.y, fma((_e3263.member_1.y - _e3263.member.y), _e3135.member_3.x, _e3263.member.y))), 0.0);
                                                                                                        phi_7189_ = vec3<f32>((_e3284.x * _e3145.x), (_e3284.y * _e3145.y), (_e3284.z * _e3145.z));
                                                                                                    } else {
                                                                                                        phi_7189_ = vec3<f32>(_e3145.x, _e3145.y, _e3145.z);
                                                                                                    }
                                                                                                    let _e3296 = phi_7189_;
                                                                                                    phi_7193_ = _e2815;
                                                                                                    phi_7196_ = _e2817;
                                                                                                    phi_7198_ = 0u;
                                                                                                    phi_7200_ = vec3<f32>(0.0, 0.0, 0.0);
                                                                                                    phi_7202_ = _e2823;
                                                                                                    phi_7204_ = true;
                                                                                                    loop {
                                                                                                        let _e3298 = phi_7193_;
                                                                                                        let _e3300 = phi_7196_;
                                                                                                        let _e3302 = phi_7198_;
                                                                                                        let _e3304 = phi_7200_;
                                                                                                        let _e3306 = phi_7202_;
                                                                                                        let _e3308 = phi_7204_;
                                                                                                        local_208 = _e3298;
                                                                                                        local_209 = _e3300;
                                                                                                        local_210 = _e3306;
                                                                                                        if _e3308 {
                                                                                                            let _e3309 = (_e3302 < _e646);
                                                                                                            if _e3309 {
                                                                                                                if (_e3302 < 64u) {
                                                                                                                    let _e3314 = global_8.member.member[_e3302];
                                                                                                                    let _e3323 = (vec3<f32>(_e3314.member.x, _e3314.member.y, _e3314.member.z) - _e3135.member_1);
                                                                                                                    let _e3332 = (_e3323 * (1.0 / sqrt(fma(_e3323.z, _e3323.z, fma(_e3323.x, _e3323.x, (_e3323.y * _e3323.y))))));
                                                                                                                    let _e3333 = type_42(_e3135.member_1, _e3332);
                                                                                                                    let _e3335 = (_e3314.member.x - _e3135.member_1.x);
                                                                                                                    let _e3337 = (_e3314.member.y - _e3135.member_1.y);
                                                                                                                    let _e3339 = (_e3314.member.z - _e3135.member_1.z);
                                                                                                                    let _e3343 = sqrt(fma(_e3339, _e3339, fma(_e3335, _e3335, (_e3337 * _e3337))));
                                                                                                                    if (_e3314.member.w == 1.0) {
                                                                                                                        let _e3944 = (_e3135.member_1.x - _e3314.member.x);
                                                                                                                        let _e3945 = (_e3135.member_1.y - _e3314.member.y);
                                                                                                                        let _e3946 = (_e3135.member_1.z - _e3314.member.z);
                                                                                                                        let _e3950 = (_e3314.member_1.x - _e3314.member.x);
                                                                                                                        let _e3951 = (_e3314.member_1.y - _e3314.member.y);
                                                                                                                        let _e3952 = (_e3314.member_1.z - _e3314.member.z);
                                                                                                                        let _e3964 = (fma(_e3952, _e3946, fma(_e3950, _e3944, (_e3951 * _e3945))) / sqrt((fma(_e3952, _e3952, fma(_e3950, _e3950, (_e3951 * _e3951))) * fma(_e3946, _e3946, fma(_e3944, _e3944, (_e3945 * _e3945))))));
                                                                                                                        let _e3966 = abs(_e3964);
                                                                                                                        let _e3967 = (1.0 - _e3966);
                                                                                                                        let _e3970 = sqrt(select(_e3967, 0.0, (_e3967 < 0.0)));
                                                                                                                        let _e3977 = fma(fma(fma(fma(fma(fma(fma(-0.0012624911032617092, _e3966, 0.006670089904218912), _e3966, -0.01708812639117241), _e3966, 0.03089188039302826), _e3966, -0.050174303352832794), _e3966, 0.08897899091243744), _e3966, -0.21459880471229553), _e3966, 1.570796251296997);
                                                                                                                        if (_e3964 >= 0.0) {
                                                                                                                            phi_7315_ = (_e3977 * _e3970);
                                                                                                                        } else {
                                                                                                                            phi_7315_ = fma(-(_e3977), _e3970, 3.1415927410125732);
                                                                                                                        }
                                                                                                                        let _e3982 = phi_7315_;
                                                                                                                        let _e3986 = (1.0 - pow((_e3982 / _e3314.member_1.w), 2.0));
                                                                                                                        let _e3988 = select(_e3986, 0.0, (_e3986 < 0.0));
                                                                                                                        phi_7321_ = 0u;
                                                                                                                        phi_7324_ = _e3300;
                                                                                                                        phi_7326_ = _e3304;
                                                                                                                        phi_7328_ = true;
                                                                                                                        loop {
                                                                                                                            let _e3992 = phi_7321_;
                                                                                                                            let _e3994 = phi_7324_;
                                                                                                                            let _e3996 = phi_7326_;
                                                                                                                            let _e3998 = phi_7328_;
                                                                                                                            local_201 = _e3994;
                                                                                                                            local_202 = _e3996;
                                                                                                                            if _e3998 {
                                                                                                                                if (_e3992 < _e149) {
                                                                                                                                    if (_e3992 < 128u) {
                                                                                                                                        let _e4329 = global_5.member.member[_e3992];
                                                                                                                                        let _e4335 = (_e4329.member_1.x - _e4329.member.x);
                                                                                                                                        let _e4338 = (_e4329.member_1.y - _e4329.member.y);
                                                                                                                                        let _e4341 = (_e4329.member_1.z - _e4329.member.z);
                                                                                                                                        let _e4343 = (_e4329.member_2.x - _e4329.member.x);
                                                                                                                                        let _e4345 = (_e4329.member_2.y - _e4329.member.y);
                                                                                                                                        let _e4347 = (_e4329.member_2.z - _e4329.member.z);
                                                                                                                                        let _e4352 = fma(_e3332.y, _e4347, -((_e4345 * _e3332.z)));
                                                                                                                                        let _e4356 = fma(_e3332.z, _e4343, -((_e4347 * _e3332.x)));
                                                                                                                                        let _e4359 = fma(_e3332.x, _e4345, -((_e4343 * _e3332.y)));
                                                                                                                                        let _e4362 = fma(_e4341, _e4359, fma(_e4335, _e4352, (_e4338 * _e4356)));
                                                                                                                                        if (abs(_e4362) < 1.1920928955078125e-7) {
                                                                                                                                            phi_15872_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                        } else {
                                                                                                                                            let _e4365 = (1.0 / _e4362);
                                                                                                                                            let _e4366 = (_e3135.member_1.x - _e4329.member.x);
                                                                                                                                            let _e4367 = (_e3135.member_1.y - _e4329.member.y);
                                                                                                                                            let _e4368 = (_e3135.member_1.z - _e4329.member.z);
                                                                                                                                            let _e4372 = (fma(_e4368, _e4359, fma(_e4366, _e4352, (_e4367 * _e4356))) * _e4365);
                                                                                                                                            if (_e4372 > -9.999999747378752e-5) {
                                                                                                                                                phi_15755_ = (_e4372 < 0.0);
                                                                                                                                            } else {
                                                                                                                                                phi_15755_ = false;
                                                                                                                                            }
                                                                                                                                            let _e4376 = phi_15755_;
                                                                                                                                            let _e4377 = select(_e4372, 1.1920928955078125e-7, _e4376);
                                                                                                                                            if (_e4377 < 0.0) {
                                                                                                                                                phi_15765_ = true;
                                                                                                                                            } else {
                                                                                                                                                phi_15765_ = (_e4377 > 1.0);
                                                                                                                                            }
                                                                                                                                            let _e4381 = phi_15765_;
                                                                                                                                            if _e4381 {
                                                                                                                                                phi_15865_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                            } else {
                                                                                                                                                let _e4384 = fma(_e4367, _e4341, -((_e4338 * _e4368)));
                                                                                                                                                let _e4387 = fma(_e4368, _e4335, -((_e4341 * _e4366)));
                                                                                                                                                let _e4390 = fma(_e4366, _e4338, -((_e4335 * _e4367)));
                                                                                                                                                let _e4393 = fma(_e3332.z, _e4390, fma(_e3332.x, _e4384, (_e3332.y * _e4387)));
                                                                                                                                                let _e4394 = (_e4393 * _e4365);
                                                                                                                                                if (_e4394 < 0.0) {
                                                                                                                                                    phi_15794_ = true;
                                                                                                                                                } else {
                                                                                                                                                    phi_15794_ = (fma(_e4393, _e4365, _e4377) > 1.0);
                                                                                                                                                }
                                                                                                                                                let _e4399 = phi_15794_;
                                                                                                                                                if _e4399 {
                                                                                                                                                    phi_15858_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                } else {
                                                                                                                                                    let _e4402 = fma(_e4347, _e4390, fma(_e4343, _e4384, (_e4345 * _e4387)));
                                                                                                                                                    let _e4403 = (_e4402 * _e4365);
                                                                                                                                                    if (_e4403 < 0.0) {
                                                                                                                                                        phi_15855_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                    } else {
                                                                                                                                                        let _e4406 = fma(_e4402, _e4365, -0.009999999776482582);
                                                                                                                                                        let _e4414 = fma(_e4338, _e4347, -((_e4345 * _e4341)));
                                                                                                                                                        let _e4417 = fma(_e4341, _e4343, -((_e4347 * _e4335)));
                                                                                                                                                        let _e4420 = fma(_e4335, _e4345, -((_e4343 * _e4338)));
                                                                                                                                                        let _e4425 = (1.0 / sqrt(fma(_e4420, _e4420, fma(_e4414, _e4414, (_e4417 * _e4417)))));
                                                                                                                                                        phi_15855_ = type_47(_e3333, (_e3135.member_1 + vec3<f32>((_e3332.x * _e4406), (_e3332.y * _e4406), (_e3332.z * _e4406))), vec3<f32>((_e4414 * _e4425), (_e4417 * _e4425), (_e4420 * _e4425)), vec2<f32>(_e4377, _e4394), _e4403, type_46(0u, false), select(select(u32(_e4329.member.w), 0u, (_e4329.member.w < 0.0)), 4294967295u, (_e4329.member.w > 4294967040.0)));
                                                                                                                                                    }
                                                                                                                                                    let _e4438 = phi_15855_;
                                                                                                                                                    phi_15858_ = _e4438;
                                                                                                                                                }
                                                                                                                                                let _e4440 = phi_15858_;
                                                                                                                                                phi_15865_ = _e4440;
                                                                                                                                            }
                                                                                                                                            let _e4442 = phi_15865_;
                                                                                                                                            phi_15872_ = _e4442;
                                                                                                                                        }
                                                                                                                                        let _e4444 = phi_15872_;
                                                                                                                                        if (_e4444.member_4 < _e3343) {
                                                                                                                                            if (_e4329.member_2.w == 1.0) {
                                                                                                                                                if ((_e3992 % 2u) == 0u) {
                                                                                                                                                    let _e4478 = (3u * (_e3992 / 2u));
                                                                                                                                                    let _e4479 = (_e4478 < 192u);
                                                                                                                                                    if _e4479 {
                                                                                                                                                        let _e4483 = global_6.member.member_1[_e4478];
                                                                                                                                                        if _e4479 {
                                                                                                                                                            let _e4490 = (_e4478 + 1u);
                                                                                                                                                            if (_e4490 < 192u) {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            let _e4495 = global_6.member.member_1[_e4490];
                                                                                                                                                            local_191 = type_50(vec2<f32>(_e4483.x, _e4483.y), vec2<f32>(_e4483.z, _e4483.w), vec2<f32>(_e4495.x, _e4495.y));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e7155 = local_191;
                                                                                                                                                    phi_7587_ = _e7155;
                                                                                                                                                } else {
                                                                                                                                                    let _e4454 = (3u * ((_e3992 - 1u) / 2u));
                                                                                                                                                    let _e4455 = (_e4454 + 1u);
                                                                                                                                                    if (_e4455 < 192u) {
                                                                                                                                                        let _e4460 = global_6.member.member_1[_e4455];
                                                                                                                                                        let _e4464 = (_e4454 + 2u);
                                                                                                                                                        let _e4465 = (_e4464 < 192u);
                                                                                                                                                        if _e4465 {
                                                                                                                                                            let _e4469 = global_6.member.member_1[_e4464];
                                                                                                                                                            if _e4465 {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            local_192 = type_50(vec2<f32>(_e4460.z, _e4460.w), vec2<f32>(_e4469.x, _e4469.y), vec2<f32>(_e4469.z, _e4469.w));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e7157 = local_192;
                                                                                                                                                    phi_7587_ = _e7157;
                                                                                                                                                }
                                                                                                                                                let _e4501 = phi_7587_;
                                                                                                                                                let _e4522 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e4501.member_2.x - _e4501.member.x), _e4444.member_3.y, fma((_e4501.member_1.x - _e4501.member.x), _e4444.member_3.x, _e4501.member.x)), fma((_e4501.member_2.y - _e4501.member.y), _e4444.member_3.y, fma((_e4501.member_1.y - _e4501.member.y), _e4444.member_3.x, _e4501.member.y))), 0.0);
                                                                                                                                                phi_7625_ = (_e4522.w == 1.0);
                                                                                                                                            } else {
                                                                                                                                                phi_7625_ = true;
                                                                                                                                            }
                                                                                                                                            let _e4526 = phi_7625_;
                                                                                                                                            phi_7632_ = select(_e3994, true, _e4526);
                                                                                                                                            phi_7633_ = select(false, true, _e4526);
                                                                                                                                            phi_7634_ = select(true, false, _e4526);
                                                                                                                                        } else {
                                                                                                                                            phi_7632_ = _e3994;
                                                                                                                                            phi_7633_ = false;
                                                                                                                                            phi_7634_ = true;
                                                                                                                                        }
                                                                                                                                        let _e4531 = phi_7632_;
                                                                                                                                        let _e4533 = phi_7633_;
                                                                                                                                        let _e4535 = phi_7634_;
                                                                                                                                        local_194 = _e4531;
                                                                                                                                        if _e4535 {
                                                                                                                                            phi_7639_ = (_e3992 + 1u);
                                                                                                                                        } else {
                                                                                                                                            phi_7639_ = _e3992;
                                                                                                                                        }
                                                                                                                                        let _e4538 = phi_7639_;
                                                                                                                                        local_193 = _e4538;
                                                                                                                                        local_196 = select(false, true, _e4535);
                                                                                                                                        local_197 = select(_e4533, false, _e4535);
                                                                                                                                    } else {
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    let _e7165 = local_193;
                                                                                                                                    phi_7322_ = _e7165;
                                                                                                                                    let _e7168 = local_194;
                                                                                                                                    phi_7325_ = _e7168;
                                                                                                                                    let _e7173 = local_196;
                                                                                                                                    phi_8085_ = _e7173;
                                                                                                                                    let _e7176 = local_197;
                                                                                                                                    phi_8086_ = _e7176;
                                                                                                                                    phi_8087_ = false;
                                                                                                                                } else {
                                                                                                                                    phi_7652_ = 0u;
                                                                                                                                    phi_7655_ = _e3994;
                                                                                                                                    phi_7657_ = false;
                                                                                                                                    phi_7659_ = false;
                                                                                                                                    phi_7661_ = true;
                                                                                                                                    loop {
                                                                                                                                        let _e4001 = phi_7652_;
                                                                                                                                        let _e4003 = phi_7655_;
                                                                                                                                        let _e4005 = phi_7657_;
                                                                                                                                        let _e4007 = phi_7659_;
                                                                                                                                        let _e4009 = phi_7661_;
                                                                                                                                        local_195 = _e4003;
                                                                                                                                        local_198 = _e4007;
                                                                                                                                        local_199 = _e4005;
                                                                                                                                        if _e4009 {
                                                                                                                                            if (_e4001 < 4096u) {
                                                                                                                                                let _e4014 = global_4.member.member[_e4001];
                                                                                                                                                let _e4015 = (_e4001 + 1u);
                                                                                                                                                if (_e4015 < 4096u) {
                                                                                                                                                    let _e4020 = global_4.member.member[_e4015];
                                                                                                                                                    if (_e4014.x == _e4020.x) {
                                                                                                                                                        phi_7689_ = (_e4014.y == _e4020.y);
                                                                                                                                                    } else {
                                                                                                                                                        phi_7689_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e4030 = phi_7689_;
                                                                                                                                                    if _e4030 {
                                                                                                                                                        phi_7694_ = (_e4014.z == _e4020.z);
                                                                                                                                                    } else {
                                                                                                                                                        phi_7694_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e4033 = phi_7694_;
                                                                                                                                                    if _e4033 {
                                                                                                                                                        let _e4084 = select(select(u32(_e4014.w), 0u, (_e4014.w < 0.0)), 4294967295u, (_e4014.w > 4294967040.0));
                                                                                                                                                        if (_e4084 < 1365u) {
                                                                                                                                                            let _e4089 = global_3.member.member[_e4084];
                                                                                                                                                            let _e4095 = (_e4089.member_1.x - _e4089.member.x);
                                                                                                                                                            let _e4098 = (_e4089.member_1.y - _e4089.member.y);
                                                                                                                                                            let _e4101 = (_e4089.member_1.z - _e4089.member.z);
                                                                                                                                                            let _e4103 = (_e4089.member_2.x - _e4089.member.x);
                                                                                                                                                            let _e4105 = (_e4089.member_2.y - _e4089.member.y);
                                                                                                                                                            let _e4107 = (_e4089.member_2.z - _e4089.member.z);
                                                                                                                                                            let _e4112 = fma(_e3332.y, _e4107, -((_e4105 * _e3332.z)));
                                                                                                                                                            let _e4116 = fma(_e3332.z, _e4103, -((_e4107 * _e3332.x)));
                                                                                                                                                            let _e4119 = fma(_e3332.x, _e4105, -((_e4103 * _e3332.y)));
                                                                                                                                                            let _e4122 = fma(_e4101, _e4119, fma(_e4095, _e4112, (_e4098 * _e4116)));
                                                                                                                                                            if (abs(_e4122) < 1.1920928955078125e-7) {
                                                                                                                                                                phi_16227_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                            } else {
                                                                                                                                                                let _e4125 = (1.0 / _e4122);
                                                                                                                                                                let _e4126 = (_e3135.member_1.x - _e4089.member.x);
                                                                                                                                                                let _e4127 = (_e3135.member_1.y - _e4089.member.y);
                                                                                                                                                                let _e4128 = (_e3135.member_1.z - _e4089.member.z);
                                                                                                                                                                let _e4132 = (fma(_e4128, _e4119, fma(_e4126, _e4112, (_e4127 * _e4116))) * _e4125);
                                                                                                                                                                if (_e4132 > -9.999999747378752e-5) {
                                                                                                                                                                    phi_16110_ = (_e4132 < 0.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_16110_ = false;
                                                                                                                                                                }
                                                                                                                                                                let _e4136 = phi_16110_;
                                                                                                                                                                let _e4137 = select(_e4132, 1.1920928955078125e-7, _e4136);
                                                                                                                                                                if (_e4137 < 0.0) {
                                                                                                                                                                    phi_16120_ = true;
                                                                                                                                                                } else {
                                                                                                                                                                    phi_16120_ = (_e4137 > 1.0);
                                                                                                                                                                }
                                                                                                                                                                let _e4141 = phi_16120_;
                                                                                                                                                                if _e4141 {
                                                                                                                                                                    phi_16220_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                } else {
                                                                                                                                                                    let _e4144 = fma(_e4127, _e4101, -((_e4098 * _e4128)));
                                                                                                                                                                    let _e4147 = fma(_e4128, _e4095, -((_e4101 * _e4126)));
                                                                                                                                                                    let _e4150 = fma(_e4126, _e4098, -((_e4095 * _e4127)));
                                                                                                                                                                    let _e4153 = fma(_e3332.z, _e4150, fma(_e3332.x, _e4144, (_e3332.y * _e4147)));
                                                                                                                                                                    let _e4154 = (_e4153 * _e4125);
                                                                                                                                                                    if (_e4154 < 0.0) {
                                                                                                                                                                        phi_16149_ = true;
                                                                                                                                                                    } else {
                                                                                                                                                                        phi_16149_ = (fma(_e4153, _e4125, _e4137) > 1.0);
                                                                                                                                                                    }
                                                                                                                                                                    let _e4159 = phi_16149_;
                                                                                                                                                                    if _e4159 {
                                                                                                                                                                        phi_16213_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e4162 = fma(_e4107, _e4150, fma(_e4103, _e4144, (_e4105 * _e4147)));
                                                                                                                                                                        let _e4163 = (_e4162 * _e4125);
                                                                                                                                                                        if (_e4163 < 0.0) {
                                                                                                                                                                            phi_16210_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                        } else {
                                                                                                                                                                            let _e4166 = fma(_e4162, _e4125, -0.009999999776482582);
                                                                                                                                                                            let _e4174 = fma(_e4098, _e4107, -((_e4105 * _e4101)));
                                                                                                                                                                            let _e4177 = fma(_e4101, _e4103, -((_e4107 * _e4095)));
                                                                                                                                                                            let _e4180 = fma(_e4095, _e4105, -((_e4103 * _e4098)));
                                                                                                                                                                            let _e4185 = (1.0 / sqrt(fma(_e4180, _e4180, fma(_e4174, _e4174, (_e4177 * _e4177)))));
                                                                                                                                                                            phi_16210_ = type_47(_e3333, (_e3135.member_1 + vec3<f32>((_e3332.x * _e4166), (_e3332.y * _e4166), (_e3332.z * _e4166))), vec3<f32>((_e4174 * _e4185), (_e4177 * _e4185), (_e4180 * _e4185)), vec2<f32>(_e4137, _e4154), _e4163, type_46(0u, false), select(select(u32(_e4089.member.w), 0u, (_e4089.member.w < 0.0)), 4294967295u, (_e4089.member.w > 4294967040.0)));
                                                                                                                                                                        }
                                                                                                                                                                        let _e4198 = phi_16210_;
                                                                                                                                                                        phi_16213_ = _e4198;
                                                                                                                                                                    }
                                                                                                                                                                    let _e4200 = phi_16213_;
                                                                                                                                                                    phi_16220_ = _e4200;
                                                                                                                                                                }
                                                                                                                                                                let _e4202 = phi_16220_;
                                                                                                                                                                phi_16227_ = _e4202;
                                                                                                                                                            }
                                                                                                                                                            let _e4204 = phi_16227_;
                                                                                                                                                            if (_e4204.member_4 < _e3343) {
                                                                                                                                                                if (_e4089.member_2.w == 1.0) {
                                                                                                                                                                    if ((_e4084 % 2u) == 0u) {
                                                                                                                                                                        let _e4238 = (3u * (_e4084 / 2u));
                                                                                                                                                                        let _e4239 = (_e4238 < 2047u);
                                                                                                                                                                        if _e4239 {
                                                                                                                                                                            let _e4243 = global_6.member.member[_e4238];
                                                                                                                                                                            if _e4239 {
                                                                                                                                                                                let _e4250 = (_e4238 + 1u);
                                                                                                                                                                                if (_e4250 < 2047u) {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                let _e4255 = global_6.member.member[_e4250];
                                                                                                                                                                                local_185 = type_50(vec2<f32>(_e4243.x, _e4243.y), vec2<f32>(_e4243.z, _e4243.w), vec2<f32>(_e4255.x, _e4255.y));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e7123 = local_185;
                                                                                                                                                                        phi_7844_ = _e7123;
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e4214 = (3u * ((_e4084 - 1u) / 2u));
                                                                                                                                                                        let _e4215 = (_e4214 + 1u);
                                                                                                                                                                        if (_e4215 < 2047u) {
                                                                                                                                                                            let _e4220 = global_6.member.member[_e4215];
                                                                                                                                                                            let _e4224 = (_e4214 + 2u);
                                                                                                                                                                            let _e4225 = (_e4224 < 2047u);
                                                                                                                                                                            if _e4225 {
                                                                                                                                                                                let _e4229 = global_6.member.member[_e4224];
                                                                                                                                                                                if _e4225 {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                local_186 = type_50(vec2<f32>(_e4220.z, _e4220.w), vec2<f32>(_e4229.x, _e4229.y), vec2<f32>(_e4229.z, _e4229.w));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e7125 = local_186;
                                                                                                                                                                        phi_7844_ = _e7125;
                                                                                                                                                                    }
                                                                                                                                                                    let _e4261 = phi_7844_;
                                                                                                                                                                    let _e4282 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e4261.member_2.x - _e4261.member.x), _e4204.member_3.y, fma((_e4261.member_1.x - _e4261.member.x), _e4204.member_3.x, _e4261.member.x)), fma((_e4261.member_2.y - _e4261.member.y), _e4204.member_3.y, fma((_e4261.member_1.y - _e4261.member.y), _e4204.member_3.x, _e4261.member.y))), 0.0);
                                                                                                                                                                    phi_7992_ = (_e4282.w == 1.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_7992_ = true;
                                                                                                                                                                }
                                                                                                                                                                let _e4286 = phi_7992_;
                                                                                                                                                                phi_7999_ = select(_e4003, true, _e4286);
                                                                                                                                                                phi_8000_ = select(false, true, _e4286);
                                                                                                                                                                phi_8001_ = select(true, false, _e4286);
                                                                                                                                                            } else {
                                                                                                                                                                phi_7999_ = _e4003;
                                                                                                                                                                phi_8000_ = false;
                                                                                                                                                                phi_8001_ = true;
                                                                                                                                                            }
                                                                                                                                                            let _e4291 = phi_7999_;
                                                                                                                                                            let _e4293 = phi_8000_;
                                                                                                                                                            let _e4295 = phi_8001_;
                                                                                                                                                            local_188 = _e4291;
                                                                                                                                                            if _e4295 {
                                                                                                                                                                phi_8013_ = select(select(u32(_e4020.w), 0u, (_e4020.w < 0.0)), 4294967295u, (_e4020.w > 4294967040.0));
                                                                                                                                                            } else {
                                                                                                                                                                phi_8013_ = _e4001;
                                                                                                                                                            }
                                                                                                                                                            let _e4303 = phi_8013_;
                                                                                                                                                            local_187 = _e4303;
                                                                                                                                                            local_189 = select(false, true, _e4295);
                                                                                                                                                            local_190 = select(_e4293, false, _e4295);
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        let _e7133 = local_187;
                                                                                                                                                        phi_7653_ = _e7133;
                                                                                                                                                        let _e7136 = local_188;
                                                                                                                                                        phi_8054_ = _e7136;
                                                                                                                                                        let _e7139 = local_189;
                                                                                                                                                        phi_8055_ = _e7139;
                                                                                                                                                        let _e7142 = local_190;
                                                                                                                                                        phi_8056_ = _e7142;
                                                                                                                                                    } else {
                                                                                                                                                        let _e4038 = ((_e4014.x - _e3135.member_1.x) / _e3332.x);
                                                                                                                                                        let _e4040 = ((_e4014.y - _e3135.member_1.y) / _e3332.y);
                                                                                                                                                        let _e4042 = ((_e4014.z - _e3135.member_1.z) / _e3332.z);
                                                                                                                                                        let _e4046 = ((_e4020.x - _e3135.member_1.x) / _e3332.x);
                                                                                                                                                        let _e4047 = ((_e4020.y - _e3135.member_1.y) / _e3332.y);
                                                                                                                                                        let _e4048 = ((_e4020.z - _e3135.member_1.z) / _e3332.z);
                                                                                                                                                        let _e4056 = max(max(min(_e4038, _e4046), min(_e4040, _e4047)), min(_e4042, _e4048));
                                                                                                                                                        let _e4058 = min(min(max(_e4038, _e4046), max(_e4040, _e4047)), max(_e4042, _e4048));
                                                                                                                                                        if (_e4056 <= _e4058) {
                                                                                                                                                            phi_16437_ = (_e4058 > 0.0);
                                                                                                                                                        } else {
                                                                                                                                                            phi_16437_ = false;
                                                                                                                                                        }
                                                                                                                                                        let _e4062 = phi_16437_;
                                                                                                                                                        if (select(3.4028234663852886e38, _e4056, _e4062) < _e3343) {
                                                                                                                                                            phi_8053_ = select(select(u32(_e4014.w), 0u, (_e4014.w < 0.0)), 4294967295u, (_e4014.w > 4294967040.0));
                                                                                                                                                        } else {
                                                                                                                                                            phi_8053_ = select(select(u32(_e4020.w), 0u, (_e4020.w < 0.0)), 4294967295u, (_e4020.w > 4294967040.0));
                                                                                                                                                        }
                                                                                                                                                        let _e4078 = phi_8053_;
                                                                                                                                                        phi_7653_ = _e4078;
                                                                                                                                                        phi_8054_ = _e4003;
                                                                                                                                                        phi_8055_ = true;
                                                                                                                                                        phi_8056_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e4307 = phi_7653_;
                                                                                                                                                    let _e4309 = phi_8054_;
                                                                                                                                                    let _e4311 = phi_8055_;
                                                                                                                                                    let _e4313 = phi_8056_;
                                                                                                                                                    local_180 = _e4307;
                                                                                                                                                    if _e4311 {
                                                                                                                                                        let _e4314 = (_e4307 == 0u);
                                                                                                                                                        phi_7656_ = select(_e4309, false, _e4314);
                                                                                                                                                        phi_8067_ = select(false, true, _e4314);
                                                                                                                                                        phi_8068_ = select(true, false, _e4314);
                                                                                                                                                    } else {
                                                                                                                                                        phi_7656_ = _e4309;
                                                                                                                                                        phi_8067_ = false;
                                                                                                                                                        phi_8068_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e4319 = phi_7656_;
                                                                                                                                                    let _e4321 = phi_8067_;
                                                                                                                                                    let _e4323 = phi_8068_;
                                                                                                                                                    local_181 = _e4319;
                                                                                                                                                    local_182 = _e4321;
                                                                                                                                                    local_183 = select(_e4313, false, _e4311);
                                                                                                                                                    local_184 = _e4323;
                                                                                                                                                } else {
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            continue;
                                                                                                                                        } else {
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                        continuing {
                                                                                                                                            let _e7097 = local_180;
                                                                                                                                            phi_7652_ = _e7097;
                                                                                                                                            let _e7100 = local_181;
                                                                                                                                            phi_7655_ = _e7100;
                                                                                                                                            let _e7103 = local_182;
                                                                                                                                            phi_7657_ = _e7103;
                                                                                                                                            let _e7106 = local_183;
                                                                                                                                            phi_7659_ = _e7106;
                                                                                                                                            let _e7109 = local_184;
                                                                                                                                            phi_7661_ = _e7109;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    phi_7322_ = _e3992;
                                                                                                                                    let _e7170 = local_195;
                                                                                                                                    phi_7325_ = _e7170;
                                                                                                                                    phi_8085_ = false;
                                                                                                                                    let _e7178 = local_198;
                                                                                                                                    phi_8086_ = _e7178;
                                                                                                                                    let _e7181 = local_199;
                                                                                                                                    phi_8087_ = _e7181;
                                                                                                                                }
                                                                                                                                let _e4542 = phi_7322_;
                                                                                                                                let _e4544 = phi_7325_;
                                                                                                                                let _e4546 = phi_8085_;
                                                                                                                                let _e4548 = phi_8086_;
                                                                                                                                let _e4550 = phi_8087_;
                                                                                                                                let _e4551 = select(_e4550, true, _e4548);
                                                                                                                                local_176 = _e4542;
                                                                                                                                local_177 = _e4544;
                                                                                                                                if _e4551 {
                                                                                                                                    if _e4544 {
                                                                                                                                        phi_8113_ = 0.0;
                                                                                                                                    } else {
                                                                                                                                        phi_8113_ = max(fma(_e3332.z, _e3135.member_2.z, fma(_e3332.x, _e3135.member_2.x, (_e3332.y * _e3135.member_2.y))), 0.0);
                                                                                                                                    }
                                                                                                                                    let _e4565 = phi_8113_;
                                                                                                                                    let _e4566 = (select(_e3988, 1.0, (_e3988 > 1.0)) * _e4565);
                                                                                                                                    phi_7327_ = vec3<f32>(fma(((_e4566 * _e3314.member_2.x) * _e3314.member_2.w), _e3296.x, _e3996.x), fma(((_e4566 * _e3314.member_2.y) * _e3314.member_2.w), _e3296.y, _e3996.y), fma(((_e4566 * _e3314.member_2.z) * _e3314.member_2.w), _e3296.z, _e3996.z));
                                                                                                                                } else {
                                                                                                                                    phi_7327_ = _e3996;
                                                                                                                                }
                                                                                                                                let _e4588 = phi_7327_;
                                                                                                                                local_178 = _e4588;
                                                                                                                                local_179 = select(select(_e4546, false, _e4548), false, _e4551);
                                                                                                                                continue;
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            continuing {
                                                                                                                                let _e7085 = local_176;
                                                                                                                                phi_7321_ = _e7085;
                                                                                                                                let _e7088 = local_177;
                                                                                                                                phi_7324_ = _e7088;
                                                                                                                                let _e7091 = local_178;
                                                                                                                                phi_7326_ = _e7091;
                                                                                                                                let _e7094 = local_179;
                                                                                                                                phi_7328_ = _e7094;
                                                                                                                            }
                                                                                                                        }
                                                                                                                        phi_8963_ = _e3298;
                                                                                                                        let _e7189 = local_201;
                                                                                                                        phi_8964_ = _e7189;
                                                                                                                        let _e7192 = local_202;
                                                                                                                        phi_8965_ = _e7192;
                                                                                                                    } else {
                                                                                                                        phi_8144_ = 0u;
                                                                                                                        phi_8147_ = _e3298;
                                                                                                                        phi_8149_ = _e3304;
                                                                                                                        phi_8151_ = true;
                                                                                                                        loop {
                                                                                                                            let _e3347 = phi_8144_;
                                                                                                                            let _e3349 = phi_8147_;
                                                                                                                            let _e3351 = phi_8149_;
                                                                                                                            let _e3353 = phi_8151_;
                                                                                                                            local_200 = _e3349;
                                                                                                                            local_203 = _e3351;
                                                                                                                            if _e3353 {
                                                                                                                                if (_e3347 < _e149) {
                                                                                                                                    if (_e3347 < 128u) {
                                                                                                                                        let _e3684 = global_5.member.member[_e3347];
                                                                                                                                        let _e3690 = (_e3684.member_1.x - _e3684.member.x);
                                                                                                                                        let _e3693 = (_e3684.member_1.y - _e3684.member.y);
                                                                                                                                        let _e3696 = (_e3684.member_1.z - _e3684.member.z);
                                                                                                                                        let _e3698 = (_e3684.member_2.x - _e3684.member.x);
                                                                                                                                        let _e3700 = (_e3684.member_2.y - _e3684.member.y);
                                                                                                                                        let _e3702 = (_e3684.member_2.z - _e3684.member.z);
                                                                                                                                        let _e3707 = fma(_e3332.y, _e3702, -((_e3700 * _e3332.z)));
                                                                                                                                        let _e3711 = fma(_e3332.z, _e3698, -((_e3702 * _e3332.x)));
                                                                                                                                        let _e3714 = fma(_e3332.x, _e3700, -((_e3698 * _e3332.y)));
                                                                                                                                        let _e3717 = fma(_e3696, _e3714, fma(_e3690, _e3707, (_e3693 * _e3711)));
                                                                                                                                        if (abs(_e3717) < 1.1920928955078125e-7) {
                                                                                                                                            phi_16659_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                        } else {
                                                                                                                                            let _e3720 = (1.0 / _e3717);
                                                                                                                                            let _e3721 = (_e3135.member_1.x - _e3684.member.x);
                                                                                                                                            let _e3722 = (_e3135.member_1.y - _e3684.member.y);
                                                                                                                                            let _e3723 = (_e3135.member_1.z - _e3684.member.z);
                                                                                                                                            let _e3727 = (fma(_e3723, _e3714, fma(_e3721, _e3707, (_e3722 * _e3711))) * _e3720);
                                                                                                                                            if (_e3727 > -9.999999747378752e-5) {
                                                                                                                                                phi_16542_ = (_e3727 < 0.0);
                                                                                                                                            } else {
                                                                                                                                                phi_16542_ = false;
                                                                                                                                            }
                                                                                                                                            let _e3731 = phi_16542_;
                                                                                                                                            let _e3732 = select(_e3727, 1.1920928955078125e-7, _e3731);
                                                                                                                                            if (_e3732 < 0.0) {
                                                                                                                                                phi_16552_ = true;
                                                                                                                                            } else {
                                                                                                                                                phi_16552_ = (_e3732 > 1.0);
                                                                                                                                            }
                                                                                                                                            let _e3736 = phi_16552_;
                                                                                                                                            if _e3736 {
                                                                                                                                                phi_16652_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                            } else {
                                                                                                                                                let _e3739 = fma(_e3722, _e3696, -((_e3693 * _e3723)));
                                                                                                                                                let _e3742 = fma(_e3723, _e3690, -((_e3696 * _e3721)));
                                                                                                                                                let _e3745 = fma(_e3721, _e3693, -((_e3690 * _e3722)));
                                                                                                                                                let _e3748 = fma(_e3332.z, _e3745, fma(_e3332.x, _e3739, (_e3332.y * _e3742)));
                                                                                                                                                let _e3749 = (_e3748 * _e3720);
                                                                                                                                                if (_e3749 < 0.0) {
                                                                                                                                                    phi_16581_ = true;
                                                                                                                                                } else {
                                                                                                                                                    phi_16581_ = (fma(_e3748, _e3720, _e3732) > 1.0);
                                                                                                                                                }
                                                                                                                                                let _e3754 = phi_16581_;
                                                                                                                                                if _e3754 {
                                                                                                                                                    phi_16645_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                } else {
                                                                                                                                                    let _e3757 = fma(_e3702, _e3745, fma(_e3698, _e3739, (_e3700 * _e3742)));
                                                                                                                                                    let _e3758 = (_e3757 * _e3720);
                                                                                                                                                    if (_e3758 < 0.0) {
                                                                                                                                                        phi_16642_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                    } else {
                                                                                                                                                        let _e3761 = fma(_e3757, _e3720, -0.009999999776482582);
                                                                                                                                                        let _e3769 = fma(_e3693, _e3702, -((_e3700 * _e3696)));
                                                                                                                                                        let _e3772 = fma(_e3696, _e3698, -((_e3702 * _e3690)));
                                                                                                                                                        let _e3775 = fma(_e3690, _e3700, -((_e3698 * _e3693)));
                                                                                                                                                        let _e3780 = (1.0 / sqrt(fma(_e3775, _e3775, fma(_e3769, _e3769, (_e3772 * _e3772)))));
                                                                                                                                                        phi_16642_ = type_47(_e3333, (_e3135.member_1 + vec3<f32>((_e3332.x * _e3761), (_e3332.y * _e3761), (_e3332.z * _e3761))), vec3<f32>((_e3769 * _e3780), (_e3772 * _e3780), (_e3775 * _e3780)), vec2<f32>(_e3732, _e3749), _e3758, type_46(0u, false), select(select(u32(_e3684.member.w), 0u, (_e3684.member.w < 0.0)), 4294967295u, (_e3684.member.w > 4294967040.0)));
                                                                                                                                                    }
                                                                                                                                                    let _e3793 = phi_16642_;
                                                                                                                                                    phi_16645_ = _e3793;
                                                                                                                                                }
                                                                                                                                                let _e3795 = phi_16645_;
                                                                                                                                                phi_16652_ = _e3795;
                                                                                                                                            }
                                                                                                                                            let _e3797 = phi_16652_;
                                                                                                                                            phi_16659_ = _e3797;
                                                                                                                                        }
                                                                                                                                        let _e3799 = phi_16659_;
                                                                                                                                        if (_e3799.member_4 < _e3343) {
                                                                                                                                            if (_e3684.member_2.w == 1.0) {
                                                                                                                                                if ((_e3347 % 2u) == 0u) {
                                                                                                                                                    let _e3833 = (3u * (_e3347 / 2u));
                                                                                                                                                    let _e3834 = (_e3833 < 192u);
                                                                                                                                                    if _e3834 {
                                                                                                                                                        let _e3838 = global_6.member.member_1[_e3833];
                                                                                                                                                        if _e3834 {
                                                                                                                                                            let _e3845 = (_e3833 + 1u);
                                                                                                                                                            if (_e3845 < 192u) {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            let _e3850 = global_6.member.member_1[_e3845];
                                                                                                                                                            local_167 = type_50(vec2<f32>(_e3838.x, _e3838.y), vec2<f32>(_e3838.z, _e3838.w), vec2<f32>(_e3850.x, _e3850.y));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e7053 = local_167;
                                                                                                                                                    phi_8410_ = _e7053;
                                                                                                                                                } else {
                                                                                                                                                    let _e3809 = (3u * ((_e3347 - 1u) / 2u));
                                                                                                                                                    let _e3810 = (_e3809 + 1u);
                                                                                                                                                    if (_e3810 < 192u) {
                                                                                                                                                        let _e3815 = global_6.member.member_1[_e3810];
                                                                                                                                                        let _e3819 = (_e3809 + 2u);
                                                                                                                                                        let _e3820 = (_e3819 < 192u);
                                                                                                                                                        if _e3820 {
                                                                                                                                                            let _e3824 = global_6.member.member_1[_e3819];
                                                                                                                                                            if _e3820 {
                                                                                                                                                            } else {
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            local_168 = type_50(vec2<f32>(_e3815.z, _e3815.w), vec2<f32>(_e3824.x, _e3824.y), vec2<f32>(_e3824.z, _e3824.w));
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let _e7055 = local_168;
                                                                                                                                                    phi_8410_ = _e7055;
                                                                                                                                                }
                                                                                                                                                let _e3856 = phi_8410_;
                                                                                                                                                let _e3877 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e3856.member_2.x - _e3856.member.x), _e3799.member_3.y, fma((_e3856.member_1.x - _e3856.member.x), _e3799.member_3.x, _e3856.member.x)), fma((_e3856.member_2.y - _e3856.member.y), _e3799.member_3.y, fma((_e3856.member_1.y - _e3856.member.y), _e3799.member_3.x, _e3856.member.y))), 0.0);
                                                                                                                                                phi_8448_ = (_e3877.w == 1.0);
                                                                                                                                            } else {
                                                                                                                                                phi_8448_ = true;
                                                                                                                                            }
                                                                                                                                            let _e3881 = phi_8448_;
                                                                                                                                            phi_8455_ = select(_e3349, true, _e3881);
                                                                                                                                            phi_8456_ = select(false, true, _e3881);
                                                                                                                                            phi_8457_ = select(true, false, _e3881);
                                                                                                                                        } else {
                                                                                                                                            phi_8455_ = _e3349;
                                                                                                                                            phi_8456_ = false;
                                                                                                                                            phi_8457_ = true;
                                                                                                                                        }
                                                                                                                                        let _e3886 = phi_8455_;
                                                                                                                                        let _e3888 = phi_8456_;
                                                                                                                                        let _e3890 = phi_8457_;
                                                                                                                                        local_170 = _e3886;
                                                                                                                                        if _e3890 {
                                                                                                                                            phi_8462_ = (_e3347 + 1u);
                                                                                                                                        } else {
                                                                                                                                            phi_8462_ = _e3347;
                                                                                                                                        }
                                                                                                                                        let _e3893 = phi_8462_;
                                                                                                                                        local_169 = _e3893;
                                                                                                                                        local_172 = select(false, true, _e3890);
                                                                                                                                        local_173 = select(_e3888, false, _e3890);
                                                                                                                                    } else {
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    let _e7063 = local_169;
                                                                                                                                    phi_8145_ = _e7063;
                                                                                                                                    let _e7066 = local_170;
                                                                                                                                    phi_8148_ = _e7066;
                                                                                                                                    let _e7071 = local_172;
                                                                                                                                    phi_8908_ = _e7071;
                                                                                                                                    let _e7074 = local_173;
                                                                                                                                    phi_8909_ = _e7074;
                                                                                                                                    phi_8910_ = false;
                                                                                                                                } else {
                                                                                                                                    phi_8475_ = 0u;
                                                                                                                                    phi_8478_ = _e3349;
                                                                                                                                    phi_8480_ = false;
                                                                                                                                    phi_8482_ = false;
                                                                                                                                    phi_8484_ = true;
                                                                                                                                    loop {
                                                                                                                                        let _e3356 = phi_8475_;
                                                                                                                                        let _e3358 = phi_8478_;
                                                                                                                                        let _e3360 = phi_8480_;
                                                                                                                                        let _e3362 = phi_8482_;
                                                                                                                                        let _e3364 = phi_8484_;
                                                                                                                                        local_171 = _e3358;
                                                                                                                                        local_174 = _e3362;
                                                                                                                                        local_175 = _e3360;
                                                                                                                                        if _e3364 {
                                                                                                                                            if (_e3356 < 4096u) {
                                                                                                                                                let _e3369 = global_4.member.member[_e3356];
                                                                                                                                                let _e3370 = (_e3356 + 1u);
                                                                                                                                                if (_e3370 < 4096u) {
                                                                                                                                                    let _e3375 = global_4.member.member[_e3370];
                                                                                                                                                    if (_e3369.x == _e3375.x) {
                                                                                                                                                        phi_8512_ = (_e3369.y == _e3375.y);
                                                                                                                                                    } else {
                                                                                                                                                        phi_8512_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e3385 = phi_8512_;
                                                                                                                                                    if _e3385 {
                                                                                                                                                        phi_8517_ = (_e3369.z == _e3375.z);
                                                                                                                                                    } else {
                                                                                                                                                        phi_8517_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e3388 = phi_8517_;
                                                                                                                                                    if _e3388 {
                                                                                                                                                        let _e3439 = select(select(u32(_e3369.w), 0u, (_e3369.w < 0.0)), 4294967295u, (_e3369.w > 4294967040.0));
                                                                                                                                                        if (_e3439 < 1365u) {
                                                                                                                                                            let _e3444 = global_3.member.member[_e3439];
                                                                                                                                                            let _e3450 = (_e3444.member_1.x - _e3444.member.x);
                                                                                                                                                            let _e3453 = (_e3444.member_1.y - _e3444.member.y);
                                                                                                                                                            let _e3456 = (_e3444.member_1.z - _e3444.member.z);
                                                                                                                                                            let _e3458 = (_e3444.member_2.x - _e3444.member.x);
                                                                                                                                                            let _e3460 = (_e3444.member_2.y - _e3444.member.y);
                                                                                                                                                            let _e3462 = (_e3444.member_2.z - _e3444.member.z);
                                                                                                                                                            let _e3467 = fma(_e3332.y, _e3462, -((_e3460 * _e3332.z)));
                                                                                                                                                            let _e3471 = fma(_e3332.z, _e3458, -((_e3462 * _e3332.x)));
                                                                                                                                                            let _e3474 = fma(_e3332.x, _e3460, -((_e3458 * _e3332.y)));
                                                                                                                                                            let _e3477 = fma(_e3456, _e3474, fma(_e3450, _e3467, (_e3453 * _e3471)));
                                                                                                                                                            if (abs(_e3477) < 1.1920928955078125e-7) {
                                                                                                                                                                phi_17014_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                            } else {
                                                                                                                                                                let _e3480 = (1.0 / _e3477);
                                                                                                                                                                let _e3481 = (_e3135.member_1.x - _e3444.member.x);
                                                                                                                                                                let _e3482 = (_e3135.member_1.y - _e3444.member.y);
                                                                                                                                                                let _e3483 = (_e3135.member_1.z - _e3444.member.z);
                                                                                                                                                                let _e3487 = (fma(_e3483, _e3474, fma(_e3481, _e3467, (_e3482 * _e3471))) * _e3480);
                                                                                                                                                                if (_e3487 > -9.999999747378752e-5) {
                                                                                                                                                                    phi_16897_ = (_e3487 < 0.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_16897_ = false;
                                                                                                                                                                }
                                                                                                                                                                let _e3491 = phi_16897_;
                                                                                                                                                                let _e3492 = select(_e3487, 1.1920928955078125e-7, _e3491);
                                                                                                                                                                if (_e3492 < 0.0) {
                                                                                                                                                                    phi_16907_ = true;
                                                                                                                                                                } else {
                                                                                                                                                                    phi_16907_ = (_e3492 > 1.0);
                                                                                                                                                                }
                                                                                                                                                                let _e3496 = phi_16907_;
                                                                                                                                                                if _e3496 {
                                                                                                                                                                    phi_17007_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                } else {
                                                                                                                                                                    let _e3499 = fma(_e3482, _e3456, -((_e3453 * _e3483)));
                                                                                                                                                                    let _e3502 = fma(_e3483, _e3450, -((_e3456 * _e3481)));
                                                                                                                                                                    let _e3505 = fma(_e3481, _e3453, -((_e3450 * _e3482)));
                                                                                                                                                                    let _e3508 = fma(_e3332.z, _e3505, fma(_e3332.x, _e3499, (_e3332.y * _e3502)));
                                                                                                                                                                    let _e3509 = (_e3508 * _e3480);
                                                                                                                                                                    if (_e3509 < 0.0) {
                                                                                                                                                                        phi_16936_ = true;
                                                                                                                                                                    } else {
                                                                                                                                                                        phi_16936_ = (fma(_e3508, _e3480, _e3492) > 1.0);
                                                                                                                                                                    }
                                                                                                                                                                    let _e3514 = phi_16936_;
                                                                                                                                                                    if _e3514 {
                                                                                                                                                                        phi_17000_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e3517 = fma(_e3462, _e3505, fma(_e3458, _e3499, (_e3460 * _e3502)));
                                                                                                                                                                        let _e3518 = (_e3517 * _e3480);
                                                                                                                                                                        if (_e3518 < 0.0) {
                                                                                                                                                                            phi_16997_ = type_47(type_42(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0)), vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), vec2<f32>(0.0, 0.0), 1000.0, type_46(0u, false), 0u);
                                                                                                                                                                        } else {
                                                                                                                                                                            let _e3521 = fma(_e3517, _e3480, -0.009999999776482582);
                                                                                                                                                                            let _e3529 = fma(_e3453, _e3462, -((_e3460 * _e3456)));
                                                                                                                                                                            let _e3532 = fma(_e3456, _e3458, -((_e3462 * _e3450)));
                                                                                                                                                                            let _e3535 = fma(_e3450, _e3460, -((_e3458 * _e3453)));
                                                                                                                                                                            let _e3540 = (1.0 / sqrt(fma(_e3535, _e3535, fma(_e3529, _e3529, (_e3532 * _e3532)))));
                                                                                                                                                                            phi_16997_ = type_47(_e3333, (_e3135.member_1 + vec3<f32>((_e3332.x * _e3521), (_e3332.y * _e3521), (_e3332.z * _e3521))), vec3<f32>((_e3529 * _e3540), (_e3532 * _e3540), (_e3535 * _e3540)), vec2<f32>(_e3492, _e3509), _e3518, type_46(0u, false), select(select(u32(_e3444.member.w), 0u, (_e3444.member.w < 0.0)), 4294967295u, (_e3444.member.w > 4294967040.0)));
                                                                                                                                                                        }
                                                                                                                                                                        let _e3553 = phi_16997_;
                                                                                                                                                                        phi_17000_ = _e3553;
                                                                                                                                                                    }
                                                                                                                                                                    let _e3555 = phi_17000_;
                                                                                                                                                                    phi_17007_ = _e3555;
                                                                                                                                                                }
                                                                                                                                                                let _e3557 = phi_17007_;
                                                                                                                                                                phi_17014_ = _e3557;
                                                                                                                                                            }
                                                                                                                                                            let _e3559 = phi_17014_;
                                                                                                                                                            if (_e3559.member_4 < _e3343) {
                                                                                                                                                                if (_e3444.member_2.w == 1.0) {
                                                                                                                                                                    if ((_e3439 % 2u) == 0u) {
                                                                                                                                                                        let _e3593 = (3u * (_e3439 / 2u));
                                                                                                                                                                        let _e3594 = (_e3593 < 2047u);
                                                                                                                                                                        if _e3594 {
                                                                                                                                                                            let _e3598 = global_6.member.member[_e3593];
                                                                                                                                                                            if _e3594 {
                                                                                                                                                                                let _e3605 = (_e3593 + 1u);
                                                                                                                                                                                if (_e3605 < 2047u) {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                let _e3610 = global_6.member.member[_e3605];
                                                                                                                                                                                local_161 = type_50(vec2<f32>(_e3598.x, _e3598.y), vec2<f32>(_e3598.z, _e3598.w), vec2<f32>(_e3610.x, _e3610.y));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e7021 = local_161;
                                                                                                                                                                        phi_8667_ = _e7021;
                                                                                                                                                                    } else {
                                                                                                                                                                        let _e3569 = (3u * ((_e3439 - 1u) / 2u));
                                                                                                                                                                        let _e3570 = (_e3569 + 1u);
                                                                                                                                                                        if (_e3570 < 2047u) {
                                                                                                                                                                            let _e3575 = global_6.member.member[_e3570];
                                                                                                                                                                            let _e3579 = (_e3569 + 2u);
                                                                                                                                                                            let _e3580 = (_e3579 < 2047u);
                                                                                                                                                                            if _e3580 {
                                                                                                                                                                                let _e3584 = global_6.member.member[_e3579];
                                                                                                                                                                                if _e3580 {
                                                                                                                                                                                } else {
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                local_162 = type_50(vec2<f32>(_e3575.z, _e3575.w), vec2<f32>(_e3584.x, _e3584.y), vec2<f32>(_e3584.z, _e3584.w));
                                                                                                                                                                            } else {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let _e7023 = local_162;
                                                                                                                                                                        phi_8667_ = _e7023;
                                                                                                                                                                    }
                                                                                                                                                                    let _e3616 = phi_8667_;
                                                                                                                                                                    let _e3637 = textureSampleLevel(global_10, global_11, vec2<f32>(fma((_e3616.member_2.x - _e3616.member.x), _e3559.member_3.y, fma((_e3616.member_1.x - _e3616.member.x), _e3559.member_3.x, _e3616.member.x)), fma((_e3616.member_2.y - _e3616.member.y), _e3559.member_3.y, fma((_e3616.member_1.y - _e3616.member.y), _e3559.member_3.x, _e3616.member.y))), 0.0);
                                                                                                                                                                    phi_8815_ = (_e3637.w == 1.0);
                                                                                                                                                                } else {
                                                                                                                                                                    phi_8815_ = true;
                                                                                                                                                                }
                                                                                                                                                                let _e3641 = phi_8815_;
                                                                                                                                                                phi_8822_ = select(_e3358, true, _e3641);
                                                                                                                                                                phi_8823_ = select(false, true, _e3641);
                                                                                                                                                                phi_8824_ = select(true, false, _e3641);
                                                                                                                                                            } else {
                                                                                                                                                                phi_8822_ = _e3358;
                                                                                                                                                                phi_8823_ = false;
                                                                                                                                                                phi_8824_ = true;
                                                                                                                                                            }
                                                                                                                                                            let _e3646 = phi_8822_;
                                                                                                                                                            let _e3648 = phi_8823_;
                                                                                                                                                            let _e3650 = phi_8824_;
                                                                                                                                                            local_164 = _e3646;
                                                                                                                                                            if _e3650 {
                                                                                                                                                                phi_8836_ = select(select(u32(_e3375.w), 0u, (_e3375.w < 0.0)), 4294967295u, (_e3375.w > 4294967040.0));
                                                                                                                                                            } else {
                                                                                                                                                                phi_8836_ = _e3356;
                                                                                                                                                            }
                                                                                                                                                            let _e3658 = phi_8836_;
                                                                                                                                                            local_163 = _e3658;
                                                                                                                                                            local_165 = select(false, true, _e3650);
                                                                                                                                                            local_166 = select(_e3648, false, _e3650);
                                                                                                                                                        } else {
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        let _e7031 = local_163;
                                                                                                                                                        phi_8476_ = _e7031;
                                                                                                                                                        let _e7034 = local_164;
                                                                                                                                                        phi_8877_ = _e7034;
                                                                                                                                                        let _e7037 = local_165;
                                                                                                                                                        phi_8878_ = _e7037;
                                                                                                                                                        let _e7040 = local_166;
                                                                                                                                                        phi_8879_ = _e7040;
                                                                                                                                                    } else {
                                                                                                                                                        let _e3393 = ((_e3369.x - _e3135.member_1.x) / _e3332.x);
                                                                                                                                                        let _e3395 = ((_e3369.y - _e3135.member_1.y) / _e3332.y);
                                                                                                                                                        let _e3397 = ((_e3369.z - _e3135.member_1.z) / _e3332.z);
                                                                                                                                                        let _e3401 = ((_e3375.x - _e3135.member_1.x) / _e3332.x);
                                                                                                                                                        let _e3402 = ((_e3375.y - _e3135.member_1.y) / _e3332.y);
                                                                                                                                                        let _e3403 = ((_e3375.z - _e3135.member_1.z) / _e3332.z);
                                                                                                                                                        let _e3411 = max(max(min(_e3393, _e3401), min(_e3395, _e3402)), min(_e3397, _e3403));
                                                                                                                                                        let _e3413 = min(min(max(_e3393, _e3401), max(_e3395, _e3402)), max(_e3397, _e3403));
                                                                                                                                                        if (_e3411 <= _e3413) {
                                                                                                                                                            phi_17224_ = (_e3413 > 0.0);
                                                                                                                                                        } else {
                                                                                                                                                            phi_17224_ = false;
                                                                                                                                                        }
                                                                                                                                                        let _e3417 = phi_17224_;
                                                                                                                                                        if (select(3.4028234663852886e38, _e3411, _e3417) < _e3343) {
                                                                                                                                                            phi_8876_ = select(select(u32(_e3369.w), 0u, (_e3369.w < 0.0)), 4294967295u, (_e3369.w > 4294967040.0));
                                                                                                                                                        } else {
                                                                                                                                                            phi_8876_ = select(select(u32(_e3375.w), 0u, (_e3375.w < 0.0)), 4294967295u, (_e3375.w > 4294967040.0));
                                                                                                                                                        }
                                                                                                                                                        let _e3433 = phi_8876_;
                                                                                                                                                        phi_8476_ = _e3433;
                                                                                                                                                        phi_8877_ = _e3358;
                                                                                                                                                        phi_8878_ = true;
                                                                                                                                                        phi_8879_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e3662 = phi_8476_;
                                                                                                                                                    let _e3664 = phi_8877_;
                                                                                                                                                    let _e3666 = phi_8878_;
                                                                                                                                                    let _e3668 = phi_8879_;
                                                                                                                                                    local_156 = _e3662;
                                                                                                                                                    if _e3666 {
                                                                                                                                                        let _e3669 = (_e3662 == 0u);
                                                                                                                                                        phi_8479_ = select(_e3664, false, _e3669);
                                                                                                                                                        phi_8890_ = select(false, true, _e3669);
                                                                                                                                                        phi_8891_ = select(true, false, _e3669);
                                                                                                                                                    } else {
                                                                                                                                                        phi_8479_ = _e3664;
                                                                                                                                                        phi_8890_ = false;
                                                                                                                                                        phi_8891_ = false;
                                                                                                                                                    }
                                                                                                                                                    let _e3674 = phi_8479_;
                                                                                                                                                    let _e3676 = phi_8890_;
                                                                                                                                                    let _e3678 = phi_8891_;
                                                                                                                                                    local_157 = _e3674;
                                                                                                                                                    local_158 = _e3676;
                                                                                                                                                    local_159 = select(_e3668, false, _e3666);
                                                                                                                                                    local_160 = _e3678;
                                                                                                                                                } else {
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            continue;
                                                                                                                                        } else {
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                        continuing {
                                                                                                                                            let _e6995 = local_156;
                                                                                                                                            phi_8475_ = _e6995;
                                                                                                                                            let _e6998 = local_157;
                                                                                                                                            phi_8478_ = _e6998;
                                                                                                                                            let _e7001 = local_158;
                                                                                                                                            phi_8480_ = _e7001;
                                                                                                                                            let _e7004 = local_159;
                                                                                                                                            phi_8482_ = _e7004;
                                                                                                                                            let _e7007 = local_160;
                                                                                                                                            phi_8484_ = _e7007;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    phi_8145_ = _e3347;
                                                                                                                                    let _e7068 = local_171;
                                                                                                                                    phi_8148_ = _e7068;
                                                                                                                                    phi_8908_ = false;
                                                                                                                                    let _e7076 = local_174;
                                                                                                                                    phi_8909_ = _e7076;
                                                                                                                                    let _e7079 = local_175;
                                                                                                                                    phi_8910_ = _e7079;
                                                                                                                                }
                                                                                                                                let _e3897 = phi_8145_;
                                                                                                                                let _e3899 = phi_8148_;
                                                                                                                                let _e3901 = phi_8908_;
                                                                                                                                let _e3903 = phi_8909_;
                                                                                                                                let _e3905 = phi_8910_;
                                                                                                                                let _e3906 = select(_e3905, true, _e3903);
                                                                                                                                local_152 = _e3897;
                                                                                                                                local_153 = _e3899;
                                                                                                                                if _e3906 {
                                                                                                                                    if _e3899 {
                                                                                                                                        phi_8936_ = 0.0;
                                                                                                                                    } else {
                                                                                                                                        phi_8936_ = max(fma(_e3332.z, _e3135.member_2.z, fma(_e3332.x, _e3135.member_2.x, (_e3332.y * _e3135.member_2.y))), 0.0);
                                                                                                                                    }
                                                                                                                                    let _e3920 = phi_8936_;
                                                                                                                                    phi_8150_ = vec3<f32>(fma(((_e3920 * _e3314.member_2.x) * _e3314.member_2.w), _e3296.x, _e3351.x), fma(((_e3920 * _e3314.member_2.y) * _e3314.member_2.w), _e3296.y, _e3351.y), fma(((_e3920 * _e3314.member_2.z) * _e3314.member_2.w), _e3296.z, _e3351.z));
                                                                                                                                } else {
                                                                                                                                    phi_8150_ = _e3351;
                                                                                                                                }
                                                                                                                                let _e3942 = phi_8150_;
                                                                                                                                local_154 = _e3942;
                                                                                                                                local_155 = select(select(_e3901, false, _e3903), false, _e3906);
                                                                                                                                continue;
                                                                                                                            } else {
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            continuing {
                                                                                                                                let _e6983 = local_152;
                                                                                                                                phi_8144_ = _e6983;
                                                                                                                                let _e6986 = local_153;
                                                                                                                                phi_8147_ = _e6986;
                                                                                                                                let _e6989 = local_154;
                                                                                                                                phi_8149_ = _e6989;
                                                                                                                                let _e6992 = local_155;
                                                                                                                                phi_8151_ = _e6992;
                                                                                                                            }
                                                                                                                        }
                                                                                                                        let _e7186 = local_200;
                                                                                                                        phi_8963_ = _e7186;
                                                                                                                        phi_8964_ = _e3300;
                                                                                                                        let _e7194 = local_203;
                                                                                                                        phi_8965_ = _e7194;
                                                                                                                    }
                                                                                                                    let _e4591 = phi_8963_;
                                                                                                                    let _e4593 = phi_8964_;
                                                                                                                    let _e4595 = phi_8965_;
                                                                                                                    local_204 = _e4591;
                                                                                                                    local_205 = _e4593;
                                                                                                                    local_206 = (_e3302 + 1u);
                                                                                                                    local_207 = _e4595;
                                                                                                                } else {
                                                                                                                    break;
                                                                                                                }
                                                                                                                let _e7197 = local_204;
                                                                                                                phi_7194_ = _e7197;
                                                                                                                let _e7200 = local_205;
                                                                                                                phi_7197_ = _e7200;
                                                                                                                let _e7203 = local_206;
                                                                                                                phi_7199_ = _e7203;
                                                                                                                let _e7206 = local_207;
                                                                                                                phi_7201_ = _e7206;
                                                                                                            } else {
                                                                                                                phi_7194_ = _e3298;
                                                                                                                phi_7197_ = _e3300;
                                                                                                                phi_7199_ = _e3302;
                                                                                                                phi_7201_ = _e3304;
                                                                                                            }
                                                                                                            let _e4598 = phi_7194_;
                                                                                                            let _e4600 = phi_7197_;
                                                                                                            let _e4602 = phi_7199_;
                                                                                                            let _e4604 = phi_7201_;
                                                                                                            local_146 = _e4598;
                                                                                                            local_147 = _e4600;
                                                                                                            local_148 = _e4602;
                                                                                                            local_149 = _e4604;
                                                                                                            local_150 = select(_e3304, _e3306, vec3<bool>(_e3309));
                                                                                                            local_151 = select(false, true, _e3309);
                                                                                                            continue;
                                                                                                        } else {
                                                                                                            break;
                                                                                                        }
                                                                                                        continuing {
                                                                                                            let _e6965 = local_146;
                                                                                                            phi_7193_ = _e6965;
                                                                                                            let _e6968 = local_147;
                                                                                                            phi_7196_ = _e6968;
                                                                                                            let _e6971 = local_148;
                                                                                                            phi_7198_ = _e6971;
                                                                                                            let _e6974 = local_149;
                                                                                                            phi_7200_ = _e6974;
                                                                                                            let _e6977 = local_150;
                                                                                                            phi_7202_ = _e6977;
                                                                                                            let _e6980 = local_151;
                                                                                                            phi_7204_ = _e6980;
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    break;
                                                                                                }
                                                                                                let _e7209 = local_208;
                                                                                                phi_8982_ = _e7209;
                                                                                                let _e7212 = local_209;
                                                                                                phi_8983_ = _e7212;
                                                                                                let _e7215 = local_210;
                                                                                                phi_8984_ = _e7215;
                                                                                            } else {
                                                                                                phi_8982_ = _e2815;
                                                                                                phi_8983_ = _e2817;
                                                                                                phi_8984_ = vec3<f32>(0.0, 0.0, 0.0);
                                                                                            }
                                                                                            let _e4609 = phi_8982_;
                                                                                            let _e4611 = phi_8983_;
                                                                                            let _e4613 = phi_8984_;
                                                                                            let _e4617 = (1.0 - _e2777.member_1.w);
                                                                                            phi_6485_ = _e4609;
                                                                                            phi_6488_ = _e4611;
                                                                                            phi_6494_ = _e4613;
                                                                                            phi_6496_ = vec3<f32>(fma(_e2825.x, _e2777.member_1.w, (_e4613.x * _e4617)), fma(_e2825.y, _e2777.member_1.w, (_e4613.y * _e4617)), fma(_e2825.z, _e2777.member_1.w, (_e4613.z * _e4617)));
                                                                                        } else {
                                                                                            phi_6485_ = _e2815;
                                                                                            phi_6488_ = _e2817;
                                                                                            phi_6494_ = _e2823;
                                                                                            phi_6496_ = _e2825;
                                                                                        }
                                                                                        let _e4629 = phi_6485_;
                                                                                        let _e4631 = phi_6488_;
                                                                                        let _e4633 = phi_6494_;
                                                                                        let _e4635 = phi_6496_;
                                                                                        local_131 = _e4629;
                                                                                        local_132 = _e4631;
                                                                                        local_135 = _e4633;
                                                                                        local_136 = _e4635;
                                                                                        local_137 = select(true, false, _e3136);
                                                                                    } else {
                                                                                        break;
                                                                                    }
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                                continue;
                                                                            } else {
                                                                                break;
                                                                            }
                                                                            continuing {
                                                                                let _e6907 = local_131;
                                                                                phi_6484_ = _e6907;
                                                                                let _e6910 = local_132;
                                                                                phi_6487_ = _e6910;
                                                                                let _e6913 = local_133;
                                                                                phi_6489_ = _e6913;
                                                                                let _e6916 = local_134;
                                                                                phi_6491_ = _e6916;
                                                                                let _e6919 = local_135;
                                                                                phi_6493_ = _e6919;
                                                                                let _e6922 = local_136;
                                                                                phi_6495_ = _e6922;
                                                                                let _e6925 = local_137;
                                                                                phi_6497_ = _e6925;
                                                                            }
                                                                        }
                                                                        let _e7237 = local_213;
                                                                        phi_6154_ = _e7237;
                                                                        let _e7240 = local_214;
                                                                        phi_6157_ = _e7240;
                                                                        phi_6159_ = _e2804;
                                                                        let _e7248 = local_217;
                                                                        phi_6161_ = _e7248;
                                                                        let _e7251 = local_218;
                                                                        phi_6163_ = _e7251;
                                                                        let _e7254 = local_219;
                                                                        phi_6165_ = _e7254;
                                                                    }
                                                                    let _e4858 = phi_6154_;
                                                                    let _e4860 = phi_6157_;
                                                                    let _e4862 = phi_6159_;
                                                                    let _e4864 = phi_6161_;
                                                                    let _e4866 = phi_6163_;
                                                                    let _e4868 = phi_6165_;
                                                                    local_124 = _e4858;
                                                                    local_125 = _e4860;
                                                                    local_126 = _e4862;
                                                                    local_127 = _e4864;
                                                                    local_128 = _e4866;
                                                                    local_129 = _e4868;
                                                                    local_130 = select(false, true, _e2813);
                                                                    continue;
                                                                } else {
                                                                    break;
                                                                }
                                                                continuing {
                                                                    let _e6886 = local_124;
                                                                    phi_6153_ = _e6886;
                                                                    let _e6889 = local_125;
                                                                    phi_6156_ = _e6889;
                                                                    let _e6892 = local_126;
                                                                    phi_6158_ = _e6892;
                                                                    let _e6895 = local_127;
                                                                    phi_6160_ = _e6895;
                                                                    let _e6898 = local_128;
                                                                    phi_6162_ = _e6898;
                                                                    let _e6901 = local_129;
                                                                    phi_6164_ = _e6901;
                                                                    let _e6904 = local_130;
                                                                    phi_6166_ = _e6904;
                                                                }
                                                            }
                                                            let _e7257 = local_220;
                                                            phi_9019_ = _e7257;
                                                            let _e7260 = local_221;
                                                            phi_9020_ = _e7260;
                                                            let _e7263 = local_222;
                                                            phi_9021_ = _e7263;
                                                            let _e7266 = local_223;
                                                            phi_9022_ = _e7266;
                                                        } else {
                                                            phi_9019_ = _e620;
                                                            phi_9020_ = _e622;
                                                            phi_9021_ = _e624;
                                                            phi_9022_ = _e2760;
                                                        }
                                                        let _e4871 = phi_9019_;
                                                        let _e4873 = phi_9020_;
                                                        let _e4875 = phi_9021_;
                                                        let _e4877 = phi_9022_;
                                                        phi_1382_ = _e4871;
                                                        phi_1385_ = _e4873;
                                                        phi_1387_ = _e4875;
                                                        phi_1389_ = _e2754;
                                                        phi_1391_ = _e2756;
                                                        phi_1393_ = _e2758;
                                                        phi_1395_ = _e632;
                                                        phi_1397_ = _e634;
                                                        phi_1399_ = _e636;
                                                        phi_1401_ = _e638;
                                                        phi_1403_ = _e4877;
                                                    }
                                                    let _e6166 = phi_1382_;
                                                    let _e6168 = phi_1385_;
                                                    let _e6170 = phi_1387_;
                                                    let _e6172 = phi_1389_;
                                                    let _e6174 = phi_1391_;
                                                    let _e6176 = phi_1393_;
                                                    let _e6178 = phi_1395_;
                                                    let _e6180 = phi_1397_;
                                                    let _e6182 = phi_1399_;
                                                    let _e6184 = phi_1401_;
                                                    let _e6186 = phi_1403_;
                                                    local_12 = _e6166;
                                                    local_13 = _e6168;
                                                    local_14 = _e6170;
                                                    local_15 = _e6172;
                                                    local_16 = _e6174;
                                                    local_17 = _e6176;
                                                    local_18 = _e6178;
                                                    local_19 = _e6180;
                                                    local_20 = _e6182;
                                                    local_21 = _e6184;
                                                    local_22 = _e6186;
                                                    local_23 = select(false, true, _e647);
                                                    continue;
                                                } else {
                                                    break;
                                                }
                                                continuing {
                                                    let _e6466 = local_12;
                                                    phi_1381_ = _e6466;
                                                    let _e6469 = local_13;
                                                    phi_1384_ = _e6469;
                                                    let _e6472 = local_14;
                                                    phi_1386_ = _e6472;
                                                    let _e6475 = local_15;
                                                    phi_1388_ = _e6475;
                                                    let _e6478 = local_16;
                                                    phi_1390_ = _e6478;
                                                    let _e6481 = local_17;
                                                    phi_1392_ = _e6481;
                                                    let _e6484 = local_18;
                                                    phi_1394_ = _e6484;
                                                    let _e6487 = local_19;
                                                    phi_1396_ = _e6487;
                                                    let _e6490 = local_20;
                                                    phi_1398_ = _e6490;
                                                    let _e6493 = local_21;
                                                    phi_1400_ = _e6493;
                                                    let _e6496 = local_22;
                                                    phi_1402_ = _e6496;
                                                    let _e6499 = local_23;
                                                    phi_1404_ = _e6499;
                                                }
                                            }
                                        } else {
                                            phi_17890_ = false;
                                            break;
                                        }
                                        let _e7502 = local_280;
                                        phi_9031_ = _e7502;
                                    } else {
                                        phi_9031_ = vec3<f32>(0.0, 0.0, 0.0);
                                    }
                                    let _e6189 = phi_9031_;
                                    global_12 = vec4<f32>(_e6189.x, _e6189.y, _e6189.z, 1.0);
                                    phi_17890_ = true;
                                    break;
                                }
                            } else {
                                phi_17890_ = false;
                                break;
                            }
                        } else {
                            phi_17890_ = false;
                            break;
                        }
                        continue;
                        continuing {
                            let _e6423 = local_2;
                            phi_680_ = _e6423;
                            let _e6426 = local_3;
                            phi_683_ = _e6426;
                        }
                    }
                    let _e6195 = phi_17890_;
                    phi_17913_ = _e6195;
                    if _e6195 {
                        break;
                    }
                    phi_17913_ = _e6195;
                    break;
                }
                continue;
                continuing {
                    let _e6417 = local;
                    phi_359_ = _e6417;
                    let _e6420 = local_1;
                    phi_362_ = _e6420;
                }
            }
            let _e6414 = phi_17913_;
            if _e6414 {
                break;
            }
            break;
        }
    }
    return;
}

@vertex 
fn vs_main(@builtin(vertex_index) param: u32) -> @builtin(position) vec4<f32> {
    global = i32(param);
    function_();
    let _e5 = global_1.y;
    global_1.y = -(_e5);
    let _e7 = global_1;
    return _e7;
}

@fragment 
fn fs_main(@builtin(position) param_1: vec4<f32>) -> @location(0) vec4<f32> {
    global_2 = param_1;
    function_1();
    let _e3 = global_12;
    return _e3;
}
