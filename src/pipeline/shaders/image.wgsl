struct CameraUniform {
    window_to_render: mat4x4<f32>,
    base_to_viewport_window: mat4x4<f32>,
    base_to_cropped_base: mat4x4<f32>,
    view_to_crop: mat4x4<f32>,
    base_to_image_area: mat4x4<f32>,
    base_to_export_area: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct ParameterUniform {
    brightness: f32,
    contrast: f32,
    shadows: f32,
    highlights: f32,
    tint: f32,
    temperature: f32,
    saturation: f32
};
@group(0) @binding(1)
var<uniform> parameters: ParameterUniform;

struct CropUniform {
    width: f32,
    height: f32,
    ratio: f32,
    display_grid: i32
};
@group(0) @binding(2)
var<uniform> crop: CropUniform;

struct RadialParameter {
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    angle: f32,
    feather: f32,
    brightness: f32,
    _1: f32,
}
struct RadialParameters {
    entries: array<RadialParameter, 128>,
    count: u32
}
@group(0) @binding(3)
var<uniform> radial_parameters: RadialParameters;

struct Vertex {
    @location(0) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) render_position: vec4<f32>,
    @location(0) view_coords: vec2<f32>,
    @location(1) crop_coords: vec2<f32>,
    @location(2) image_coords: vec2<f32>,
    @location(3) export_coords: vec2<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let base: vec4<f32> = vec4<f32>(vertex.uv, 0.0, 1.0);
    let render_position = base * camera.base_to_viewport_window * camera.window_to_render;
    let view_coords = base * camera.base_to_cropped_base;
    let crop_coords = view_coords * camera.view_to_crop * camera.base_to_export_area;
    let image_coords = base * camera.base_to_image_area;
    let export_coords = base * camera.base_to_export_area;

    out.render_position = render_position;
    out.view_coords = view_coords.xy / view_coords.w;
    out.crop_coords = crop_coords.xy / crop_coords.w;
    out.image_coords = image_coords.xy / image_coords.w;
    out.export_coords = export_coords.xy / export_coords.w;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;
@group(1) @binding(2)
var t_output: texture_storage_2d<rgba8unorm, write>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (all(in.view_coords >= vec2(0.0) && in.view_coords <= vec2(1.0))) {
        let lab_actual: vec3<f32> = get_pixel_color(in.view_coords, in.image_coords);
        let lab_crop: vec3<f32> = draw_crop_area(in, lab_actual);
        let lab_final: vec3<f32> = draw_grid(in, lab_crop);

        let rgb_actual: vec3<f32> = lab_to_rgb(lab_actual);
        let rgb_final: vec3<f32> = lab_to_rgb(lab_final);

        let gamma_corrected: vec3<f32> = pow(rgb_actual, vec3(1.0/2.2));
        textureStore(t_output, vec2<i32>(in.export_coords.xy), vec4<f32>(gamma_corrected, 1.0));

        return vec4<f32>(rgb_final, 1.0);
    } else {
        textureStore(t_output, vec2<i32>(in.export_coords.xy), vec4(1.0));
        return vec4(1.0);
    }
}

fn get_pixel_color(view_coords: vec2<f32>, image_coords: vec2<f32>) -> vec3<f32> {
    let texture_sample: vec4<f32> = textureSample(t_diffuse, s_diffuse, view_coords);
    let rgb: vec3<f32> = texture_sample.xyz;
    let lab: vec3<f32> = rgb_to_lab(rgb);
    return apply_parameters(lab, image_coords);
}

fn apply_parameters(lab: vec3<f32>, position: vec2<f32>) -> vec3<f32> {
    let globally_applied: vec3<f32> = apply_global_parameters(lab);
    let masked: vec3<f32> = apply_all_radial_parameters(globally_applied, position);
    return masked;
}

fn apply_global_parameters(lab: vec3<f32>) -> vec3<f32> {
    var applied: vec3<f32> = lab;

    // Color adjustment
    applied += vec3<f32>(0.0, parameters.tint, parameters.temperature);
    applied *= vec3<f32>(1.0, parameters.saturation, parameters.saturation);

    applied = to_lightness_adjustment_space(applied);

    // Lightness adjustment
    var brightness_value = calculate_brightness_value(applied);
    applied = apply_brightness(applied, brightness_value);
    applied -= vec3<f32>(0.5, 0.0, 0.0);
    applied *= vec3<f32>(parameters.contrast, 1.0, 1.0);
    applied += vec3<f32>(0.5, 0.0, 0.0);

    applied = from_lightness_adjustment_space(applied);
    return applied;
}

