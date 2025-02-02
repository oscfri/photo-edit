struct CameraUniform {
    window_to_render: mat4x4<f32>,
    base_to_viewport_window: mat4x4<f32>,
    base_to_cropped_base: mat4x4<f32>,
    base_to_cropped_base2: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct CropUniform {
    top_left: vec2<f32>,
    bottom_right: vec2<f32>,
    visible: i32
};
@group(0) @binding(2)
var<uniform> crop: CropUniform;

struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) render_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tex_coords2: vec2<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let base: vec4<f32> = vec4<f32>(vertex.uv - 0.5, 0.0, 1.0);
    let render_position = base * camera.base_to_viewport_window * camera.window_to_render;

    out.render_position = render_position;
    out.tex_coords = (base * camera.base_to_cropped_base).xy;
    out.tex_coords2 = (base * camera.base_to_cropped_base * camera.base_to_cropped_base2).xy;
    return out;
}

struct ParameterUniform {
    brightness: f32,
    contrast: f32,
    tint: f32,
    temperature: f32,
    saturation: f32
};
@group(0) @binding(1)
var<uniform> parameters: ParameterUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_sample: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let rgb: vec3<f32> = texture_sample.xyz;
    let lab: vec3<f32> = rgb_to_lab(rgb);
    let lab_applied: vec3<f32> = apply_parameters(lab);
    let rgb_applied: vec3<f32> = lab_to_rgb(lab_applied);
    let rgb_final: vec3<f32> = draw_crop_area(in, rgb_applied);

    return vec4<f32>(rgb_final, 1.0);
}

fn apply_parameters(lab: vec3<f32>) -> vec3<f32> {
    var adjusted: vec3<f32> = lab;

    let multiplier: vec3<f32> = (vec3<f32>(parameters.contrast, parameters.saturation, parameters.saturation) + 100.0) / 100.0;
    let adder: vec3<f32> = vec3<f32>(parameters.brightness, parameters.tint, parameters.temperature);

    adjusted -= vec3<f32>(50.0, 0.0, 0.0); // Center brightness value
    adjusted *= multiplier;
    adjusted += adder;
    adjusted += vec3<f32>(50.0, 0.0, 0.0); // Revert brightness value

    return adjusted;
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
    let position = vertex.tex_coords2;
    if (position.x < 0.0 || position.x > 1.0 ||
            position.y < 0.0 || position.y > 1.0) {
        return false;
    } else {
        return true;
    }
}

fn in_crop_border(vertex: VertexOutput) -> bool {
    let position = vertex.tex_coords2;
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