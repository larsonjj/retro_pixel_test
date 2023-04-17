//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct CustomMaterial {
    color: vec4<f32>,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
}

const COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;

// Automatic smoothing
// independent of geometry and perspective
fn texturePointSmooth(tex: texture_2d<f32>, smp: sampler, uv: vec2<f32>, pixel_size: vec2<f32> ) -> vec4<f32>
{
	var ddx: vec2<f32> = dpdx(uv);
	var ddy: vec2<f32> = dpdy(uv);
	var lxy: vec2<f32> = sqrt(ddx * ddx + ddy * ddy);
    var half: vec2<f32> = vec2<f32>(0.5);
	
	var uv_pixels: vec2<f32> = uv / pixel_size;
	
	var uv_pixels_floor: vec2<f32> = round(uv_pixels) - half;
	var uv_dxy_pixels: vec2<f32> = uv_pixels - uv_pixels_floor;
	
	var uv_dxy_pixels_clamped = clamp((uv_dxy_pixels - half) * pixel_size / lxy + half, vec2<f32>(0.0), vec2<f32>(1.0));
	
	var _uv = uv_pixels_floor * pixel_size + uv_dxy_pixels_clamped * pixel_size;
	
	return textureSampleGrad(tex, smp, _uv, ddx, ddy);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var texture_dimensions: vec2<f32> = vec2<f32>(textureDimensions(texture));
    return texturePointSmooth(texture, texture_sampler, input.uv, 1.0 / texture_dimensions);
}