fn to_lightness_adjustment_space(lab: vec3<f32>) -> vec3<f32> {
    return lab * vec3<f32>(1.0, 1.0 / (lab.x + 0.1), 1.0 / (lab.x + 0.1));
}

fn from_lightness_adjustment_space(lab: vec3<f32>) -> vec3<f32> {
    return lab * vec3<f32>(1.0, lab.x + 0.1, lab.x + 0.1);
}

fn calculate_brightness_value(lab: vec3<f32>) -> f32 {
    var shadows_modifier: f32 = clamp(0.5 - lab.x, 0.0, 1.0);
    var highlights_modifier: f32 = clamp(lab.x - 0.5, 0.0, 1.0);

    return shadows_modifier * parameters.shadows +
        highlights_modifier * parameters.highlights +
        parameters.brightness;
}

fn apply_brightness(lab: vec3<f32>, brightness: f32) -> vec3<f32> {
    return lab * vec3<f32>((brightness * 0.01) + 1.0, 1.0, 1.0);
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

    let alpha = calculate_alpha(position, radial_parameter);

    if (alpha > 0.0) {
        var applied: vec3<f32> = lab;
        applied = to_lightness_adjustment_space(applied);
        applied = apply_brightness(applied, radial_parameter.brightness);
        applied = from_lightness_adjustment_space(applied);

        return lab * (1.0 - alpha) + applied * alpha;
    } else {
        return lab;
    }
}

fn calculate_alpha(position: vec2<f32>, radial_parameter: RadialParameter) -> f32 {
    let angle_matrix = mat2x2<f32>(
        cos(radial_parameter.angle), -sin(radial_parameter.angle),
        sin(radial_parameter.angle), cos(radial_parameter.angle)
    );
    let scale_matrix = mat2x2<f32>(
        1.0 / (radial_parameter.width * radial_parameter.width), 0.0,
        0.0, 1.0 / (radial_parameter.height * radial_parameter.height)
    );

    let difference = (vec2<f32>(radial_parameter.center_x, radial_parameter.center_y) - position) * angle_matrix;

    if (difference.x < 0.0 && 1.0 / radial_parameter.height == 0.0) {
        return 1.0;
    }

    let mahalanobis_matrix = scale_matrix;
    let distance = sqrt(dot(difference, mahalanobis_matrix * difference));
    if (radial_parameter.feather <= 0.0) {
        if (distance > 1.0) {
            return 0.0;
        } else {
            return 1.0;
        }
    } else {
        let feathered_distance = (distance - (1.0 - radial_parameter.feather)) / (radial_parameter.feather);
        return cubic_hermite(feathered_distance);
    }
}

fn cubic_hermite(x: f32) -> f32 {
    if (x > 1.0) {
        return 0.0;
    } else if (x < 0.0) {
        return 1.0;
    } else {
        return 2.0 * x * x * x - 3.0 * x * x + 1.0;
    }
}

fn draw_crop_area(vertex: VertexOutput, lab: vec3<f32>) -> vec3<f32> {
    if (in_crop_area(vertex)) {
        return lab;
    } else if (in_crop_border(vertex)) {
        return vec3<f32>(1.0 - lab.x, lab.yz);
    } else {
        return lab * vec3<f32>(0.5, 0.25, 0.25);
    }
}

fn in_crop_area(vertex: VertexOutput) -> bool {
    let position = vertex.crop_coords;
    if (position.x < 0.0 || position.x > crop.width ||
            position.y < 0.0 || position.y > crop.height) {
        return false;
    } else {
        return true;
    }
}

fn in_crop_border(vertex: VertexOutput) -> bool {
    let position = vertex.crop_coords * crop.ratio;
    if (position.x < -1.0 || position.x > crop.width * crop.ratio + 1.0 ||
            position.y < -1.0 || position.y > crop.height * crop.ratio + 1.0) {
        return false;
    } else {
        return true;
    }
}

