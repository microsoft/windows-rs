Texture2D<float4> g_input    : register(t0, space0);
RWTexture2D<float4> g_output : register(u0, space0);

[numthreads(8, 8, 1)]
void copyCs(uint3 dispatchThreadId : SV_DispatchThreadID)
{
	g_output[dispatchThreadId.xy] = g_input[dispatchThreadId.xy];
}
