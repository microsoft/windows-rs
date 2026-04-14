/*-------------------------------------------------------------------------------------
 *
 * Copyright (c) Microsoft Corporation
 *
 *-------------------------------------------------------------------------------------*/


/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __d3d10_h__
#define __d3d10_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __ID3D10DeviceChild_FWD_DEFINED__
#define __ID3D10DeviceChild_FWD_DEFINED__
typedef interface ID3D10DeviceChild ID3D10DeviceChild;

#endif 	/* __ID3D10DeviceChild_FWD_DEFINED__ */


#ifndef __ID3D10DepthStencilState_FWD_DEFINED__
#define __ID3D10DepthStencilState_FWD_DEFINED__
typedef interface ID3D10DepthStencilState ID3D10DepthStencilState;

#endif 	/* __ID3D10DepthStencilState_FWD_DEFINED__ */


#ifndef __ID3D10BlendState_FWD_DEFINED__
#define __ID3D10BlendState_FWD_DEFINED__
typedef interface ID3D10BlendState ID3D10BlendState;

#endif 	/* __ID3D10BlendState_FWD_DEFINED__ */


#ifndef __ID3D10RasterizerState_FWD_DEFINED__
#define __ID3D10RasterizerState_FWD_DEFINED__
typedef interface ID3D10RasterizerState ID3D10RasterizerState;

#endif 	/* __ID3D10RasterizerState_FWD_DEFINED__ */


#ifndef __ID3D10Resource_FWD_DEFINED__
#define __ID3D10Resource_FWD_DEFINED__
typedef interface ID3D10Resource ID3D10Resource;

#endif 	/* __ID3D10Resource_FWD_DEFINED__ */


#ifndef __ID3D10Buffer_FWD_DEFINED__
#define __ID3D10Buffer_FWD_DEFINED__
typedef interface ID3D10Buffer ID3D10Buffer;

#endif 	/* __ID3D10Buffer_FWD_DEFINED__ */


#ifndef __ID3D10Texture1D_FWD_DEFINED__
#define __ID3D10Texture1D_FWD_DEFINED__
typedef interface ID3D10Texture1D ID3D10Texture1D;

#endif 	/* __ID3D10Texture1D_FWD_DEFINED__ */


#ifndef __ID3D10Texture2D_FWD_DEFINED__
#define __ID3D10Texture2D_FWD_DEFINED__
typedef interface ID3D10Texture2D ID3D10Texture2D;

#endif 	/* __ID3D10Texture2D_FWD_DEFINED__ */


#ifndef __ID3D10Texture3D_FWD_DEFINED__
#define __ID3D10Texture3D_FWD_DEFINED__
typedef interface ID3D10Texture3D ID3D10Texture3D;

#endif 	/* __ID3D10Texture3D_FWD_DEFINED__ */


#ifndef __ID3D10View_FWD_DEFINED__
#define __ID3D10View_FWD_DEFINED__
typedef interface ID3D10View ID3D10View;

#endif 	/* __ID3D10View_FWD_DEFINED__ */


#ifndef __ID3D10ShaderResourceView_FWD_DEFINED__
#define __ID3D10ShaderResourceView_FWD_DEFINED__
typedef interface ID3D10ShaderResourceView ID3D10ShaderResourceView;

#endif 	/* __ID3D10ShaderResourceView_FWD_DEFINED__ */


#ifndef __ID3D10RenderTargetView_FWD_DEFINED__
#define __ID3D10RenderTargetView_FWD_DEFINED__
typedef interface ID3D10RenderTargetView ID3D10RenderTargetView;

#endif 	/* __ID3D10RenderTargetView_FWD_DEFINED__ */


#ifndef __ID3D10DepthStencilView_FWD_DEFINED__
#define __ID3D10DepthStencilView_FWD_DEFINED__
typedef interface ID3D10DepthStencilView ID3D10DepthStencilView;

#endif 	/* __ID3D10DepthStencilView_FWD_DEFINED__ */


#ifndef __ID3D10VertexShader_FWD_DEFINED__
#define __ID3D10VertexShader_FWD_DEFINED__
typedef interface ID3D10VertexShader ID3D10VertexShader;

#endif 	/* __ID3D10VertexShader_FWD_DEFINED__ */


#ifndef __ID3D10GeometryShader_FWD_DEFINED__
#define __ID3D10GeometryShader_FWD_DEFINED__
typedef interface ID3D10GeometryShader ID3D10GeometryShader;

#endif 	/* __ID3D10GeometryShader_FWD_DEFINED__ */


#ifndef __ID3D10PixelShader_FWD_DEFINED__
#define __ID3D10PixelShader_FWD_DEFINED__
typedef interface ID3D10PixelShader ID3D10PixelShader;

#endif 	/* __ID3D10PixelShader_FWD_DEFINED__ */


#ifndef __ID3D10InputLayout_FWD_DEFINED__
#define __ID3D10InputLayout_FWD_DEFINED__
typedef interface ID3D10InputLayout ID3D10InputLayout;

#endif 	/* __ID3D10InputLayout_FWD_DEFINED__ */


#ifndef __ID3D10SamplerState_FWD_DEFINED__
#define __ID3D10SamplerState_FWD_DEFINED__
typedef interface ID3D10SamplerState ID3D10SamplerState;

#endif 	/* __ID3D10SamplerState_FWD_DEFINED__ */


#ifndef __ID3D10Asynchronous_FWD_DEFINED__
#define __ID3D10Asynchronous_FWD_DEFINED__
typedef interface ID3D10Asynchronous ID3D10Asynchronous;

#endif 	/* __ID3D10Asynchronous_FWD_DEFINED__ */


#ifndef __ID3D10Query_FWD_DEFINED__
#define __ID3D10Query_FWD_DEFINED__
typedef interface ID3D10Query ID3D10Query;

#endif 	/* __ID3D10Query_FWD_DEFINED__ */


#ifndef __ID3D10Predicate_FWD_DEFINED__
#define __ID3D10Predicate_FWD_DEFINED__
typedef interface ID3D10Predicate ID3D10Predicate;

#endif 	/* __ID3D10Predicate_FWD_DEFINED__ */


#ifndef __ID3D10Counter_FWD_DEFINED__
#define __ID3D10Counter_FWD_DEFINED__
typedef interface ID3D10Counter ID3D10Counter;

#endif 	/* __ID3D10Counter_FWD_DEFINED__ */


#ifndef __ID3D10Device_FWD_DEFINED__
#define __ID3D10Device_FWD_DEFINED__
typedef interface ID3D10Device ID3D10Device;

#endif 	/* __ID3D10Device_FWD_DEFINED__ */


#ifndef __ID3D10Multithread_FWD_DEFINED__
#define __ID3D10Multithread_FWD_DEFINED__
typedef interface ID3D10Multithread ID3D10Multithread;

#endif 	/* __ID3D10Multithread_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "dxgi.h"
#include "d3dcommon.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_d3d10_0000_0000 */
/* [local] */ 

#ifndef _D3D10_CONSTANTS
#define _D3D10_CONSTANTS
#define	D3D10_16BIT_INDEX_STRIP_CUT_VALUE	( 0xffff )

#define	D3D10_32BIT_INDEX_STRIP_CUT_VALUE	( 0xffffffff )

#define	D3D10_8BIT_INDEX_STRIP_CUT_VALUE	( 0xff )

#define	D3D10_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT	( 9 )

#define	D3D10_CLIP_OR_CULL_DISTANCE_COUNT	( 8 )

#define	D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT	( 2 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT	( 14 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS	( 4 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT	( 15 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS	( 4 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT	( 15 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST	( 1 )

#define	D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS	( 1 )

#define	D3D10_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT	( 64 )

#define	D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS	( 4 )

#define	D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT	( 1 )

#define	D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST	( 1 )

#define	D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS	( 1 )

#define	D3D10_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS	( 1 )

#define	D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT	( 128 )

#define	D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST	( 1 )

#define	D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS	( 1 )

#define	D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT	( 128 )

#define	D3D10_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS	( 1 )

#define	D3D10_COMMONSHADER_SAMPLER_REGISTER_COUNT	( 16 )

#define	D3D10_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST	( 1 )

#define	D3D10_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS	( 1 )

#define	D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT	( 16 )

#define	D3D10_COMMONSHADER_SUBROUTINE_NESTING_LIMIT	( 32 )

#define	D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENTS	( 4 )

#define	D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_COMMONSHADER_TEMP_REGISTER_COUNT	( 4096 )

#define	D3D10_COMMONSHADER_TEMP_REGISTER_READS_PER_INST	( 3 )

#define	D3D10_COMMONSHADER_TEMP_REGISTER_READ_PORTS	( 3 )

#define	D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX	( 10 )

#define	D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN	( -10 )

#define	D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE	( -8 )

#define	D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE	( 7 )

#define D3D10_DEFAULT_BLEND_FACTOR_ALPHA	( 1.0f )
#define D3D10_DEFAULT_BLEND_FACTOR_BLUE	( 1.0f )
#define D3D10_DEFAULT_BLEND_FACTOR_GREEN	( 1.0f )
#define D3D10_DEFAULT_BLEND_FACTOR_RED	( 1.0f )
#define D3D10_DEFAULT_BORDER_COLOR_COMPONENT	( 0.0f )
#define	D3D10_DEFAULT_DEPTH_BIAS	( 0 )

#define D3D10_DEFAULT_DEPTH_BIAS_CLAMP	( 0.0f )
#define D3D10_DEFAULT_MAX_ANISOTROPY	( 16.0f )
#define D3D10_DEFAULT_MIP_LOD_BIAS	( 0.0f )
#define	D3D10_DEFAULT_RENDER_TARGET_ARRAY_INDEX	( 0 )

#define	D3D10_DEFAULT_SAMPLE_MASK	( 0xffffffff )

#define	D3D10_DEFAULT_SCISSOR_ENDX	( 0 )

#define	D3D10_DEFAULT_SCISSOR_ENDY	( 0 )

#define	D3D10_DEFAULT_SCISSOR_STARTX	( 0 )

#define	D3D10_DEFAULT_SCISSOR_STARTY	( 0 )

#define D3D10_DEFAULT_SLOPE_SCALED_DEPTH_BIAS	( 0.0f )
#define	D3D10_DEFAULT_STENCIL_READ_MASK	( 0xff )

#define	D3D10_DEFAULT_STENCIL_REFERENCE	( 0 )

#define	D3D10_DEFAULT_STENCIL_WRITE_MASK	( 0xff )

#define	D3D10_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX	( 0 )

#define	D3D10_DEFAULT_VIEWPORT_HEIGHT	( 0 )

#define D3D10_DEFAULT_VIEWPORT_MAX_DEPTH	( 0.0f )
#define D3D10_DEFAULT_VIEWPORT_MIN_DEPTH	( 0.0f )
#define	D3D10_DEFAULT_VIEWPORT_TOPLEFTX	( 0 )

#define	D3D10_DEFAULT_VIEWPORT_TOPLEFTY	( 0 )

#define	D3D10_DEFAULT_VIEWPORT_WIDTH	( 0 )

#define D3D10_FLOAT16_FUSED_TOLERANCE_IN_ULP	( 0.6 )
#define D3D10_FLOAT32_MAX	( 3.402823466e+38f )
#define D3D10_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP	( 0.6f )
#define D3D10_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR	( 2.4f )
#define D3D10_FLOAT_TO_SRGB_EXPONENT_NUMERATOR	( 1.0f )
#define D3D10_FLOAT_TO_SRGB_OFFSET	( 0.055f )
#define D3D10_FLOAT_TO_SRGB_SCALE_1	( 12.92f )
#define D3D10_FLOAT_TO_SRGB_SCALE_2	( 1.055f )
#define D3D10_FLOAT_TO_SRGB_THRESHOLD	( 0.0031308f )
#define D3D10_FTOI_INSTRUCTION_MAX_INPUT	( 2147483647.999f )
#define D3D10_FTOI_INSTRUCTION_MIN_INPUT	( -2147483648.999f )
#define D3D10_FTOU_INSTRUCTION_MAX_INPUT	( 4294967295.999f )
#define D3D10_FTOU_INSTRUCTION_MIN_INPUT	( 0.0f )
#define	D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS	( 1 )

#define	D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_GS_INPUT_PRIM_CONST_REGISTER_COUNT	( 1 )

#define	D3D10_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST	( 2 )

#define	D3D10_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS	( 1 )

#define	D3D10_GS_INPUT_REGISTER_COMPONENTS	( 4 )

#define	D3D10_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_GS_INPUT_REGISTER_COUNT	( 16 )

#define	D3D10_GS_INPUT_REGISTER_READS_PER_INST	( 2 )

#define	D3D10_GS_INPUT_REGISTER_READ_PORTS	( 1 )

#define	D3D10_GS_INPUT_REGISTER_VERTICES	( 6 )

#define	D3D10_GS_OUTPUT_ELEMENTS	( 32 )

#define	D3D10_GS_OUTPUT_REGISTER_COMPONENTS	( 4 )

#define	D3D10_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_GS_OUTPUT_REGISTER_COUNT	( 32 )

#define	D3D10_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES	( 0 )

#define	D3D10_IA_DEFAULT_PRIMITIVE_TOPOLOGY	( 0 )

#define	D3D10_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES	( 0 )

#define	D3D10_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT	( 1 )

#define	D3D10_IA_INSTANCE_ID_BIT_COUNT	( 32 )

#define	D3D10_IA_INTEGER_ARITHMETIC_BIT_COUNT	( 32 )

#define	D3D10_IA_PRIMITIVE_ID_BIT_COUNT	( 32 )

#define	D3D10_IA_VERTEX_ID_BIT_COUNT	( 32 )

#define	D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT	( 16 )

#define	D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS	( 64 )

#define	D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT	( 16 )

#define	D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT	( 0xffffffff )

#define	D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER	( 0xffffffff )

#define D3D10_LINEAR_GAMMA	( 1.0f )
#define D3D10_MAX_BORDER_COLOR_COMPONENT	( 1.0f )
#define D3D10_MAX_DEPTH	( 1.0f )
#define	D3D10_MAX_MAXANISOTROPY	( 16 )

#define	D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT	( 32 )

#define D3D10_MAX_POSITION_VALUE	( 3.402823466e+34f )
#define	D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP	( 17 )

#define D3D10_MIN_BORDER_COLOR_COMPONENT	( 0.0f )
#define D3D10_MIN_DEPTH	( 0.0f )
#define	D3D10_MIN_MAXANISOTROPY	( 0 )

#define D3D10_MIP_LOD_BIAS_MAX	( 15.99f )
#define D3D10_MIP_LOD_BIAS_MIN	( -16.0f )
#define	D3D10_MIP_LOD_FRACTIONAL_BIT_COUNT	( 6 )

#define	D3D10_MIP_LOD_RANGE_BIT_COUNT	( 8 )

#define D3D10_MULTISAMPLE_ANTIALIAS_LINE_WIDTH	( 1.4f )
#define	D3D10_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT	( 0 )

#define	D3D10_PIXEL_ADDRESS_RANGE_BIT_COUNT	( 13 )

#define	D3D10_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT	( 15 )

#define	D3D10_PS_FRONTFACING_DEFAULT_VALUE	( 0xffffffff )

#define	D3D10_PS_FRONTFACING_FALSE_VALUE	( 0 )

#define	D3D10_PS_FRONTFACING_TRUE_VALUE	( 0xffffffff )

#define	D3D10_PS_INPUT_REGISTER_COMPONENTS	( 4 )

#define	D3D10_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_PS_INPUT_REGISTER_COUNT	( 32 )

#define	D3D10_PS_INPUT_REGISTER_READS_PER_INST	( 2 )

#define	D3D10_PS_INPUT_REGISTER_READ_PORTS	( 1 )

#define D3D10_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT	( 0.0f )
#define	D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS	( 1 )

#define	D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_PS_OUTPUT_DEPTH_REGISTER_COUNT	( 1 )

#define	D3D10_PS_OUTPUT_REGISTER_COMPONENTS	( 4 )

#define	D3D10_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_PS_OUTPUT_REGISTER_COUNT	( 8 )

#define D3D10_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT	( 0.5f )
#define	D3D10_REQ_BLEND_OBJECT_COUNT_PER_CONTEXT	( 4096 )

#define	D3D10_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP	( 27 )

#define	D3D10_REQ_CONSTANT_BUFFER_ELEMENT_COUNT	( 4096 )

#define	D3D10_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_CONTEXT	( 4096 )

#define	D3D10_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP	( 32 )

#define	D3D10_REQ_DRAW_VERTEX_COUNT_2_TO_EXP	( 32 )

#define	D3D10_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION	( 8192 )

#define	D3D10_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT	( 1024 )

#define	D3D10_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT	( 4096 )

#define	D3D10_REQ_MAXANISOTROPY	( 16 )

#define	D3D10_REQ_MIP_LEVELS	( 14 )

#define	D3D10_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES	( 2048 )

#define	D3D10_REQ_RASTERIZER_OBJECT_COUNT_PER_CONTEXT	( 4096 )

#define	D3D10_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH	( 8192 )

#define	D3D10_REQ_RESOURCE_SIZE_IN_MEGABYTES	( 128 )

#define	D3D10_REQ_RESOURCE_VIEW_COUNT_PER_CONTEXT_2_TO_EXP	( 20 )

#define	D3D10_REQ_SAMPLER_OBJECT_COUNT_PER_CONTEXT	( 4096 )

#define	D3D10_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION	( 512 )

#define	D3D10_REQ_TEXTURE1D_U_DIMENSION	( 8192 )

#define	D3D10_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION	( 512 )

#define	D3D10_REQ_TEXTURE2D_U_OR_V_DIMENSION	( 8192 )

#define	D3D10_REQ_TEXTURE3D_U_V_OR_W_DIMENSION	( 2048 )

#define	D3D10_REQ_TEXTURECUBE_DIMENSION	( 8192 )

#define	D3D10_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL	( 0 )

#define	D3D10_SHADER_MAJOR_VERSION	( 4 )

#define	D3D10_SHADER_MINOR_VERSION	( 0 )

#define	D3D10_SHIFT_INSTRUCTION_PAD_VALUE	( 0 )

#define	D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT	( 5 )

#define	D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT	( 8 )

#define	D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES	( 2048 )

#define	D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES	( 256 )

#define	D3D10_SO_BUFFER_SLOT_COUNT	( 4 )

#define	D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP	( 0xffffffff )

#define	D3D10_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER	( 1 )

#define	D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT	( 64 )

#define D3D10_SRGB_GAMMA	( 2.2f )
#define D3D10_SRGB_TO_FLOAT_DENOMINATOR_1	( 12.92f )
#define D3D10_SRGB_TO_FLOAT_DENOMINATOR_2	( 1.055f )
#define D3D10_SRGB_TO_FLOAT_EXPONENT	( 2.4f )
#define D3D10_SRGB_TO_FLOAT_OFFSET	( 0.055f )
#define D3D10_SRGB_TO_FLOAT_THRESHOLD	( 0.04045f )
#define D3D10_SRGB_TO_FLOAT_TOLERANCE_IN_ULP	( 0.5f )
#define	D3D10_STANDARD_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_STANDARD_COMPONENT_BIT_COUNT_DOUBLED	( 64 )

#define	D3D10_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE	( 4 )

#define	D3D10_STANDARD_PIXEL_COMPONENT_COUNT	( 128 )

#define	D3D10_STANDARD_PIXEL_ELEMENT_COUNT	( 32 )

#define	D3D10_STANDARD_VECTOR_SIZE	( 4 )

#define	D3D10_STANDARD_VERTEX_ELEMENT_COUNT	( 16 )

#define	D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT	( 64 )

#define	D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT	( 8 )

#define	D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT	( 6 )

#define	D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT	( 18 )

#define	D3D10_UNBOUND_MEMORY_ACCESS_RESULT	( 0 )

#define	D3D10_VIEWPORT_AND_SCISSORRECT_MAX_INDEX	( 15 )

#define	D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE	( 16 )

#define	D3D10_VIEWPORT_BOUNDS_MAX	( 16383 )

#define	D3D10_VIEWPORT_BOUNDS_MIN	( -16384 )

#define	D3D10_VS_INPUT_REGISTER_COMPONENTS	( 4 )

#define	D3D10_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_VS_INPUT_REGISTER_COUNT	( 16 )

#define	D3D10_VS_INPUT_REGISTER_READS_PER_INST	( 2 )

#define	D3D10_VS_INPUT_REGISTER_READ_PORTS	( 1 )

#define	D3D10_VS_OUTPUT_REGISTER_COMPONENTS	( 4 )

#define	D3D10_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT	( 32 )

#define	D3D10_VS_OUTPUT_REGISTER_COUNT	( 16 )