fn draw_grid(vertex: VertexOutput, lab: vec3<f32>) -> vec3<f32> {
    if (crop.display_grid == 0) {
        return lab;
    } else if (in_big_grid(vertex)) {
        if (lab.x > 0.8) {
            return vec3<f32>(0.7, lab.yz * 0.5);
        } else {
            return vec3<f32>(0.9, lab.yz * 0.5);
        }
    } else if (in_small_grid(vertex)) {
        if (lab.x > 0.7) {
            return vec3<f32>(0.6, lab.yz * 0.5);
        } else {
            return vec3<f32>(0.8, lab.yz * 0.5);
        }
    } else {
        return lab;
    }
}

fn in_big_grid(vertex: VertexOutput) -> bool {
    let position = vertex.crop_coords * crop.ratio;
    let width = crop.width * crop.ratio;
    let height = crop.height * crop.ratio;

    let in_vertical_grid = 
        in_pixel(position.x, width * 2.0 / 6.0) ||
        in_pixel(position.x, width * 4.0 / 6.0);
    let in_horizontal_grid =
        in_pixel(position.y, height * 2.0 / 6.0) ||
        in_pixel(position.y, height * 4.0 / 6.0);

    if (in_vertical_grid) {
        return true;
    } else if (in_horizontal_grid) {
        return true;
    } else {
        return false;
    }
}

fn in_small_grid(vertex: VertexOutput) -> bool {
    let position = vertex.crop_coords * crop.ratio;
    let width = crop.width * crop.ratio;
    let height = crop.height * crop.ratio;

    let in_vertical_grid = 
        in_pixel(position.x, width * 1.0 / 6.0) ||
        in_pixel(position.x, width * 3.0 / 6.0) ||
        in_pixel(position.x, width * 5.0 / 6.0);
    let in_horizontal_grid =
        in_pixel(position.y, height * 1.0 / 6.0) ||
        in_pixel(position.y, height * 3.0 / 6.0) ||
        in_pixel(position.y, height * 5.0 / 6.0);

    if (i32(position.y) % 2 == 0 && in_vertical_grid) {
        return true;
    } else if (i32(position.x) % 2 == 0 && in_horizontal_grid) {
        return true;
    } else {
        return false;
    }
}

fn in_pixel(x1: f32, x2: f32) -> bool {
    return x1 >= x2 - 0.75 && x1 <= x2 + 0.75;
}

/**
 * Using Oklab color space
 * Conversions based on: https://bottosson.github.io/posts/oklab/
 */
fn rgb_to_lab(rgb: vec3<f32>) -> vec3<f32> {
    let lms: vec3<f32> = vec3<f32>(
        0.4122214708 * rgb.x + 0.5363325363 * rgb.y + 0.0514459929 * rgb.z,
        0.2119034982 * rgb.x + 0.6806995451 * rgb.y + 0.1073969566 * rgb.z,
        0.0883024619 * rgb.x + 0.2817188376 * rgb.y + 0.6299787005 * rgb.z
    );
    let lms_root: vec3<f32> = pow(lms, vec3(1.0 / 3.0));
    return vec3<f32>(
        0.2104542553 * lms_root.x + 0.7936177850 * lms_root.y - 0.0040720468 * lms_root.z,
        1.9779984951 * lms_root.x - 2.4285922050 * lms_root.y + 0.4505937099 * lms_root.z,
        0.0259040371 * lms_root.x + 0.7827717662 * lms_root.y - 0.8086757660 * lms_root.z,
    );
}

fn lab_to_rgb(lab: vec3<f32>) -> vec3<f32> {
    let lms_root: vec3<f32> = vec3<f32>(
        lab.x + 0.3963377774 * lab.y + 0.2158037573 * lab.z,
        lab.x - 0.1055613458 * lab.y - 0.0638541728 * lab.z,
        lab.x - 0.0894841775 * lab.y - 1.2914855480 * lab.z
    );
    let lms: vec3<f32> = pow(lms_root, vec3(3.0));
    return vec3<f32>(
         4.0767416621 * lms.x - 3.3077115913 * lms.y + 0.2309699292 * lms.z,
        -1.2684380046 * lms.x + 2.6097574011 * lms.y - 0.3413193965 * lms.z,
        -0.0041960863 * lms.x - 0.7034186147 * lms.y + 1.7076147010 * lms.z,
    );
}