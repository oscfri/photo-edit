struct CameraUniform {
    window_to_render: mat4x4<f32>,
    base_to_viewport_window: mat4x4<f32>,
    base_to_cropped_base: mat4x4<f32>,
    base_to_cropped_base2: mat4x4<f32>,
    base_to_image_area: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct ParameterUniform {
    brightness: f32,
    contrast: f32,
    tint: f32,
    temperature: f32,
    saturation: f32
};
@group(0) @binding(1)
var<uniform> parameters: ParameterUniform;

struct CropUniform {
    visible: i32
};
@group(0) @binding(2)
var<uniform> crop: CropUniform;

struct RadialParameter {
    center_x: f32,
    center_y: f32,
    radius: f32,
    brightness: f32
}
struct RadialParameters {
    entries: array<RadialParameter, 128>,
    count: u32
}
@group(0) @binding(3)
var<uniform> radial_parameters: RadialParameters;

struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) render_position: vec4<f32>,
    @location(0) view_coords: vec2<f32>,
    @location(1) crop_coords: vec2<f32>,
    @location(2) image_coords: vec2<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let base: vec4<f32> = vec4<f32>(vertex.uv, 0.0, 1.0);
    let render_position = base * camera.base_to_viewport_window * camera.window_to_render;
    let view_coords = base * camera.base_to_cropped_base;
    let crop_coords = view_coords * camera.base_to_cropped_base2;
    let image_coords = base * camera.base_to_image_area;

    out.render_position = render_position;
    out.view_coords = view_coords.xy / view_coords.w;
    out.crop_coords = crop_coords.xy / crop_coords.w;
    out.image_coords = image_coords.xy / image_coords.w;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_sample: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.view_coords);
    let rgb: vec3<f32> = texture_sample.xyz;
    let lab: vec3<f32> = rgb_to_lab(rgb);
    let lab_applied: vec3<f32> = apply_parameters(lab, in.image_coords);
    let rgb_applied: vec3<f32> = lab_to_rgb(lab_applied);
    let rgb_final: vec3<f32> = draw_crop_area(in, rgb_applied);

    return vec4<f32>(rgb_final, 1.0);
}

fn apply_parameters(lab: vec3<f32>, position: vec2<f32>) -> vec3<f32> {
    let globally_applied: vec3<f32> = apply_global_parameters(lab);
    let masked: vec3<f32> = apply_all_radial_parameters(globally_applied, position);
    return masked;
}

fn apply_global_parameters(lab: vec3<f32>) -> vec3<f32> {
    var applied: vec4<f32> = vec4<f32>(lab, 1.0);

    let matrix: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(parameters.contrast, 0.0, 0.0, parameters.brightness),
        vec4<f32>(0.0, parameters.saturation, 0.0, parameters.tint),
        vec4<f32>(0.0, 0.0, parameters.saturation, parameters.temperature),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    applied -= vec4<f32>(50.0, 0.0, 0.0, 0.0); // Center brightness value
    applied = applied * matrix;
    applied += vec4<f32>(50.0, 0.0, 0.0, 0.0); // Revert brightness value

    return applied.xyz / applied.w;
}

fn apply_all_radial_parameters(lab: vec3<f32>, position: vec2<f32>) -> vec3<f32> {
    var applied: vec3<f32> = lab;

    for (var i = 0u; i < radial_parameters.count; i++) {
        applied = apply_radial_parameters(i, applied, position);
    }

    return applied;
}

fn apply_radial_parameters(index: u32, lab: vec3<f32>, position: vec2<f32>) -> vec3<f32> {
    let radial_parameter = radial_parameters.entries[index];
    let distance = distance(position, vec2<f32>(radial_parameter.center_x, radial_parameter.center_y));
    let radius = radial_parameter.radius;
    let alpha = clamp((radius - distance) / radius, 0.0, 1.0);
    if (alpha > 0.0) {
        return lab + vec3<f32>(radial_parameter.brightness, 0.0, 0.0) * alpha;
    } else {
        return lab;
    }
}

fn draw_crop_area(vertex: VertexOutput, rgb: vec3<f32>) -> vec3<f32> {
    if (crop.visible == 0) {
        return rgb;
    } else if (in_crop_area(vertex)) {
        return rgb;
    } else if (in_crop_border(vertex)) {
        return vec3<f32>(1.0, 1.0, 1.0);
    } else {
        return rgb * 0.25;
    }
}