#define	D3D10_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT	( 10 )

#define	D3D10_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP	( 25 )

#define	D3D10_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP	( 25 )

#define	D3D_MAJOR_VERSION	( 10 )

#define	D3D_MINOR_VERSION	( 0 )

#define	D3D_SPEC_DATE_DAY	( 8 )

#define	D3D_SPEC_DATE_MONTH	( 8 )

#define	D3D_SPEC_DATE_YEAR	( 2006 )

#define D3D_SPEC_VERSION	( 1.050005 )
#endif
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if !defined( __d3d10_1_h__ ) && !(D3D10_HEADER_MINOR_VERSION >= 1)
#define D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT
#define D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT
#endif
#define	_FACD3D10	( 0x879 )

#define	_FACD3D10DEBUG	( ( _FACD3D10 + 1 )  )

#define MAKE_D3D10_HRESULT( code )  MAKE_HRESULT( 1, _FACD3D10, code )
#define MAKE_D3D10_STATUS( code )    MAKE_HRESULT( 0, _FACD3D10, code )
/* Direct3D errors are now found in winerror.h */
typedef 
enum D3D10_INPUT_CLASSIFICATION
    {
        D3D10_INPUT_PER_VERTEX_DATA	= 0,
        D3D10_INPUT_PER_INSTANCE_DATA	= 1
    } 	D3D10_INPUT_CLASSIFICATION;

#define	D3D10_APPEND_ALIGNED_ELEMENT	( 0xffffffff )

typedef struct D3D10_INPUT_ELEMENT_DESC
    {
    LPCSTR SemanticName;
    UINT SemanticIndex;
    DXGI_FORMAT Format;
    UINT InputSlot;
    UINT AlignedByteOffset;
    D3D10_INPUT_CLASSIFICATION InputSlotClass;
    UINT InstanceDataStepRate;
    } 	D3D10_INPUT_ELEMENT_DESC;

typedef 
enum D3D10_FILL_MODE
    {
        D3D10_FILL_WIREFRAME	= 2,
        D3D10_FILL_SOLID	= 3
    } 	D3D10_FILL_MODE;

typedef D3D_PRIMITIVE_TOPOLOGY D3D10_PRIMITIVE_TOPOLOGY;

typedef D3D_PRIMITIVE D3D10_PRIMITIVE;

typedef 
enum D3D10_CULL_MODE
    {
        D3D10_CULL_NONE	= 1,
        D3D10_CULL_FRONT	= 2,
        D3D10_CULL_BACK	= 3
    } 	D3D10_CULL_MODE;

typedef struct D3D10_SO_DECLARATION_ENTRY
    {
    LPCSTR SemanticName;
    UINT SemanticIndex;
    BYTE StartComponent;
    BYTE ComponentCount;
    BYTE OutputSlot;
    } 	D3D10_SO_DECLARATION_ENTRY;

typedef struct D3D10_VIEWPORT
    {
    INT TopLeftX;
    INT TopLeftY;
    UINT Width;
    UINT Height;
    FLOAT MinDepth;
    FLOAT MaxDepth;
    } 	D3D10_VIEWPORT;

typedef 
enum D3D10_RESOURCE_DIMENSION
    {
        D3D10_RESOURCE_DIMENSION_UNKNOWN	= 0,
        D3D10_RESOURCE_DIMENSION_BUFFER	= 1,
        D3D10_RESOURCE_DIMENSION_TEXTURE1D	= 2,
        D3D10_RESOURCE_DIMENSION_TEXTURE2D	= 3,
        D3D10_RESOURCE_DIMENSION_TEXTURE3D	= 4
    } 	D3D10_RESOURCE_DIMENSION;

typedef D3D_SRV_DIMENSION D3D10_SRV_DIMENSION;

typedef 
enum D3D10_DSV_DIMENSION
    {
        D3D10_DSV_DIMENSION_UNKNOWN	= 0,
        D3D10_DSV_DIMENSION_TEXTURE1D	= 1,
        D3D10_DSV_DIMENSION_TEXTURE1DARRAY	= 2,
        D3D10_DSV_DIMENSION_TEXTURE2D	= 3,
        D3D10_DSV_DIMENSION_TEXTURE2DARRAY	= 4,
        D3D10_DSV_DIMENSION_TEXTURE2DMS	= 5,
        D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY	= 6
    } 	D3D10_DSV_DIMENSION;

typedef 
enum D3D10_RTV_DIMENSION
    {
        D3D10_RTV_DIMENSION_UNKNOWN	= 0,
        D3D10_RTV_DIMENSION_BUFFER	= 1,
        D3D10_RTV_DIMENSION_TEXTURE1D	= 2,
        D3D10_RTV_DIMENSION_TEXTURE1DARRAY	= 3,
        D3D10_RTV_DIMENSION_TEXTURE2D	= 4,
        D3D10_RTV_DIMENSION_TEXTURE2DARRAY	= 5,
        D3D10_RTV_DIMENSION_TEXTURE2DMS	= 6,
        D3D10_RTV_DIMENSION_TEXTURE2DMSARRAY	= 7,
        D3D10_RTV_DIMENSION_TEXTURE3D	= 8
    } 	D3D10_RTV_DIMENSION;

typedef 
enum D3D10_USAGE
    {
        D3D10_USAGE_DEFAULT	= 0,
        D3D10_USAGE_IMMUTABLE	= 1,
        D3D10_USAGE_DYNAMIC	= 2,
        D3D10_USAGE_STAGING	= 3
    } 	D3D10_USAGE;

typedef 
enum D3D10_BIND_FLAG
    {
        D3D10_BIND_VERTEX_BUFFER	= 0x1L,
        D3D10_BIND_INDEX_BUFFER	= 0x2L,
        D3D10_BIND_CONSTANT_BUFFER	= 0x4L,
        D3D10_BIND_SHADER_RESOURCE	= 0x8L,
        D3D10_BIND_STREAM_OUTPUT	= 0x10L,
        D3D10_BIND_RENDER_TARGET	= 0x20L,
        D3D10_BIND_DEPTH_STENCIL	= 0x40L
    } 	D3D10_BIND_FLAG;

typedef 
enum D3D10_CPU_ACCESS_FLAG
    {
        D3D10_CPU_ACCESS_WRITE	= 0x10000L,
        D3D10_CPU_ACCESS_READ	= 0x20000L
    } 	D3D10_CPU_ACCESS_FLAG;

typedef 
enum D3D10_RESOURCE_MISC_FLAG
    {
        D3D10_RESOURCE_MISC_GENERATE_MIPS	= 0x1L,
        D3D10_RESOURCE_MISC_SHARED	= 0x2L,
        D3D10_RESOURCE_MISC_TEXTURECUBE	= 0x4L,
        D3D10_RESOURCE_MISC_SHARED_KEYEDMUTEX	= 0x10L,
        D3D10_RESOURCE_MISC_GDI_COMPATIBLE	= 0x20L
    } 	D3D10_RESOURCE_MISC_FLAG;

typedef 
enum D3D10_MAP
    {
        D3D10_MAP_READ	= 1,
        D3D10_MAP_WRITE	= 2,
        D3D10_MAP_READ_WRITE	= 3,
        D3D10_MAP_WRITE_DISCARD	= 4,
        D3D10_MAP_WRITE_NO_OVERWRITE	= 5
    } 	D3D10_MAP;

typedef 
enum D3D10_MAP_FLAG
    {
        D3D10_MAP_FLAG_DO_NOT_WAIT	= 0x100000L
    } 	D3D10_MAP_FLAG;

typedef 
enum D3D10_RAISE_FLAG
    {
        D3D10_RAISE_FLAG_DRIVER_INTERNAL_ERROR	= 0x1L
    } 	D3D10_RAISE_FLAG;

typedef 
enum D3D10_CLEAR_FLAG
    {
        D3D10_CLEAR_DEPTH	= 0x1L,
        D3D10_CLEAR_STENCIL	= 0x2L
    } 	D3D10_CLEAR_FLAG;

typedef RECT D3D10_RECT;

typedef struct D3D10_BOX
    {
    UINT left;
    UINT top;
    UINT front;
    UINT right;
    UINT bottom;
    UINT back;
    } 	D3D10_BOX;




extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0000_v0_0_s_ifspec;

#ifndef __ID3D10DeviceChild_INTERFACE_DEFINED__
#define __ID3D10DeviceChild_INTERFACE_DEFINED__

