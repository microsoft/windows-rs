// Draws glyphs from an atlas texture. Each instance is a screen rectangle in pixels, the
// matching rectangle of texels in the atlas, a color, and how to adjust the coverage. The atlas
// holds coverage in red, green, and blue: three values for a ClearType glyph and one value
// repeated for a grayscale glyph.

cbuffer Constants : register(b0) {
    float2 viewport;
    // DirectWrite's alpha correction for the display's gamma, from `RenderingParams::gamma_ratios`.
    float4 gamma;
};

struct Instance {
    float4 bounds;
    float4 texels;
    float4 color;
    // DirectWrite's enhanced contrast and ClearType level for the coverage, and 1.0 for ClearType
    // coverage or 0.0 for grayscale. A level of 1.0 keeps the per-channel coverage as is.
    float3 coverage;
    float padding;
};

StructuredBuffer<Instance> instances : register(t0);
Texture2D<float4> atlas : register(t1);

struct Vertex {
    float4 position : SV_Position;
    float2 texel : TEXCOORD;
    nointerpolation float4 color : COLOR;
    nointerpolation float3 coverage : COVERAGE;
};

Vertex vs(uint vertex : SV_VertexID, uint id : SV_InstanceID) {
    Instance instance = instances[id];
    float2 corner = float2(vertex & 1, vertex >> 1);
    float2 position = instance.bounds.xy + corner * instance.bounds.zw;

    Vertex output;
    output.position = float4(position / viewport * float2(2, -2) + float2(-1, 1), 0, 1);
    output.texel = instance.texels.xy + corner * instance.texels.zw;
    output.color = instance.color;
    output.coverage = instance.coverage;
    return output;
}

struct Output {
    float4 color : SV_Target0;
    float4 alpha : SV_Target1;
};

// Adjusts the coverage as DirectWrite does, then writes the premultiplied color to the first
// target and the per-channel alpha to the second. The blend state computes
// `color + target * (1 - alpha)` for each channel.
Output ps(Vertex input) {
    float3 coverage = atlas.Load(int3(input.texel, 0)).rgb;
    float3 color = input.color.rgb;
    float average = (coverage.r + coverage.g + coverage.b) / 3;
    coverage = lerp(average, coverage, input.coverage.y);

    // Light text gets less contrast, and white text none.
    float lightness = dot(color, float3(0.30, 0.59, 0.11));
    float contrast = input.coverage.x * saturate(4 * (0.75 - lightness));
    coverage = coverage * (contrast + 1) / (coverage * contrast + 1);

    float3 intensity = lerp(dot(color, float3(0.25, 0.5, 0.25)), color, input.coverage.z);
    float3 ratio = (gamma.x * intensity + gamma.y) * coverage + gamma.z * intensity + gamma.w;
    coverage += coverage * (1 - coverage) * ratio;
    float3 alpha = coverage * input.color.a;

    Output output;
    output.color = float4(input.color.rgb * alpha, max(alpha.r, max(alpha.g, alpha.b)));
    output.alpha = float4(alpha, output.color.a);
    return output;
}
