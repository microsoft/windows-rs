// Draws glyphs from an atlas texture. Each instance is a screen rectangle in pixels, the
// matching rectangle of texels in the atlas, a color, and how to adjust the coverage. The atlas holds coverage in red, green, and blue:
// three values for a ClearType glyph and one value repeated for a grayscale glyph.

cbuffer Constants : register(b0) {
    float2 viewport;
};

struct Instance {
    float4 bounds;
    float4 texels;
    float4 color;
    // DirectWrite's enhanced contrast and ClearType level for the coverage. A level of 1.0 keeps
    // the per-channel coverage as is.
    float2 coverage;
    float2 padding;
};

StructuredBuffer<Instance> instances : register(t0);
Texture2D<float4> atlas : register(t1);

struct Vertex {
    float4 position : SV_Position;
    float2 texel : TEXCOORD;
    nointerpolation float4 color : COLOR;
    nointerpolation float2 coverage : COVERAGE;
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

// Writes the premultiplied color to the first target and the per-channel alpha to the second.
// The blend state computes `color + target * (1 - alpha)` for each channel.
Output ps(Vertex input) {
    float3 coverage = atlas.Load(int3(input.texel, 0)).rgb;
    float average = (coverage.r + coverage.g + coverage.b) / 3;
    float contrast = input.coverage.x;
    coverage = lerp(average, coverage, input.coverage.y);
    coverage = coverage * (contrast + 1) / (coverage * contrast + 1);
    float3 alpha = coverage * input.color.a;

    Output output;
    output.color = float4(input.color.rgb * alpha, max(alpha.r, max(alpha.g, alpha.b)));
    output.alpha = float4(alpha, output.color.a);
    return output;
}