/* interface ID3D10DeviceChild */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10DeviceChild;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C00-342C-4106-A19F-4F2704F689F0")
    ID3D10DeviceChild : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE GetDevice( 
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateData( 
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrivateData( 
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrivateDataInterface( 
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10DeviceChildVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10DeviceChild * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10DeviceChild * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10DeviceChild * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10DeviceChild * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10DeviceChild * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10DeviceChild * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10DeviceChild * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        END_INTERFACE
    } ID3D10DeviceChildVtbl;

    interface ID3D10DeviceChild
    {
        CONST_VTBL struct ID3D10DeviceChildVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10DeviceChild_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10DeviceChild_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10DeviceChild_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10DeviceChild_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10DeviceChild_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10DeviceChild_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10DeviceChild_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10DeviceChild_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0001 */
/* [local] */ 

typedef 
enum D3D10_COMPARISON_FUNC
    {
        D3D10_COMPARISON_NEVER	= 1,
        D3D10_COMPARISON_LESS	= 2,
        D3D10_COMPARISON_EQUAL	= 3,
        D3D10_COMPARISON_LESS_EQUAL	= 4,
        D3D10_COMPARISON_GREATER	= 5,
        D3D10_COMPARISON_NOT_EQUAL	= 6,
        D3D10_COMPARISON_GREATER_EQUAL	= 7,
        D3D10_COMPARISON_ALWAYS	= 8
    } 	D3D10_COMPARISON_FUNC;

typedef 
enum D3D10_DEPTH_WRITE_MASK
    {
        D3D10_DEPTH_WRITE_MASK_ZERO	= 0,
        D3D10_DEPTH_WRITE_MASK_ALL	= 1
    } 	D3D10_DEPTH_WRITE_MASK;

typedef 
enum D3D10_STENCIL_OP
    {
        D3D10_STENCIL_OP_KEEP	= 1,
        D3D10_STENCIL_OP_ZERO	= 2,
        D3D10_STENCIL_OP_REPLACE	= 3,
        D3D10_STENCIL_OP_INCR_SAT	= 4,
        D3D10_STENCIL_OP_DECR_SAT	= 5,
        D3D10_STENCIL_OP_INVERT	= 6,
        D3D10_STENCIL_OP_INCR	= 7,
        D3D10_STENCIL_OP_DECR	= 8
    } 	D3D10_STENCIL_OP;

typedef struct D3D10_DEPTH_STENCILOP_DESC
    {
    D3D10_STENCIL_OP StencilFailOp;
    D3D10_STENCIL_OP StencilDepthFailOp;
    D3D10_STENCIL_OP StencilPassOp;
    D3D10_COMPARISON_FUNC StencilFunc;
    } 	D3D10_DEPTH_STENCILOP_DESC;

typedef struct D3D10_DEPTH_STENCIL_DESC
    {
    BOOL DepthEnable;
    D3D10_DEPTH_WRITE_MASK DepthWriteMask;
    D3D10_COMPARISON_FUNC DepthFunc;
    BOOL StencilEnable;
    UINT8 StencilReadMask;
    UINT8 StencilWriteMask;
    D3D10_DEPTH_STENCILOP_DESC FrontFace;
    D3D10_DEPTH_STENCILOP_DESC BackFace;
    } 	D3D10_DEPTH_STENCIL_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0001_v0_0_s_ifspec;

#ifndef __ID3D10DepthStencilState_INTERFACE_DEFINED__
#define __ID3D10DepthStencilState_INTERFACE_DEFINED__

/* interface ID3D10DepthStencilState */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10DepthStencilState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2B4B1CC8-A4AD-41f8-8322-CA86FC3EC675")
    ID3D10DepthStencilState : public ID3D10DeviceChild
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_DEPTH_STENCIL_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10DepthStencilStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10DepthStencilState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10DepthStencilState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10DepthStencilState * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10DepthStencilState * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10DepthStencilState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10DepthStencilState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10DepthStencilState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DepthStencilState, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10DepthStencilState * This,
            /* [annotation] */ 
            _Out_  D3D10_DEPTH_STENCIL_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10DepthStencilStateVtbl;

    interface ID3D10DepthStencilState
    {
        CONST_VTBL struct ID3D10DepthStencilStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10DepthStencilState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10DepthStencilState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10DepthStencilState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10DepthStencilState_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10DepthStencilState_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10DepthStencilState_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10DepthStencilState_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10DepthStencilState_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10DepthStencilState_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0002 */
/* [local] */ 

typedef 
enum D3D10_BLEND
    {
        D3D10_BLEND_ZERO	= 1,
        D3D10_BLEND_ONE	= 2,
        D3D10_BLEND_SRC_COLOR	= 3,
        D3D10_BLEND_INV_SRC_COLOR	= 4,
        D3D10_BLEND_SRC_ALPHA	= 5,
        D3D10_BLEND_INV_SRC_ALPHA	= 6,
        D3D10_BLEND_DEST_ALPHA	= 7,
        D3D10_BLEND_INV_DEST_ALPHA	= 8,
        D3D10_BLEND_DEST_COLOR	= 9,
        D3D10_BLEND_INV_DEST_COLOR	= 10,
        D3D10_BLEND_SRC_ALPHA_SAT	= 11,
        D3D10_BLEND_BLEND_FACTOR	= 14,
        D3D10_BLEND_INV_BLEND_FACTOR	= 15,
        D3D10_BLEND_SRC1_COLOR	= 16,
        D3D10_BLEND_INV_SRC1_COLOR	= 17,
        D3D10_BLEND_SRC1_ALPHA	= 18,
        D3D10_BLEND_INV_SRC1_ALPHA	= 19
    } 	D3D10_BLEND;

typedef 
enum D3D10_BLEND_OP
    {
        D3D10_BLEND_OP_ADD	= 1,
        D3D10_BLEND_OP_SUBTRACT	= 2,
        D3D10_BLEND_OP_REV_SUBTRACT	= 3,
        D3D10_BLEND_OP_MIN	= 4,
        D3D10_BLEND_OP_MAX	= 5
    } 	D3D10_BLEND_OP;

typedef 
enum D3D10_COLOR_WRITE_ENABLE
    {
        D3D10_COLOR_WRITE_ENABLE_RED	= 1,
        D3D10_COLOR_WRITE_ENABLE_GREEN	= 2,
        D3D10_COLOR_WRITE_ENABLE_BLUE	= 4,
        D3D10_COLOR_WRITE_ENABLE_ALPHA	= 8,
        D3D10_COLOR_WRITE_ENABLE_ALL	= ( ( ( D3D10_COLOR_WRITE_ENABLE_RED | D3D10_COLOR_WRITE_ENABLE_GREEN )  | D3D10_COLOR_WRITE_ENABLE_BLUE )  | D3D10_COLOR_WRITE_ENABLE_ALPHA ) 
    } 	D3D10_COLOR_WRITE_ENABLE;

typedef struct D3D10_BLEND_DESC
    {
    BOOL AlphaToCoverageEnable;
    BOOL BlendEnable[ 8 ];
    D3D10_BLEND SrcBlend;
    D3D10_BLEND DestBlend;
    D3D10_BLEND_OP BlendOp;
    D3D10_BLEND SrcBlendAlpha;
    D3D10_BLEND DestBlendAlpha;
    D3D10_BLEND_OP BlendOpAlpha;
    UINT8 RenderTargetWriteMask[ 8 ];
    } 	D3D10_BLEND_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0002_v0_0_s_ifspec;

#ifndef __ID3D10BlendState_INTERFACE_DEFINED__
#define __ID3D10BlendState_INTERFACE_DEFINED__

/* interface ID3D10BlendState */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10BlendState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EDAD8D19-8A35-4d6d-8566-2EA276CDE161")
    ID3D10BlendState : public ID3D10DeviceChild
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_BLEND_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10BlendStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10BlendState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10BlendState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10BlendState * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10BlendState * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10BlendState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10BlendState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10BlendState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10BlendState, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10BlendState * This,
            /* [annotation] */ 
            _Out_  D3D10_BLEND_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10BlendStateVtbl;

    interface ID3D10BlendState
    {
        CONST_VTBL struct ID3D10BlendStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10BlendState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10BlendState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10BlendState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10BlendState_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10BlendState_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10BlendState_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10BlendState_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10BlendState_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10BlendState_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0003 */
/* [local] */ 

typedef struct D3D10_RASTERIZER_DESC
    {
    D3D10_FILL_MODE FillMode;
    D3D10_CULL_MODE CullMode;
    BOOL FrontCounterClockwise;
    INT DepthBias;
    FLOAT DepthBiasClamp;
    FLOAT SlopeScaledDepthBias;
    BOOL DepthClipEnable;
    BOOL ScissorEnable;
    BOOL MultisampleEnable;
    BOOL AntialiasedLineEnable;
    } 	D3D10_RASTERIZER_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0003_v0_0_s_ifspec;

#ifndef __ID3D10RasterizerState_INTERFACE_DEFINED__
#define __ID3D10RasterizerState_INTERFACE_DEFINED__

/* interface ID3D10RasterizerState */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10RasterizerState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2A07292-89AF-4345-BE2E-C53D9FBB6E9F")
    ID3D10RasterizerState : public ID3D10DeviceChild
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_RASTERIZER_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10RasterizerStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10RasterizerState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10RasterizerState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10RasterizerState * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10RasterizerState * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10RasterizerState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10RasterizerState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10RasterizerState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10RasterizerState, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10RasterizerState * This,
            /* [annotation] */ 
            _Out_  D3D10_RASTERIZER_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10RasterizerStateVtbl;

    interface ID3D10RasterizerState
    {
        CONST_VTBL struct ID3D10RasterizerStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10RasterizerState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10RasterizerState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10RasterizerState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10RasterizerState_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10RasterizerState_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10RasterizerState_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10RasterizerState_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10RasterizerState_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10RasterizerState_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0004 */
/* [local] */ 

#if !defined( D3D10_NO_HELPERS ) && defined( __cplusplus )
inline UINT D3D10CalcSubresource( UINT MipSlice, UINT ArraySlice, UINT MipLevels )
{ return MipSlice + ArraySlice * MipLevels; }
#endif
typedef struct D3D10_SUBRESOURCE_DATA
    {
    const void *pSysMem;
    UINT SysMemPitch;
    UINT SysMemSlicePitch;
    } 	D3D10_SUBRESOURCE_DATA;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0004_v0_0_s_ifspec;

#ifndef __ID3D10Resource_INTERFACE_DEFINED__
#define __ID3D10Resource_INTERFACE_DEFINED__

/* interface ID3D10Resource */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Resource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C01-342C-4106-A19F-4F2704F689F0")
    ID3D10Resource : public ID3D10DeviceChild
    {
    public:
        virtual void STDMETHODCALLTYPE GetType( 
            /* [annotation] */ 
            _Out_  D3D10_RESOURCE_DIMENSION *rType) = 0;
        
        virtual void STDMETHODCALLTYPE SetEvictionPriority( 
            /* [annotation] */ 
            _In_  UINT EvictionPriority) = 0;
        
        virtual UINT STDMETHODCALLTYPE GetEvictionPriority( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10ResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Resource * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Resource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Resource * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Resource * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Resource * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Resource * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Resource * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetType)
        void ( STDMETHODCALLTYPE *GetType )( 
            ID3D10Resource * This,
            /* [annotation] */ 
            _Out_  D3D10_RESOURCE_DIMENSION *rType);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, SetEvictionPriority)
        void ( STDMETHODCALLTYPE *SetEvictionPriority )( 
            ID3D10Resource * This,
            /* [annotation] */ 
            _In_  UINT EvictionPriority);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetEvictionPriority)
        UINT ( STDMETHODCALLTYPE *GetEvictionPriority )( 
            ID3D10Resource * This);
        
        END_INTERFACE
    } ID3D10ResourceVtbl;

    interface ID3D10Resource
    {
        CONST_VTBL struct ID3D10ResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Resource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Resource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Resource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Resource_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Resource_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Resource_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Resource_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Resource_GetType(This,rType)	\
    ( (This)->lpVtbl -> GetType(This,rType) ) 

#define ID3D10Resource_SetEvictionPriority(This,EvictionPriority)	\
    ( (This)->lpVtbl -> SetEvictionPriority(This,EvictionPriority) ) 

#define ID3D10Resource_GetEvictionPriority(This)	\
    ( (This)->lpVtbl -> GetEvictionPriority(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Resource_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0005 */
/* [local] */ 

typedef struct D3D10_BUFFER_DESC
    {
    UINT ByteWidth;
    D3D10_USAGE Usage;
    UINT BindFlags;
    UINT CPUAccessFlags;
    UINT MiscFlags;
    } 	D3D10_BUFFER_DESC;

#if !defined( D3D10_NO_HELPERS ) && defined( __cplusplus )
struct CD3D10_BUFFER_DESC : public D3D10_BUFFER_DESC
{
    CD3D10_BUFFER_DESC() = default;
    explicit CD3D10_BUFFER_DESC( const D3D10_BUFFER_DESC& o ) :
        D3D10_BUFFER_DESC( o )
    {}
    explicit CD3D10_BUFFER_DESC(
        UINT byteWidth,
        UINT bindFlags,
        D3D10_USAGE usage = D3D10_USAGE_DEFAULT,
        UINT cpuaccessFlags = 0,
        UINT miscFlags = 0 )
    {
        ByteWidth = byteWidth;
        Usage = usage;
        BindFlags = bindFlags;
        CPUAccessFlags = cpuaccessFlags ;
        MiscFlags = miscFlags;
    }
    ~CD3D10_BUFFER_DESC() {}
};
#endif


extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0005_v0_0_s_ifspec;

#ifndef __ID3D10Buffer_INTERFACE_DEFINED__
#define __ID3D10Buffer_INTERFACE_DEFINED__

/* interface ID3D10Buffer */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Buffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C02-342C-4106-A19F-4F2704F689F0")
    ID3D10Buffer : public ID3D10Resource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Map( 
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  void **ppData) = 0;
        
        virtual void STDMETHODCALLTYPE Unmap( void) = 0;
        
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_BUFFER_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10BufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Buffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Buffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Buffer * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetType)
        void ( STDMETHODCALLTYPE *GetType )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _Out_  D3D10_RESOURCE_DIMENSION *rType);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, SetEvictionPriority)
        void ( STDMETHODCALLTYPE *SetEvictionPriority )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _In_  UINT EvictionPriority);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetEvictionPriority)
        UINT ( STDMETHODCALLTYPE *GetEvictionPriority )( 
            ID3D10Buffer * This);
        
        DECLSPEC_XFGVIRT(ID3D10Buffer, Map)
        HRESULT ( STDMETHODCALLTYPE *Map )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  void **ppData);
        
        DECLSPEC_XFGVIRT(ID3D10Buffer, Unmap)
        void ( STDMETHODCALLTYPE *Unmap )( 
            ID3D10Buffer * This);
        
        DECLSPEC_XFGVIRT(ID3D10Buffer, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10Buffer * This,
            /* [annotation] */ 
            _Out_  D3D10_BUFFER_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10BufferVtbl;

    interface ID3D10Buffer
    {
        CONST_VTBL struct ID3D10BufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Buffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Buffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Buffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Buffer_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Buffer_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Buffer_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Buffer_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Buffer_GetType(This,rType)	\
    ( (This)->lpVtbl -> GetType(This,rType) ) 

#define ID3D10Buffer_SetEvictionPriority(This,EvictionPriority)	\
    ( (This)->lpVtbl -> SetEvictionPriority(This,EvictionPriority) ) 

#define ID3D10Buffer_GetEvictionPriority(This)	\
    ( (This)->lpVtbl -> GetEvictionPriority(This) ) 


#define ID3D10Buffer_Map(This,MapType,MapFlags,ppData)	\
    ( (This)->lpVtbl -> Map(This,MapType,MapFlags,ppData) ) 

#define ID3D10Buffer_Unmap(This)	\
    ( (This)->lpVtbl -> Unmap(This) ) 

#define ID3D10Buffer_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Buffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0006 */
/* [local] */ 

typedef struct D3D10_TEXTURE1D_DESC
    {
    UINT Width;
    UINT MipLevels;
    UINT ArraySize;
    DXGI_FORMAT Format;
    D3D10_USAGE Usage;
    UINT BindFlags;
    UINT CPUAccessFlags;
    UINT MiscFlags;
    } 	D3D10_TEXTURE1D_DESC;

#if !defined( D3D10_NO_HELPERS ) && defined( __cplusplus )
struct CD3D10_TEXTURE1D_DESC : public D3D10_TEXTURE1D_DESC
{
    CD3D10_TEXTURE1D_DESC() = default;
    explicit CD3D10_TEXTURE1D_DESC( const D3D10_TEXTURE1D_DESC& o ) :
        D3D10_TEXTURE1D_DESC( o )
    {}
    explicit CD3D10_TEXTURE1D_DESC(
        DXGI_FORMAT format,
        UINT width,
        UINT arraySize = 1,
        UINT mipLevels = 0,
        UINT bindFlags = D3D10_BIND_SHADER_RESOURCE,
        D3D10_USAGE usage = D3D10_USAGE_DEFAULT,
        UINT cpuaccessFlags= 0,
        UINT miscFlags = 0 )
    {
        Width = width;
        MipLevels = mipLevels;
        ArraySize = arraySize;
        Format = format;
        Usage = usage;
        BindFlags = bindFlags;
        CPUAccessFlags = cpuaccessFlags;
        MiscFlags = miscFlags;
    }
    ~CD3D10_TEXTURE1D_DESC() {}
};
#endif


extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0006_v0_0_s_ifspec;

#ifndef __ID3D10Texture1D_INTERFACE_DEFINED__
#define __ID3D10Texture1D_INTERFACE_DEFINED__

/* interface ID3D10Texture1D */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Texture1D;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C03-342C-4106-A19F-4F2704F689F0")
    ID3D10Texture1D : public ID3D10Resource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Map( 
            /* [annotation] */ 
            _In_  UINT Subresource,
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  void **ppData) = 0;
        
        virtual void STDMETHODCALLTYPE Unmap( 
            /* [annotation] */ 
            _In_  UINT Subresource) = 0;
        
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_TEXTURE1D_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10Texture1DVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Texture1D * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Texture1D * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Texture1D * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetType)
        void ( STDMETHODCALLTYPE *GetType )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _Out_  D3D10_RESOURCE_DIMENSION *rType);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, SetEvictionPriority)
        void ( STDMETHODCALLTYPE *SetEvictionPriority )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _In_  UINT EvictionPriority);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetEvictionPriority)
        UINT ( STDMETHODCALLTYPE *GetEvictionPriority )( 
            ID3D10Texture1D * This);
        
        DECLSPEC_XFGVIRT(ID3D10Texture1D, Map)
        HRESULT ( STDMETHODCALLTYPE *Map )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _In_  UINT Subresource,
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  void **ppData);
        
        DECLSPEC_XFGVIRT(ID3D10Texture1D, Unmap)
        void ( STDMETHODCALLTYPE *Unmap )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _In_  UINT Subresource);
        
        DECLSPEC_XFGVIRT(ID3D10Texture1D, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10Texture1D * This,
            /* [annotation] */ 
            _Out_  D3D10_TEXTURE1D_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10Texture1DVtbl;

    interface ID3D10Texture1D
    {
        CONST_VTBL struct ID3D10Texture1DVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Texture1D_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Texture1D_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Texture1D_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Texture1D_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Texture1D_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Texture1D_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Texture1D_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Texture1D_GetType(This,rType)	\
    ( (This)->lpVtbl -> GetType(This,rType) ) 

#define ID3D10Texture1D_SetEvictionPriority(This,EvictionPriority)	\
    ( (This)->lpVtbl -> SetEvictionPriority(This,EvictionPriority) ) 

#define ID3D10Texture1D_GetEvictionPriority(This)	\
    ( (This)->lpVtbl -> GetEvictionPriority(This) ) 


#define ID3D10Texture1D_Map(This,Subresource,MapType,MapFlags,ppData)	\
    ( (This)->lpVtbl -> Map(This,Subresource,MapType,MapFlags,ppData) ) 

#define ID3D10Texture1D_Unmap(This,Subresource)	\
    ( (This)->lpVtbl -> Unmap(This,Subresource) ) 

#define ID3D10Texture1D_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Texture1D_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0007 */
/* [local] */ 

typedef struct D3D10_TEXTURE2D_DESC
    {
    UINT Width;
    UINT Height;
    UINT MipLevels;
    UINT ArraySize;
    DXGI_FORMAT Format;
    DXGI_SAMPLE_DESC SampleDesc;
    D3D10_USAGE Usage;
    UINT BindFlags;
    UINT CPUAccessFlags;
    UINT MiscFlags;
    } 	D3D10_TEXTURE2D_DESC;

#if !defined( D3D10_NO_HELPERS ) && defined( __cplusplus )
struct CD3D10_TEXTURE2D_DESC : public D3D10_TEXTURE2D_DESC
{
    CD3D10_TEXTURE2D_DESC() = default;
    explicit CD3D10_TEXTURE2D_DESC( const D3D10_TEXTURE2D_DESC& o ) :
        D3D10_TEXTURE2D_DESC( o )
    {}
    explicit CD3D10_TEXTURE2D_DESC(
        DXGI_FORMAT format,
        UINT width,
        UINT height,
        UINT arraySize = 1,
        UINT mipLevels = 0,
        UINT bindFlags = D3D10_BIND_SHADER_RESOURCE,
        D3D10_USAGE usage = D3D10_USAGE_DEFAULT,
        UINT cpuaccessFlags = 0,
        UINT sampleCount = 1,
        UINT sampleQuality = 0,
        UINT miscFlags = 0 )
    {
        Width = width;
        Height = height;
        MipLevels = mipLevels;
        ArraySize = arraySize;
        Format = format;
        SampleDesc.Count = sampleCount;
        SampleDesc.Quality = sampleQuality;
        Usage = usage;
        BindFlags = bindFlags;
        CPUAccessFlags = cpuaccessFlags;
        MiscFlags = miscFlags;
    }
    ~CD3D10_TEXTURE2D_DESC() {}
};
#endif
typedef struct D3D10_MAPPED_TEXTURE2D
    {
    void *pData;
    UINT RowPitch;
    } 	D3D10_MAPPED_TEXTURE2D;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0007_v0_0_s_ifspec;

#ifndef __ID3D10Texture2D_INTERFACE_DEFINED__
#define __ID3D10Texture2D_INTERFACE_DEFINED__

/* interface ID3D10Texture2D */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Texture2D;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C04-342C-4106-A19F-4F2704F689F0")
    ID3D10Texture2D : public ID3D10Resource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Map( 
            /* [annotation] */ 
            _In_  UINT Subresource,
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  D3D10_MAPPED_TEXTURE2D *pMappedTex2D) = 0;
        
        virtual void STDMETHODCALLTYPE Unmap( 
            /* [annotation] */ 
            _In_  UINT Subresource) = 0;
        
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_TEXTURE2D_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10Texture2DVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Texture2D * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Texture2D * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Texture2D * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetType)
        void ( STDMETHODCALLTYPE *GetType )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _Out_  D3D10_RESOURCE_DIMENSION *rType);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, SetEvictionPriority)
        void ( STDMETHODCALLTYPE *SetEvictionPriority )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _In_  UINT EvictionPriority);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetEvictionPriority)
        UINT ( STDMETHODCALLTYPE *GetEvictionPriority )( 
            ID3D10Texture2D * This);
        
        DECLSPEC_XFGVIRT(ID3D10Texture2D, Map)
        HRESULT ( STDMETHODCALLTYPE *Map )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _In_  UINT Subresource,
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  D3D10_MAPPED_TEXTURE2D *pMappedTex2D);
        
        DECLSPEC_XFGVIRT(ID3D10Texture2D, Unmap)
        void ( STDMETHODCALLTYPE *Unmap )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _In_  UINT Subresource);
        
        DECLSPEC_XFGVIRT(ID3D10Texture2D, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10Texture2D * This,
            /* [annotation] */ 
            _Out_  D3D10_TEXTURE2D_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10Texture2DVtbl;

    interface ID3D10Texture2D
    {
        CONST_VTBL struct ID3D10Texture2DVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Texture2D_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Texture2D_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Texture2D_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Texture2D_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Texture2D_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Texture2D_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Texture2D_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Texture2D_GetType(This,rType)	\
    ( (This)->lpVtbl -> GetType(This,rType) ) 

#define ID3D10Texture2D_SetEvictionPriority(This,EvictionPriority)	\
    ( (This)->lpVtbl -> SetEvictionPriority(This,EvictionPriority) ) 

#define ID3D10Texture2D_GetEvictionPriority(This)	\
    ( (This)->lpVtbl -> GetEvictionPriority(This) ) 


#define ID3D10Texture2D_Map(This,Subresource,MapType,MapFlags,pMappedTex2D)	\
    ( (This)->lpVtbl -> Map(This,Subresource,MapType,MapFlags,pMappedTex2D) ) 

#define ID3D10Texture2D_Unmap(This,Subresource)	\
    ( (This)->lpVtbl -> Unmap(This,Subresource) ) 

#define ID3D10Texture2D_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Texture2D_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0008 */
/* [local] */ 

typedef struct D3D10_TEXTURE3D_DESC
    {
    UINT Width;
    UINT Height;
    UINT Depth;
    UINT MipLevels;
    DXGI_FORMAT Format;
    D3D10_USAGE Usage;
    UINT BindFlags;
    UINT CPUAccessFlags;
    UINT MiscFlags;
    } 	D3D10_TEXTURE3D_DESC;

#if !defined( D3D10_NO_HELPERS ) && defined( __cplusplus )
struct CD3D10_TEXTURE3D_DESC : public D3D10_TEXTURE3D_DESC
{
    CD3D10_TEXTURE3D_DESC() = default;
    explicit CD3D10_TEXTURE3D_DESC( const D3D10_TEXTURE3D_DESC& o ) :
        D3D10_TEXTURE3D_DESC( o )
    {}
    explicit CD3D10_TEXTURE3D_DESC(
        DXGI_FORMAT format,
        UINT width,
        UINT height,
        UINT depth,
        UINT mipLevels = 0,
        UINT bindFlags = D3D10_BIND_SHADER_RESOURCE,
        D3D10_USAGE usage = D3D10_USAGE_DEFAULT,
        UINT cpuaccessFlags = 0,
        UINT miscFlags = 0 )
    {
        Width = width;
        Height = height;
        Depth = depth;
        MipLevels = mipLevels;
        Format = format;
        Usage = usage;
        BindFlags = bindFlags;
        CPUAccessFlags = cpuaccessFlags;
        MiscFlags = miscFlags;
    }
    ~CD3D10_TEXTURE3D_DESC() {}
};
#endif
typedef struct D3D10_MAPPED_TEXTURE3D
    {
    void *pData;
    UINT RowPitch;
    UINT DepthPitch;
    } 	D3D10_MAPPED_TEXTURE3D;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0008_v0_0_s_ifspec;

#ifndef __ID3D10Texture3D_INTERFACE_DEFINED__
#define __ID3D10Texture3D_INTERFACE_DEFINED__

/* interface ID3D10Texture3D */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Texture3D;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C05-342C-4106-A19F-4F2704F689F0")
    ID3D10Texture3D : public ID3D10Resource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Map( 
            /* [annotation] */ 
            _In_  UINT Subresource,
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  D3D10_MAPPED_TEXTURE3D *pMappedTex3D) = 0;
        
        virtual void STDMETHODCALLTYPE Unmap( 
            /* [annotation] */ 
            _In_  UINT Subresource) = 0;
        
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_TEXTURE3D_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10Texture3DVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Texture3D * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Texture3D * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Texture3D * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetType)
        void ( STDMETHODCALLTYPE *GetType )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _Out_  D3D10_RESOURCE_DIMENSION *rType);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, SetEvictionPriority)
        void ( STDMETHODCALLTYPE *SetEvictionPriority )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _In_  UINT EvictionPriority);
        
        DECLSPEC_XFGVIRT(ID3D10Resource, GetEvictionPriority)
        UINT ( STDMETHODCALLTYPE *GetEvictionPriority )( 
            ID3D10Texture3D * This);
        
        DECLSPEC_XFGVIRT(ID3D10Texture3D, Map)
        HRESULT ( STDMETHODCALLTYPE *Map )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _In_  UINT Subresource,
            /* [annotation] */ 
            _In_  D3D10_MAP MapType,
            /* [annotation] */ 
            _In_  UINT MapFlags,
            /* [annotation] */ 
            _Out_  D3D10_MAPPED_TEXTURE3D *pMappedTex3D);
        
        DECLSPEC_XFGVIRT(ID3D10Texture3D, Unmap)
        void ( STDMETHODCALLTYPE *Unmap )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _In_  UINT Subresource);
        
        DECLSPEC_XFGVIRT(ID3D10Texture3D, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10Texture3D * This,
            /* [annotation] */ 
            _Out_  D3D10_TEXTURE3D_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10Texture3DVtbl;

    interface ID3D10Texture3D
    {
        CONST_VTBL struct ID3D10Texture3DVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Texture3D_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Texture3D_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Texture3D_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Texture3D_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Texture3D_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Texture3D_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Texture3D_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Texture3D_GetType(This,rType)	\
    ( (This)->lpVtbl -> GetType(This,rType) ) 

#define ID3D10Texture3D_SetEvictionPriority(This,EvictionPriority)	\
    ( (This)->lpVtbl -> SetEvictionPriority(This,EvictionPriority) ) 

#define ID3D10Texture3D_GetEvictionPriority(This)	\
    ( (This)->lpVtbl -> GetEvictionPriority(This) ) 


#define ID3D10Texture3D_Map(This,Subresource,MapType,MapFlags,pMappedTex3D)	\
    ( (This)->lpVtbl -> Map(This,Subresource,MapType,MapFlags,pMappedTex3D) ) 

#define ID3D10Texture3D_Unmap(This,Subresource)	\
    ( (This)->lpVtbl -> Unmap(This,Subresource) ) 

#define ID3D10Texture3D_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Texture3D_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0009 */
/* [local] */ 

typedef 
enum D3D10_TEXTURECUBE_FACE
    {
        D3D10_TEXTURECUBE_FACE_POSITIVE_X	= 0,
        D3D10_TEXTURECUBE_FACE_NEGATIVE_X	= 1,
        D3D10_TEXTURECUBE_FACE_POSITIVE_Y	= 2,
        D3D10_TEXTURECUBE_FACE_NEGATIVE_Y	= 3,
        D3D10_TEXTURECUBE_FACE_POSITIVE_Z	= 4,
        D3D10_TEXTURECUBE_FACE_NEGATIVE_Z	= 5
    } 	D3D10_TEXTURECUBE_FACE;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0009_v0_0_s_ifspec;

#ifndef __ID3D10View_INTERFACE_DEFINED__
#define __ID3D10View_INTERFACE_DEFINED__

/* interface ID3D10View */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10View;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C902B03F-60A7-49BA-9936-2A3AB37A7E33")
    ID3D10View : public ID3D10DeviceChild
    {
    public:
        virtual void STDMETHODCALLTYPE GetResource( 
            /* [annotation] */ 
            _Out_  ID3D10Resource **ppResource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10ViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10View * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10View * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10View * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10View * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10View * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10View * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10View * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10View, GetResource)
        void ( STDMETHODCALLTYPE *GetResource )( 
            ID3D10View * This,
            /* [annotation] */ 
            _Out_  ID3D10Resource **ppResource);
        
        END_INTERFACE
    } ID3D10ViewVtbl;

    interface ID3D10View
    {
        CONST_VTBL struct ID3D10ViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10View_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10View_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10View_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10View_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10View_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10View_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10View_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10View_GetResource(This,ppResource)	\
    ( (This)->lpVtbl -> GetResource(This,ppResource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10View_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0010 */
/* [local] */ 

typedef struct D3D10_BUFFER_SRV
    {
    union 
        {
        UINT FirstElement;
        UINT ElementOffset;
        } 	;
    union 
        {
        UINT NumElements;
        UINT ElementWidth;
        } 	;
    } 	D3D10_BUFFER_SRV;

typedef struct D3D10_TEX1D_SRV
    {
    UINT MostDetailedMip;
    UINT MipLevels;
    } 	D3D10_TEX1D_SRV;

typedef struct D3D10_TEX1D_ARRAY_SRV
    {
    UINT MostDetailedMip;
    UINT MipLevels;
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX1D_ARRAY_SRV;

typedef struct D3D10_TEX2D_SRV
    {
    UINT MostDetailedMip;
    UINT MipLevels;
    } 	D3D10_TEX2D_SRV;

typedef struct D3D10_TEX2D_ARRAY_SRV
    {
    UINT MostDetailedMip;
    UINT MipLevels;
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX2D_ARRAY_SRV;

typedef struct D3D10_TEX3D_SRV
    {
    UINT MostDetailedMip;
    UINT MipLevels;
    } 	D3D10_TEX3D_SRV;

typedef struct D3D10_TEXCUBE_SRV
    {
    UINT MostDetailedMip;
    UINT MipLevels;
    } 	D3D10_TEXCUBE_SRV;

typedef struct D3D10_TEX2DMS_SRV
    {
    UINT UnusedField_NothingToDefine;
    } 	D3D10_TEX2DMS_SRV;

typedef struct D3D10_TEX2DMS_ARRAY_SRV
    {
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX2DMS_ARRAY_SRV;

typedef struct D3D10_SHADER_RESOURCE_VIEW_DESC
    {
    DXGI_FORMAT Format;
    D3D10_SRV_DIMENSION ViewDimension;
    union 
        {
        D3D10_BUFFER_SRV Buffer;
        D3D10_TEX1D_SRV Texture1D;
        D3D10_TEX1D_ARRAY_SRV Texture1DArray;
        D3D10_TEX2D_SRV Texture2D;
        D3D10_TEX2D_ARRAY_SRV Texture2DArray;
        D3D10_TEX2DMS_SRV Texture2DMS;
        D3D10_TEX2DMS_ARRAY_SRV Texture2DMSArray;
        D3D10_TEX3D_SRV Texture3D;
        D3D10_TEXCUBE_SRV TextureCube;
        } 	;
    } 	D3D10_SHADER_RESOURCE_VIEW_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0010_v0_0_s_ifspec;

#ifndef __ID3D10ShaderResourceView_INTERFACE_DEFINED__
#define __ID3D10ShaderResourceView_INTERFACE_DEFINED__

/* interface ID3D10ShaderResourceView */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10ShaderResourceView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C07-342C-4106-A19F-4F2704F689F0")
    ID3D10ShaderResourceView : public ID3D10View
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_SHADER_RESOURCE_VIEW_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10ShaderResourceViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10ShaderResourceView * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10ShaderResourceView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10ShaderResourceView * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10ShaderResourceView * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10ShaderResourceView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10ShaderResourceView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10ShaderResourceView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10View, GetResource)
        void ( STDMETHODCALLTYPE *GetResource )( 
            ID3D10ShaderResourceView * This,
            /* [annotation] */ 
            _Out_  ID3D10Resource **ppResource);
        
        DECLSPEC_XFGVIRT(ID3D10ShaderResourceView, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10ShaderResourceView * This,
            /* [annotation] */ 
            _Out_  D3D10_SHADER_RESOURCE_VIEW_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10ShaderResourceViewVtbl;

    interface ID3D10ShaderResourceView
    {
        CONST_VTBL struct ID3D10ShaderResourceViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10ShaderResourceView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10ShaderResourceView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10ShaderResourceView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10ShaderResourceView_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10ShaderResourceView_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10ShaderResourceView_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10ShaderResourceView_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10ShaderResourceView_GetResource(This,ppResource)	\
    ( (This)->lpVtbl -> GetResource(This,ppResource) ) 


#define ID3D10ShaderResourceView_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10ShaderResourceView_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0011 */
/* [local] */ 

typedef struct D3D10_BUFFER_RTV
    {
    union 
        {
        UINT FirstElement;
        UINT ElementOffset;
        } 	;
    union 
        {
        UINT NumElements;
        UINT ElementWidth;
        } 	;
    } 	D3D10_BUFFER_RTV;

typedef struct D3D10_TEX1D_RTV
    {
    UINT MipSlice;
    } 	D3D10_TEX1D_RTV;

typedef struct D3D10_TEX1D_ARRAY_RTV
    {
    UINT MipSlice;
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX1D_ARRAY_RTV;

typedef struct D3D10_TEX2D_RTV
    {
    UINT MipSlice;
    } 	D3D10_TEX2D_RTV;

typedef struct D3D10_TEX2DMS_RTV
    {
    UINT UnusedField_NothingToDefine;
    } 	D3D10_TEX2DMS_RTV;

typedef struct D3D10_TEX2D_ARRAY_RTV
    {
    UINT MipSlice;
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX2D_ARRAY_RTV;

typedef struct D3D10_TEX2DMS_ARRAY_RTV
    {
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX2DMS_ARRAY_RTV;

typedef struct D3D10_TEX3D_RTV
    {
    UINT MipSlice;
    UINT FirstWSlice;
    UINT WSize;
    } 	D3D10_TEX3D_RTV;

typedef struct D3D10_RENDER_TARGET_VIEW_DESC
    {
    DXGI_FORMAT Format;
    D3D10_RTV_DIMENSION ViewDimension;
    union 
        {
        D3D10_BUFFER_RTV Buffer;
        D3D10_TEX1D_RTV Texture1D;
        D3D10_TEX1D_ARRAY_RTV Texture1DArray;
        D3D10_TEX2D_RTV Texture2D;
        D3D10_TEX2D_ARRAY_RTV Texture2DArray;
        D3D10_TEX2DMS_RTV Texture2DMS;
        D3D10_TEX2DMS_ARRAY_RTV Texture2DMSArray;
        D3D10_TEX3D_RTV Texture3D;
        } 	;
    } 	D3D10_RENDER_TARGET_VIEW_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0011_v0_0_s_ifspec;

#ifndef __ID3D10RenderTargetView_INTERFACE_DEFINED__
#define __ID3D10RenderTargetView_INTERFACE_DEFINED__

/* interface ID3D10RenderTargetView */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10RenderTargetView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C08-342C-4106-A19F-4F2704F689F0")
    ID3D10RenderTargetView : public ID3D10View
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_RENDER_TARGET_VIEW_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10RenderTargetViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10RenderTargetView * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10RenderTargetView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10RenderTargetView * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10RenderTargetView * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10RenderTargetView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10RenderTargetView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10RenderTargetView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10View, GetResource)
        void ( STDMETHODCALLTYPE *GetResource )( 
            ID3D10RenderTargetView * This,
            /* [annotation] */ 
            _Out_  ID3D10Resource **ppResource);
        
        DECLSPEC_XFGVIRT(ID3D10RenderTargetView, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10RenderTargetView * This,
            /* [annotation] */ 
            _Out_  D3D10_RENDER_TARGET_VIEW_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10RenderTargetViewVtbl;

    interface ID3D10RenderTargetView
    {
        CONST_VTBL struct ID3D10RenderTargetViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10RenderTargetView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10RenderTargetView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10RenderTargetView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10RenderTargetView_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10RenderTargetView_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10RenderTargetView_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10RenderTargetView_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10RenderTargetView_GetResource(This,ppResource)	\
    ( (This)->lpVtbl -> GetResource(This,ppResource) ) 


#define ID3D10RenderTargetView_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10RenderTargetView_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0012 */
/* [local] */ 

typedef struct D3D10_TEX1D_DSV
    {
    UINT MipSlice;
    } 	D3D10_TEX1D_DSV;

typedef struct D3D10_TEX1D_ARRAY_DSV
    {
    UINT MipSlice;
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX1D_ARRAY_DSV;

typedef struct D3D10_TEX2D_DSV
    {
    UINT MipSlice;
    } 	D3D10_TEX2D_DSV;

typedef struct D3D10_TEX2D_ARRAY_DSV
    {
    UINT MipSlice;
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX2D_ARRAY_DSV;

typedef struct D3D10_TEX2DMS_DSV
    {
    UINT UnusedField_NothingToDefine;
    } 	D3D10_TEX2DMS_DSV;

typedef struct D3D10_TEX2DMS_ARRAY_DSV
    {
    UINT FirstArraySlice;
    UINT ArraySize;
    } 	D3D10_TEX2DMS_ARRAY_DSV;

typedef struct D3D10_DEPTH_STENCIL_VIEW_DESC
    {
    DXGI_FORMAT Format;
    D3D10_DSV_DIMENSION ViewDimension;
    union 
        {
        D3D10_TEX1D_DSV Texture1D;
        D3D10_TEX1D_ARRAY_DSV Texture1DArray;
        D3D10_TEX2D_DSV Texture2D;
        D3D10_TEX2D_ARRAY_DSV Texture2DArray;
        D3D10_TEX2DMS_DSV Texture2DMS;
        D3D10_TEX2DMS_ARRAY_DSV Texture2DMSArray;
        } 	;
    } 	D3D10_DEPTH_STENCIL_VIEW_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0012_v0_0_s_ifspec;

#ifndef __ID3D10DepthStencilView_INTERFACE_DEFINED__
#define __ID3D10DepthStencilView_INTERFACE_DEFINED__

/* interface ID3D10DepthStencilView */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10DepthStencilView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C09-342C-4106-A19F-4F2704F689F0")
    ID3D10DepthStencilView : public ID3D10View
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_DEPTH_STENCIL_VIEW_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10DepthStencilViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10DepthStencilView * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10DepthStencilView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10DepthStencilView * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10DepthStencilView * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10DepthStencilView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10DepthStencilView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10DepthStencilView * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10View, GetResource)
        void ( STDMETHODCALLTYPE *GetResource )( 
            ID3D10DepthStencilView * This,
            /* [annotation] */ 
            _Out_  ID3D10Resource **ppResource);
        
        DECLSPEC_XFGVIRT(ID3D10DepthStencilView, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10DepthStencilView * This,
            /* [annotation] */ 
            _Out_  D3D10_DEPTH_STENCIL_VIEW_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10DepthStencilViewVtbl;

    interface ID3D10DepthStencilView
    {
        CONST_VTBL struct ID3D10DepthStencilViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10DepthStencilView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10DepthStencilView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10DepthStencilView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10DepthStencilView_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10DepthStencilView_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10DepthStencilView_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10DepthStencilView_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10DepthStencilView_GetResource(This,ppResource)	\
    ( (This)->lpVtbl -> GetResource(This,ppResource) ) 


#define ID3D10DepthStencilView_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10DepthStencilView_INTERFACE_DEFINED__ */


#ifndef __ID3D10VertexShader_INTERFACE_DEFINED__
#define __ID3D10VertexShader_INTERFACE_DEFINED__

/* interface ID3D10VertexShader */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10VertexShader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C0A-342C-4106-A19F-4F2704F689F0")
    ID3D10VertexShader : public ID3D10DeviceChild
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10VertexShaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10VertexShader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10VertexShader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10VertexShader * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10VertexShader * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10VertexShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10VertexShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10VertexShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        END_INTERFACE
    } ID3D10VertexShaderVtbl;

    interface ID3D10VertexShader
    {
        CONST_VTBL struct ID3D10VertexShaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10VertexShader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10VertexShader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10VertexShader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10VertexShader_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10VertexShader_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10VertexShader_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10VertexShader_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10VertexShader_INTERFACE_DEFINED__ */


#ifndef __ID3D10GeometryShader_INTERFACE_DEFINED__
#define __ID3D10GeometryShader_INTERFACE_DEFINED__

/* interface ID3D10GeometryShader */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10GeometryShader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6316BE88-54CD-4040-AB44-20461BC81F68")
    ID3D10GeometryShader : public ID3D10DeviceChild
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10GeometryShaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10GeometryShader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10GeometryShader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10GeometryShader * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10GeometryShader * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10GeometryShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10GeometryShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10GeometryShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        END_INTERFACE
    } ID3D10GeometryShaderVtbl;

    interface ID3D10GeometryShader
    {
        CONST_VTBL struct ID3D10GeometryShaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10GeometryShader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10GeometryShader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10GeometryShader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10GeometryShader_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10GeometryShader_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10GeometryShader_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10GeometryShader_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10GeometryShader_INTERFACE_DEFINED__ */


#ifndef __ID3D10PixelShader_INTERFACE_DEFINED__
#define __ID3D10PixelShader_INTERFACE_DEFINED__

/* interface ID3D10PixelShader */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10PixelShader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4968B601-9D00-4cde-8346-8E7F675819B6")
    ID3D10PixelShader : public ID3D10DeviceChild
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10PixelShaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10PixelShader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10PixelShader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10PixelShader * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10PixelShader * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10PixelShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10PixelShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10PixelShader * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        END_INTERFACE
    } ID3D10PixelShaderVtbl;

    interface ID3D10PixelShader
    {
        CONST_VTBL struct ID3D10PixelShaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10PixelShader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10PixelShader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10PixelShader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10PixelShader_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10PixelShader_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10PixelShader_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10PixelShader_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10PixelShader_INTERFACE_DEFINED__ */


#ifndef __ID3D10InputLayout_INTERFACE_DEFINED__
#define __ID3D10InputLayout_INTERFACE_DEFINED__

/* interface ID3D10InputLayout */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10InputLayout;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C0B-342C-4106-A19F-4F2704F689F0")
    ID3D10InputLayout : public ID3D10DeviceChild
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10InputLayoutVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10InputLayout * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10InputLayout * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10InputLayout * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10InputLayout * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10InputLayout * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10InputLayout * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10InputLayout * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        END_INTERFACE
    } ID3D10InputLayoutVtbl;

    interface ID3D10InputLayout
    {
        CONST_VTBL struct ID3D10InputLayoutVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10InputLayout_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10InputLayout_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10InputLayout_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10InputLayout_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10InputLayout_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10InputLayout_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10InputLayout_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10InputLayout_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0017 */
/* [local] */ 

typedef 
enum D3D10_FILTER
    {
        D3D10_FILTER_MIN_MAG_MIP_POINT	= 0,
        D3D10_FILTER_MIN_MAG_POINT_MIP_LINEAR	= 0x1,
        D3D10_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT	= 0x4,
        D3D10_FILTER_MIN_POINT_MAG_MIP_LINEAR	= 0x5,
        D3D10_FILTER_MIN_LINEAR_MAG_MIP_POINT	= 0x10,
        D3D10_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR	= 0x11,
        D3D10_FILTER_MIN_MAG_LINEAR_MIP_POINT	= 0x14,
        D3D10_FILTER_MIN_MAG_MIP_LINEAR	= 0x15,
        D3D10_FILTER_ANISOTROPIC	= 0x55,
        D3D10_FILTER_COMPARISON_MIN_MAG_MIP_POINT	= 0x80,
        D3D10_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR	= 0x81,
        D3D10_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT	= 0x84,
        D3D10_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR	= 0x85,
        D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT	= 0x90,
        D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR	= 0x91,
        D3D10_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT	= 0x94,
        D3D10_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR	= 0x95,
        D3D10_FILTER_COMPARISON_ANISOTROPIC	= 0xd5,
        D3D10_FILTER_TEXT_1BIT	= 0x80000000
    } 	D3D10_FILTER;

typedef 
enum D3D10_FILTER_TYPE
    {
        D3D10_FILTER_TYPE_POINT	= 0,
        D3D10_FILTER_TYPE_LINEAR	= 1
    } 	D3D10_FILTER_TYPE;

#define	D3D10_FILTER_TYPE_MASK	( 0x3 )

#define	D3D10_MIN_FILTER_SHIFT	( 4 )

#define	D3D10_MAG_FILTER_SHIFT	( 2 )

#define	D3D10_MIP_FILTER_SHIFT	( 0 )

#define	D3D10_COMPARISON_FILTERING_BIT	( 0x80 )

#define	D3D10_ANISOTROPIC_FILTERING_BIT	( 0x40 )

#define	D3D10_TEXT_1BIT_BIT	( 0x80000000 )

#define D3D10_ENCODE_BASIC_FILTER( min, mag, mip, bComparison )                                           \
                                   ( ( D3D10_FILTER ) (                                                   \
                                   ( ( bComparison ) ? D3D10_COMPARISON_FILTERING_BIT : 0 ) |             \
                                   ( ( ( min ) & D3D10_FILTER_TYPE_MASK ) << D3D10_MIN_FILTER_SHIFT ) |   \
                                   ( ( ( mag ) & D3D10_FILTER_TYPE_MASK ) << D3D10_MAG_FILTER_SHIFT ) |   \
                                   ( ( ( mip ) & D3D10_FILTER_TYPE_MASK ) << D3D10_MIP_FILTER_SHIFT ) ) )   
#define D3D10_ENCODE_ANISOTROPIC_FILTER( bComparison )                                                \
                                         ( ( D3D10_FILTER ) (                                         \
                                         D3D10_ANISOTROPIC_FILTERING_BIT |                            \
                                         D3D10_ENCODE_BASIC_FILTER( D3D10_FILTER_TYPE_LINEAR,         \
                                                                    D3D10_FILTER_TYPE_LINEAR,         \
                                                                    D3D10_FILTER_TYPE_LINEAR,         \
                                                                    bComparison ) ) )                   
#define D3D10_DECODE_MIN_FILTER( d3d10Filter )                                                              \
                                 ( ( D3D10_FILTER_TYPE )                                                    \
                                 ( ( ( d3d10Filter ) >> D3D10_MIN_FILTER_SHIFT ) & D3D10_FILTER_TYPE_MASK ) ) 
#define D3D10_DECODE_MAG_FILTER( d3d10Filter )                                                              \
                                 ( ( D3D10_FILTER_TYPE )                                                    \
                                 ( ( ( d3d10Filter ) >> D3D10_MAG_FILTER_SHIFT ) & D3D10_FILTER_TYPE_MASK ) ) 
#define D3D10_DECODE_MIP_FILTER( d3d10Filter )                                                              \
                                 ( ( D3D10_FILTER_TYPE )                                                    \
                                 ( ( ( d3d10Filter ) >> D3D10_MIP_FILTER_SHIFT ) & D3D10_FILTER_TYPE_MASK ) ) 
#define D3D10_DECODE_IS_COMPARISON_FILTER( d3d10Filter )                                                    \
                                 ( ( d3d10Filter ) & D3D10_COMPARISON_FILTERING_BIT )                         
#define D3D10_DECODE_IS_ANISOTROPIC_FILTER( d3d10Filter )                                               \
                            ( ( ( d3d10Filter ) & D3D10_ANISOTROPIC_FILTERING_BIT ) &&                  \
                            ( D3D10_FILTER_TYPE_LINEAR == D3D10_DECODE_MIN_FILTER( d3d10Filter ) ) &&   \
                            ( D3D10_FILTER_TYPE_LINEAR == D3D10_DECODE_MAG_FILTER( d3d10Filter ) ) &&   \
                            ( D3D10_FILTER_TYPE_LINEAR == D3D10_DECODE_MIP_FILTER( d3d10Filter ) ) )      
#define D3D10_DECODE_IS_TEXT_1BIT_FILTER( d3d10Filter )                                             \
                                 ( ( d3d10Filter ) == D3D10_TEXT_1BIT_BIT )                           
typedef 
enum D3D10_TEXTURE_ADDRESS_MODE
    {
        D3D10_TEXTURE_ADDRESS_WRAP	= 1,
        D3D10_TEXTURE_ADDRESS_MIRROR	= 2,
        D3D10_TEXTURE_ADDRESS_CLAMP	= 3,
        D3D10_TEXTURE_ADDRESS_BORDER	= 4,
        D3D10_TEXTURE_ADDRESS_MIRROR_ONCE	= 5
    } 	D3D10_TEXTURE_ADDRESS_MODE;

typedef struct D3D10_SAMPLER_DESC
    {
    D3D10_FILTER Filter;
    D3D10_TEXTURE_ADDRESS_MODE AddressU;
    D3D10_TEXTURE_ADDRESS_MODE AddressV;
    D3D10_TEXTURE_ADDRESS_MODE AddressW;
    FLOAT MipLODBias;
    UINT MaxAnisotropy;
    D3D10_COMPARISON_FUNC ComparisonFunc;
    FLOAT BorderColor[ 4 ];
    FLOAT MinLOD;
    FLOAT MaxLOD;
    } 	D3D10_SAMPLER_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0017_v0_0_s_ifspec;

#ifndef __ID3D10SamplerState_INTERFACE_DEFINED__
#define __ID3D10SamplerState_INTERFACE_DEFINED__

/* interface ID3D10SamplerState */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10SamplerState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C0C-342C-4106-A19F-4F2704F689F0")
    ID3D10SamplerState : public ID3D10DeviceChild
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_SAMPLER_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10SamplerStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10SamplerState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10SamplerState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10SamplerState * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10SamplerState * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10SamplerState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10SamplerState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10SamplerState * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10SamplerState, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10SamplerState * This,
            /* [annotation] */ 
            _Out_  D3D10_SAMPLER_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10SamplerStateVtbl;

    interface ID3D10SamplerState
    {
        CONST_VTBL struct ID3D10SamplerStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10SamplerState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10SamplerState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10SamplerState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10SamplerState_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10SamplerState_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10SamplerState_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10SamplerState_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10SamplerState_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10SamplerState_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0018 */
/* [local] */ 

typedef 
enum D3D10_FORMAT_SUPPORT
    {
        D3D10_FORMAT_SUPPORT_BUFFER	= 0x1,
        D3D10_FORMAT_SUPPORT_IA_VERTEX_BUFFER	= 0x2,
        D3D10_FORMAT_SUPPORT_IA_INDEX_BUFFER	= 0x4,
        D3D10_FORMAT_SUPPORT_SO_BUFFER	= 0x8,
        D3D10_FORMAT_SUPPORT_TEXTURE1D	= 0x10,
        D3D10_FORMAT_SUPPORT_TEXTURE2D	= 0x20,
        D3D10_FORMAT_SUPPORT_TEXTURE3D	= 0x40,
        D3D10_FORMAT_SUPPORT_TEXTURECUBE	= 0x80,
        D3D10_FORMAT_SUPPORT_SHADER_LOAD	= 0x100,
        D3D10_FORMAT_SUPPORT_SHADER_SAMPLE	= 0x200,
        D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON	= 0x400,
        D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT	= 0x800,
        D3D10_FORMAT_SUPPORT_MIP	= 0x1000,
        D3D10_FORMAT_SUPPORT_MIP_AUTOGEN	= 0x2000,
        D3D10_FORMAT_SUPPORT_RENDER_TARGET	= 0x4000,
        D3D10_FORMAT_SUPPORT_BLENDABLE	= 0x8000,
        D3D10_FORMAT_SUPPORT_DEPTH_STENCIL	= 0x10000,
        D3D10_FORMAT_SUPPORT_CPU_LOCKABLE	= 0x20000,
        D3D10_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE	= 0x40000,
        D3D10_FORMAT_SUPPORT_DISPLAY	= 0x80000,
        D3D10_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT	= 0x100000,
        D3D10_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET	= 0x200000,
        D3D10_FORMAT_SUPPORT_MULTISAMPLE_LOAD	= 0x400000,
        D3D10_FORMAT_SUPPORT_SHADER_GATHER	= 0x800000,
        D3D10_FORMAT_SUPPORT_BACK_BUFFER_CAST	= 0x1000000
    } 	D3D10_FORMAT_SUPPORT;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0018_v0_0_s_ifspec;

#ifndef __ID3D10Asynchronous_INTERFACE_DEFINED__
#define __ID3D10Asynchronous_INTERFACE_DEFINED__

/* interface ID3D10Asynchronous */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Asynchronous;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C0D-342C-4106-A19F-4F2704F689F0")
    ID3D10Asynchronous : public ID3D10DeviceChild
    {
    public:
        virtual void STDMETHODCALLTYPE Begin( void) = 0;
        
        virtual void STDMETHODCALLTYPE End( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation] */ 
            _Out_writes_bytes_opt_(DataSize)  void *pData,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_  UINT GetDataFlags) = 0;
        
        virtual UINT STDMETHODCALLTYPE GetDataSize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10AsynchronousVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Asynchronous * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Asynchronous * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Asynchronous * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Asynchronous * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Asynchronous * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Asynchronous * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Asynchronous * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, Begin)
        void ( STDMETHODCALLTYPE *Begin )( 
            ID3D10Asynchronous * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, End)
        void ( STDMETHODCALLTYPE *End )( 
            ID3D10Asynchronous * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            ID3D10Asynchronous * This,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(DataSize)  void *pData,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_  UINT GetDataFlags);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetDataSize)
        UINT ( STDMETHODCALLTYPE *GetDataSize )( 
            ID3D10Asynchronous * This);
        
        END_INTERFACE
    } ID3D10AsynchronousVtbl;

    interface ID3D10Asynchronous
    {
        CONST_VTBL struct ID3D10AsynchronousVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Asynchronous_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Asynchronous_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Asynchronous_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Asynchronous_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Asynchronous_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Asynchronous_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Asynchronous_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Asynchronous_Begin(This)	\
    ( (This)->lpVtbl -> Begin(This) ) 

#define ID3D10Asynchronous_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#define ID3D10Asynchronous_GetData(This,pData,DataSize,GetDataFlags)	\
    ( (This)->lpVtbl -> GetData(This,pData,DataSize,GetDataFlags) ) 

#define ID3D10Asynchronous_GetDataSize(This)	\
    ( (This)->lpVtbl -> GetDataSize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Asynchronous_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0019 */
/* [local] */ 

typedef 
enum D3D10_ASYNC_GETDATA_FLAG
    {
        D3D10_ASYNC_GETDATA_DONOTFLUSH	= 0x1
    } 	D3D10_ASYNC_GETDATA_FLAG;

typedef 
enum D3D10_QUERY
    {
        D3D10_QUERY_EVENT	= 0,
        D3D10_QUERY_OCCLUSION	= ( D3D10_QUERY_EVENT + 1 ) ,
        D3D10_QUERY_TIMESTAMP	= ( D3D10_QUERY_OCCLUSION + 1 ) ,
        D3D10_QUERY_TIMESTAMP_DISJOINT	= ( D3D10_QUERY_TIMESTAMP + 1 ) ,
        D3D10_QUERY_PIPELINE_STATISTICS	= ( D3D10_QUERY_TIMESTAMP_DISJOINT + 1 ) ,
        D3D10_QUERY_OCCLUSION_PREDICATE	= ( D3D10_QUERY_PIPELINE_STATISTICS + 1 ) ,
        D3D10_QUERY_SO_STATISTICS	= ( D3D10_QUERY_OCCLUSION_PREDICATE + 1 ) ,
        D3D10_QUERY_SO_OVERFLOW_PREDICATE	= ( D3D10_QUERY_SO_STATISTICS + 1 ) 
    } 	D3D10_QUERY;

typedef 
enum D3D10_QUERY_MISC_FLAG
    {
        D3D10_QUERY_MISC_PREDICATEHINT	= 0x1
    } 	D3D10_QUERY_MISC_FLAG;

typedef struct D3D10_QUERY_DESC
    {
    D3D10_QUERY Query;
    UINT MiscFlags;
    } 	D3D10_QUERY_DESC;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0019_v0_0_s_ifspec;

#ifndef __ID3D10Query_INTERFACE_DEFINED__
#define __ID3D10Query_INTERFACE_DEFINED__

/* interface ID3D10Query */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Query;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C0E-342C-4106-A19F-4F2704F689F0")
    ID3D10Query : public ID3D10Asynchronous
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_QUERY_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10QueryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Query * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Query * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Query * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Query * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Query * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Query * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Query * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, Begin)
        void ( STDMETHODCALLTYPE *Begin )( 
            ID3D10Query * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, End)
        void ( STDMETHODCALLTYPE *End )( 
            ID3D10Query * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            ID3D10Query * This,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(DataSize)  void *pData,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_  UINT GetDataFlags);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetDataSize)
        UINT ( STDMETHODCALLTYPE *GetDataSize )( 
            ID3D10Query * This);
        
        DECLSPEC_XFGVIRT(ID3D10Query, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10Query * This,
            /* [annotation] */ 
            _Out_  D3D10_QUERY_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10QueryVtbl;

    interface ID3D10Query
    {
        CONST_VTBL struct ID3D10QueryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Query_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Query_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Query_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Query_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Query_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Query_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Query_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Query_Begin(This)	\
    ( (This)->lpVtbl -> Begin(This) ) 

#define ID3D10Query_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#define ID3D10Query_GetData(This,pData,DataSize,GetDataFlags)	\
    ( (This)->lpVtbl -> GetData(This,pData,DataSize,GetDataFlags) ) 

#define ID3D10Query_GetDataSize(This)	\
    ( (This)->lpVtbl -> GetDataSize(This) ) 


#define ID3D10Query_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Query_INTERFACE_DEFINED__ */


#ifndef __ID3D10Predicate_INTERFACE_DEFINED__
#define __ID3D10Predicate_INTERFACE_DEFINED__

/* interface ID3D10Predicate */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Predicate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C10-342C-4106-A19F-4F2704F689F0")
    ID3D10Predicate : public ID3D10Query
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10PredicateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Predicate * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Predicate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Predicate * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Predicate * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Predicate * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Predicate * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Predicate * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, Begin)
        void ( STDMETHODCALLTYPE *Begin )( 
            ID3D10Predicate * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, End)
        void ( STDMETHODCALLTYPE *End )( 
            ID3D10Predicate * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            ID3D10Predicate * This,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(DataSize)  void *pData,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_  UINT GetDataFlags);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetDataSize)
        UINT ( STDMETHODCALLTYPE *GetDataSize )( 
            ID3D10Predicate * This);
        
        DECLSPEC_XFGVIRT(ID3D10Query, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10Predicate * This,
            /* [annotation] */ 
            _Out_  D3D10_QUERY_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10PredicateVtbl;

    interface ID3D10Predicate
    {
        CONST_VTBL struct ID3D10PredicateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Predicate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Predicate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Predicate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Predicate_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Predicate_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Predicate_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Predicate_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Predicate_Begin(This)	\
    ( (This)->lpVtbl -> Begin(This) ) 

#define ID3D10Predicate_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#define ID3D10Predicate_GetData(This,pData,DataSize,GetDataFlags)	\
    ( (This)->lpVtbl -> GetData(This,pData,DataSize,GetDataFlags) ) 

#define ID3D10Predicate_GetDataSize(This)	\
    ( (This)->lpVtbl -> GetDataSize(This) ) 


#define ID3D10Predicate_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Predicate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0021 */
/* [local] */ 

typedef struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT
    {
    UINT64 Frequency;
    BOOL Disjoint;
    } 	D3D10_QUERY_DATA_TIMESTAMP_DISJOINT;

typedef struct D3D10_QUERY_DATA_PIPELINE_STATISTICS
    {
    UINT64 IAVertices;
    UINT64 IAPrimitives;
    UINT64 VSInvocations;
    UINT64 GSInvocations;
    UINT64 GSPrimitives;
    UINT64 CInvocations;
    UINT64 CPrimitives;
    UINT64 PSInvocations;
    } 	D3D10_QUERY_DATA_PIPELINE_STATISTICS;

typedef struct D3D10_QUERY_DATA_SO_STATISTICS
    {
    UINT64 NumPrimitivesWritten;
    UINT64 PrimitivesStorageNeeded;
    } 	D3D10_QUERY_DATA_SO_STATISTICS;

typedef 
enum D3D10_COUNTER
    {
        D3D10_COUNTER_GPU_IDLE	= 0,
        D3D10_COUNTER_VERTEX_PROCESSING	= ( D3D10_COUNTER_GPU_IDLE + 1 ) ,
        D3D10_COUNTER_GEOMETRY_PROCESSING	= ( D3D10_COUNTER_VERTEX_PROCESSING + 1 ) ,
        D3D10_COUNTER_PIXEL_PROCESSING	= ( D3D10_COUNTER_GEOMETRY_PROCESSING + 1 ) ,
        D3D10_COUNTER_OTHER_GPU_PROCESSING	= ( D3D10_COUNTER_PIXEL_PROCESSING + 1 ) ,
        D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION	= ( D3D10_COUNTER_OTHER_GPU_PROCESSING + 1 ) ,
        D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION	= ( D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION + 1 ) ,
        D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION	= ( D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION + 1 ) ,
        D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION	= ( D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION + 1 ) ,
        D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION	= ( D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION + 1 ) ,
        D3D10_COUNTER_VS_MEMORY_LIMITED	= ( D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION + 1 ) ,
        D3D10_COUNTER_VS_COMPUTATION_LIMITED	= ( D3D10_COUNTER_VS_MEMORY_LIMITED + 1 ) ,
        D3D10_COUNTER_GS_MEMORY_LIMITED	= ( D3D10_COUNTER_VS_COMPUTATION_LIMITED + 1 ) ,
        D3D10_COUNTER_GS_COMPUTATION_LIMITED	= ( D3D10_COUNTER_GS_MEMORY_LIMITED + 1 ) ,
        D3D10_COUNTER_PS_MEMORY_LIMITED	= ( D3D10_COUNTER_GS_COMPUTATION_LIMITED + 1 ) ,
        D3D10_COUNTER_PS_COMPUTATION_LIMITED	= ( D3D10_COUNTER_PS_MEMORY_LIMITED + 1 ) ,
        D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE	= ( D3D10_COUNTER_PS_COMPUTATION_LIMITED + 1 ) ,
        D3D10_COUNTER_TEXTURE_CACHE_HIT_RATE	= ( D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE + 1 ) ,
        D3D10_COUNTER_DEVICE_DEPENDENT_0	= 0x40000000
    } 	D3D10_COUNTER;

typedef 
enum D3D10_COUNTER_TYPE
    {
        D3D10_COUNTER_TYPE_FLOAT32	= 0,
        D3D10_COUNTER_TYPE_UINT16	= ( D3D10_COUNTER_TYPE_FLOAT32 + 1 ) ,
        D3D10_COUNTER_TYPE_UINT32	= ( D3D10_COUNTER_TYPE_UINT16 + 1 ) ,
        D3D10_COUNTER_TYPE_UINT64	= ( D3D10_COUNTER_TYPE_UINT32 + 1 ) 
    } 	D3D10_COUNTER_TYPE;

typedef struct D3D10_COUNTER_DESC
    {
    D3D10_COUNTER Counter;
    UINT MiscFlags;
    } 	D3D10_COUNTER_DESC;

typedef struct D3D10_COUNTER_INFO
    {
    D3D10_COUNTER LastDeviceDependentCounter;
    UINT NumSimultaneousCounters;
    UINT8 NumDetectableParallelUnits;
    } 	D3D10_COUNTER_INFO;



extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0021_v0_0_s_ifspec;

#ifndef __ID3D10Counter_INTERFACE_DEFINED__
#define __ID3D10Counter_INTERFACE_DEFINED__

/* interface ID3D10Counter */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Counter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C11-342C-4106-A19F-4F2704F689F0")
    ID3D10Counter : public ID3D10Asynchronous
    {
    public:
        virtual void STDMETHODCALLTYPE GetDesc( 
            /* [annotation] */ 
            _Out_  D3D10_COUNTER_DESC *pDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10CounterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Counter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Counter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Counter * This);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetDevice)
        void ( STDMETHODCALLTYPE *GetDevice )( 
            ID3D10Counter * This,
            /* [annotation] */ 
            _Out_  ID3D10Device **ppDevice);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Counter * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Counter * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10DeviceChild, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Counter * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, Begin)
        void ( STDMETHODCALLTYPE *Begin )( 
            ID3D10Counter * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, End)
        void ( STDMETHODCALLTYPE *End )( 
            ID3D10Counter * This);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            ID3D10Counter * This,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(DataSize)  void *pData,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_  UINT GetDataFlags);
        
        DECLSPEC_XFGVIRT(ID3D10Asynchronous, GetDataSize)
        UINT ( STDMETHODCALLTYPE *GetDataSize )( 
            ID3D10Counter * This);
        
        DECLSPEC_XFGVIRT(ID3D10Counter, GetDesc)
        void ( STDMETHODCALLTYPE *GetDesc )( 
            ID3D10Counter * This,
            /* [annotation] */ 
            _Out_  D3D10_COUNTER_DESC *pDesc);
        
        END_INTERFACE
    } ID3D10CounterVtbl;

    interface ID3D10Counter
    {
        CONST_VTBL struct ID3D10CounterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Counter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Counter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Counter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Counter_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define ID3D10Counter_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Counter_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Counter_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 


#define ID3D10Counter_Begin(This)	\
    ( (This)->lpVtbl -> Begin(This) ) 

#define ID3D10Counter_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#define ID3D10Counter_GetData(This,pData,DataSize,GetDataFlags)	\
    ( (This)->lpVtbl -> GetData(This,pData,DataSize,GetDataFlags) ) 

#define ID3D10Counter_GetDataSize(This)	\
    ( (This)->lpVtbl -> GetDataSize(This) ) 


#define ID3D10Counter_GetDesc(This,pDesc)	\
    ( (This)->lpVtbl -> GetDesc(This,pDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Counter_INTERFACE_DEFINED__ */


#ifndef __ID3D10Device_INTERFACE_DEFINED__
#define __ID3D10Device_INTERFACE_DEFINED__

/* interface ID3D10Device */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Device;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4C0F-342C-4106-A19F-4F2704F689F0")
    ID3D10Device : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE VSSetConstantBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppConstantBuffers) = 0;
        
        virtual void STDMETHODCALLTYPE PSSetShaderResources( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10ShaderResourceView *const *ppShaderResourceViews) = 0;
        
        virtual void STDMETHODCALLTYPE PSSetShader( 
            /* [annotation] */ 
            _In_opt_  ID3D10PixelShader *pPixelShader) = 0;
        
        virtual void STDMETHODCALLTYPE PSSetSamplers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _In_reads_opt_(NumSamplers)  ID3D10SamplerState *const *ppSamplers) = 0;
        
        virtual void STDMETHODCALLTYPE VSSetShader( 
            /* [annotation] */ 
            _In_opt_  ID3D10VertexShader *pVertexShader) = 0;
        
        virtual void STDMETHODCALLTYPE DrawIndexed( 
            /* [annotation] */ 
            _In_  UINT IndexCount,
            /* [annotation] */ 
            _In_  UINT StartIndexLocation,
            /* [annotation] */ 
            _In_  INT BaseVertexLocation) = 0;
        
        virtual void STDMETHODCALLTYPE Draw( 
            /* [annotation] */ 
            _In_  UINT VertexCount,
            /* [annotation] */ 
            _In_  UINT StartVertexLocation) = 0;
        
        virtual void STDMETHODCALLTYPE PSSetConstantBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppConstantBuffers) = 0;
        
        virtual void STDMETHODCALLTYPE IASetInputLayout( 
            /* [annotation] */ 
            _In_opt_  ID3D10InputLayout *pInputLayout) = 0;
        
        virtual void STDMETHODCALLTYPE IASetVertexBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppVertexBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  const UINT *pStrides,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  const UINT *pOffsets) = 0;
        
        virtual void STDMETHODCALLTYPE IASetIndexBuffer( 
            /* [annotation] */ 
            _In_opt_  ID3D10Buffer *pIndexBuffer,
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format,
            /* [annotation] */ 
            _In_  UINT Offset) = 0;
        
        virtual void STDMETHODCALLTYPE DrawIndexedInstanced( 
            /* [annotation] */ 
            _In_  UINT IndexCountPerInstance,
            /* [annotation] */ 
            _In_  UINT InstanceCount,
            /* [annotation] */ 
            _In_  UINT StartIndexLocation,
            /* [annotation] */ 
            _In_  INT BaseVertexLocation,
            /* [annotation] */ 
            _In_  UINT StartInstanceLocation) = 0;
        
        virtual void STDMETHODCALLTYPE DrawInstanced( 
            /* [annotation] */ 
            _In_  UINT VertexCountPerInstance,
            /* [annotation] */ 
            _In_  UINT InstanceCount,
            /* [annotation] */ 
            _In_  UINT StartVertexLocation,
            /* [annotation] */ 
            _In_  UINT StartInstanceLocation) = 0;
        
        virtual void STDMETHODCALLTYPE GSSetConstantBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppConstantBuffers) = 0;
        
        virtual void STDMETHODCALLTYPE GSSetShader( 
            /* [annotation] */ 
            _In_opt_  ID3D10GeometryShader *pShader) = 0;
        
        virtual void STDMETHODCALLTYPE IASetPrimitiveTopology( 
            /* [annotation] */ 
            _In_  D3D10_PRIMITIVE_TOPOLOGY Topology) = 0;
        
        virtual void STDMETHODCALLTYPE VSSetShaderResources( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10ShaderResourceView *const *ppShaderResourceViews) = 0;
        
        virtual void STDMETHODCALLTYPE VSSetSamplers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _In_reads_opt_(NumSamplers)  ID3D10SamplerState *const *ppSamplers) = 0;
        
        virtual void STDMETHODCALLTYPE SetPredication( 
            /* [annotation] */ 
            _In_opt_  ID3D10Predicate *pPredicate,
            /* [annotation] */ 
            _In_  BOOL PredicateValue) = 0;
        
        virtual void STDMETHODCALLTYPE GSSetShaderResources( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10ShaderResourceView *const *ppShaderResourceViews) = 0;
        
        virtual void STDMETHODCALLTYPE GSSetSamplers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _In_reads_opt_(NumSamplers)  ID3D10SamplerState *const *ppSamplers) = 0;
        
        virtual void STDMETHODCALLTYPE OMSetRenderTargets( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10RenderTargetView *const *ppRenderTargetViews,
            /* [annotation] */ 
            _In_opt_  ID3D10DepthStencilView *pDepthStencilView) = 0;
        
        virtual void STDMETHODCALLTYPE OMSetBlendState( 
            /* [annotation] */ 
            _In_opt_  ID3D10BlendState *pBlendState,
            /* [annotation] */ 
            _In_  const FLOAT BlendFactor[ 4 ],
            /* [annotation] */ 
            _In_  UINT SampleMask) = 0;
        
        virtual void STDMETHODCALLTYPE OMSetDepthStencilState( 
            /* [annotation] */ 
            _In_opt_  ID3D10DepthStencilState *pDepthStencilState,
            /* [annotation] */ 
            _In_  UINT StencilRef) = 0;
        
        virtual void STDMETHODCALLTYPE SOSetTargets( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_SO_BUFFER_SLOT_COUNT)  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppSOTargets,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  const UINT *pOffsets) = 0;
        
        virtual void STDMETHODCALLTYPE DrawAuto( void) = 0;
        
        virtual void STDMETHODCALLTYPE RSSetState( 
            /* [annotation] */ 
            _In_opt_  ID3D10RasterizerState *pRasterizerState) = 0;
        
        virtual void STDMETHODCALLTYPE RSSetViewports( 
            /* [annotation] */ 
            _In_range_(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE)  UINT NumViewports,
            /* [annotation] */ 
            _In_reads_opt_(NumViewports)  const D3D10_VIEWPORT *pViewports) = 0;
        
        virtual void STDMETHODCALLTYPE RSSetScissorRects( 
            /* [annotation] */ 
            _In_range_(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE)  UINT NumRects,
            /* [annotation] */ 
            _In_reads_opt_(NumRects)  const D3D10_RECT *pRects) = 0;
        
        virtual void STDMETHODCALLTYPE CopySubresourceRegion( 
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  UINT DstSubresource,
            /* [annotation] */ 
            _In_  UINT DstX,
            /* [annotation] */ 
            _In_  UINT DstY,
            /* [annotation] */ 
            _In_  UINT DstZ,
            /* [annotation] */ 
            _In_  ID3D10Resource *pSrcResource,
            /* [annotation] */ 
            _In_  UINT SrcSubresource,
            /* [annotation] */ 
            _In_opt_  const D3D10_BOX *pSrcBox) = 0;
        
        virtual void STDMETHODCALLTYPE CopyResource( 
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  ID3D10Resource *pSrcResource) = 0;
        
        virtual void STDMETHODCALLTYPE UpdateSubresource( 
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  UINT DstSubresource,
            /* [annotation] */ 
            _In_opt_  const D3D10_BOX *pDstBox,
            /* [annotation] */ 
            _In_  const void *pSrcData,
            /* [annotation] */ 
            _In_  UINT SrcRowPitch,
            /* [annotation] */ 
            _In_  UINT SrcDepthPitch) = 0;
        
        virtual void STDMETHODCALLTYPE ClearRenderTargetView( 
            /* [annotation] */ 
            _In_  ID3D10RenderTargetView *pRenderTargetView,
            /* [annotation] */ 
            _In_  const FLOAT ColorRGBA[ 4 ]) = 0;
        
        virtual void STDMETHODCALLTYPE ClearDepthStencilView( 
            /* [annotation] */ 
            _In_  ID3D10DepthStencilView *pDepthStencilView,
            /* [annotation] */ 
            _In_  UINT ClearFlags,
            /* [annotation] */ 
            _In_  FLOAT Depth,
            /* [annotation] */ 
            _In_  UINT8 Stencil) = 0;
        
        virtual void STDMETHODCALLTYPE GenerateMips( 
            /* [annotation] */ 
            _In_  ID3D10ShaderResourceView *pShaderResourceView) = 0;
        
        virtual void STDMETHODCALLTYPE ResolveSubresource( 
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  UINT DstSubresource,
            /* [annotation] */ 
            _In_  ID3D10Resource *pSrcResource,
            /* [annotation] */ 
            _In_  UINT SrcSubresource,
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format) = 0;
        
        virtual void STDMETHODCALLTYPE VSGetConstantBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppConstantBuffers) = 0;
        
        virtual void STDMETHODCALLTYPE PSGetShaderResources( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10ShaderResourceView **ppShaderResourceViews) = 0;
        
        virtual void STDMETHODCALLTYPE PSGetShader( 
            /* [annotation] */ 
            _Out_  ID3D10PixelShader **ppPixelShader) = 0;
        
        virtual void STDMETHODCALLTYPE PSGetSamplers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _Out_writes_opt_(NumSamplers)  ID3D10SamplerState **ppSamplers) = 0;
        
        virtual void STDMETHODCALLTYPE VSGetShader( 
            /* [annotation] */ 
            _Out_  ID3D10VertexShader **ppVertexShader) = 0;
        
        virtual void STDMETHODCALLTYPE PSGetConstantBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppConstantBuffers) = 0;
        
        virtual void STDMETHODCALLTYPE IAGetInputLayout( 
            /* [annotation] */ 
            _Out_  ID3D10InputLayout **ppInputLayout) = 0;
        
        virtual void STDMETHODCALLTYPE IAGetVertexBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppVertexBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  UINT *pStrides,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  UINT *pOffsets) = 0;
        
        virtual void STDMETHODCALLTYPE IAGetIndexBuffer( 
            /* [annotation] */ 
            _Out_opt_  ID3D10Buffer **pIndexBuffer,
            /* [annotation] */ 
            _Out_opt_  DXGI_FORMAT *Format,
            /* [annotation] */ 
            _Out_opt_  UINT *Offset) = 0;
        
        virtual void STDMETHODCALLTYPE GSGetConstantBuffers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppConstantBuffers) = 0;
        
        virtual void STDMETHODCALLTYPE GSGetShader( 
            /* [annotation] */ 
            _Out_  ID3D10GeometryShader **ppGeometryShader) = 0;
        
        virtual void STDMETHODCALLTYPE IAGetPrimitiveTopology( 
            /* [annotation] */ 
            _Out_  D3D10_PRIMITIVE_TOPOLOGY *pTopology) = 0;
        
        virtual void STDMETHODCALLTYPE VSGetShaderResources( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10ShaderResourceView **ppShaderResourceViews) = 0;
        
        virtual void STDMETHODCALLTYPE VSGetSamplers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _Out_writes_opt_(NumSamplers)  ID3D10SamplerState **ppSamplers) = 0;
        
        virtual void STDMETHODCALLTYPE GetPredication( 
            /* [annotation] */ 
            _Out_opt_  ID3D10Predicate **ppPredicate,
            /* [annotation] */ 
            _Out_opt_  BOOL *pPredicateValue) = 0;
        
        virtual void STDMETHODCALLTYPE GSGetShaderResources( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10ShaderResourceView **ppShaderResourceViews) = 0;
        
        virtual void STDMETHODCALLTYPE GSGetSamplers( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _Out_writes_opt_(NumSamplers)  ID3D10SamplerState **ppSamplers) = 0;
        
        virtual void STDMETHODCALLTYPE OMGetRenderTargets( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10RenderTargetView **ppRenderTargetViews,
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilView **ppDepthStencilView) = 0;
        
        virtual void STDMETHODCALLTYPE OMGetBlendState( 
            /* [annotation] */ 
            _Out_opt_  ID3D10BlendState **ppBlendState,
            /* [annotation] */ 
            _Out_opt_  FLOAT BlendFactor[ 4 ],
            /* [annotation] */ 
            _Out_opt_  UINT *pSampleMask) = 0;
        
        virtual void STDMETHODCALLTYPE OMGetDepthStencilState( 
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilState **ppDepthStencilState,
            /* [annotation] */ 
            _Out_opt_  UINT *pStencilRef) = 0;
        
        virtual void STDMETHODCALLTYPE SOGetTargets( 
            /* [annotation] */ 
            _In_range_( 0, D3D10_SO_BUFFER_SLOT_COUNT )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppSOTargets,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  UINT *pOffsets) = 0;
        
        virtual void STDMETHODCALLTYPE RSGetState( 
            /* [annotation] */ 
            _Out_  ID3D10RasterizerState **ppRasterizerState) = 0;
        
        virtual void STDMETHODCALLTYPE RSGetViewports( 
            /* [annotation] */ 
            _Inout_ /*_range(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE )*/   UINT *NumViewports,
            /* [annotation] */ 
            _Out_writes_opt_(*NumViewports)  D3D10_VIEWPORT *pViewports) = 0;
        
        virtual void STDMETHODCALLTYPE RSGetScissorRects( 
            /* [annotation] */ 
            _Inout_ /*_range(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE )*/   UINT *NumRects,
            /* [annotation] */ 
            _Out_writes_opt_(*NumRects)  D3D10_RECT *pRects) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceRemovedReason( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetExceptionMode( 
            UINT RaiseFlags) = 0;
        
        virtual UINT STDMETHODCALLTYPE GetExceptionMode( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateData( 
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrivateData( 
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrivateDataInterface( 
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData) = 0;
        
        virtual void STDMETHODCALLTYPE ClearState( void) = 0;
        
        virtual void STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBuffer( 
            /* [annotation] */ 
            _In_  const D3D10_BUFFER_DESC *pDesc,
            /* [annotation] */ 
            _In_opt_  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_opt_  ID3D10Buffer **ppBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTexture1D( 
            /* [annotation] */ 
            _In_  const D3D10_TEXTURE1D_DESC *pDesc,
            /* [annotation] */ 
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels * pDesc->ArraySize))  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_  ID3D10Texture1D **ppTexture1D) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTexture2D( 
            /* [annotation] */ 
            _In_  const D3D10_TEXTURE2D_DESC *pDesc,
            /* [annotation] */ 
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels * pDesc->ArraySize))  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_  ID3D10Texture2D **ppTexture2D) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTexture3D( 
            /* [annotation] */ 
            _In_  const D3D10_TEXTURE3D_DESC *pDesc,
            /* [annotation] */ 
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels))  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_  ID3D10Texture3D **ppTexture3D) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateShaderResourceView( 
            /* [annotation] */ 
            _In_  ID3D10Resource *pResource,
            /* [annotation] */ 
            _In_opt_  const D3D10_SHADER_RESOURCE_VIEW_DESC *pDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10ShaderResourceView **ppSRView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRenderTargetView( 
            /* [annotation] */ 
            _In_  ID3D10Resource *pResource,
            /* [annotation] */ 
            _In_opt_  const D3D10_RENDER_TARGET_VIEW_DESC *pDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10RenderTargetView **ppRTView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDepthStencilView( 
            /* [annotation] */ 
            _In_  ID3D10Resource *pResource,
            /* [annotation] */ 
            _In_opt_  const D3D10_DEPTH_STENCIL_VIEW_DESC *pDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilView **ppDepthStencilView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInputLayout( 
            /* [annotation] */ 
            _In_reads_(NumElements)  const D3D10_INPUT_ELEMENT_DESC *pInputElementDescs,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT )  UINT NumElements,
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecodeWithInputSignature,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10InputLayout **ppInputLayout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateVertexShader( 
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10VertexShader **ppVertexShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateGeometryShader( 
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10GeometryShader **ppGeometryShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateGeometryShaderWithStreamOutput( 
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _In_reads_opt_(NumEntries)  const D3D10_SO_DECLARATION_ENTRY *pSODeclaration,
            /* [annotation] */ 
            _In_range_( 0, D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT )  UINT NumEntries,
            /* [annotation] */ 
            _In_  UINT OutputStreamStride,
            /* [annotation] */ 
            _Out_opt_  ID3D10GeometryShader **ppGeometryShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePixelShader( 
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10PixelShader **ppPixelShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBlendState( 
            /* [annotation] */ 
            _In_  const D3D10_BLEND_DESC *pBlendStateDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10BlendState **ppBlendState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDepthStencilState( 
            /* [annotation] */ 
            _In_  const D3D10_DEPTH_STENCIL_DESC *pDepthStencilDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilState **ppDepthStencilState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRasterizerState( 
            /* [annotation] */ 
            _In_  const D3D10_RASTERIZER_DESC *pRasterizerDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10RasterizerState **ppRasterizerState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSamplerState( 
            /* [annotation] */ 
            _In_  const D3D10_SAMPLER_DESC *pSamplerDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10SamplerState **ppSamplerState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateQuery( 
            /* [annotation] */ 
            _In_  const D3D10_QUERY_DESC *pQueryDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10Query **ppQuery) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePredicate( 
            /* [annotation] */ 
            _In_  const D3D10_QUERY_DESC *pPredicateDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10Predicate **ppPredicate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateCounter( 
            /* [annotation] */ 
            _In_  const D3D10_COUNTER_DESC *pCounterDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10Counter **ppCounter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckFormatSupport( 
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format,
            /* [annotation] */ 
            _Out_  UINT *pFormatSupport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckMultisampleQualityLevels( 
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format,
            /* [annotation] */ 
            _In_  UINT SampleCount,
            /* [annotation] */ 
            _Out_  UINT *pNumQualityLevels) = 0;
        
        virtual void STDMETHODCALLTYPE CheckCounterInfo( 
            /* [annotation] */ 
            _Out_  D3D10_COUNTER_INFO *pCounterInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckCounter( 
            /* [annotation] */ 
            _In_  const D3D10_COUNTER_DESC *pDesc,
            /* [annotation] */ 
            _Out_  D3D10_COUNTER_TYPE *pType,
            /* [annotation] */ 
            _Out_  UINT *pActiveCounters,
            /* [annotation] */ 
            _Out_writes_opt_(*pNameLength)  LPSTR szName,
            /* [annotation] */ 
            _Inout_opt_  UINT *pNameLength,
            /* [annotation] */ 
            _Out_writes_opt_(*pUnitsLength)  LPSTR szUnits,
            /* [annotation] */ 
            _Inout_opt_  UINT *pUnitsLength,
            /* [annotation] */ 
            _Out_writes_opt_(*pDescriptionLength)  LPSTR szDescription,
            /* [annotation] */ 
            _Inout_opt_  UINT *pDescriptionLength) = 0;
        
        virtual UINT STDMETHODCALLTYPE GetCreationFlags( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenSharedResource( 
            /* [annotation] */ 
            _In_  HANDLE hResource,
            /* [annotation] */ 
            _In_  REFIID ReturnedInterface,
            /* [annotation] */ 
            _Out_opt_  void **ppResource) = 0;
        
        virtual void STDMETHODCALLTYPE SetTextFilterSize( 
            /* [annotation] */ 
            _In_  UINT Width,
            /* [annotation] */ 
            _In_  UINT Height) = 0;
        
        virtual void STDMETHODCALLTYPE GetTextFilterSize( 
            /* [annotation] */ 
            _Out_opt_  UINT *pWidth,
            /* [annotation] */ 
            _Out_opt_  UINT *pHeight) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10DeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Device * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSSetConstantBuffers)
        void ( STDMETHODCALLTYPE *VSSetConstantBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppConstantBuffers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSSetShaderResources)
        void ( STDMETHODCALLTYPE *PSSetShaderResources )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10ShaderResourceView *const *ppShaderResourceViews);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSSetShader)
        void ( STDMETHODCALLTYPE *PSSetShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10PixelShader *pPixelShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSSetSamplers)
        void ( STDMETHODCALLTYPE *PSSetSamplers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _In_reads_opt_(NumSamplers)  ID3D10SamplerState *const *ppSamplers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSSetShader)
        void ( STDMETHODCALLTYPE *VSSetShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10VertexShader *pVertexShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, DrawIndexed)
        void ( STDMETHODCALLTYPE *DrawIndexed )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  UINT IndexCount,
            /* [annotation] */ 
            _In_  UINT StartIndexLocation,
            /* [annotation] */ 
            _In_  INT BaseVertexLocation);
        
        DECLSPEC_XFGVIRT(ID3D10Device, Draw)
        void ( STDMETHODCALLTYPE *Draw )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  UINT VertexCount,
            /* [annotation] */ 
            _In_  UINT StartVertexLocation);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSSetConstantBuffers)
        void ( STDMETHODCALLTYPE *PSSetConstantBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppConstantBuffers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IASetInputLayout)
        void ( STDMETHODCALLTYPE *IASetInputLayout )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10InputLayout *pInputLayout);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IASetVertexBuffers)
        void ( STDMETHODCALLTYPE *IASetVertexBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppVertexBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  const UINT *pStrides,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  const UINT *pOffsets);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IASetIndexBuffer)
        void ( STDMETHODCALLTYPE *IASetIndexBuffer )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10Buffer *pIndexBuffer,
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format,
            /* [annotation] */ 
            _In_  UINT Offset);
        
        DECLSPEC_XFGVIRT(ID3D10Device, DrawIndexedInstanced)
        void ( STDMETHODCALLTYPE *DrawIndexedInstanced )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  UINT IndexCountPerInstance,
            /* [annotation] */ 
            _In_  UINT InstanceCount,
            /* [annotation] */ 
            _In_  UINT StartIndexLocation,
            /* [annotation] */ 
            _In_  INT BaseVertexLocation,
            /* [annotation] */ 
            _In_  UINT StartInstanceLocation);
        
        DECLSPEC_XFGVIRT(ID3D10Device, DrawInstanced)
        void ( STDMETHODCALLTYPE *DrawInstanced )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  UINT VertexCountPerInstance,
            /* [annotation] */ 
            _In_  UINT InstanceCount,
            /* [annotation] */ 
            _In_  UINT StartVertexLocation,
            /* [annotation] */ 
            _In_  UINT StartInstanceLocation);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSSetConstantBuffers)
        void ( STDMETHODCALLTYPE *GSSetConstantBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppConstantBuffers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSSetShader)
        void ( STDMETHODCALLTYPE *GSSetShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10GeometryShader *pShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IASetPrimitiveTopology)
        void ( STDMETHODCALLTYPE *IASetPrimitiveTopology )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  D3D10_PRIMITIVE_TOPOLOGY Topology);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSSetShaderResources)
        void ( STDMETHODCALLTYPE *VSSetShaderResources )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10ShaderResourceView *const *ppShaderResourceViews);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSSetSamplers)
        void ( STDMETHODCALLTYPE *VSSetSamplers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _In_reads_opt_(NumSamplers)  ID3D10SamplerState *const *ppSamplers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, SetPredication)
        void ( STDMETHODCALLTYPE *SetPredication )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10Predicate *pPredicate,
            /* [annotation] */ 
            _In_  BOOL PredicateValue);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSSetShaderResources)
        void ( STDMETHODCALLTYPE *GSSetShaderResources )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10ShaderResourceView *const *ppShaderResourceViews);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSSetSamplers)
        void ( STDMETHODCALLTYPE *GSSetSamplers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _In_reads_opt_(NumSamplers)  ID3D10SamplerState *const *ppSamplers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, OMSetRenderTargets)
        void ( STDMETHODCALLTYPE *OMSetRenderTargets )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT )  UINT NumViews,
            /* [annotation] */ 
            _In_reads_opt_(NumViews)  ID3D10RenderTargetView *const *ppRenderTargetViews,
            /* [annotation] */ 
            _In_opt_  ID3D10DepthStencilView *pDepthStencilView);
        
        DECLSPEC_XFGVIRT(ID3D10Device, OMSetBlendState)
        void ( STDMETHODCALLTYPE *OMSetBlendState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10BlendState *pBlendState,
            /* [annotation] */ 
            _In_  const FLOAT BlendFactor[ 4 ],
            /* [annotation] */ 
            _In_  UINT SampleMask);
        
        DECLSPEC_XFGVIRT(ID3D10Device, OMSetDepthStencilState)
        void ( STDMETHODCALLTYPE *OMSetDepthStencilState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10DepthStencilState *pDepthStencilState,
            /* [annotation] */ 
            _In_  UINT StencilRef);
        
        DECLSPEC_XFGVIRT(ID3D10Device, SOSetTargets)
        void ( STDMETHODCALLTYPE *SOSetTargets )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_SO_BUFFER_SLOT_COUNT)  UINT NumBuffers,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  ID3D10Buffer *const *ppSOTargets,
            /* [annotation] */ 
            _In_reads_opt_(NumBuffers)  const UINT *pOffsets);
        
        DECLSPEC_XFGVIRT(ID3D10Device, DrawAuto)
        void ( STDMETHODCALLTYPE *DrawAuto )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(ID3D10Device, RSSetState)
        void ( STDMETHODCALLTYPE *RSSetState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_opt_  ID3D10RasterizerState *pRasterizerState);
        
        DECLSPEC_XFGVIRT(ID3D10Device, RSSetViewports)
        void ( STDMETHODCALLTYPE *RSSetViewports )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE)  UINT NumViewports,
            /* [annotation] */ 
            _In_reads_opt_(NumViewports)  const D3D10_VIEWPORT *pViewports);
        
        DECLSPEC_XFGVIRT(ID3D10Device, RSSetScissorRects)
        void ( STDMETHODCALLTYPE *RSSetScissorRects )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE)  UINT NumRects,
            /* [annotation] */ 
            _In_reads_opt_(NumRects)  const D3D10_RECT *pRects);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CopySubresourceRegion)
        void ( STDMETHODCALLTYPE *CopySubresourceRegion )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  UINT DstSubresource,
            /* [annotation] */ 
            _In_  UINT DstX,
            /* [annotation] */ 
            _In_  UINT DstY,
            /* [annotation] */ 
            _In_  UINT DstZ,
            /* [annotation] */ 
            _In_  ID3D10Resource *pSrcResource,
            /* [annotation] */ 
            _In_  UINT SrcSubresource,
            /* [annotation] */ 
            _In_opt_  const D3D10_BOX *pSrcBox);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CopyResource)
        void ( STDMETHODCALLTYPE *CopyResource )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  ID3D10Resource *pSrcResource);
        
        DECLSPEC_XFGVIRT(ID3D10Device, UpdateSubresource)
        void ( STDMETHODCALLTYPE *UpdateSubresource )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  UINT DstSubresource,
            /* [annotation] */ 
            _In_opt_  const D3D10_BOX *pDstBox,
            /* [annotation] */ 
            _In_  const void *pSrcData,
            /* [annotation] */ 
            _In_  UINT SrcRowPitch,
            /* [annotation] */ 
            _In_  UINT SrcDepthPitch);
        
        DECLSPEC_XFGVIRT(ID3D10Device, ClearRenderTargetView)
        void ( STDMETHODCALLTYPE *ClearRenderTargetView )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10RenderTargetView *pRenderTargetView,
            /* [annotation] */ 
            _In_  const FLOAT ColorRGBA[ 4 ]);
        
        DECLSPEC_XFGVIRT(ID3D10Device, ClearDepthStencilView)
        void ( STDMETHODCALLTYPE *ClearDepthStencilView )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10DepthStencilView *pDepthStencilView,
            /* [annotation] */ 
            _In_  UINT ClearFlags,
            /* [annotation] */ 
            _In_  FLOAT Depth,
            /* [annotation] */ 
            _In_  UINT8 Stencil);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GenerateMips)
        void ( STDMETHODCALLTYPE *GenerateMips )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10ShaderResourceView *pShaderResourceView);
        
        DECLSPEC_XFGVIRT(ID3D10Device, ResolveSubresource)
        void ( STDMETHODCALLTYPE *ResolveSubresource )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10Resource *pDstResource,
            /* [annotation] */ 
            _In_  UINT DstSubresource,
            /* [annotation] */ 
            _In_  ID3D10Resource *pSrcResource,
            /* [annotation] */ 
            _In_  UINT SrcSubresource,
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSGetConstantBuffers)
        void ( STDMETHODCALLTYPE *VSGetConstantBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppConstantBuffers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSGetShaderResources)
        void ( STDMETHODCALLTYPE *PSGetShaderResources )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10ShaderResourceView **ppShaderResourceViews);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSGetShader)
        void ( STDMETHODCALLTYPE *PSGetShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_  ID3D10PixelShader **ppPixelShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSGetSamplers)
        void ( STDMETHODCALLTYPE *PSGetSamplers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _Out_writes_opt_(NumSamplers)  ID3D10SamplerState **ppSamplers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSGetShader)
        void ( STDMETHODCALLTYPE *VSGetShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_  ID3D10VertexShader **ppVertexShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, PSGetConstantBuffers)
        void ( STDMETHODCALLTYPE *PSGetConstantBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppConstantBuffers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IAGetInputLayout)
        void ( STDMETHODCALLTYPE *IAGetInputLayout )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_  ID3D10InputLayout **ppInputLayout);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IAGetVertexBuffers)
        void ( STDMETHODCALLTYPE *IAGetVertexBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppVertexBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  UINT *pStrides,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  UINT *pOffsets);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IAGetIndexBuffer)
        void ( STDMETHODCALLTYPE *IAGetIndexBuffer )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_opt_  ID3D10Buffer **pIndexBuffer,
            /* [annotation] */ 
            _Out_opt_  DXGI_FORMAT *Format,
            /* [annotation] */ 
            _Out_opt_  UINT *Offset);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSGetConstantBuffers)
        void ( STDMETHODCALLTYPE *GSGetConstantBuffers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppConstantBuffers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSGetShader)
        void ( STDMETHODCALLTYPE *GSGetShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_  ID3D10GeometryShader **ppGeometryShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, IAGetPrimitiveTopology)
        void ( STDMETHODCALLTYPE *IAGetPrimitiveTopology )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_  D3D10_PRIMITIVE_TOPOLOGY *pTopology);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSGetShaderResources)
        void ( STDMETHODCALLTYPE *VSGetShaderResources )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10ShaderResourceView **ppShaderResourceViews);
        
        DECLSPEC_XFGVIRT(ID3D10Device, VSGetSamplers)
        void ( STDMETHODCALLTYPE *VSGetSamplers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _Out_writes_opt_(NumSamplers)  ID3D10SamplerState **ppSamplers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GetPredication)
        void ( STDMETHODCALLTYPE *GetPredication )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_opt_  ID3D10Predicate **ppPredicate,
            /* [annotation] */ 
            _Out_opt_  BOOL *pPredicateValue);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSGetShaderResources)
        void ( STDMETHODCALLTYPE *GSGetShaderResources )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10ShaderResourceView **ppShaderResourceViews);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GSGetSamplers)
        void ( STDMETHODCALLTYPE *GSGetSamplers )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            /* [annotation] */ 
            _In_range_( 0, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            /* [annotation] */ 
            _Out_writes_opt_(NumSamplers)  ID3D10SamplerState **ppSamplers);
        
        DECLSPEC_XFGVIRT(ID3D10Device, OMGetRenderTargets)
        void ( STDMETHODCALLTYPE *OMGetRenderTargets )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT )  UINT NumViews,
            /* [annotation] */ 
            _Out_writes_opt_(NumViews)  ID3D10RenderTargetView **ppRenderTargetViews,
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilView **ppDepthStencilView);
        
        DECLSPEC_XFGVIRT(ID3D10Device, OMGetBlendState)
        void ( STDMETHODCALLTYPE *OMGetBlendState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_opt_  ID3D10BlendState **ppBlendState,
            /* [annotation] */ 
            _Out_opt_  FLOAT BlendFactor[ 4 ],
            /* [annotation] */ 
            _Out_opt_  UINT *pSampleMask);
        
        DECLSPEC_XFGVIRT(ID3D10Device, OMGetDepthStencilState)
        void ( STDMETHODCALLTYPE *OMGetDepthStencilState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilState **ppDepthStencilState,
            /* [annotation] */ 
            _Out_opt_  UINT *pStencilRef);
        
        DECLSPEC_XFGVIRT(ID3D10Device, SOGetTargets)
        void ( STDMETHODCALLTYPE *SOGetTargets )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_range_( 0, D3D10_SO_BUFFER_SLOT_COUNT )  UINT NumBuffers,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  ID3D10Buffer **ppSOTargets,
            /* [annotation] */ 
            _Out_writes_opt_(NumBuffers)  UINT *pOffsets);
        
        DECLSPEC_XFGVIRT(ID3D10Device, RSGetState)
        void ( STDMETHODCALLTYPE *RSGetState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_  ID3D10RasterizerState **ppRasterizerState);
        
        DECLSPEC_XFGVIRT(ID3D10Device, RSGetViewports)
        void ( STDMETHODCALLTYPE *RSGetViewports )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Inout_ /*_range(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE )*/   UINT *NumViewports,
            /* [annotation] */ 
            _Out_writes_opt_(*NumViewports)  D3D10_VIEWPORT *pViewports);
        
        DECLSPEC_XFGVIRT(ID3D10Device, RSGetScissorRects)
        void ( STDMETHODCALLTYPE *RSGetScissorRects )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Inout_ /*_range(0, D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE )*/   UINT *NumRects,
            /* [annotation] */ 
            _Out_writes_opt_(*NumRects)  D3D10_RECT *pRects);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GetDeviceRemovedReason)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceRemovedReason )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(ID3D10Device, SetExceptionMode)
        HRESULT ( STDMETHODCALLTYPE *SetExceptionMode )( 
            ID3D10Device * This,
            UINT RaiseFlags);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GetExceptionMode)
        UINT ( STDMETHODCALLTYPE *GetExceptionMode )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _Inout_  UINT *pDataSize,
            /* [annotation] */ 
            _Out_writes_bytes_opt_(*pDataSize)  void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Device, SetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateData )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_  UINT DataSize,
            /* [annotation] */ 
            _In_reads_bytes_opt_(DataSize)  const void *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Device, SetPrivateDataInterface)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateDataInterface )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  REFGUID guid,
            /* [annotation] */ 
            _In_opt_  const IUnknown *pData);
        
        DECLSPEC_XFGVIRT(ID3D10Device, ClearState)
        void ( STDMETHODCALLTYPE *ClearState )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(ID3D10Device, Flush)
        void ( STDMETHODCALLTYPE *Flush )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateBuffer)
        HRESULT ( STDMETHODCALLTYPE *CreateBuffer )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_BUFFER_DESC *pDesc,
            /* [annotation] */ 
            _In_opt_  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_opt_  ID3D10Buffer **ppBuffer);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateTexture1D)
        HRESULT ( STDMETHODCALLTYPE *CreateTexture1D )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_TEXTURE1D_DESC *pDesc,
            /* [annotation] */ 
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels * pDesc->ArraySize))  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_  ID3D10Texture1D **ppTexture1D);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateTexture2D)
        HRESULT ( STDMETHODCALLTYPE *CreateTexture2D )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_TEXTURE2D_DESC *pDesc,
            /* [annotation] */ 
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels * pDesc->ArraySize))  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_  ID3D10Texture2D **ppTexture2D);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateTexture3D)
        HRESULT ( STDMETHODCALLTYPE *CreateTexture3D )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_TEXTURE3D_DESC *pDesc,
            /* [annotation] */ 
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels))  const D3D10_SUBRESOURCE_DATA *pInitialData,
            /* [annotation] */ 
            _Out_  ID3D10Texture3D **ppTexture3D);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateShaderResourceView)
        HRESULT ( STDMETHODCALLTYPE *CreateShaderResourceView )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10Resource *pResource,
            /* [annotation] */ 
            _In_opt_  const D3D10_SHADER_RESOURCE_VIEW_DESC *pDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10ShaderResourceView **ppSRView);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateRenderTargetView)
        HRESULT ( STDMETHODCALLTYPE *CreateRenderTargetView )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10Resource *pResource,
            /* [annotation] */ 
            _In_opt_  const D3D10_RENDER_TARGET_VIEW_DESC *pDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10RenderTargetView **ppRTView);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateDepthStencilView)
        HRESULT ( STDMETHODCALLTYPE *CreateDepthStencilView )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  ID3D10Resource *pResource,
            /* [annotation] */ 
            _In_opt_  const D3D10_DEPTH_STENCIL_VIEW_DESC *pDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilView **ppDepthStencilView);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateInputLayout)
        HRESULT ( STDMETHODCALLTYPE *CreateInputLayout )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_reads_(NumElements)  const D3D10_INPUT_ELEMENT_DESC *pInputElementDescs,
            /* [annotation] */ 
            _In_range_( 0, D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT )  UINT NumElements,
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecodeWithInputSignature,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10InputLayout **ppInputLayout);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateVertexShader)
        HRESULT ( STDMETHODCALLTYPE *CreateVertexShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10VertexShader **ppVertexShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateGeometryShader)
        HRESULT ( STDMETHODCALLTYPE *CreateGeometryShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10GeometryShader **ppGeometryShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateGeometryShaderWithStreamOutput)
        HRESULT ( STDMETHODCALLTYPE *CreateGeometryShaderWithStreamOutput )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _In_reads_opt_(NumEntries)  const D3D10_SO_DECLARATION_ENTRY *pSODeclaration,
            /* [annotation] */ 
            _In_range_( 0, D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT )  UINT NumEntries,
            /* [annotation] */ 
            _In_  UINT OutputStreamStride,
            /* [annotation] */ 
            _Out_opt_  ID3D10GeometryShader **ppGeometryShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreatePixelShader)
        HRESULT ( STDMETHODCALLTYPE *CreatePixelShader )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            /* [annotation] */ 
            _In_  SIZE_T BytecodeLength,
            /* [annotation] */ 
            _Out_opt_  ID3D10PixelShader **ppPixelShader);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateBlendState)
        HRESULT ( STDMETHODCALLTYPE *CreateBlendState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_BLEND_DESC *pBlendStateDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10BlendState **ppBlendState);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateDepthStencilState)
        HRESULT ( STDMETHODCALLTYPE *CreateDepthStencilState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_DEPTH_STENCIL_DESC *pDepthStencilDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10DepthStencilState **ppDepthStencilState);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateRasterizerState)
        HRESULT ( STDMETHODCALLTYPE *CreateRasterizerState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_RASTERIZER_DESC *pRasterizerDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10RasterizerState **ppRasterizerState);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateSamplerState)
        HRESULT ( STDMETHODCALLTYPE *CreateSamplerState )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_SAMPLER_DESC *pSamplerDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10SamplerState **ppSamplerState);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateQuery)
        HRESULT ( STDMETHODCALLTYPE *CreateQuery )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_QUERY_DESC *pQueryDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10Query **ppQuery);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreatePredicate)
        HRESULT ( STDMETHODCALLTYPE *CreatePredicate )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_QUERY_DESC *pPredicateDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10Predicate **ppPredicate);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CreateCounter)
        HRESULT ( STDMETHODCALLTYPE *CreateCounter )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_COUNTER_DESC *pCounterDesc,
            /* [annotation] */ 
            _Out_opt_  ID3D10Counter **ppCounter);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CheckFormatSupport)
        HRESULT ( STDMETHODCALLTYPE *CheckFormatSupport )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format,
            /* [annotation] */ 
            _Out_  UINT *pFormatSupport);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CheckMultisampleQualityLevels)
        HRESULT ( STDMETHODCALLTYPE *CheckMultisampleQualityLevels )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  DXGI_FORMAT Format,
            /* [annotation] */ 
            _In_  UINT SampleCount,
            /* [annotation] */ 
            _Out_  UINT *pNumQualityLevels);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CheckCounterInfo)
        void ( STDMETHODCALLTYPE *CheckCounterInfo )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_  D3D10_COUNTER_INFO *pCounterInfo);
        
        DECLSPEC_XFGVIRT(ID3D10Device, CheckCounter)
        HRESULT ( STDMETHODCALLTYPE *CheckCounter )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  const D3D10_COUNTER_DESC *pDesc,
            /* [annotation] */ 
            _Out_  D3D10_COUNTER_TYPE *pType,
            /* [annotation] */ 
            _Out_  UINT *pActiveCounters,
            /* [annotation] */ 
            _Out_writes_opt_(*pNameLength)  LPSTR szName,
            /* [annotation] */ 
            _Inout_opt_  UINT *pNameLength,
            /* [annotation] */ 
            _Out_writes_opt_(*pUnitsLength)  LPSTR szUnits,
            /* [annotation] */ 
            _Inout_opt_  UINT *pUnitsLength,
            /* [annotation] */ 
            _Out_writes_opt_(*pDescriptionLength)  LPSTR szDescription,
            /* [annotation] */ 
            _Inout_opt_  UINT *pDescriptionLength);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GetCreationFlags)
        UINT ( STDMETHODCALLTYPE *GetCreationFlags )( 
            ID3D10Device * This);
        
        DECLSPEC_XFGVIRT(ID3D10Device, OpenSharedResource)
        HRESULT ( STDMETHODCALLTYPE *OpenSharedResource )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  HANDLE hResource,
            /* [annotation] */ 
            _In_  REFIID ReturnedInterface,
            /* [annotation] */ 
            _Out_opt_  void **ppResource);
        
        DECLSPEC_XFGVIRT(ID3D10Device, SetTextFilterSize)
        void ( STDMETHODCALLTYPE *SetTextFilterSize )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _In_  UINT Width,
            /* [annotation] */ 
            _In_  UINT Height);
        
        DECLSPEC_XFGVIRT(ID3D10Device, GetTextFilterSize)
        void ( STDMETHODCALLTYPE *GetTextFilterSize )( 
            ID3D10Device * This,
            /* [annotation] */ 
            _Out_opt_  UINT *pWidth,
            /* [annotation] */ 
            _Out_opt_  UINT *pHeight);
        
        END_INTERFACE
    } ID3D10DeviceVtbl;

    interface ID3D10Device
    {
        CONST_VTBL struct ID3D10DeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Device_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Device_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Device_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Device_VSSetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers)	\
    ( (This)->lpVtbl -> VSSetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers) ) 