fn in_crop_area(vertex: VertexOutput) -> bool {
    let position = vertex.crop_coords;
    if (position.x < 0.0 || position.x > 1.0 ||
            position.y < 0.0 || position.y > 1.0) {
        return false;
    } else {
        return true;
    }
}

fn in_crop_border(vertex: VertexOutput) -> bool {
    let position = vertex.crop_coords;
    if (position.x < 0.0 || position.x > 1.0 ||
            position.y < 0.0 || position.y > 1.0) {
        return false;
    } else {
        return true;
    }
}

/**
 * Conversions based on: https://www.easyrgb.com/en/math.php
 */

const REFERENCE_X: f32 = 95.047;
const REFERENCE_Y: f32 = 100.0;
const REFERENCE_Z: f32 = 108.883;

// Conversions RGB -> LAB

fn rgb_to_lab(rgb: vec3<f32>) -> vec3<f32> {
    let xyz: vec3<f32> = rgb_to_xyz(rgb);
    return xyz_to_lab(xyz);
}

fn scale_rgb_to_xyz(value: f32) -> f32 {
    if value > 0.04045 {
        return 100.0 * pow((value + 0.055) / 1.055, 2.4);
    } else {
        return 100.0 * value / 12.92;
    }
}

fn rgb_to_xyz(rgb: vec3<f32>) -> vec3<f32> {
    let scaled_red: f32 = scale_rgb_to_xyz(rgb.x);
    let scaled_green: f32 = scale_rgb_to_xyz(rgb.y);
    let scaled_blue: f32 = scale_rgb_to_xyz(rgb.z);
    return vec3<f32>(
        scaled_red * 0.4124 + scaled_green * 0.3576 + scaled_blue * 0.1805,
        scaled_red * 0.2126 + scaled_green * 0.7152 + scaled_blue * 0.0722,
        scaled_red * 0.0193 + scaled_green * 0.1192 + scaled_blue * 0.9505
    );
}

fn scale_xyz_to_lab(value: f32) -> f32 {
    if value > 0.008856 {
        return pow(value, 1.0 / 3.0);
    } else {
        return (7.787 * value) + (16.0 / 116.0);
    }
}

fn xyz_to_lab(xyz: vec3<f32>) -> vec3<f32> {
    let var_x: f32 = scale_xyz_to_lab(xyz.x / REFERENCE_X);
    let var_y: f32 = scale_xyz_to_lab(xyz.y / REFERENCE_Y);
    let var_z: f32 = scale_xyz_to_lab(xyz.z / REFERENCE_Z);
    return vec3<f32>(
        (116.0 * var_y) - 16.0,
        500.0 * (var_x - var_y),
        200.0 * (var_y - var_z)
    );
}

// Conversions LAB -> RGB

fn lab_to_rgb(lab: vec3<f32>) -> vec3<f32> {
    var xyz: vec3<f32> = lab_to_xyz(lab);
    return xyz_to_rgb(xyz);
}

fn scale_lab_to_xyz(value: f32) -> f32 {
    if value > pow(0.008856, 1.0 / 3.0) {
        return pow(value, 3.0);
    } else {
        return (value - 16.0 / 116.0) / 7.787;
    }
}

fn lab_to_xyz(lab: vec3<f32>) -> vec3<f32> {
    let var_y: f32 = (lab.x + 16.0) / 116.0;
    let var_x: f32 = (lab.y / 500.0) + var_y;
    let var_z: f32 = var_y - (lab.z / 200.0);

    return vec3<f32>(
        scale_lab_to_xyz(var_x) * REFERENCE_X,
        scale_lab_to_xyz(var_y) * REFERENCE_Y,
        scale_lab_to_xyz(var_z) * REFERENCE_Z
    );
}

fn scale_xyz_to_rgb(value: f32) -> f32 {
    let scaled_value: f32 = value / 100.0;
    if scaled_value > 0.0031308 {
        return 1.055 * pow(scaled_value, 1.0 / 2.4) - 0.055;
    } else {
        return scaled_value * 12.92;
    }
}

fn xyz_to_rgb(xyz: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        scale_xyz_to_rgb( xyz.x * 3.2406 - xyz.y * 1.5372 - xyz.z * 0.4986),
        scale_xyz_to_rgb(-xyz.x * 0.9689 + xyz.y * 1.8758 + xyz.z * 0.0415),
        scale_xyz_to_rgb( xyz.x * 0.0557 - xyz.y * 0.2040 + xyz.z * 1.0570)
    );
}