#define ID3D10Device_PSSetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews)	\
    ( (This)->lpVtbl -> PSSetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews) ) 

#define ID3D10Device_PSSetShader(This,pPixelShader)	\
    ( (This)->lpVtbl -> PSSetShader(This,pPixelShader) ) 

#define ID3D10Device_PSSetSamplers(This,StartSlot,NumSamplers,ppSamplers)	\
    ( (This)->lpVtbl -> PSSetSamplers(This,StartSlot,NumSamplers,ppSamplers) ) 

#define ID3D10Device_VSSetShader(This,pVertexShader)	\
    ( (This)->lpVtbl -> VSSetShader(This,pVertexShader) ) 

#define ID3D10Device_DrawIndexed(This,IndexCount,StartIndexLocation,BaseVertexLocation)	\
    ( (This)->lpVtbl -> DrawIndexed(This,IndexCount,StartIndexLocation,BaseVertexLocation) ) 

#define ID3D10Device_Draw(This,VertexCount,StartVertexLocation)	\
    ( (This)->lpVtbl -> Draw(This,VertexCount,StartVertexLocation) ) 

#define ID3D10Device_PSSetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers)	\
    ( (This)->lpVtbl -> PSSetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers) ) 

#define ID3D10Device_IASetInputLayout(This,pInputLayout)	\
    ( (This)->lpVtbl -> IASetInputLayout(This,pInputLayout) ) 

#define ID3D10Device_IASetVertexBuffers(This,StartSlot,NumBuffers,ppVertexBuffers,pStrides,pOffsets)	\
    ( (This)->lpVtbl -> IASetVertexBuffers(This,StartSlot,NumBuffers,ppVertexBuffers,pStrides,pOffsets) ) 

#define ID3D10Device_IASetIndexBuffer(This,pIndexBuffer,Format,Offset)	\
    ( (This)->lpVtbl -> IASetIndexBuffer(This,pIndexBuffer,Format,Offset) ) 

#define ID3D10Device_DrawIndexedInstanced(This,IndexCountPerInstance,InstanceCount,StartIndexLocation,BaseVertexLocation,StartInstanceLocation)	\
    ( (This)->lpVtbl -> DrawIndexedInstanced(This,IndexCountPerInstance,InstanceCount,StartIndexLocation,BaseVertexLocation,StartInstanceLocation) ) 

#define ID3D10Device_DrawInstanced(This,VertexCountPerInstance,InstanceCount,StartVertexLocation,StartInstanceLocation)	\
    ( (This)->lpVtbl -> DrawInstanced(This,VertexCountPerInstance,InstanceCount,StartVertexLocation,StartInstanceLocation) ) 

#define ID3D10Device_GSSetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers)	\
    ( (This)->lpVtbl -> GSSetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers) ) 

#define ID3D10Device_GSSetShader(This,pShader)	\
    ( (This)->lpVtbl -> GSSetShader(This,pShader) ) 

#define ID3D10Device_IASetPrimitiveTopology(This,Topology)	\
    ( (This)->lpVtbl -> IASetPrimitiveTopology(This,Topology) ) 

#define ID3D10Device_VSSetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews)	\
    ( (This)->lpVtbl -> VSSetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews) ) 

#define ID3D10Device_VSSetSamplers(This,StartSlot,NumSamplers,ppSamplers)	\
    ( (This)->lpVtbl -> VSSetSamplers(This,StartSlot,NumSamplers,ppSamplers) ) 

#define ID3D10Device_SetPredication(This,pPredicate,PredicateValue)	\
    ( (This)->lpVtbl -> SetPredication(This,pPredicate,PredicateValue) ) 

#define ID3D10Device_GSSetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews)	\
    ( (This)->lpVtbl -> GSSetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews) ) 

#define ID3D10Device_GSSetSamplers(This,StartSlot,NumSamplers,ppSamplers)	\
    ( (This)->lpVtbl -> GSSetSamplers(This,StartSlot,NumSamplers,ppSamplers) ) 

#define ID3D10Device_OMSetRenderTargets(This,NumViews,ppRenderTargetViews,pDepthStencilView)	\
    ( (This)->lpVtbl -> OMSetRenderTargets(This,NumViews,ppRenderTargetViews,pDepthStencilView) ) 

#define ID3D10Device_OMSetBlendState(This,pBlendState,BlendFactor,SampleMask)	\
    ( (This)->lpVtbl -> OMSetBlendState(This,pBlendState,BlendFactor,SampleMask) ) 

#define ID3D10Device_OMSetDepthStencilState(This,pDepthStencilState,StencilRef)	\
    ( (This)->lpVtbl -> OMSetDepthStencilState(This,pDepthStencilState,StencilRef) ) 

#define ID3D10Device_SOSetTargets(This,NumBuffers,ppSOTargets,pOffsets)	\
    ( (This)->lpVtbl -> SOSetTargets(This,NumBuffers,ppSOTargets,pOffsets) ) 

#define ID3D10Device_DrawAuto(This)	\
    ( (This)->lpVtbl -> DrawAuto(This) ) 

#define ID3D10Device_RSSetState(This,pRasterizerState)	\
    ( (This)->lpVtbl -> RSSetState(This,pRasterizerState) ) 

#define ID3D10Device_RSSetViewports(This,NumViewports,pViewports)	\
    ( (This)->lpVtbl -> RSSetViewports(This,NumViewports,pViewports) ) 

#define ID3D10Device_RSSetScissorRects(This,NumRects,pRects)	\
    ( (This)->lpVtbl -> RSSetScissorRects(This,NumRects,pRects) ) 

#define ID3D10Device_CopySubresourceRegion(This,pDstResource,DstSubresource,DstX,DstY,DstZ,pSrcResource,SrcSubresource,pSrcBox)	\
    ( (This)->lpVtbl -> CopySubresourceRegion(This,pDstResource,DstSubresource,DstX,DstY,DstZ,pSrcResource,SrcSubresource,pSrcBox) ) 

#define ID3D10Device_CopyResource(This,pDstResource,pSrcResource)	\
    ( (This)->lpVtbl -> CopyResource(This,pDstResource,pSrcResource) ) 

#define ID3D10Device_UpdateSubresource(This,pDstResource,DstSubresource,pDstBox,pSrcData,SrcRowPitch,SrcDepthPitch)	\
    ( (This)->lpVtbl -> UpdateSubresource(This,pDstResource,DstSubresource,pDstBox,pSrcData,SrcRowPitch,SrcDepthPitch) ) 

#define ID3D10Device_ClearRenderTargetView(This,pRenderTargetView,ColorRGBA)	\
    ( (This)->lpVtbl -> ClearRenderTargetView(This,pRenderTargetView,ColorRGBA) ) 

#define ID3D10Device_ClearDepthStencilView(This,pDepthStencilView,ClearFlags,Depth,Stencil)	\
    ( (This)->lpVtbl -> ClearDepthStencilView(This,pDepthStencilView,ClearFlags,Depth,Stencil) ) 

#define ID3D10Device_GenerateMips(This,pShaderResourceView)	\
    ( (This)->lpVtbl -> GenerateMips(This,pShaderResourceView) ) 

#define ID3D10Device_ResolveSubresource(This,pDstResource,DstSubresource,pSrcResource,SrcSubresource,Format)	\
    ( (This)->lpVtbl -> ResolveSubresource(This,pDstResource,DstSubresource,pSrcResource,SrcSubresource,Format) ) 

#define ID3D10Device_VSGetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers)	\
    ( (This)->lpVtbl -> VSGetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers) ) 

#define ID3D10Device_PSGetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews)	\
    ( (This)->lpVtbl -> PSGetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews) ) 

#define ID3D10Device_PSGetShader(This,ppPixelShader)	\
    ( (This)->lpVtbl -> PSGetShader(This,ppPixelShader) ) 

#define ID3D10Device_PSGetSamplers(This,StartSlot,NumSamplers,ppSamplers)	\
    ( (This)->lpVtbl -> PSGetSamplers(This,StartSlot,NumSamplers,ppSamplers) ) 

#define ID3D10Device_VSGetShader(This,ppVertexShader)	\
    ( (This)->lpVtbl -> VSGetShader(This,ppVertexShader) ) 

#define ID3D10Device_PSGetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers)	\
    ( (This)->lpVtbl -> PSGetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers) ) 

#define ID3D10Device_IAGetInputLayout(This,ppInputLayout)	\
    ( (This)->lpVtbl -> IAGetInputLayout(This,ppInputLayout) ) 

#define ID3D10Device_IAGetVertexBuffers(This,StartSlot,NumBuffers,ppVertexBuffers,pStrides,pOffsets)	\
    ( (This)->lpVtbl -> IAGetVertexBuffers(This,StartSlot,NumBuffers,ppVertexBuffers,pStrides,pOffsets) ) 

#define ID3D10Device_IAGetIndexBuffer(This,pIndexBuffer,Format,Offset)	\
    ( (This)->lpVtbl -> IAGetIndexBuffer(This,pIndexBuffer,Format,Offset) ) 

#define ID3D10Device_GSGetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers)	\
    ( (This)->lpVtbl -> GSGetConstantBuffers(This,StartSlot,NumBuffers,ppConstantBuffers) ) 

#define ID3D10Device_GSGetShader(This,ppGeometryShader)	\
    ( (This)->lpVtbl -> GSGetShader(This,ppGeometryShader) ) 

#define ID3D10Device_IAGetPrimitiveTopology(This,pTopology)	\
    ( (This)->lpVtbl -> IAGetPrimitiveTopology(This,pTopology) ) 

#define ID3D10Device_VSGetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews)	\
    ( (This)->lpVtbl -> VSGetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews) ) 

#define ID3D10Device_VSGetSamplers(This,StartSlot,NumSamplers,ppSamplers)	\
    ( (This)->lpVtbl -> VSGetSamplers(This,StartSlot,NumSamplers,ppSamplers) ) 

#define ID3D10Device_GetPredication(This,ppPredicate,pPredicateValue)	\
    ( (This)->lpVtbl -> GetPredication(This,ppPredicate,pPredicateValue) ) 

#define ID3D10Device_GSGetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews)	\
    ( (This)->lpVtbl -> GSGetShaderResources(This,StartSlot,NumViews,ppShaderResourceViews) ) 

#define ID3D10Device_GSGetSamplers(This,StartSlot,NumSamplers,ppSamplers)	\
    ( (This)->lpVtbl -> GSGetSamplers(This,StartSlot,NumSamplers,ppSamplers) ) 

#define ID3D10Device_OMGetRenderTargets(This,NumViews,ppRenderTargetViews,ppDepthStencilView)	\
    ( (This)->lpVtbl -> OMGetRenderTargets(This,NumViews,ppRenderTargetViews,ppDepthStencilView) ) 

#define ID3D10Device_OMGetBlendState(This,ppBlendState,BlendFactor,pSampleMask)	\
    ( (This)->lpVtbl -> OMGetBlendState(This,ppBlendState,BlendFactor,pSampleMask) ) 

#define ID3D10Device_OMGetDepthStencilState(This,ppDepthStencilState,pStencilRef)	\
    ( (This)->lpVtbl -> OMGetDepthStencilState(This,ppDepthStencilState,pStencilRef) ) 

#define ID3D10Device_SOGetTargets(This,NumBuffers,ppSOTargets,pOffsets)	\
    ( (This)->lpVtbl -> SOGetTargets(This,NumBuffers,ppSOTargets,pOffsets) ) 

#define ID3D10Device_RSGetState(This,ppRasterizerState)	\
    ( (This)->lpVtbl -> RSGetState(This,ppRasterizerState) ) 

#define ID3D10Device_RSGetViewports(This,NumViewports,pViewports)	\
    ( (This)->lpVtbl -> RSGetViewports(This,NumViewports,pViewports) ) 

#define ID3D10Device_RSGetScissorRects(This,NumRects,pRects)	\
    ( (This)->lpVtbl -> RSGetScissorRects(This,NumRects,pRects) ) 

#define ID3D10Device_GetDeviceRemovedReason(This)	\
    ( (This)->lpVtbl -> GetDeviceRemovedReason(This) ) 

#define ID3D10Device_SetExceptionMode(This,RaiseFlags)	\
    ( (This)->lpVtbl -> SetExceptionMode(This,RaiseFlags) ) 

#define ID3D10Device_GetExceptionMode(This)	\
    ( (This)->lpVtbl -> GetExceptionMode(This) ) 

#define ID3D10Device_GetPrivateData(This,guid,pDataSize,pData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,guid,pDataSize,pData) ) 

#define ID3D10Device_SetPrivateData(This,guid,DataSize,pData)	\
    ( (This)->lpVtbl -> SetPrivateData(This,guid,DataSize,pData) ) 

#define ID3D10Device_SetPrivateDataInterface(This,guid,pData)	\
    ( (This)->lpVtbl -> SetPrivateDataInterface(This,guid,pData) ) 

#define ID3D10Device_ClearState(This)	\
    ( (This)->lpVtbl -> ClearState(This) ) 

#define ID3D10Device_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define ID3D10Device_CreateBuffer(This,pDesc,pInitialData,ppBuffer)	\
    ( (This)->lpVtbl -> CreateBuffer(This,pDesc,pInitialData,ppBuffer) ) 

#define ID3D10Device_CreateTexture1D(This,pDesc,pInitialData,ppTexture1D)	\
    ( (This)->lpVtbl -> CreateTexture1D(This,pDesc,pInitialData,ppTexture1D) ) 

#define ID3D10Device_CreateTexture2D(This,pDesc,pInitialData,ppTexture2D)	\
    ( (This)->lpVtbl -> CreateTexture2D(This,pDesc,pInitialData,ppTexture2D) ) 

#define ID3D10Device_CreateTexture3D(This,pDesc,pInitialData,ppTexture3D)	\
    ( (This)->lpVtbl -> CreateTexture3D(This,pDesc,pInitialData,ppTexture3D) ) 

#define ID3D10Device_CreateShaderResourceView(This,pResource,pDesc,ppSRView)	\
    ( (This)->lpVtbl -> CreateShaderResourceView(This,pResource,pDesc,ppSRView) ) 

#define ID3D10Device_CreateRenderTargetView(This,pResource,pDesc,ppRTView)	\
    ( (This)->lpVtbl -> CreateRenderTargetView(This,pResource,pDesc,ppRTView) ) 

#define ID3D10Device_CreateDepthStencilView(This,pResource,pDesc,ppDepthStencilView)	\
    ( (This)->lpVtbl -> CreateDepthStencilView(This,pResource,pDesc,ppDepthStencilView) ) 

#define ID3D10Device_CreateInputLayout(This,pInputElementDescs,NumElements,pShaderBytecodeWithInputSignature,BytecodeLength,ppInputLayout)	\
    ( (This)->lpVtbl -> CreateInputLayout(This,pInputElementDescs,NumElements,pShaderBytecodeWithInputSignature,BytecodeLength,ppInputLayout) ) 

#define ID3D10Device_CreateVertexShader(This,pShaderBytecode,BytecodeLength,ppVertexShader)	\
    ( (This)->lpVtbl -> CreateVertexShader(This,pShaderBytecode,BytecodeLength,ppVertexShader) ) 

#define ID3D10Device_CreateGeometryShader(This,pShaderBytecode,BytecodeLength,ppGeometryShader)	\
    ( (This)->lpVtbl -> CreateGeometryShader(This,pShaderBytecode,BytecodeLength,ppGeometryShader) ) 

#define ID3D10Device_CreateGeometryShaderWithStreamOutput(This,pShaderBytecode,BytecodeLength,pSODeclaration,NumEntries,OutputStreamStride,ppGeometryShader)	\
    ( (This)->lpVtbl -> CreateGeometryShaderWithStreamOutput(This,pShaderBytecode,BytecodeLength,pSODeclaration,NumEntries,OutputStreamStride,ppGeometryShader) ) 

#define ID3D10Device_CreatePixelShader(This,pShaderBytecode,BytecodeLength,ppPixelShader)	\
    ( (This)->lpVtbl -> CreatePixelShader(This,pShaderBytecode,BytecodeLength,ppPixelShader) ) 

#define ID3D10Device_CreateBlendState(This,pBlendStateDesc,ppBlendState)	\
    ( (This)->lpVtbl -> CreateBlendState(This,pBlendStateDesc,ppBlendState) ) 

#define ID3D10Device_CreateDepthStencilState(This,pDepthStencilDesc,ppDepthStencilState)	\
    ( (This)->lpVtbl -> CreateDepthStencilState(This,pDepthStencilDesc,ppDepthStencilState) ) 

#define ID3D10Device_CreateRasterizerState(This,pRasterizerDesc,ppRasterizerState)	\
    ( (This)->lpVtbl -> CreateRasterizerState(This,pRasterizerDesc,ppRasterizerState) ) 

#define ID3D10Device_CreateSamplerState(This,pSamplerDesc,ppSamplerState)	\
    ( (This)->lpVtbl -> CreateSamplerState(This,pSamplerDesc,ppSamplerState) ) 

#define ID3D10Device_CreateQuery(This,pQueryDesc,ppQuery)	\
    ( (This)->lpVtbl -> CreateQuery(This,pQueryDesc,ppQuery) ) 

#define ID3D10Device_CreatePredicate(This,pPredicateDesc,ppPredicate)	\
    ( (This)->lpVtbl -> CreatePredicate(This,pPredicateDesc,ppPredicate) ) 

#define ID3D10Device_CreateCounter(This,pCounterDesc,ppCounter)	\
    ( (This)->lpVtbl -> CreateCounter(This,pCounterDesc,ppCounter) ) 

#define ID3D10Device_CheckFormatSupport(This,Format,pFormatSupport)	\
    ( (This)->lpVtbl -> CheckFormatSupport(This,Format,pFormatSupport) ) 

#define ID3D10Device_CheckMultisampleQualityLevels(This,Format,SampleCount,pNumQualityLevels)	\
    ( (This)->lpVtbl -> CheckMultisampleQualityLevels(This,Format,SampleCount,pNumQualityLevels) ) 

#define ID3D10Device_CheckCounterInfo(This,pCounterInfo)	\
    ( (This)->lpVtbl -> CheckCounterInfo(This,pCounterInfo) ) 

#define ID3D10Device_CheckCounter(This,pDesc,pType,pActiveCounters,szName,pNameLength,szUnits,pUnitsLength,szDescription,pDescriptionLength)	\
    ( (This)->lpVtbl -> CheckCounter(This,pDesc,pType,pActiveCounters,szName,pNameLength,szUnits,pUnitsLength,szDescription,pDescriptionLength) ) 

#define ID3D10Device_GetCreationFlags(This)	\
    ( (This)->lpVtbl -> GetCreationFlags(This) ) 

#define ID3D10Device_OpenSharedResource(This,hResource,ReturnedInterface,ppResource)	\
    ( (This)->lpVtbl -> OpenSharedResource(This,hResource,ReturnedInterface,ppResource) ) 

#define ID3D10Device_SetTextFilterSize(This,Width,Height)	\
    ( (This)->lpVtbl -> SetTextFilterSize(This,Width,Height) ) 

#define ID3D10Device_GetTextFilterSize(This,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetTextFilterSize(This,pWidth,pHeight) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Device_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0023 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0023_v0_0_s_ifspec;

#ifndef __ID3D10Multithread_INTERFACE_DEFINED__
#define __ID3D10Multithread_INTERFACE_DEFINED__

/* interface ID3D10Multithread */
/* [unique][local][object][uuid] */ 


EXTERN_C const IID IID_ID3D10Multithread;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B7E4E00-342C-4106-A19F-4F2704F689F0")
    ID3D10Multithread : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE Enter( void) = 0;
        
        virtual void STDMETHODCALLTYPE Leave( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE SetMultithreadProtected( 
            /* [annotation] */ 
            _In_  BOOL bMTProtect) = 0;
        
        virtual BOOL STDMETHODCALLTYPE GetMultithreadProtected( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ID3D10MultithreadVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ID3D10Multithread * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ID3D10Multithread * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ID3D10Multithread * This);
        
        DECLSPEC_XFGVIRT(ID3D10Multithread, Enter)
        void ( STDMETHODCALLTYPE *Enter )( 
            ID3D10Multithread * This);
        
        DECLSPEC_XFGVIRT(ID3D10Multithread, Leave)
        void ( STDMETHODCALLTYPE *Leave )( 
            ID3D10Multithread * This);
        
        DECLSPEC_XFGVIRT(ID3D10Multithread, SetMultithreadProtected)
        BOOL ( STDMETHODCALLTYPE *SetMultithreadProtected )( 
            ID3D10Multithread * This,
            /* [annotation] */ 
            _In_  BOOL bMTProtect);
        
        DECLSPEC_XFGVIRT(ID3D10Multithread, GetMultithreadProtected)
        BOOL ( STDMETHODCALLTYPE *GetMultithreadProtected )( 
            ID3D10Multithread * This);
        
        END_INTERFACE
    } ID3D10MultithreadVtbl;

    interface ID3D10Multithread
    {
        CONST_VTBL struct ID3D10MultithreadVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ID3D10Multithread_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ID3D10Multithread_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ID3D10Multithread_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ID3D10Multithread_Enter(This)	\
    ( (This)->lpVtbl -> Enter(This) ) 

#define ID3D10Multithread_Leave(This)	\
    ( (This)->lpVtbl -> Leave(This) ) 

#define ID3D10Multithread_SetMultithreadProtected(This,bMTProtect)	\
    ( (This)->lpVtbl -> SetMultithreadProtected(This,bMTProtect) ) 

#define ID3D10Multithread_GetMultithreadProtected(This)	\
    ( (This)->lpVtbl -> GetMultithreadProtected(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ID3D10Multithread_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_d3d10_0000_0024 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum D3D10_CREATE_DEVICE_FLAG
    {
        D3D10_CREATE_DEVICE_SINGLETHREADED	= 0x1,
        D3D10_CREATE_DEVICE_DEBUG	= 0x2,
        D3D10_CREATE_DEVICE_SWITCH_TO_REF	= 0x4,
        D3D10_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS	= 0x8,
        D3D10_CREATE_DEVICE_ALLOW_NULL_FROM_MAP	= 0x10,
        D3D10_CREATE_DEVICE_BGRA_SUPPORT	= 0x20,
        D3D10_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY	= 0x80,
        D3D10_CREATE_DEVICE_STRICT_VALIDATION	= 0x200,
        D3D10_CREATE_DEVICE_DEBUGGABLE	= 0x400
    } 	D3D10_CREATE_DEVICE_FLAG;


#define	D3D10_SDK_VERSION	( 29 )

#if !defined( D3D10_IGNORE_SDK_LAYERS ) 
#include "d3d10sdklayers.h" 
#endif 
#include "d3d10misc.h" 
#include "d3d10shader.h" 
#include "d3d10effect.h" 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
DEFINE_GUID(IID_ID3D10DeviceChild,0x9B7E4C00,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10DepthStencilState,0x2B4B1CC8,0xA4AD,0x41f8,0x83,0x22,0xCA,0x86,0xFC,0x3E,0xC6,0x75);
DEFINE_GUID(IID_ID3D10BlendState,0xEDAD8D19,0x8A35,0x4d6d,0x85,0x66,0x2E,0xA2,0x76,0xCD,0xE1,0x61);
DEFINE_GUID(IID_ID3D10RasterizerState,0xA2A07292,0x89AF,0x4345,0xBE,0x2E,0xC5,0x3D,0x9F,0xBB,0x6E,0x9F);
DEFINE_GUID(IID_ID3D10Resource,0x9B7E4C01,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Buffer,0x9B7E4C02,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Texture1D,0x9B7E4C03,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Texture2D,0x9B7E4C04,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Texture3D,0x9B7E4C05,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10View,0xC902B03F,0x60A7,0x49BA,0x99,0x36,0x2A,0x3A,0xB3,0x7A,0x7E,0x33);
DEFINE_GUID(IID_ID3D10ShaderResourceView,0x9B7E4C07,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10RenderTargetView,0x9B7E4C08,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10DepthStencilView,0x9B7E4C09,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10VertexShader,0x9B7E4C0A,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10GeometryShader,0x6316BE88,0x54CD,0x4040,0xAB,0x44,0x20,0x46,0x1B,0xC8,0x1F,0x68);
DEFINE_GUID(IID_ID3D10PixelShader,0x4968B601,0x9D00,0x4cde,0x83,0x46,0x8E,0x7F,0x67,0x58,0x19,0xB6);
DEFINE_GUID(IID_ID3D10InputLayout,0x9B7E4C0B,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10SamplerState,0x9B7E4C0C,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Asynchronous,0x9B7E4C0D,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Query,0x9B7E4C0E,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Predicate,0x9B7E4C10,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Counter,0x9B7E4C11,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Device,0x9B7E4C0F,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);
DEFINE_GUID(IID_ID3D10Multithread,0x9B7E4E00,0x342C,0x4106,0xA1,0x9F,0x4F,0x27,0x04,0xF6,0x89,0xF0);


extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_d3d10_0000_0024_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


