/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* amprt.h
*
* Define the C++ interfaces exported by the C++ AMP runtime
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/
#pragma once

#ifndef _SILENCE_AMP_DEPRECATION_WARNINGS
#error <amprt.h> is part of C++ AMP which is deprecated by Microsoft and will be REMOVED. \
You can define _SILENCE_AMP_DEPRECATION_WARNINGS to acknowledge that you have received this warning.
#endif // _SILENCE_AMP_DEPRECATION_WARNINGS

#if defined(WINAPI_FAMILY) && (WINAPI_FAMILY == WINAPI_FAMILY_PHONE_APP)
    #error ERROR: C++ AMP runtime is not supported for applications where WINAPI_FAMILY == WINAPI_FAMILY_PHONE_APP.
#endif

#if !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64) )
    #error ERROR: C++ AMP runtime is supported only on X64, X86, ARM, and _M_ARM64 architectures.
#endif

#if defined (_M_CEE)
    #error ERROR: C++ AMP runtime is not supported when compiling /clr.
#endif

#ifndef __cplusplus
    #error ERROR: C++ AMP runtime is supported only for C++.
#endif

#if !defined(_CXXAMP)

#if defined(_DEBUG)
    #pragma comment(lib, "vcampd")
#else   // _DEBUG
    #pragma comment(lib, "vcamp")
#endif  // _DEBUG

#endif // _CXXAMP

#if !defined(_CXXAMP)

#define __GPU      restrict(amp,cpu)
#define __GPU_ONLY restrict(amp)
#define __CPU_ONLY restrict(cpu)

#else

#define __GPU
#define __GPU_ONLY
#define __CPU_ONLY

#endif // _CXXAMP

#include <Unknwn.h>
#include <crtdbg.h>
#include <string>
#include <vector>
#include <iterator>

#if defined(_CXXAMP)
#include <strsafe.h>
#endif // _CXXAMP

#include <future>
#include <functional>
#include <map>
#include <unordered_map>
#include <set>
#include <unordered_set>
#include <concrt.h>
#include <type_traits>

#include "amprt_exceptions.h"

#if !defined(_AMPIMP)
#define _AMPIMP     __declspec(dllimport)
#endif

#pragma pack(push,8)

// Part of runtime-compiler interface
extern "C"
{
    // Access mode of fields
    enum _Access_mode
    {
        _No_access = 0,
        _Read_access = (1 << 0),
        _Write_access = (1 << 1),
        _Is_array_mode = (1 << 30),
        _Read_write_access = _Read_access | _Write_access,
    };
}

namespace Concurrency
{
    /// <summary>
    ///     Enumeration type used to denote the various types of access to data.
    /// </summary>
    enum access_type
    {
        access_type_none = 0,
        access_type_read = (1 << 0),
        access_type_write = (1 << 1),
        access_type_read_write = access_type_read | access_type_write,
        access_type_auto = (1 << 31),
    };

// Forward declarations
class accelerator_view;
class accelerator;

namespace details
{
    const size_t ERROR_MSG_BUFFER_SIZE = 1024;

    //  A reference counter to be used as the base class for all reference counted types.
    class _Reference_counter
    {
    public:

        //  Constructor.
        _Reference_counter()  : _M_rc(0) {}

        //  Destructor.
        virtual ~_Reference_counter() noexcept(false) {}

        // Add a reference.
        // Thread-safe.
        size_t _Add_reference()
        {
            return InterlockedIncrement(reinterpret_cast<LONG volatile*>(&_M_rc));
        }

        // Remove a reference.
        // Thread-safe.
        size_t _Remove_reference()
        {
            _ASSERTE(_M_rc > 0);

            size_t refCount = InterlockedDecrement(reinterpret_cast<LONG volatile*>(&_M_rc));

            if (refCount == 0)
                this->_Release();

            return refCount;
        }

        // Release the counter
        _AMPIMP void _Release();

        // Return the reference count value
        size_t _Get_reference_count()
        {
            return _M_rc;
        }

    private:
        size_t _M_rc;
    };

    // A smart pointer to a reference counted object
    // T must be a type derived from _Reference_counter
    template <class T>
    class _Reference_counted_obj_ptr
    {
    public:

        // Constructor
        _Reference_counted_obj_ptr(T* _Ptr = NULL) :  _M_obj_ptr(_Ptr)
        {
            _Init();
        }

        // Copy constructor
        _Reference_counted_obj_ptr(const _Reference_counted_obj_ptr &_Other) : _M_obj_ptr(_Other._M_obj_ptr)
        {
            _Init();
        }

        // Move constructor
        _Reference_counted_obj_ptr(_Reference_counted_obj_ptr &&_Other) noexcept : _M_obj_ptr(_Other._M_obj_ptr)
        {
            _Other._M_obj_ptr = nullptr;
            // No change to ref-count
        }

        // Destructor
        ~_Reference_counted_obj_ptr()
        {
            if (_M_obj_ptr != NULL) {
                _UnInitialize(_M_obj_ptr);
            }
        }

        // Assignment operator
        _Reference_counted_obj_ptr& operator=(const _Reference_counted_obj_ptr &_Other)
        {
            if (_M_obj_ptr != _Other._M_obj_ptr)
            {
                T *oldPtr = _M_obj_ptr;
                _M_obj_ptr = _Other._M_obj_ptr;
                _Init();

                if (oldPtr != NULL) {
                    _UnInitialize(oldPtr);
                }
            }
            return *this;
        }

        // Move-assignment operator
        _Reference_counted_obj_ptr& operator=(_Reference_counted_obj_ptr &&_Other) noexcept
        {
            if (_M_obj_ptr != _Other._M_obj_ptr)
            {
                T *oldPtr = _M_obj_ptr;
                _M_obj_ptr = _Other._M_obj_ptr;
                _Other._M_obj_ptr = nullptr;
                // No change to ref-count of the adopted pointer.

                if (oldPtr != nullptr)
                {
                    _UnInitialize(oldPtr);
                }
            }
            return *this;
        }

        _Ret_ T* operator->() const
        {
            return _M_obj_ptr;
        }

        T& operator*() const
        {
            return *_M_obj_ptr;
        }

        operator T*() const
        {
            return _M_obj_ptr;
        }

        _Ret_ T* _Get_ptr() const
        {
            return _M_obj_ptr;
        }

    private:
        T *_M_obj_ptr;

        void _Init()
        {
            if (_M_obj_ptr == NULL)
                return;

            reinterpret_cast<_Reference_counter*>(_M_obj_ptr)->_Add_reference();
        }

        static void _UnInitialize(_In_ T *_Obj_ptr)
        {
            reinterpret_cast<_Reference_counter*>(_Obj_ptr)->_Remove_reference();
        }
    };

    // Forward declarations
    class _Trace;
    class _Amp_runtime_trace;
    class _Buffer;
    class _Texture;
    class _Sampler;
    class _Ubiquitous_buffer;
    class _D3D_interop;
    class _Accelerator_view_impl;
    class _CPU_accelerator_view_impl;
    class _D3D_accelerator_view_impl;
    class _Accelerator_impl;
    class _Event_impl;
    class _DPC_runtime_factory;
    class _View_shape;
    struct _Buffer_descriptor;
    class _Accelerator_view_hasher;
    struct _DPC_shader_blob;
    struct _View_info;

    // The enum specifies the base type for short vector type.
    enum _Short_vector_base_type_id : unsigned int
    {
        _Uint_type = 0,
        _Int_type = 1,
        _Float_type = 2,
        _Unorm_type = 3,
        _Norm_type = 4,
        _Double_type = 5,
        _Invalid_type = 0xFFFFFFFF
    };

    typedef enum _Short_vector_base_type_id _Texture_base_type_id;

} // namespace Concurrency::details

typedef details::_Reference_counted_obj_ptr<details::_Accelerator_view_impl> _Accelerator_view_impl_ptr;
typedef details::_Reference_counted_obj_ptr<details::_Accelerator_impl> _Accelerator_impl_ptr;
typedef details::_Reference_counted_obj_ptr<details::_Buffer> _Buffer_ptr;
typedef details::_Reference_counted_obj_ptr<details::_Texture> _Texture_ptr;
typedef details::_Reference_counted_obj_ptr<details::_Sampler> _Sampler_ptr;
typedef details::_Reference_counted_obj_ptr<details::_Ubiquitous_buffer> _Ubiquitous_buffer_ptr;
typedef details::_Reference_counted_obj_ptr<details::_Event_impl> _Event_impl_ptr;
typedef details::_Reference_counted_obj_ptr<details::_View_shape> _View_shape_ptr;

namespace details
{
    // The _Event class.
    class _Event
    {
        friend class _Buffer;
        friend class _Texture;
        friend class accelerator_view;
        friend class _D3D_accelerator_view_impl;

    public:
        /// <summary>
        ///     Constructor of the _Event.
        /// </summary>
        _AMPIMP _Event();

        /// <summary>
        ///     Destructor of the _Event.
        /// </summary>
        _AMPIMP ~_Event();

        /// <summary>
        ///     Copy constructor
        /// </summary>
        _AMPIMP _Event(const _Event & _Other);

        /// <summary>
        ///     Assignment operator
        /// </summary>
        _AMPIMP _Event & operator=(const _Event & _Other);

        /// <summary>
        ///     Poll whether the _Event has completed or not. Swallows any exceptions
        /// </summary>
        /// <returns>
        ///     true, if the _Event has completed, false otherwise
        /// </returns>
        _AMPIMP bool _Is_finished_nothrow();

        /// <summary>
        ///     Poll whether the _Event has completed or not and throws any exceptions that occur
        /// </summary>
        /// <returns>
        ///     true, if the _Event has completed, false otherwise
        /// </returns>
        _AMPIMP bool _Is_finished();

        /// <summary>
        ///     Wait until the _Event completes and throw any exceptions that occur.
        /// </summary>
        _AMPIMP void _Get();

        /// <summary>
        ///     Tells if this is an empty event
        /// </summary>
        /// <returns>
        ///     true, if the _Event is empty
        ///     false, otherwise
        /// </returns>
        _AMPIMP bool _Is_empty() const;

        /// <summary>
        ///     Creates an event which is an ordered collection of this and _Ev
        /// </summary>
        /// <returns>
        ///     The composite event
        /// </returns>
        _AMPIMP _Event _Add_event(_Event _Ev);

        /// <summary>
        ///     Creates an event which is an ordered collection of this and a continuation task
        /// </summary>
        /// <returns>
        ///     The composite event
        /// </returns>
        _AMPIMP _Event _Add_continuation(const std::function<_Event __cdecl ()> &_Continuation_task);

        /// <summary>
        ///     Return true if the other _Event is same as this _Event; false otherwise
        /// </summary>
        _AMPIMP bool operator==(const _Event &_Other) const;

        /// <summary>
        ///     Return false if the other _Event is same as this _Event; true otherwise
        /// </summary>
        _AMPIMP bool operator!=(const _Event &_Other) const;

    private:

        // Private constructor
        _Event(_In_ _Event_impl* _Impl);

        _Event_impl_ptr _M_ptr_event_impl;
    };

    typedef _Buffer_descriptor *_View_key;

    _Ret_ _Accelerator_view_impl* _Get_accelerator_view_impl_ptr(const accelerator_view& _Accl_view);
    _Ret_ _Accelerator_impl* _Get_accelerator_impl_ptr(const accelerator& _Accl);
    _Event _Get_access_async(const _View_key _Key, accelerator_view _Av, _Access_mode _Mode, _Buffer_ptr &_Buf_ptr);
    unsigned int _Get_mipmap_levels(const _Texture *_Tex);

    inline bool _Is_valid_access_mode(_Access_mode _Mode)
    {
        if ((_Mode != _Read_access) &&
            (_Mode != _Write_access) &&
            (_Mode != _Read_write_access))
        {
            return false;
        }

        return true;
    }

    // Caution: Do not change this structure definition.
    // This struct is special and is processed by the FE to identify the buffers
    // used in a parallel_for_each and to setup the _M_data_ptr with the appropriate
    // buffer ptr value in the device code.
    typedef struct _Buffer_descriptor
    {
        friend _Event _Get_access_async(const _View_key _Key, accelerator_view _Av, _Access_mode _Mode, _Buffer_ptr &_Buf_ptr);

        // _M_data_ptr points to the raw data underlying the buffer for accessing on host
        mutable void *_M_data_ptr;

    private:
        // _M_buffer_ptr points to a _Ubiquitous_buffer that holds the data in an 1D array.
        // This is private to ensure that all assignments to this data member
        // only happen through public functions which properly manage the
        // ref count of the underlying buffer
        _Ubiquitous_buffer *_M_buffer_ptr;

    public:
        // _M_curr_cpu_access_mode specifies the current access mode of the data on the
        // cpu accelerator_view specified at the time of registration of this view
        _Access_mode _M_curr_cpu_access_mode;

        // _M_type_acess_mode specifies the access mode of the overlay type
        // array_views set it to the appropriate access mode and for arrays it is
        // always _Is_array_mode.
        _Access_mode _M_type_access_mode;

    public:
        // Public functions

        // Default constructor
        _Buffer_descriptor() __GPU
            : _M_data_ptr(NULL), _M_buffer_ptr(NULL),
            _M_curr_cpu_access_mode(_No_access), _M_type_access_mode(_Is_array_mode)
        {
        }

        _Buffer_descriptor(_In_ void *_Data_ptr, _In_ _Ubiquitous_buffer *_Buffer_ptr,
                           _Access_mode _Curr_cpu_access_mode, _Access_mode _Type_mode) __GPU
            : _M_data_ptr(_Data_ptr), _M_buffer_ptr(NULL),
            _M_curr_cpu_access_mode(_Curr_cpu_access_mode), _M_type_access_mode(_Type_mode)
        {
            _Set_buffer_ptr(_Buffer_ptr);
        }

        // Destructor
        ~_Buffer_descriptor() __GPU
        {
            _Set_buffer_ptr(NULL);
        }

        // Copy constructor
        _Buffer_descriptor(const _Buffer_descriptor &_Other) __GPU
            : _M_data_ptr(_Other._M_data_ptr), _M_buffer_ptr(NULL),
            _M_curr_cpu_access_mode(_Other._M_curr_cpu_access_mode), _M_type_access_mode(_Other._M_type_access_mode)
        {
            _Set_buffer_ptr(_Other._M_buffer_ptr);
        }

        // Assignment operator
        _Buffer_descriptor& operator=(const _Buffer_descriptor &_Other) __GPU
        {
            if (this != &_Other)
            {
                _M_data_ptr = _Other._M_data_ptr;
                _M_curr_cpu_access_mode = _Other._M_curr_cpu_access_mode;
                _M_type_access_mode = _Other._M_type_access_mode;
                _Set_buffer_ptr(_Other._M_buffer_ptr);
            }

            return *this;
        }

        _Ret_ _Ubiquitous_buffer* _Get_buffer_ptr() const __CPU_ONLY
        {
            return _M_buffer_ptr;
        }

        void _Set_buffer_ptr(_In_opt_ _Ubiquitous_buffer *_Buffer_ptr) __CPU_ONLY
        {
            if (_M_buffer_ptr != _Buffer_ptr)
            {
                if (_M_buffer_ptr != NULL) {
                    reinterpret_cast<_Reference_counter*>(_M_buffer_ptr)->_Remove_reference();
                }

                _M_buffer_ptr = _Buffer_ptr;

                if (_M_buffer_ptr != NULL) {
                    reinterpret_cast<_Reference_counter*>(_M_buffer_ptr)->_Add_reference();
                }
            }
        }

#if !defined(_CXXAMP)
        void _Set_buffer_ptr(_In_opt_ _Ubiquitous_buffer *) __GPU_ONLY
        {
            // No need to set the buffer ptr on the GPU
            _M_buffer_ptr = NULL;
        }
#endif // _CXXAMP

        bool _Is_array() const
        {
            return (_M_type_access_mode == _Is_array_mode);
        }

        _Ret_ _View_key _Get_view_key()
        {
            return this;
        }

        const _View_key _Get_view_key() const
        {
            return ((const _View_key)(this));
        }

        _AMPIMP void _Get_CPU_access(_Access_mode _Requested_mode) const;

    } _Buffer_descriptor;

    // Caution: Do not change this structure definition.
    // This struct is special and is processed by the FE to identify the textures
    // used in a parallel_for_each and to setup the _M_data_ptr with the appropriate
    // texture ptr value in the device code.
    typedef struct _Texture_descriptor
    {
        // _M_data_ptr points to the raw data underlying the texture
        mutable IUnknown *_M_data_ptr;

    private:
        // _M_texture_ptr points to a _Texture that holds the data
        // This is private to ensure that all assignments to this data member
        // only happen through public functions which properly manage the
        // ref count of the underlying texture
        _Texture *_M_texture_ptr;

        // The index of the most detailed (largest in size) mipmap level for the texture (or texture view)
        // This value is always zero for the texture and might be non-zero for the texture views
        unsigned int _M_most_detailed_mipmap_level;

        // Number of accessible mipmap levels for the texture (or texture view),
        // e.g. if the texture has 3 mipmap levels ([0, 1, 2]),
        // then read-only texture view with most detailed mipmap level equal to 1, can have 1 or 2 mipmap levels ([1] or [1, 2]).
        // Further texture_views created on top of the texture view defined above can only narrow down the range of accessible mipmap levels.
        unsigned int _M_view_mipmap_levels;

    public:
        // Public functions

        // Default constructor
        _Texture_descriptor() __GPU
            : _M_data_ptr(NULL), _M_texture_ptr(NULL), _M_most_detailed_mipmap_level(0), _M_view_mipmap_levels(0)
        {
            // Enables move constructor
        }

        // Constructor for the texture
        _Texture_descriptor(unsigned int _Most_detailed_mipmap_level, unsigned int _View_mipmap_levels) __GPU
            : _M_data_ptr(NULL), _M_texture_ptr(NULL), _M_most_detailed_mipmap_level(_Most_detailed_mipmap_level), _M_view_mipmap_levels(_View_mipmap_levels)
        {
        }

        // Constructor for the interop texture
        _Texture_descriptor(_In_ _Texture * _Texture_ptr) __CPU_ONLY
            : _M_data_ptr(NULL), _M_texture_ptr(NULL), _M_most_detailed_mipmap_level(0)
        {
            _Set_texture_ptr(_Texture_ptr);

            // Adopt number of mipmap levels from underlying texture object
            _M_view_mipmap_levels = _Get_mipmap_levels(_M_texture_ptr);
        }

        // Destructor
        ~_Texture_descriptor() __GPU
        {
            _Set_texture_ptr(NULL);
        }

        // Copy constructor
        _Texture_descriptor(const _Texture_descriptor &_Other) __GPU
            : _M_data_ptr(_Other._M_data_ptr), _M_texture_ptr(NULL),
              _M_most_detailed_mipmap_level(_Other._M_most_detailed_mipmap_level), _M_view_mipmap_levels(_Other._M_view_mipmap_levels)
        {
            _Set_texture_ptr(_Other._M_texture_ptr);
        }

        // Copy constructor with ability to redefine mipmap information
        _Texture_descriptor(const _Texture_descriptor &_Other, unsigned int _Most_detailed_mipmap_level, unsigned int _View_mipmap_levels) __GPU
            : _M_data_ptr(_Other._M_data_ptr), _M_texture_ptr(NULL),
              _M_most_detailed_mipmap_level(_Most_detailed_mipmap_level), _M_view_mipmap_levels(_View_mipmap_levels)
        {
            _Set_texture_ptr(_Other._M_texture_ptr);
        }

        // Assignment operator
        _Texture_descriptor& operator=(const _Texture_descriptor &_Other) __GPU
        {
            if (this != &_Other)
            {
                _M_data_ptr = _Other._M_data_ptr;
                _Set_texture_ptr(_Other._M_texture_ptr);
                _M_most_detailed_mipmap_level = _Other._M_most_detailed_mipmap_level;
                _M_view_mipmap_levels = _Other._M_view_mipmap_levels;
            }

            return *this;
        }

        // Move constructor
        _Texture_descriptor(_Texture_descriptor &&_Other) __CPU_ONLY noexcept
        {
            *this = std::move(_Other);
        }

        bool operator==(const _Texture_descriptor &_Other) const __GPU
        {
            return _M_texture_ptr == _Other._M_texture_ptr
                && _M_data_ptr == _Other._M_data_ptr
                && _M_most_detailed_mipmap_level == _Other._M_most_detailed_mipmap_level
                && _M_view_mipmap_levels == _Other._M_view_mipmap_levels;
        }

        _Ret_ _Texture* _Get_texture_ptr() const __CPU_ONLY
        {
            _ASSERTE(_M_texture_ptr);
            return _M_texture_ptr;
        }

        unsigned int _Get_most_detailed_mipmap_level() const __GPU
        {
            return _M_most_detailed_mipmap_level;
        }

        unsigned int _Get_view_mipmap_levels() const __GPU
        {
            return _M_view_mipmap_levels;
        }

        void _Set_view_mipmap_levels(unsigned int _View_mipmap_levels) __CPU_ONLY
        {
            _M_view_mipmap_levels = _View_mipmap_levels;
        }

        void _Set_texture_ptr(_In_opt_ _Texture *_Texture_ptr) __CPU_ONLY
        {
            if (_M_texture_ptr != _Texture_ptr)
            {
                if (_M_texture_ptr != NULL) {
                    reinterpret_cast<_Reference_counter*>(_M_texture_ptr)->_Remove_reference();
                }

                _M_texture_ptr = _Texture_ptr;

                if (_M_texture_ptr != NULL) {
                    reinterpret_cast<_Reference_counter*>(_M_texture_ptr)->_Add_reference();
                }
            }
        }

#if !defined(_CXXAMP)
        void _Set_texture_ptr(_In_opt_ _Texture *) __GPU_ONLY
        {
            // No need to set the texture ptr on the GPU
            _M_texture_ptr = NULL;
        }
#endif // _CXXAMP

        // This helper function is used to determine aliasing and copy violations
        bool _Are_mipmap_levels_overlapping(const _Texture_descriptor *_Other) const __CPU_ONLY
        {
            _ASSERTE(_Other);

            if (this->_Get_texture_ptr() != _Other->_Get_texture_ptr())
            {
                return false;
            }

            return !((_M_most_detailed_mipmap_level < _Other->_M_most_detailed_mipmap_level) ? ((_M_most_detailed_mipmap_level + _M_view_mipmap_levels - 1) < _Other->_M_most_detailed_mipmap_level)
                                                                                             : ((_Other->_M_most_detailed_mipmap_level + _Other->_M_view_mipmap_levels - 1) < _M_most_detailed_mipmap_level));
        }

    } _Texture_descriptor;

    // Caution: Do not change this structure definition.
    // This struct is special and is processed by the FE to identify the samplers
    // used in a parallel_for_each.
    typedef struct _Sampler_descriptor
    {
        // _M_data_ptr points to the sampler on accelerator
        mutable void *_M_data_ptr;

    private:
        // _M_sampler_ptr points to a _Sampler that holds the underlying sampler
        // representation. This is private to ensure that all assignments to this data member
        // only happen through public functions which properly manage the
        // ref count of the underlying _Sampler object.
        _Sampler *_M_sampler_ptr;

    public:
        // Public functions

        // Default constructor
        _Sampler_descriptor() __GPU
            : _M_data_ptr(NULL), _M_sampler_ptr(NULL)
        {
        }

        _Sampler_descriptor(_In_ _Sampler * _Sampler_ptr) __GPU
            : _M_data_ptr(NULL), _M_sampler_ptr(NULL)
        {
            _Set_sampler_ptr(_Sampler_ptr);
        }

        // Destructor
        ~_Sampler_descriptor() __GPU
        {
            _Set_sampler_ptr(NULL);
        }

        // Copy constructor
        _Sampler_descriptor(const _Sampler_descriptor &_Other) __GPU
            : _M_data_ptr(_Other._M_data_ptr), _M_sampler_ptr(NULL)
        {
            _Set_sampler_ptr(_Other._M_sampler_ptr);
        }

        // Assignment operator
        _Sampler_descriptor& operator=(const _Sampler_descriptor &_Other) __GPU
        {
            if (this != &_Other)
            {
                _M_data_ptr = _Other._M_data_ptr;
                _Set_sampler_ptr(_Other._M_sampler_ptr);
            }

            return *this;
        }

        // Move constructor
        _Sampler_descriptor(_Sampler_descriptor &&_Other) __CPU_ONLY noexcept
        {
            *this = std::move(_Other);
        }

        bool operator==(const _Sampler_descriptor &_Other) const __GPU
        {
            return _M_sampler_ptr == _Other._M_sampler_ptr && _M_data_ptr == _Other._M_data_ptr;
        }

        _Ret_ _Sampler* _Get_sampler_ptr() const __CPU_ONLY
        {
            return _M_sampler_ptr;
        }

        void _Set_sampler_ptr(_In_opt_ _Sampler *_Sampler_ptr) __CPU_ONLY
        {
            if (_M_sampler_ptr != _Sampler_ptr)
            {
                if (_M_sampler_ptr != NULL) {
                    reinterpret_cast<_Reference_counter*>(_M_sampler_ptr)->_Remove_reference();
                }

                _M_sampler_ptr = _Sampler_ptr;

                if (_M_sampler_ptr != NULL) {
                    reinterpret_cast<_Reference_counter*>(_M_sampler_ptr)->_Add_reference();
                }
            }
        }

#if !defined(_CXXAMP)
        void _Set_sampler_ptr(_In_opt_ _Sampler *) __GPU_ONLY
        {
            // No need to set the sampler ptr on the GPU
            _M_sampler_ptr = NULL;
        }
#endif // _CXXAMP

    } _Sampler_descriptor;

} // namespace Concurrency::details

// Forward declaration
class accelerator;

namespace details
{
    _AMPIMP size_t __cdecl _Get_num_devices();
    _AMPIMP _Ret_ _Accelerator_impl_ptr * __cdecl _Get_devices();
    _AMPIMP accelerator __cdecl _Select_default_accelerator();
    _AMPIMP bool __cdecl _Set_default_accelerator(_Accelerator_impl_ptr _Accl);
    _AMPIMP bool __cdecl _Is_D3D_accelerator_view(const accelerator_view& _Av);
    _AMPIMP void __cdecl _Register_async_event(const _Event &_Ev, const std::shared_future<void> &_Shared_future);
    _AMPIMP _Access_mode __cdecl _Get_recommended_buffer_host_access_mode(const accelerator_view &_Av);
}

/// <summary>
///    Queuing modes supported for accelerator views
/// </summary>
enum queuing_mode {
    queuing_mode_immediate,
    queuing_mode_automatic
};

namespace direct3d
{
    /// <summary>
    ///     Get the D3D device interface underlying a accelerator_view.
    /// </summary>
    /// <param name="_Av">
    ///     The D3D accelerator_view for which the underlying D3D device interface is returned.
    /// </param>
    /// <returns>
    ///     The IUnknown interface pointer of the D3D device underlying the accelerator_view.
    /// </returns>
    _AMPIMP _Ret_ IUnknown * __cdecl get_device(const accelerator_view &_Av);

    /// <summary>
    ///     Create a accelerator_view from a D3D device interface pointer.
    /// </summary>
    /// <param name="_D3D_device">
    ///     The D3D device interface pointer to create the accelerator_view from.
    /// </param>
    /// <param name="_Qmode">
    ///     The queuing_mode to be used for the newly created accelerator_view.
    ///     This parameter has a default value of queuing_mode_automatic.
    /// </param>
    /// <returns>
    ///     The accelerator_view created from the passed D3D device interface.
    /// </returns>
    _AMPIMP accelerator_view __cdecl create_accelerator_view(_In_ IUnknown *_D3D_device, queuing_mode _Qmode = queuing_mode_automatic);

    /// <summary>
    ///     Create and return a new accelerator view on the specified accelerator.
    /// </summary>
    /// <param name="_Accelerator">
    ///     The accelerator on which the new accelerator_view is to be created.
    /// </param>
    /// <param name="_Disable_timeout">
    ///     A boolean parameter that specifies whether timeout should be disabled
    ///     for the newly created accelerator_view. This corresponds to the
    ///     D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT flag for Direct3D device creation
    ///     and is used to indicate if the operating system should allow workloads
    ///     that take more than 2 seconds to execute, without resetting the device
    ///     per the Windows timeout detection and recovery mechanism. Use of this flag
    ///     is recommended if you need to perform time consuming tasks on the accelerator_view.
    /// </param>
    /// <param name="_Qmode">
    ///     The queuing_mode to be used for the newly created accelerator_view.
    ///     This parameter has a default value of queuing_mode_automatic.
    /// </param>
    /// <returns>
    ///     The newly created accelerator_view.
    /// </returns>
    _AMPIMP accelerator_view __cdecl create_accelerator_view(accelerator& _Accelerator, bool _Disable_timeout, queuing_mode _Qmode = queuing_mode_automatic);

    /// <summary>
    ///     Returns a boolean flag indicating if timeout is disabled
    ///     for the specified accelerator_view. This corresponds to the
    ///     D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT flag for Direct3D device creation.
    /// </summary>
    /// <param name="_Accelerator_view">
    ///     The accelerator_view for which the timeout disabled setting is to be queried.
    /// </param>
    /// <returns>
    ///     A boolean flag indicating if timeout is disabled for the specified accelerator_view.
    /// </returns>
    _AMPIMP bool __cdecl is_timeout_disabled(const accelerator_view& _Accelerator_view);

    /// <summary>
    ///     Acquire a lock on an accelerator_view for the purpose of safely performing D3D operations on resources shared
    ///     with the accelerator_view.  The accelerator_view and all C++ AMP resources associated with this accelerator_view
    ///     internally take this lock when performing operations and will block while another thread holds the D3D access lock.
    ///
    ///     This lock is non-recursive: It is undefined behavior to call this function from a thread that already holds the lock.
    ///     It is undefined behavior to perform operations on the accelerator_view or any data container associated with the
    ///     accelerator_view from the thread that holds the D3D access lock.
    ///
    ///     See also scoped_d3d_access_lock, a RAII-style class for a scope-based D3D access lock.
    /// </summary>
    /// <param name="_Av">
    ///     The accelerator_view to lock.
    /// </param>
    _AMPIMP void __cdecl d3d_access_lock(accelerator_view &_Av);

    /// <summary>
    ///     Attempt to acquire the D3D access lock on an accelerator_view without blocking.
    /// </summary>
    /// <param name="_Av">
    ///     The accelerator_view to lock.
    /// </param>
    /// <returns>
    ///     true if the lock was acquired, or false if it is currently held by another thread.
    /// </returns>
    _AMPIMP bool __cdecl d3d_access_try_lock(accelerator_view &_Av);

    /// <summary>
    ///     Release the D3D access lock on the given accelerator_view.  If the calling thread does
    ///     not hold the lock on the accelerator_view the results are undefined.
    /// </summary>
    /// <param name="_Av">
    ///     The accelerator_view for which the lock is to be released.
    /// </param>
    _AMPIMP void __cdecl d3d_access_unlock(accelerator_view &_Av);

    /// <summary>
    ///     Tag type to indicate the D3D access lock should be adopted rather than
    ///     acquired.
    /// </summary>
    struct adopt_d3d_access_lock_t {};

    /// <summary>
    ///  RAII wrapper for a D3D access lock on an accelerator_view.
    /// </summary>
    class scoped_d3d_access_lock
    {
    public:
        /// <summary>
        ///     Acquire a D3D access lock on the given accelerator_view.  The lock is released
        ///     when this object goes out of scope.  Construction will block until the lock
        ///     is acquired.
        /// </summary>
        /// <param name="_Av">
        ///     The accelerator_view to lock.
        /// </param>
        _AMPIMP explicit scoped_d3d_access_lock(accelerator_view &_Av);

        /// <summary>
        ///     Construct a scoped_d3d_access_lock on an accelerator_view for which the lock
        ///     is already held (e.g. was acquired by d3d_access_try_lock).  The D3D access
        ///     lock must already be held by the calling thread and not controlled by any other
        ///     scoped_d3d_access_lock.
        /// </summary>
        /// <param name="_Av">
        ///     The accelerator_view for the lock to adopt.
        /// </param>
        /// <param name="_T">
        ///     The adopt_d3d_access_lock object.
        /// </param>
        _AMPIMP explicit scoped_d3d_access_lock(accelerator_view &_Av, adopt_d3d_access_lock_t _T);

        /// <summary>
        ///     Destructor for scoped_d3d_access_lock: unlock the accelerator_view.
        /// </summary>
        _AMPIMP ~scoped_d3d_access_lock();

        /// <summary>
        ///     Move constructor for scoped_d3d_access_lock:  Take ownership of
        ///     a lock from another scoped_d3d_access_lock.
        /// </summary>
        /// <param name="_Other">
        ///     The accelerator_view from which to move.
        /// </param>
        _AMPIMP scoped_d3d_access_lock(scoped_d3d_access_lock &&_Other) noexcept;

        /// <summary>
        ///     Move assignment operator for scoped_d3d_access_lock:  Take ownership
        ///     of a lock from another scoped_d3d_access_lock, releasing the previous
        ///     lock.
        /// </summary>
        /// <param name="_Other">
        ///     The accelerator_view from which to move.
        /// </param>
        /// <returns>
        ///     A reference to this scoped_accelerator_view_lock.
        /// </returns>
        _AMPIMP scoped_d3d_access_lock& operator=(scoped_d3d_access_lock &&_Other) noexcept;

    private:
        // No copy constructor
        scoped_d3d_access_lock(const scoped_d3d_access_lock &_Other);

        // No assignment operator
        scoped_d3d_access_lock & operator=(const scoped_d3d_access_lock &_Other);

        _Accelerator_view_impl_ptr _M_impl;
    };
} // namespace direct3d

/// <summary>
///  Class represents a accelerator abstraction for C++ AMP data-parallel devices
/// </summary>
class accelerator
{
    friend class accelerator_view;

    friend class details::_Ubiquitous_buffer;

    friend _AMPIMP accelerator details::_Select_default_accelerator();

    _AMPIMP friend accelerator_view __cdecl direct3d::create_accelerator_view(accelerator& _Accelerator, bool _Disable_timeout, queuing_mode _Qmode /* = queuing_mode_automatic */);

    friend _Ret_ details::_Accelerator_impl* details::_Get_accelerator_impl_ptr(const accelerator& _Accl);

public:

    /// <summary>
    ///     String constant for default accelerator
    /// </summary>
    _AMPIMP static const wchar_t default_accelerator[];

    /// <summary>
    ///     String constant for cpu accelerator
    /// </summary>
    _AMPIMP static const wchar_t cpu_accelerator[];

    /// <summary>
    ///     String constant for direct3d WARP accelerator
    /// </summary>
    _AMPIMP static const wchar_t direct3d_warp[];

    /// <summary>
    ///     String constant for direct3d reference accelerator
    /// </summary>
    _AMPIMP static const wchar_t direct3d_ref[];

    /// <summary>
    ///     Construct a accelerator representing the default accelerator
    /// </summary>
    _AMPIMP accelerator();

    /// <summary>
    ///     Construct a accelerator representing the accelerator with the
    ///     specified device instance path
    /// </summary>
    explicit accelerator(const std::wstring &_Device_path) : _M_impl(NULL)
    {
        _Init(_Device_path.c_str());
    }

    /// <summary>
    ///     Destructor
    /// </summary>
    _AMPIMP ~accelerator();

    /// <summary>
    ///     Copy constructor
    /// </summary>
    _AMPIMP accelerator(const accelerator &_Other);

    /// <summary>
    ///     Assignment operator
    /// </summary>
    _AMPIMP accelerator &operator=(const accelerator &_Other);

    /// <summary>
    ///     Returns the vector of accelerator objects representing all available accelerators
    /// </summary>
    /// <returns>
    ///     The vector of available accelerators
    /// </returns>
    static inline std::vector<accelerator> get_all()
    {
        std::vector<accelerator> _AcceleratorVector;
        size_t _NumDevices = details::_Get_num_devices();
        for (size_t _I = 0; (_I < _NumDevices); ++_I)
        {
            _AcceleratorVector.push_back(details::_Get_devices()[_I]);
        }

        return _AcceleratorVector;
    }

    /// <summary>
    ///     Sets the default accelerator to be used for any operation
    ///     that implicitly uses the default accelerator. This method
    ///     only succeeds if the runtime selected default accelerator
    ///     has not already been used in an operation that implicitly
    ///     uses the default accelerator
    /// </summary>
    /// <returns>
    ///     A boolean value indicating if the call succeeds in setting
    ///     the default accelerator
    /// </returns>
    static inline bool set_default(const std::wstring& _Path)
    {
        accelerator _Accl(_Path);
        return details::_Set_default_accelerator(_Accl._M_impl);
    }

    /// <summary>
    ///     Returns the auto selection accelerator_view which when specified
    ///     as the parallel_for_each target results in the target accelerator_view
    ///     for executing the parallel_for_each kernel to be automatically selected
    ///     by the runtime. For all other purposes, the accelerator_view returned
    ///     by this method is the same as the default accelerator_view of the default
    ///     accelerator
    /// </summary>
    _AMPIMP static accelerator_view __cdecl get_auto_selection_view();

    /// <summary>
    ///     Returns the system-wide unique device instance path as a std::wstring
    /// </summary>
    std::wstring get_device_path() const
    {
        return _Get_device_path();
    }

    __declspec(property(get=get_device_path)) std::wstring device_path;

    /// <summary>
    ///     Get the version for this accelerator
    /// </summary>
    _AMPIMP unsigned int get_version() const;
    __declspec(property(get=get_version)) unsigned int version; // hiword=major, loword=minor

    /// <summary>
    ///     Returns the device description as a std::wstring
    /// </summary>
    std::wstring get_description() const
    {
        return _Get_description();
    }

    __declspec(property(get=get_description)) std::wstring description;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator
    ///     was created with DEBUG layer enabled for extensive error reporting
    /// </summary>
    _AMPIMP bool get_is_debug() const;
    __declspec(property(get=get_is_debug)) bool is_debug;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator is emulated.
    ///     This is true, for example, with the direct3d reference and WARP accelerators.
    /// </summary>
    _AMPIMP bool get_is_emulated() const;
    __declspec(property(get=get_is_emulated)) bool is_emulated;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator
    ///     is attached to a display
    /// </summary>
    _AMPIMP bool get_has_display() const;
    __declspec(property(get=get_has_display)) bool has_display;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator
    ///     supports full double precision (including double division,
    ///     precise_math functions, int to double, double to int conversions)
    ///     in a parallel_for_each kernel.
    /// </summary>
    _AMPIMP bool get_supports_double_precision() const;
    __declspec(property(get=get_supports_double_precision)) bool supports_double_precision;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator
    ///     has limited double precision support (excludes double division,
    ///     precise_math functions, int to double, double to int conversions)
    ///     for a parallel_for_each kernel.
    /// </summary>
    _AMPIMP bool get_supports_limited_double_precision() const;
    __declspec(property(get=get_supports_limited_double_precision)) bool supports_limited_double_precision;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator
    ///     supports memory accessible both by the accelerator and the CPU.
    /// </summary>
    _AMPIMP bool get_supports_cpu_shared_memory() const;
    __declspec(property(get=get_supports_cpu_shared_memory)) bool supports_cpu_shared_memory;

    /// <summary>
    ///     Return the default accelerator view associated with this accelerator
    /// </summary>
    _AMPIMP accelerator_view get_default_view() const;
    __declspec(property(get=get_default_view)) accelerator_view default_view;

    /// <summary>
    ///     Get the dedicated memory for this accelerator in KB
    /// </summary>
    _AMPIMP size_t get_dedicated_memory() const;
    __declspec(property(get=get_dedicated_memory)) size_t dedicated_memory;

    /// <summary>
    ///     Get the default cpu access_type for buffers created on this accelerator
    /// </summary>
    _AMPIMP access_type get_default_cpu_access_type() const;
    __declspec(property(get=get_default_cpu_access_type)) access_type default_cpu_access_type;

    /// <summary>
    ///     Set the default cpu access_type for arrays created on this accelerator
    ///     or for implicit memory allocations as part of array_views accessed
    ///     on this accelerator. This method only succeeds if the default_cpu_access_type
    ///     for the accelerator has not already been overridden by a previous call to this method
    ///     and the runtime selected default_cpu_access_type for this accelerator has not yet
    ///     been used for allocating an array or for an implicit memory allocation backing an
    ///     array_view accessed on this accelerator.
    /// </summary>
    /// <param name="_Default_cpu_access_type">
    ///     The default cpu access_type to be used for array/array_view memory allocations
    ///     on this accelerator.
    /// </param>
    /// <returns>
    ///     A boolean value indicating if the default cpu access_type for the accelerator
    ///     was successfully set.
    /// </returns>
    _AMPIMP bool set_default_cpu_access_type(access_type _Default_cpu_access_type);

    /// <summary>
    ///     Create and return a new accelerator view on this accelerator
    ///     with the specified queuing mode. When unspecified the accelerator_view
    ///     is created with queuing_mode_automatic queuing mode.
    /// </summary>
    _AMPIMP accelerator_view create_view(queuing_mode qmode = queuing_mode_automatic);

    /// <summary>
    ///     Return true if the other accelerator is same as this accelerator; false otherwise
    /// </summary>
    _AMPIMP bool operator==(const accelerator &_Other) const;

    /// <summary>
    ///     Return false if the other accelerator is same as this accelerator; true otherwise
    /// </summary>
    _AMPIMP bool operator!=(const accelerator &_Other) const;

private:

    // Private constructor
    _AMPIMP accelerator(_Accelerator_impl_ptr _Impl);

    // Private helper methods
    _AMPIMP const wchar_t *_Get_device_path() const;
    _AMPIMP const wchar_t *_Get_description() const;

    _AMPIMP void _Init(const wchar_t *_Path);

private:

    _Accelerator_impl_ptr _M_impl;
};

/// <summary>
///  Class represents a future corresponding to a C++ AMP asynchronous operation
/// </summary>
class completion_future
{
    friend class details::_Amp_runtime_trace;
public:

    /// <summary>
    ///     Default constructor
    /// </summary>
    completion_future()
    {
    }

    /// <summary>
    ///     Copy constructor
    /// </summary>
    completion_future(const completion_future& _Other)
        : _M_shared_future(_Other._M_shared_future),
        _M_task(_Other._M_task)
    {
    }

    /// <summary>
    ///     Move constructor
    /// </summary>
    completion_future(completion_future&& _Other) noexcept
        : _M_shared_future(std::move(_Other._M_shared_future)),
        _M_task(std::move(_Other._M_task))
    {
    }

    /// <summary>
    ///     Destructor
    /// </summary>
    ~completion_future()
    {
    }

    /// <summary>
    ///     Copy assignment operator
    /// </summary>
    completion_future& operator=(const completion_future& _Other)
    {
        if (this != &_Other) {
            _M_shared_future = _Other._M_shared_future;
            _M_task = _Other._M_task;
        }

        return (*this);
    }

    /// <summary>
    ///     Move assignment operator
    /// </summary>
    completion_future& operator=(completion_future&& _Other) noexcept
    {
        if (this != &_Other) {
            _M_shared_future = std::move(_Other._M_shared_future);
            _M_task = std::move(_Other._M_task);
        }

        return (*this);
    }

    /// <summary>
    ///     Waits until the associated asynchronous operation completes
    ///     Throws the stored exception if one was encountered during the
    ///     asynchronous operation
    /// </summary>
    void get() const
    {
        _M_shared_future.get();
    }

    /// <summary>
    ///     Returns true if the object is associated with an asynchronous
    ///     operation
    /// </summary>
    /// <returns>
    ///     true if the object is associated with an asynchronous operation
    ///     and false otherwise
    /// </returns>
    bool valid() const
    {
        return _M_shared_future.valid();
    }

    /// <summary>
    ///     Blocks until the associated asynchronous operation completes
    /// </summary>
    void wait() const
    {
        _M_shared_future.wait();
    }

    /// <summary>
    ///     Blocks until the associated asynchronous operation completes or
    ///     _Rel_time has elapsed
    /// </summary>
    /// <returns>
    ///     - future_status::deferred if the associated asynchronous operation is not running
    ///     - future_status::ready if the associated asynchronous operation is finished
    ///     - future_status::timeout if the time period specified has elapsed
    /// </returns>
    template <class _Rep, class _Period>
    std::future_status wait_for(const std::chrono::duration<_Rep, _Period>& _Rel_time) const
    {
        return _M_shared_future.wait_for(_Rel_time);
    }

    /// <summary>
    ///     Blocks until the associated asynchronous operation completes or
    ///     until the current time exceeds _Abs_time
    /// </summary>
    /// <returns>
    ///     - future_status::deferred if the associated asynchronous operation is not running
    ///     - future_status::ready if the associated asynchronous operation is finished
    ///     - future_status::timeout if the time point specified has been reached
    /// </returns>
    template <class _Clock, class _Duration>
    std::future_status wait_until(const std::chrono::time_point<_Clock, _Duration>& _Abs_time) const
    {
        return _M_shared_future.wait_until(_Abs_time);
    }

    /// <summary>
    ///     Returns a std::shared_future&lt;void&gt; object corresponding to the
    ///     associated asynchronous operation
    /// </summary>
    /// <returns>
    ///     A std::shared_future&lt;void&gt; object corresponding to the associated
    ///     asynchronous operation
    /// </returns>
    operator std::shared_future<void>() const
    {
        return _M_shared_future;
    }

    /// <summary>
    ///     Chains a callback Functor to the completion_future to be executed
    ///     when the associated asynchronous operation finishes execution
    /// </summary>
    template <typename _Functor>
    void then(const _Functor &_Func) const
    {
        this->to_task().then(_Func);
    }

    /// <summary>
    ///     Returns a concurrency::task&lt;void&gt; object corresponding to the
    ///     associated asynchronous operation
    /// </summary>
    /// <returns>
    ///     A concurrency::task&lt;void&gt; object corresponding to the associated
    ///     asynchronous operation
    /// </returns>
    concurrency::task<void> to_task() const
    {
        return _M_task;
    }

private:

    // Private constructor
    completion_future(const std::shared_future<void> &_Shared_future,
                      const concurrency::task<void>& _Task)
                      : _M_shared_future(_Shared_future), _M_task(_Task)
    {
    }

    std::shared_future<void> _M_shared_future;
    concurrency::task<void> _M_task;
};

/// <summary>
///  Class represents a virtual device abstraction on a C++ AMP data-parallel accelerator
/// </summary>
class accelerator_view
{
    friend class accelerator;
    friend class details::_Buffer;
    friend class details::_Texture;
    friend class details::_Sampler;
    friend class details::_Ubiquitous_buffer;
    friend class details::_D3D_interop;
    friend class details::_D3D_accelerator_view_impl;
    friend class details::_CPU_accelerator_view_impl;
    friend class details::_Accelerator_view_hasher;

    _AMPIMP friend _Ret_ IUnknown * __cdecl direct3d::get_device(const accelerator_view &_Av);

    _AMPIMP friend accelerator_view __cdecl direct3d::create_accelerator_view(_In_ IUnknown *_D3D_device, queuing_mode qmode /* = queuing_mode_automatic */);

    _AMPIMP friend accelerator_view __cdecl direct3d::create_accelerator_view(accelerator& _Accelerator, bool _Disable_timeout, queuing_mode _Qmode /* = queuing_mode_automatic */);

    _AMPIMP friend bool __cdecl direct3d::is_timeout_disabled(const accelerator_view& _Accelerator_view);

    friend _Ret_ details::_Accelerator_view_impl* details::_Get_accelerator_view_impl_ptr(const accelerator_view& _Accl_view);

public:

    /// <summary>
    ///     Destructor
    /// </summary>
    _AMPIMP ~accelerator_view();

    /// <summary>
    ///     Copy constructor
    /// </summary>
    _AMPIMP accelerator_view(const accelerator_view &_Other);

    /// <summary>
    ///     Assignment operator
    /// </summary>
    _AMPIMP accelerator_view &operator=(const accelerator_view &_Other);

    /// <summary>
    ///     Get the accelerator for this accelerator view
    /// </summary>
    _AMPIMP accelerator get_accelerator() const;
    __declspec(property(get=get_accelerator)) Concurrency::accelerator accelerator;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator view
    ///     was created with DEBUG layer enabled for extensive error reporting
    /// </summary>
    _AMPIMP bool get_is_debug() const;
    __declspec(property(get=get_is_debug)) bool is_debug;

    /// <summary>
    ///     Get the version for this accelerator view
    /// </summary>
    _AMPIMP unsigned int get_version() const;
    __declspec(property(get=get_version)) unsigned int version; // hiword=major, loword=minor

    /// <summary>
    ///     Get the queuing mode for this accelerator view
    /// </summary>
    _AMPIMP queuing_mode get_queuing_mode() const;
    __declspec(property(get=get_queuing_mode)) Concurrency::queuing_mode queuing_mode;

    /// <summary>
    ///     Returns a boolean value indicating whether the accelerator view
    ///     when passed to a parallel_for_each would result in automatic
    ///     selection of an appropriate execution target by the runtime
    /// </summary>
    _AMPIMP bool get_is_auto_selection() const;
    __declspec(property(get=get_is_auto_selection)) bool is_auto_selection;

    /// <summary>
    ///     Return true if the other accelerator view is same as this accelerator view; false otherwise
    /// </summary>
    _AMPIMP bool operator==(const accelerator_view &_Other) const;

    /// <summary>
    ///     Return false if the other accelerator view is same as this accelerator view; true otherwise
    /// </summary>
    _AMPIMP bool operator!=(const accelerator_view &_Other) const;

    /// <summary>
    ///     Waits for completion of all commands submitted so far to this accelerator_view
    /// </summary>
    _AMPIMP void wait();

    /// <summary>
    ///     Submit all pending commands queued to this accelerator_view to the accelerator
    ///     for execution.
    /// </summary>
    _AMPIMP void flush();

    /// <summary>
    ///     Return a future to track the completion of all commands submitted so far to this accelerator_view
    /// </summary>
    _AMPIMP concurrency::completion_future create_marker();

private:

    // No default constructor
    accelerator_view();

    // Private constructor
    _AMPIMP accelerator_view(_Accelerator_view_impl_ptr _Impl, bool _Auto_selection = false);

private:

    _Accelerator_view_impl_ptr _M_impl;
    bool _M_auto_selection;
};

namespace details
{
    inline _Ret_ _Accelerator_view_impl* _Get_accelerator_view_impl_ptr(const accelerator_view& _Accl_view)
    {
        return _Accl_view._M_impl;
    }

    inline _Ret_ _Accelerator_impl* _Get_accelerator_impl_ptr(const accelerator& _Accl)
    {
         return _Accl._M_impl;
    }

    // Type defining a hasher for accelerator_view objects
    // for use with std::unordered_set and std::unordered_map
    class _Accelerator_view_hasher
    {
    public:
        size_t operator()(const accelerator_view &_Accl_view) const
        {
            std::hash<_Accelerator_view_impl*> _HashFunctor;
            return _HashFunctor(_Accl_view._M_impl._Get_ptr());
        }
    };

    typedef std::unordered_set<accelerator_view, _Accelerator_view_hasher> _Accelerator_view_unordered_set;

    // Describes the N dimensional shape of a view in a buffer
    class _View_shape : public _Reference_counter
    {
    public:

        _AMPIMP static _Ret_ _View_shape* __cdecl _Create_view_shape(unsigned int _Rank, unsigned int _Linear_offset,
                                                                     const unsigned int *_Base_extent, const unsigned int *_View_offset,
                                                                     const unsigned int *_View_extent, const bool *_Projection_info = NULL);

        _AMPIMP _Ret_ _View_shape* _Get_reduced_shape_for_copy();

        inline unsigned int _Get_rank() const
        {
            return _M_rank;
        }

        inline unsigned int _Get_linear_offset() const
        {
            return _M_linear_offset;
        }

        inline const unsigned int *_Get_base_extent() const
        {
            return _M_base_extent;
        }

        inline const unsigned int *_Get_view_offset() const
        {
            return _M_view_offset;
        }
        inline const unsigned int *_Get_view_extent() const
        {
            return _M_view_extent;
        }

        inline const bool *_Get_projection_info() const
        {
            return _M_projection_info;
        }

        inline bool _Is_projection() const
        {
            return _M_projection_info[0];
        }

        inline bool _Is_valid(size_t _Buffer_size) const
        {
            // The end point of the base shape should not be greater than the size of the buffer
            size_t endLinearOffset = _M_linear_offset + _Get_extent_size(_M_rank, _M_base_extent);
            if (endLinearOffset > _Buffer_size) {
                return false;
            }

            return _Is_valid();
        }

        inline unsigned int _Get_view_size() const
        {
            return _Get_extent_size(_M_rank, _M_view_extent);
        }

        inline unsigned int _Get_view_linear_offset() const
        {
            return _Get_linear_offset(_M_view_offset);
        }

        static inline bool
        _Compare_extent_with_elem_size(unsigned int _Rank, const unsigned int *_Extent1, size_t _Elem_size1, const unsigned int *_Extent2, size_t _Elem_size2)
        {
            _ASSERTE((_Rank >= 1) && (_Extent1 != NULL)&& (_Extent2 != NULL));

            // The extents should match accounting for the element sizes of the respective buffers
            if ((_Extent1[_Rank - 1] * _Elem_size1) != (_Extent2[_Rank - 1] * _Elem_size2))
            {
                return false;
            }

            // Now compare the extent in all but the least significant dimension
            if ((_Rank > 1) && !_Compare_extent(_Rank - 1, _Extent1, _Extent2))
            {
                return false;
            }

            return true;
        }


        static inline bool
        _Compare_extent(unsigned int _Rank, const unsigned int *_Extent1, const unsigned int *_Extent2)
        {
            for (size_t _I = 0; _I < _Rank; ++_I) {
                if (_Extent1[_I] != _Extent2[_I]) {
                    return false;
                }
            }

            return true;
        }

        inline bool _Is_view_linear(unsigned int &_Linear_offset, unsigned int &_Linear_size) const
        {
            // The effective rank for the purpose of determining linearity
            // depends on the highest dimension in which the extent is not 1
            unsigned int _First_dim_with_non_unit_extent = 0;
            while ((_First_dim_with_non_unit_extent < _M_rank) && (_M_view_extent[_First_dim_with_non_unit_extent] == 1)) {
                _First_dim_with_non_unit_extent++;
            }

            unsigned int _Effective_rank = (_M_rank - _First_dim_with_non_unit_extent);

            // It is linear if the effective rank is <= 1 or the base extent
            // and view extent are same in all but the highest dimension with
            // non-unit extent
            if ((_Effective_rank <= 1) ||
                (_Compare_extent(_Effective_rank - 1, &_M_base_extent[_First_dim_with_non_unit_extent + 1], &_M_view_extent[_First_dim_with_non_unit_extent + 1])))
            {
                _Linear_offset = _Get_view_linear_offset();
                _Linear_size = _Get_view_size();
                return true;
            }

            return false;
        }

        inline bool _Overlaps(const _View_shape* _Other) const
        {
            if (_Compare_base_shape(_Other))
            {
                // If the base shapes are identical we will do the N-dimensional
                // bounding box overlap test

                for (size_t _I = 0; _I < _M_rank; ++_I)
                {
                    if (!_Intervals_overlap(_M_view_offset[_I], _M_view_offset[_I] + _M_view_extent[_I] - 1,
                                            _Other->_M_view_offset[_I], _Other->_M_view_offset[_I] + _Other->_M_view_extent[_I] - 1))
                    {
                        return false;
                    }
                }

                return true;
            }
            else
            {
                // The base shapes are different. Check based on linear intervals
                size_t firstStart = _Get_view_linear_offset();
                size_t firstEnd = firstStart + _Get_view_size() - 1;

                size_t secondStart = _Other->_Get_view_linear_offset();
                size_t secondEnd = secondStart + _Other->_Get_view_size() - 1;

                return _Intervals_overlap(firstStart, firstEnd, secondStart, secondEnd);
            }
        }

        inline bool _Subsumes(const _View_shape* _Other) const
        {
            // Subsumption test can only be done for shapes that have the same base shape or
            // when both have a rank of 1
            if ((_M_rank == 1) && (_Other->_Get_rank() == 1))
            {
                size_t thisStart = _Get_view_linear_offset();
                size_t thisEnd = thisStart + _Get_view_size() - 1;

                size_t otherStart = _Other->_Get_view_linear_offset();
                size_t otherEnd = otherStart + _Other->_Get_view_size() - 1;

                return ((otherStart >= thisStart) && (otherEnd <= thisEnd));
            }

            if (!_Compare_base_shape(_Other)) {
                return false;
            }

            if (!_Contains(_Other->_Get_view_offset())) {
                return false;
            }

            std::vector<unsigned int> otherEndPointIndex(_M_rank);
            for (size_t _I = 0; _I < _M_rank; ++_I) {
                otherEndPointIndex[_I] = _Other->_Get_view_offset()[_I] + _Other->_Get_view_extent()[_I] - 1;
            }

            return _Contains(otherEndPointIndex.data());
        }

    private:
        // Private constructor to force construction through the _Create_view_shape method
        _View_shape(unsigned int _Rank, unsigned int _Linear_offset,
                    const unsigned int *_Base_extent, const unsigned int *_View_offset,
                    const unsigned int *_View_extent, const bool *_Projection_info);

        virtual ~_View_shape();

        // No default constructor or copy/assignment
        _View_shape();
        _View_shape(const _View_shape &_Other);
        _View_shape(_View_shape &&_Other);
        _View_shape& operator=(const _View_shape &_Other);
        _View_shape& operator=(_View_shape &&_Other);

        // Helper methods
        static bool _Intervals_overlap(size_t _First_start, size_t _First_end,
                                       size_t _Second_start, size_t _Second_end)
        {
            // Order the intervals by their start points
            if (_First_start > _Second_start) {
                size_t temp = _First_start;
                _First_start = _Second_start;
                _Second_start = temp;

                temp = _First_end;
                _First_end = _Second_end;
                _Second_end = temp;
            }

            // The start of the second one must be within the bounds of the first one
            return (_Second_start <= _First_end);
        }

        static unsigned int _Get_extent_size(unsigned int _Rank, const unsigned int *_Extent)
        {
            unsigned int totalExtent = 1;
            for (size_t _I = 0; _I < _Rank; ++_I) {
                totalExtent *= _Extent[_I];
            }

            return totalExtent;
        }

        inline bool _Is_valid() const
        {
            if (_M_rank == 0) {
                return false;
            }

            // Ensure the _M_view_offset + _M_view_extent is within the bounds of _M_base_extent
            size_t viewSize = 1;

            for (size_t _I = 0; _I < _M_rank; ++_I)
            {
                viewSize *= _M_view_extent[_I];
                if ((_M_view_offset[_I] + _M_view_extent[_I]) > _M_base_extent[_I]) {
                    return false;
                }
            }

            if (viewSize == 0) {
                return false;
            }

            return true;
        }

        inline bool _Compare_base_shape(const _View_shape* _Other) const
        {
            return ((_M_rank == _Other->_M_rank) &&
                    (_M_linear_offset == _Other->_M_linear_offset) &&
                    _Compare_extent(_M_rank, _M_base_extent, _Other->_M_base_extent));
        }

        // Checks if the element at the specified index
        // is contained within this view shape
        // Assumes the rank of the index is same as the
        // rank of this view's shape
        inline bool _Contains(const unsigned int* _Element_index) const
        {
            for (size_t _I = 0; _I < _M_rank; ++_I)
            {
                if ((_Element_index[_I] < _M_view_offset[_I]) ||
                    (_Element_index[_I] >= (_M_view_offset[_I] + _M_view_extent[_I])))
                {
                    return false;
                }
            }

            return true;
        }

        inline unsigned int _Get_linear_offset(const unsigned int* _Element_index) const
        {
            unsigned int currMultiplier = 1;
            unsigned int linearOffset = _M_linear_offset;
            for (int _I = static_cast<int>(_M_rank - 1); _I >= 0; _I--)
            {
                linearOffset += (currMultiplier * _Element_index[_I]);
                currMultiplier *= _M_base_extent[_I];
            }

            return linearOffset;
        }

    private:

        unsigned int _M_rank;
        unsigned int _M_linear_offset;
        unsigned int *_M_base_extent;
        unsigned int *_M_view_offset;
        unsigned int *_M_view_extent;
        bool *_M_projection_info;
    };

    // This function creates a new _View_shape object from an existing _View_shape object when the data underlying the view
    // needs to be reinterpreted to use a different element size than the one used by the original view.
    inline
    _Ret_ _View_shape *_Create_reinterpreted_shape(const _View_shape* _Source_shape, size_t _Curr_elem_size, size_t _New_elem_size)
    {
        unsigned int _Rank = _Source_shape->_Get_rank();
        size_t _LinearOffsetInBytes = _Source_shape->_Get_linear_offset() * _Curr_elem_size;
        size_t _BaseLSDExtentInBytes = (_Source_shape->_Get_base_extent())[_Rank - 1] * _Curr_elem_size;
        size_t _ViewLSDOffsetInBytes = (_Source_shape->_Get_view_offset())[_Rank - 1] * _Curr_elem_size;
        size_t _ViewLSDExtentInBytes = (_Source_shape->_Get_view_extent())[_Rank - 1] * _Curr_elem_size;

        _ASSERTE((_LinearOffsetInBytes % _New_elem_size) == 0);
        _ASSERTE((_BaseLSDExtentInBytes % _New_elem_size) == 0);
        _ASSERTE((_ViewLSDOffsetInBytes % _New_elem_size) == 0);
        _ASSERTE((_ViewLSDExtentInBytes % _New_elem_size) == 0);

        size_t _Temp_val = _LinearOffsetInBytes / _New_elem_size;
        _ASSERTE(_Temp_val <= UINT_MAX);
        unsigned int _New_linear_offset = static_cast<unsigned int>(_Temp_val);

        std::vector<unsigned int> _New_base_extent(_Rank);
        std::vector<unsigned int> _New_view_offset(_Rank);
        std::vector<unsigned int> _New_view_extent(_Rank);
        for (unsigned int i = 0; i < _Rank - 1; ++i) {
            _New_base_extent[i] = (_Source_shape->_Get_base_extent())[i];
            _New_view_offset[i] = (_Source_shape->_Get_view_offset())[i];
            _New_view_extent[i] = (_Source_shape->_Get_view_extent())[i];
        }

        // The extent in the least significant dimension needs to be adjusted
        _Temp_val = _BaseLSDExtentInBytes / _New_elem_size;
        _ASSERTE(_Temp_val <= UINT_MAX);
        _New_base_extent[_Rank - 1] = static_cast<unsigned int>(_Temp_val);

        _Temp_val = _ViewLSDOffsetInBytes / _New_elem_size;
        _ASSERTE(_Temp_val <= UINT_MAX);
        _New_view_offset[_Rank - 1] = static_cast<unsigned int>(_Temp_val);

        _Temp_val = _ViewLSDExtentInBytes / _New_elem_size;
        _ASSERTE(_Temp_val <= UINT_MAX);
        _New_view_extent[_Rank - 1] = static_cast<unsigned int>(_Temp_val);

        return _View_shape::_Create_view_shape(_Rank, _New_linear_offset, _New_base_extent.data(), _New_view_offset.data(), _New_view_extent.data());
    }

    inline _Access_mode _Get_synchronize_access_mode(access_type cpu_access_type)
    {
        switch(cpu_access_type)
        {
        case access_type_auto:
        case access_type_read:
            return _Read_access;
        case access_type_write:
            return _Write_access;
        case access_type_read_write:
            return _Read_write_access;
        case access_type_none:
        default:
            _ASSERTE(false);
            return _No_access;
        }
    }

    inline access_type _Get_cpu_access_type(_Access_mode _Cpu_access_mode)
    {
        access_type _Cpu_access_type = access_type_none;
        if (_Cpu_access_mode & _Read_access) {
            _Cpu_access_type = static_cast<access_type>(_Cpu_access_type | access_type_read);
        }

        if (_Cpu_access_mode & _Write_access) {
            _Cpu_access_type = static_cast<access_type>(_Cpu_access_type | access_type_write);
        }

        return _Cpu_access_type;
    }

    //  Class manages a raw buffer in a accelerator view
    class _Buffer : public _Reference_counter
    {
        friend class _CPU_accelerator_view_impl;
        friend class _D3D_accelerator_view_impl;
        friend class _D3D_temp_staging_cache;

    public:

        // Force construction through these static public method to ensure that _Buffer
        // objects are allocated in the runtime

        // Allocate a new buffer on the specified accelerator_view
        _AMPIMP static _Ret_ _Buffer * __cdecl _Create_buffer(accelerator_view _Accelerator_view, accelerator_view _Access_on_accelerator_view, size_t _Num_elems,
                                                              size_t _Elem_size, bool _Is_temp = false, access_type _Cpu_access_type = access_type_auto);

        // Create a buffer object from a pre-allocated storage on the specified accelerator_view. This can be thought
        // of as the accelerator_view "adopting" the passed data buffer.
        _AMPIMP static _Ret_ _Buffer * __cdecl _Create_buffer(_In_ void *_Data_ptr, accelerator_view _Accelerator_view, size_t _Num_elems,
                                                              size_t _Elem_size);

        // Create a staging buffer on the specified accelerator_view which can be accessed on the cpu upon mapping.
        _AMPIMP static _Ret_ _Buffer * __cdecl _Create_stage_buffer(accelerator_view _Accelerator_view, accelerator_view _Access_on_accelerator_view,
                                                                    size_t _Num_elems, size_t _Elem_size, bool _Is_temp = false);

        // Creates a temp staging buffer of the requested size. This function may create
        // a staging buffer smaller than the requested size.
        _AMPIMP static _Ret_ _Buffer * __cdecl _Get_temp_staging_buffer(accelerator_view _Av, size_t _Requested_num_elems, size_t _Elem_size);

        // Map a zero-copy or staging buffer for access on the CPU.
        _AMPIMP void _Map_buffer(_Access_mode _Map_type, bool _Wait);

        // Asynchronously map a zero-copy or staging buffer for access on the CPU.
        _AMPIMP _Event _Map_buffer_async(_Access_mode _Map_type);

        // Unmap a zero-copy or staging buffer denying CPU access
        _AMPIMP void _Unmap_buffer();

        //  Copy data to _Dest asynchronously.
        _AMPIMP _Event _Copy_to_async(_Inout_ _Buffer * _Dest, size_t _Num_elems, size_t _Src_offset = 0, size_t _Dest_offset = 0);

        //  Copy data to _Dest asynchronously.
        _AMPIMP _Event _Copy_to_async(_Inout_ _Buffer * _Dest, _View_shape_ptr _Src_shape, _View_shape_ptr _Dest_shape);

        _AMPIMP accelerator_view _Get_accelerator_view() const;
        _AMPIMP accelerator_view _Get_access_on_accelerator_view() const;

        _AMPIMP void _Register_view(_In_ _View_key _Key);
        _AMPIMP void _Unregister_view(_In_ _View_key _Key);

        // Return the raw data ptr - only a accelerator view implementation can interpret
        // this raw pointer. This method should usually not be used in the AMP header files
        // The _Get_host_ptr is the right way for accessing the host accessible ptr for a buffer
        _Ret_ void * _Get_data_ptr() const
        {
            return _M_data_ptr;
        }

        // Returns the host accessible ptr corresponding to the buffer. This would
        // return NULL when the buffer is inaccessible on the CPU
        _Ret_ void * _Get_host_ptr() const
        {
            return _M_host_ptr;
        }

        size_t _Get_elem_size() const
        {
            return _M_elem_size;
        }

        size_t _Get_num_elems() const
        {
            return _M_num_elems;
        }

        _Ret_ _Accelerator_view_impl* _Get_accelerator_view_impl() const
        {
            return _M_accelerator_view;
        }

        _Ret_ _Accelerator_view_impl* _Get_access_on_accelerator_view_impl() const
        {
            return _M_access_on_accelerator_view;
        }

        bool _Owns_data() const
        {
            return _M_owns_data;
        }

        _AMPIMP bool _Exclusively_owns_data();

        bool _Is_staging() const
        {
            return _M_is_staging;
        }

        _Access_mode _Get_allowed_host_access_mode() const
        {
            return _M_allowed_host_access_mode;
        }

        access_type _Get_allowed_host_access_type() const
        {
            return _Get_cpu_access_type(_M_allowed_host_access_mode);
        }

        bool _Is_host_accessible(_Access_mode _Requested_access_mode) const
        {
            return ((_Get_allowed_host_access_mode() & _Requested_access_mode) == _Requested_access_mode);
        }

        _Access_mode _Get_current_host_access_mode() const
        {
            return _M_current_host_access_mode;
        }

        bool _Is_temp() const
        {
            return _M_is_temp;
        }

        bool _Is_adopted() const
        {
            // Is it adopted from interop?
            return _M_is_adopted;
        }

        bool _Is_buffer() const
        {
            return _M_is_buffer;
        }

		_AMPIMP bool _Is_mappable() const;

    protected:

        // The _Buffer constructor is protected to force construction through the static
        // _Create_buffer method to ensure the object is allocated in the runtime
        _Buffer(_In_ _Accelerator_view_impl* _Av, _Pre_writable_byte_size_(_Elem_size * _Num_elems) void *_Buffer_data_ptr,
                void * _Host_ptr, _Access_mode _Allowed_host_access_mode, _Access_mode _Current_host_access_mode,
                size_t _Num_elems, size_t _Elem_size, bool _Owns_data, bool _Is_staging, bool _Is_temp, bool _Is_adopted);

        // protected destructor to force deletion through _Release
        virtual ~_Buffer();

        // No default constructor, copy constructor and assignment operator
        _Buffer();
        _Buffer(const _Buffer &rhs);
        _Buffer &operator=(const _Buffer &rhs);

        void _Set_host_ptr(_In_opt_ void *_Host_ptr, _Access_mode _Host_access_mode = _No_access)
        {
            _ASSERTE((_Host_ptr == NULL) || (_Host_access_mode != _No_access));

            _M_host_ptr = _Host_ptr;
            if (_Host_ptr == NULL) {
                _M_current_host_access_mode = _No_access;
            }
            else {
                _M_current_host_access_mode = _Host_access_mode;
            }
        }

        void _Set_data_ptr(_In_opt_ IUnknown *_Data_ptr)
        {
            _M_data_ptr = _Data_ptr;
        }

    protected:
        _Accelerator_view_impl_ptr _M_accelerator_view;
        _Accelerator_view_impl_ptr _M_access_on_accelerator_view;
        void * _M_data_ptr;
        void * _M_host_ptr;
        _Access_mode   _M_allowed_host_access_mode;
        _Access_mode _M_current_host_access_mode;
        size_t _M_elem_size;
        size_t _M_num_elems;
        bool   _M_owns_data;
        bool   _M_is_staging;

        // Used to determine how to map the staging buffer after its involved in a copy
        bool   _M_is_temp;

        bool   _M_is_adopted;
        bool   _M_is_buffer;
    private:
        // A set of view_keys to invalidate whenever the host ptr of a staging buffer is invalidated
        std::unique_ptr<std::unordered_set<_View_key>> _M_view_keys;
        Concurrency::critical_section _M_critical_section;
    };

    //  Class manages a texture in a accelerator view
    class _Texture : public _Buffer
    {
        friend class _CPU_accelerator_view_impl;
        friend class _D3D_accelerator_view_impl;
        friend class _D3D_temp_staging_cache;

    public:

        // Allocate a new texture on the specified accelerator_view
        _AMPIMP static _Ret_ _Texture * __cdecl _Create_texture(accelerator_view _Accelerator_view,
                                                                unsigned int _Rank,
                                                                size_t _Width, size_t _Height, size_t _Depth,
                                                                unsigned int _Mip_levels,
                                                                _Short_vector_base_type_id _Type_id,
                                                                unsigned int _Num_channels,
                                                                unsigned int _Bits_per_channel,
                                                                bool _Is_temp = false);

        // Create a texture object from a pre-allocated storage on the specified accelerator_view. This can be thought
        // of as the accelerator_view "adopting" the passed data buffer.
        _AMPIMP static _Ret_ _Texture * __cdecl _Adopt_texture(unsigned int _Rank, _Texture_base_type_id _Id,
                                                                _In_ IUnknown *_Data_ptr, accelerator_view _Accelerator_view,
                                                                unsigned int _View_format);

        // Create a staging texture on the specified accelerator_view which can be accessed on the cpu upon mapping.
        _AMPIMP static _Ret_ _Texture * __cdecl _Create_stage_texture(accelerator_view _Accelerator_view, accelerator_view _Access_on_accelerator_view,
                                                                      unsigned int _Rank,
                                                                      size_t _Width, size_t _Height, size_t _Depth,
                                                                      unsigned int _Mip_levels,
                                                                      unsigned int _Format,
                                                                      bool _Is_temp = false);

        // Create a staging texture on the specified accelerator_view which can be accessed on the cpu upon mapping.
        _AMPIMP static _Ret_ _Texture * __cdecl _Create_stage_texture(accelerator_view _Accelerator_view, accelerator_view _Access_on_accelerator_view,
                                                                      unsigned int _Rank,
                                                                      size_t _Width, size_t _Height, size_t _Depth,
                                                                      unsigned int _Mip_levels,
                                                                      _Short_vector_base_type_id _Type_id,
                                                                      unsigned int _Num_channels,
                                                                      unsigned int _Bits_per_channel);

        // Creates a temp staging texture. This function may create
        // a staging texture smaller than the requested size.
        _AMPIMP static _Ret_ _Texture * __cdecl _Get_temp_staging_texture(accelerator_view _Accelerator_view,
                                                                          unsigned int _Rank,
                                                                          size_t _Width, size_t _Height, size_t _Depth,
                                                                          unsigned int _Mip_levels,
                                                                          unsigned int _Format);

        // Constructs a new texture with the same properties as the given texture.
        _AMPIMP static _Ret_ _Texture * __cdecl _Clone_texture(const _Texture *_Src, const accelerator_view &_Accelerator_view, const accelerator_view &_Associated_av);

        //  Copy data to _Dest asynchronously for textures. The two textures must have been created with
        //  compatible physical formats.
        _AMPIMP _Event _Copy_to_async(_Inout_ _Texture * _Dest, const size_t *_Copy_extent,
                                      const size_t *_Src_offset, const size_t *_Dst_offset,
                                      unsigned int _Src_mipmap_level, unsigned int _Dst_mipmap_level);

        size_t _Get_width(unsigned int _Mip_offset = 0) const
        {
            return (_M_width >> _Mip_offset) ? (_M_width >> _Mip_offset) : 1U;
        }

        size_t _Get_height(unsigned int _Mip_offset = 0) const
        {
            return (_M_height >> _Mip_offset) ? (_M_height >> _Mip_offset) : 1U;
        }

        size_t _Get_depth(unsigned int _Mip_offset = 0) const
        {
            return (_M_depth >> _Mip_offset) ? (_M_depth >> _Mip_offset) : 1U;
        }

        unsigned int _Get_rank() const
        {
            return _M_rank;
        }

        unsigned int _Get_texture_format() const
        {
            return _M_texture_format;
        }

        unsigned int _Get_view_format() const
        {
            return _M_view_format;
        }

        unsigned int _Get_num_channels() const
        {
            return _M_num_channels;
        }

        unsigned int _Get_bits_per_channel() const
        {
            // For texture adopted from interop, return 0.
            return _Is_adopted() ? 0 : _M_bits_per_channel;
        }

        unsigned int _Get_bits_per_element() const
        {
            return _M_bits_per_channel * _M_num_channels;
        }

        unsigned int _Get_data_length(unsigned int _Most_detailed_mipmap_level, unsigned int _View_mipmap_levels, const size_t *_Extents = nullptr) const  // in bytes
        {
            _ASSERTE(_View_mipmap_levels);

            unsigned long long _Bits_per_byte = 8ULL;
            unsigned long long _Total_bytes = 0ULL;

            unsigned int _Mip_level = _Most_detailed_mipmap_level;

            // Sum up data length (in bytes) of all mipmap levels in the view
            for (unsigned int _Mip_offset=0; _Mip_offset < _View_mipmap_levels; ++_Mip_offset)
            {
                unsigned long long _Width = 1ULL;
                unsigned long long _Height = 1ULL;
                unsigned long long _Depth = 1ULL;

                if (_Extents)
                {
                    switch (_M_rank)
                    {
                    case 3:
                        _Depth = (_Extents[2] >> _Mip_level) ? (_Extents[2] >> _Mip_level) : 1U;
                        // deliberately fall through
                    case 2:
                        _Height = (_Extents[1] >> _Mip_level) ? (_Extents[1] >> _Mip_level) : 1U;
                        // deliberately fall through
                    case 1:
                        _Width = (_Extents[0] >> _Mip_level) ? (_Extents[0] >> _Mip_level) : 1U;
                        break;
                    default:
                        _ASSERTE(false); // textures are only rank 1-3
                    }
                }
                else
                {
                    _Width = _Get_width(_Mip_level);
                    _Height = _Get_height(_Mip_level);
                    _Depth = _Get_depth(_Mip_level);
                }

                // Note _Get_bits_per_element() can be smaller than 8
                // Use unsigned long long to avoid integer overflow
                _Total_bytes += ((_Width * _Height * _Depth * static_cast<unsigned long long>(_Get_bits_per_element())) + _Bits_per_byte - 1) / _Bits_per_byte;

                _Mip_level++;
            }

            return static_cast<unsigned int>(_Total_bytes);
        }

        unsigned int _Get_mip_levels() const
        {
            return _M_mip_levels;
        }

        size_t _Get_row_pitch() const
        {
            return _M_row_pitch;
        }

        void _Set_row_pitch(size_t _Val)
        {
            _M_row_pitch = _Val;
        }

        size_t _Get_depth_pitch() const
        {
            return _M_depth_pitch;
        }

        void _Set_depth_pitch(size_t _Val)
        {
            _M_depth_pitch = _Val;
        }

    private:
        // The _Texture constructor is private to force construction through the static
        // _Create_texture method to ensure the object is allocated in the runtime
        _Texture(_In_ _Accelerator_view_impl* _Av, _In_ void *_Texture_data_ptr, _In_opt_ void * _Host_ptr,
                 _Access_mode _Allowed_host_access_mode, _Access_mode _Current_host_access_mode,
                 unsigned int _Rank,
                 size_t _Width, size_t _Height, size_t _Depth,
                 unsigned int _Mip_levels,
                 unsigned int _Texture_format,
                 unsigned int _View_format,
                 unsigned int _Num_channels,
                 unsigned int _Bits_per_channel,
                 bool _Owns_data, bool _Is_staging, bool _Is_temp, bool _Is_adopted);

        // Private destructor to force deletion through _Release
        ~_Texture();

        // No default consturctor, copy constructor and assignment operator
        _Texture();
        _Texture(const _Texture &rhs);
        _Texture &operator=(const _Texture &rhs);

        // Texture only
        unsigned int _M_rank;
        size_t _M_width;
        size_t _M_height;
        size_t _M_depth;
        unsigned int _M_texture_format;
        unsigned int _M_view_format;
        unsigned int _M_bits_per_channel;
        unsigned int _M_num_channels;
        unsigned int _M_mip_levels;

        size_t _M_row_pitch;
        size_t _M_depth_pitch;
    };

    class _Sampler : public _Reference_counter
    {
    public:
        // Create a new sampler with configurations exposed by C++ AMP.
        _AMPIMP static _Ret_ _Sampler * __cdecl _Create(
            unsigned int _Filter_mode,
            unsigned int _Address_mode,
            float _Border_r,
            float _Border_g,
            float _Border_b,
            float _Border_a);

        // Create a sampler object given an adopted opaque data pointer
        _AMPIMP static _Ret_ _Sampler * __cdecl _Create(_In_ void *_Data_ptr);

        // Return the raw data ptr - only an accelerator view implementation can interpret
        // this raw pointer. This method should usually not be used in the AMP header files
        _Ret_ void * _Get_data_ptr() const
        {
            return _M_data_ptr;
        }

        bool _Is_adopted() const
        {
            // Is it adopted from interop?
            return _M_is_adopted;
        }

        unsigned int _Get_filter_mode() const
        {
            return _M_filter_mode;
        }

        unsigned int _Get_address_mode() const
        {
            return _M_address_mode;
        }

        const float* _Get_border_color() const
        {
            return &_M_border_color[0];
        }

    private:
        // The _Sampler constructor is private to force construction through the static
        // _Create method to ensure the object is allocated in the runtime
        _Sampler(unsigned int _Filter_mode, unsigned int _Address_mode, float _Border_r, float _Border_g, float _Border_b, float _Border_a);

        _Sampler(_In_ void *_Data_ptr);

        // Private destructor to force deletion through _Release
        ~_Sampler();

        // No default constructor, copy constructor and assignment operator
        _Sampler();
        _Sampler(const _Sampler &rhs);
        _Sampler &operator=(const _Sampler &rhs);

        void * _M_data_ptr;
        bool _M_is_adopted;
        unsigned int _M_filter_mode{};
        unsigned int _M_address_mode{};
        float _M_border_color[4]{};
    };

    // Forward declaration for copy helper functions
    _AMPIMP _Event __cdecl _Copy_impl(_In_ _Buffer *_Src, size_t _Src_offset,
                                      _Out_ _Buffer * _Dst, size_t _Dest_offset,
                                      size_t _Num_elems, size_t _Preferred_copy_chunk_num_elems = 0);

    _AMPIMP _Event __cdecl _Copy_async_impl(_In_ _Texture *_Src_tex, const size_t *_Src_offset, unsigned int _Src_mipmap_level,
                                            _Inout_ _Texture *_Dst_tex, const size_t *_Dst_offset, unsigned int _Dst_mipmap_level,
                                            const size_t *_Copy_extent, const size_t *_Preferred_copy_chunk_extent = NULL);

    inline bool _Get_chunked_staging_texture(_In_ _Texture* _Tex, _In_reads_(3) const size_t *_Copy_chunk_extent,
                                             _Inout_ size_t *_Remaining_copy_extent, _Out_writes_(3) size_t *_Curr_copy_extent,
                                             _Out_ _Texture_ptr *_Staging_texture)
    {
        bool _Truncated_copy = false;
        size_t _Allocation_extent[3] = { _Copy_chunk_extent[0], _Copy_chunk_extent[1], _Copy_chunk_extent[2] };

        unsigned int _Most_sig_idx = _Tex->_Get_rank() - 1;

        if (_Allocation_extent[_Most_sig_idx] > _Remaining_copy_extent[_Most_sig_idx]) {
            _Allocation_extent[_Most_sig_idx] = _Remaining_copy_extent[_Most_sig_idx];
        }

        _Texture_ptr _Stage = _Texture::_Get_temp_staging_texture(_Tex->_Get_accelerator_view(), _Tex->_Get_rank(),
            _Allocation_extent[0], _Allocation_extent[1], _Allocation_extent[2],
            /*_Mip_levels=*/1, _Tex->_Get_texture_format());

        std::copy(&_Allocation_extent[0], &_Allocation_extent[3], &_Curr_copy_extent[0]);
        size_t _Staging_tex_extent[3] = {_Stage->_Get_width(), _Stage->_Get_height(), _Stage->_Get_depth()};
        if (_Curr_copy_extent[_Most_sig_idx] > _Staging_tex_extent[_Most_sig_idx]) {
            _Curr_copy_extent[_Most_sig_idx] = _Staging_tex_extent[_Most_sig_idx];
        }

        // The truncation can however happen only in the most significant dimension and lower
        // dimensions should not get truncated
        if (_Curr_copy_extent[_Most_sig_idx] < _Remaining_copy_extent[_Most_sig_idx])
        {
            _Remaining_copy_extent[_Most_sig_idx] -= _Curr_copy_extent[_Most_sig_idx];
            _Truncated_copy = true;
        }

        for (unsigned int _I = 0; _I < _Most_sig_idx; _I++)
        {
            _ASSERTE(_Curr_copy_extent[_I] == _Remaining_copy_extent[_I]);
        }

        *_Staging_texture = _Stage;
        return _Truncated_copy;
    }

    #pragma warning ( push )
    #pragma warning ( disable : 6101 )
    // Suppress "warning C6101: Returning uninitialized memory '*_Dst'.:  A successful"
    // "path through the function does not set the named _Out_ parameter."
    // The callers to _Copy_data_on_host all have static_assert that _Rank has to be 1, 2, or 3 dimensions for texture
    //
    template <typename _Input_iterator, typename _Value_type>
    inline void _Copy_data_on_host_src_iter
        (int _Rank, _Input_iterator _Src, _Out_ _Value_type *_Dst,
         size_t _Width, size_t _Height, size_t _Depth,
         size_t _Dst_row_pitch_in_bytes, size_t _Dst_depth_pitch_in_bytes,
         size_t _Src_row_pitch, size_t _Src_depth_pitch)
    {
        switch(_Rank)
        {
        case 1:
            {
                _Input_iterator _End = _Src;
                std::advance(_End, _Width);
                std::copy(_Src, _End, _Dst);
            }
            break;
        case 2:
            {
                unsigned char *_Dst_ptr = reinterpret_cast<unsigned char *>(_Dst);
                _Input_iterator _Src_start = _Src;
                for (size_t _I = 0; _I < _Height; _I++)
                {
                    _Input_iterator _Src_end = _Src_start;
                    std::advance(_Src_end, _Width);

                    std::copy(_Src_start, _Src_end, reinterpret_cast<_Value_type*>(_Dst_ptr));

                    _Dst_ptr += _Dst_row_pitch_in_bytes;
                    std::advance(_Src_start, _Src_row_pitch);
                }
            }
            break;
        case 3:
            {
                unsigned char *_Dst_ptr_slice_start = reinterpret_cast<unsigned char *>(_Dst);
                _Input_iterator _Src_depth_slice_start = _Src;
                for (size_t _I = 0; _I < _Depth; _I++)
                {
                    _Input_iterator _Src_start = _Src_depth_slice_start;
                    unsigned char *_Dst_ptr = _Dst_ptr_slice_start;

                    for (size_t _J = 0; _J < _Height; _J++)
                    {
                        _Input_iterator _Src_end = _Src_start;
                        std::advance(_Src_end, _Width);

                        std::copy(_Src_start, _Src_end, reinterpret_cast<_Value_type*>(_Dst_ptr));

                        _Dst_ptr += _Dst_row_pitch_in_bytes;
                        std::advance(_Src_start, _Src_row_pitch);
                    }

                    _Dst_ptr_slice_start += _Dst_depth_pitch_in_bytes;
                    std::advance(_Src_depth_slice_start, _Src_depth_pitch);
                }
            }
            break;
        default:
            _ASSERTE(FALSE);
            break;
        }
    }
    #pragma warning ( pop ) // disable : 6101

    template <typename _Output_iterator, typename _Value_type>
    inline void _Copy_data_on_host_dst_iter
        (int _Rank, const _Value_type * _Src, _Output_iterator _Dst,
         size_t _Width, size_t _Height, size_t _Depth,
         size_t _Src_row_pitch_in_bytes, size_t _Src_depth_pitch_in_bytes,
         size_t _Dst_row_pitch, size_t _Dst_depth_pitch)
    {
        switch(_Rank)
        {
        case 1:
            {
                const _Value_type * _End = _Src + _Width;
                std::copy(_Src, _End, _Dst);
            }
            break;
        case 2:
            {
                const unsigned char *_Src_ptr = reinterpret_cast<const unsigned char *>(_Src);
                _Output_iterator _Dst_iter = _Dst;
                for (size_t _I = 0; _I < _Height; _I++)
                {
                    const _Value_type * _Src_end = reinterpret_cast<const _Value_type*>(_Src_ptr) + _Width;

                    std::copy(reinterpret_cast<const _Value_type*>(_Src_ptr), _Src_end, _Dst_iter);
                    std::advance(_Dst_iter, _Dst_row_pitch);
                    _Src_ptr += _Src_row_pitch_in_bytes;
                }
            }
            break;
        case 3:
            {
                const unsigned char *_Src_ptr_slice_start = reinterpret_cast<const unsigned char *>(_Src);
                _Output_iterator _Dst_depth_slice_start = _Dst;
                for (size_t _I = 0; _I < _Depth; _I++)
                {
                    _Output_iterator _Dst_iter = _Dst_depth_slice_start;
                    const unsigned char *_Src_ptr = _Src_ptr_slice_start;

                    for (size_t _J = 0; _J < _Height; _J++)
                    {
                        const _Value_type * _Src_end = reinterpret_cast<const _Value_type *>(_Src_ptr) + _Width;

                        std::copy(reinterpret_cast<const _Value_type*>(_Src_ptr), _Src_end, _Dst_iter);

                        std::advance(_Dst_iter, _Dst_row_pitch);
                        _Src_ptr += _Src_row_pitch_in_bytes;
                    }

                    _Src_ptr_slice_start += _Src_depth_pitch_in_bytes;
                    std::advance(_Dst_depth_slice_start, _Dst_depth_pitch);
                }
            }
            break;
        default:
            _ASSERTE(FALSE);
            break;
        }
    }

    _AMPIMP size_t __cdecl _Get_preferred_copy_chunk_size(size_t _Total_copy_size_in_bytes);

    inline size_t _Get_preferred_copy_chunk_num_elems(size_t _Total_num_elems, size_t _Elem_size)
    {
        size_t preferredChunkSize = _Get_preferred_copy_chunk_size(_Total_num_elems * _Elem_size);

        return (preferredChunkSize / _Elem_size);
    }

    inline void _Get_preferred_copy_chunk_extent(unsigned int _Rank, size_t _Width, size_t _Height,
                                                 size_t _Depth, size_t _Bits_per_element, _Out_writes_(3) size_t *_Preferred_copy_chunk_extent)
    {
        _ASSERTE(_Preferred_copy_chunk_extent != nullptr);

        size_t requestedByteSize = static_cast<size_t>((static_cast<unsigned long long>(_Width) *
                                                        static_cast<unsigned long long>(_Height) *
                                                        static_cast<unsigned long long>(_Depth) *
                                                        static_cast<unsigned long long>(_Bits_per_element)) >> 3);

        size_t preferredChunkSize = _Get_preferred_copy_chunk_size(requestedByteSize);

        // Lets align the allocation size to the element size of the texture
        size_t preferredCopyChunkNumElems = static_cast<size_t>((static_cast<unsigned long long>(preferredChunkSize) * 8U) / _Bits_per_element);

        // Lets truncate the dimensions of the requested staging texture.
        // We only truncate in the most significant dimension
        switch (_Rank)
        {
        case 1:
            _Width = preferredCopyChunkNumElems;
            break;
        case 2:
            _Height = (preferredCopyChunkNumElems + _Width - 1) / _Width;
            break;
        case 3:
            _Depth = (preferredCopyChunkNumElems + (_Height * _Width) - 1) / (_Height * _Width);
            break;
        default:
            _ASSERTE(false);
        }

        _Preferred_copy_chunk_extent[0] = _Width;
        _Preferred_copy_chunk_extent[1] = _Height;
        _Preferred_copy_chunk_extent[2] = _Depth;
    }

    // Finds the greatest common divisor of 2 unsigned integral numbers using Euclid's algorithm
    template <typename _T>
    inline _T _Greatest_common_divisor(_T _M, _T _N)
    {
        static_assert(std::is_unsigned<_T>::value, "This GCD function only supports unsigned integral types");

        _ASSERTE((_M > 0) && (_N > 0));

        if (_N > _M) {
            std::swap(_N , _M);
        }

        _T _Temp;
        while (_N > 0)
        {
            _Temp = _N;
            _N = _M % _N;
            _M = _Temp;
        }

        return _M;
    }

    // Finds the least common multiple of 2 unsigned integral numbers using their greatest_common_divisor
    template <typename _T>
    inline _T _Least_common_multiple(_T _M, _T _N)
    {
        static_assert(std::is_unsigned<_T>::value, "This LCM function only supports unsigned integral types");

        _ASSERTE((_M > 0) && (_N > 0));

        _T _Gcd = _Greatest_common_divisor(_M, _N);
        return ((_M / _Gcd) * _N);
    }

    template <typename InputIterator, typename _Value_type>
    inline _Event _Copy_impl(InputIterator _SrcFirst, InputIterator _SrcLast, size_t _NumElemsToCopy,
                             _Out_ _Buffer * _Dst, size_t _Dest_offset, size_t _Preferred_copy_chunk_num_elems = 0)
    {
        if (_NumElemsToCopy == 0) {
            return _Event();
        }

        if (_Dst == NULL) {
            throw runtime_exception("Failed to copy to buffer.", E_INVALIDARG);
        }

#pragma warning ( push )
#pragma warning ( disable : 6001 ) // Using uninitialized memory '*_Dst'
        if (((_NumElemsToCopy * sizeof(_Value_type)) + (_Dest_offset * _Dst->_Get_elem_size())) > (_Dst->_Get_num_elems() * _Dst->_Get_elem_size()))
        {
            throw runtime_exception("Invalid _Src argument(s). _Src size exceeds total size of the _Dest.", E_INVALIDARG);
        }
#pragma warning ( pop )

        _ASSERTE(_NumElemsToCopy == (size_t)(std::distance(_SrcFirst, _SrcLast)));

        // If the dest is host accessible for write then we do the copy on
        // accelerator(accelerator::cpu_accelerator).default_view
        if (_Dst->_Is_host_accessible(_Write_access))
        {
            // Lets first map the _Dst buffer
            _Event _Ev = _Dst->_Map_buffer_async(_Write_access);

            // The _Dest is accessible on host. We just need to do a std::copy using a raw pointer as OutputIterator
            _Buffer_ptr _PDestBuf = _Dst;
            _Ev = _Ev._Add_continuation(std::function<_Event()>([_PDestBuf,_Dest_offset, _SrcFirst, _SrcLast]() mutable -> _Event
            {
                _Value_type *_DestPtr = reinterpret_cast<_Value_type*>(reinterpret_cast<char*>(_PDestBuf->_Get_host_ptr()) + (_Dest_offset * _PDestBuf->_Get_elem_size()));
                std::copy(_SrcFirst, _SrcLast, _DestPtr);

                return _Event();
            }));

            return _Ev;
        }
        else
        {
            // _Dest is on a device. Lets create a temp staging buffer on the _Dest accelerator_view and copy the input over
            // We may create a staging buffer of size smaller than the copy size and in that case we will perform the copy
            // as a series of smaller copies
            _Buffer_ptr _PDestBuf = _Dst;
            size_t _NumElemsToCopyRemaining = _NumElemsToCopy;
            size_t _PreferredNumElemsToCopyPerChunk = _Preferred_copy_chunk_num_elems;
            if (_PreferredNumElemsToCopyPerChunk == 0) {
                // If a preferred copy chunk size was not specified, lets pick one based on the
                // size of the copy
                _PreferredNumElemsToCopyPerChunk = _Get_preferred_copy_chunk_num_elems(_NumElemsToCopy, sizeof(_Value_type));
            }
            size_t _CurrDstOffset = _Dest_offset;
            InputIterator _CurrStartIter = _SrcFirst;
            _Event _Ev;

            size_t _Lcm = _Least_common_multiple(_Dst->_Get_elem_size(), sizeof(_Value_type));
            size_t _AdjustmentRatio = _Lcm / sizeof(_Value_type);

            do
            {
                size_t _AllocationNumElems = _PreferredNumElemsToCopyPerChunk;
                if (_NumElemsToCopyRemaining < _AllocationNumElems) {
                    _AllocationNumElems = _NumElemsToCopyRemaining;
                }

                _Buffer_ptr _PDestStagingBuf = _Buffer::_Get_temp_staging_buffer(_Dst->_Get_accelerator_view(),
                                                                                 _AllocationNumElems, sizeof(_Value_type));

                _ASSERTE(_PDestStagingBuf != NULL);
                _ASSERTE(_PDestStagingBuf->_Get_elem_size() == sizeof(_Value_type));

                InputIterator _CurrEndIter = _CurrStartIter;
                size_t _CurrNumElemsToCopy = _AllocationNumElems;
                if (_CurrNumElemsToCopy > _PDestStagingBuf->_Get_num_elems()) {
                    _CurrNumElemsToCopy = _PDestStagingBuf->_Get_num_elems();
                }

                if (_NumElemsToCopyRemaining <= _CurrNumElemsToCopy) {
                    _CurrNumElemsToCopy = _NumElemsToCopyRemaining;
                    _CurrEndIter = _SrcLast;
                }
                else
                {
                    // We need to adjust the _CurrNumElemsToCopy to be a multiple of the
                    // least common multiple of the destination buffer's element size and sizeof(_Value_type).
                    _CurrNumElemsToCopy = (_CurrNumElemsToCopy / _AdjustmentRatio) * _AdjustmentRatio;
                    std::advance(_CurrEndIter, _CurrNumElemsToCopy);
                }

                _ASSERTE((_CurrNumElemsToCopy % _AdjustmentRatio) == 0);

                // This would not actually never block since we just created this staging buffer or are using
                // a cached one that is not in use
                _PDestStagingBuf->_Map_buffer(_Write_access, true /* _Wait */);

                // Copy from input to the staging using a raw pointer as OutputIterator
                std::copy(_CurrStartIter, _CurrEndIter, reinterpret_cast<_Value_type*>(_PDestStagingBuf->_Get_host_ptr()));

                _Ev = _Ev._Add_event(_PDestStagingBuf->_Copy_to_async(_PDestBuf, _CurrNumElemsToCopy, 0, _CurrDstOffset));

                // Adjust the iterators and offsets
                _NumElemsToCopyRemaining -= _CurrNumElemsToCopy;
                _CurrDstOffset += (_CurrNumElemsToCopy * sizeof(_Value_type)) / _Dst->_Get_elem_size();
                _CurrStartIter = _CurrEndIter;

            } while (_NumElemsToCopyRemaining != 0);

            return _Ev;
        }
    }

    // The std::advance method is only supported for InputIterators and hence we have a custom implementation
    // which forwards to the std::advance if the iterator is an input iterator and uses a loop based advance
    // implementation otherwise
    template<typename _InputIterator, typename _Distance>
    typename std::enable_if<std::is_base_of<std::input_iterator_tag, typename std::iterator_traits<_InputIterator>::iterator_category>::value>::type
    _Advance_output_iterator(_InputIterator &_Iter, _Distance _N)
    {
        std::advance(_Iter, _N);
    }

    template<typename _OutputIterator, typename _Distance>
    typename std::enable_if<!std::is_base_of<std::input_iterator_tag, typename std::iterator_traits<_OutputIterator>::iterator_category>::value>::type
    _Advance_output_iterator(_OutputIterator &_Iter, size_t _N)
    {
        for (size_t i = 0; i < _N; ++i)
        {
            _Iter++;
        }
    }

    template <typename OutputIterator, typename _Value_type>
    inline _Event _Copy_impl(_In_ _Buffer *_Src, size_t _Src_offset, size_t _Num_elems,
                             OutputIterator _DestIter, size_t _Preferred_copy_chunk_num_elems = 0)
    {
        if ((_Src == NULL) || ((_Src_offset + _Num_elems) > _Src->_Get_num_elems())) {
            throw runtime_exception("Failed to copy to buffer.", E_INVALIDARG);
        }

        if (_Num_elems == 0) {
            return _Event();
        }

        size_t _NumElemsToCopy = (_Num_elems * _Src->_Get_elem_size()) / sizeof(_Value_type);

        // If the src is host accessible for read then we do the copy on
        // accelerator(accelerator::cpu_accelerator).default_view
        if (_Src->_Is_host_accessible(_Read_access))
        {
            // Map the _Src buffer
            _Event _Ev = _Src->_Map_buffer_async(_Read_access);

            // The _Src is accessible on host. We just need to do a std::copy using a raw pointer as OutputIterator
            _Buffer_ptr _PSrcBuf = _Src;
            _Ev = _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _Src_offset, _DestIter, _NumElemsToCopy]() mutable -> _Event
            {
                // The _Src is accessible on host. We just need to do a std::copy
                const _Value_type *_PFirst = reinterpret_cast<const _Value_type*>(reinterpret_cast<char*>(_PSrcBuf->_Get_host_ptr()) + (_Src_offset * _PSrcBuf->_Get_elem_size()));
                std::copy(_PFirst, _PFirst + _NumElemsToCopy, _DestIter);

                return _Event();
            }));

            return _Ev;
        }
        else
        {
            // The _Src is on the device. We need to copy it out to a temporary staging array
            // We may create a staging buffer of size smaller than the copy size and in that case we will
            // perform the copy as a series of smaller copies

            _Event _Ev;

            _Buffer_ptr _PSrcBuf = _Src;
            size_t _PreferredNumElemsToCopyPerChunk = _Preferred_copy_chunk_num_elems;
            if (_PreferredNumElemsToCopyPerChunk == 0) {
                // If a preferred copy chunk size was not specified, lets pick one based on the
                // size of the copy
                _PreferredNumElemsToCopyPerChunk = _Get_preferred_copy_chunk_num_elems(_NumElemsToCopy, sizeof(_Value_type));
            }

            size_t _AllocationNumElems = _PreferredNumElemsToCopyPerChunk;
            if (_NumElemsToCopy < _AllocationNumElems) {
                _AllocationNumElems = _NumElemsToCopy;
            }

            _Buffer_ptr _PSrcStagingBuf = _Buffer::_Get_temp_staging_buffer(_Src->_Get_accelerator_view(),
                                                                            _AllocationNumElems, sizeof(_Value_type));

            _ASSERTE(_PSrcStagingBuf != NULL);
            _ASSERTE(_PSrcStagingBuf->_Get_elem_size() == sizeof(_Value_type));

            // The total byte size of a copy chunk must be an integral multiple of both the
            // source buffer's element size and sizeof(_Value_type).
            size_t _Lcm = _Least_common_multiple(_Src->_Get_elem_size(), sizeof(_Value_type));
            size_t _AdjustmentRatio = _Lcm / sizeof(_Value_type);

            size_t _CurrNumElemsToCopy = _AllocationNumElems;
            if (_CurrNumElemsToCopy > _PSrcStagingBuf->_Get_num_elems()) {
                _CurrNumElemsToCopy = _PSrcStagingBuf->_Get_num_elems();
            }
            if (_NumElemsToCopy <= _CurrNumElemsToCopy)
            {
                _CurrNumElemsToCopy = _NumElemsToCopy;
            }
            else
            {
                // We need to adjust the _StagingBufNumElems to be a multiple of the
                // least common multiple of the source buffer's element size and sizeof(_Value_type).
                _CurrNumElemsToCopy = (_CurrNumElemsToCopy / _AdjustmentRatio) * _AdjustmentRatio;
            }

            _ASSERTE((_CurrNumElemsToCopy % _AdjustmentRatio) == 0);

            size_t _NumElemsToCopyRemaining = _NumElemsToCopy - _CurrNumElemsToCopy;

            _Ev = _PSrcBuf->_Copy_to_async(_PSrcStagingBuf, (_CurrNumElemsToCopy * sizeof(_Value_type)) / _PSrcBuf->_Get_elem_size(), _Src_offset, 0);

            if (_NumElemsToCopyRemaining != 0)
            {
                _Ev = _Ev._Add_continuation(std::function<_Event()>([_DestIter, _PSrcBuf, _PSrcStagingBuf,
                                                                     _CurrNumElemsToCopy, _NumElemsToCopyRemaining,
                                                                     _Src_offset, _PreferredNumElemsToCopyPerChunk]() mutable -> _Event
                {
                    // Initiate an asynchronous copy of the remaining part so that this part of the copy
                    // makes progress while we consummate the copying of the first part
                    size_t _CurrSrcOffset = _Src_offset + ((_CurrNumElemsToCopy * sizeof(_Value_type)) / _PSrcBuf->_Get_elem_size());
                    OutputIterator _CurrDestIter = _DestIter;
                    _Advance_output_iterator<decltype(_CurrDestIter), size_t>(_CurrDestIter, _CurrNumElemsToCopy);
                    _Event _Ret_ev = _Copy_impl<OutputIterator, _Value_type>(_PSrcBuf._Get_ptr(), _CurrSrcOffset,
                                                                             (_NumElemsToCopyRemaining * sizeof(_Value_type)) / _PSrcBuf->_Get_elem_size(),
                                                                             _CurrDestIter, _PreferredNumElemsToCopyPerChunk);

                    // Now copy the data from staging buffer to the destination
                    _Value_type *_PFirst = reinterpret_cast<_Value_type*>(_PSrcStagingBuf->_Get_host_ptr());
                    std::copy(_PFirst, _PFirst + _CurrNumElemsToCopy, _DestIter);
                    return _Ret_ev;
                }));
            }
            else
            {
                _Ev = _Ev._Add_continuation(std::function<_Event()>([_DestIter, _PSrcStagingBuf, _CurrNumElemsToCopy]() mutable -> _Event
                {
                    _Value_type *_PFirst = reinterpret_cast<_Value_type*>(_PSrcStagingBuf->_Get_host_ptr());
                    std::copy(_PFirst, _PFirst + _CurrNumElemsToCopy, _DestIter);
                    return _Event();
                }));
            }

            return _Ev;
        }
    }

    // Structured copy between buffers across AVs
    _AMPIMP _Event __cdecl _Copy_impl(_In_ _Buffer *_Src, _View_shape_ptr _Src_shape, _Out_ _Buffer * _Dst, _View_shape_ptr _Dst_shape);

    struct _Array_copy_desc
    {
        _Array_copy_desc(
            const unsigned int _Rank,
            const unsigned int _Src_linear_offset,
            _In_reads_(_Rank) const unsigned int * _Src_extents,
            _In_reads_(_Rank) const unsigned int * _Src_copy_offset,
            const unsigned int _Dst_linear_offset,
            _In_reads_(_Rank) const unsigned int * _Dst_extents,
            _In_reads_(_Rank) const unsigned int * _Dst_copy_offset,
            _In_reads_(_Rank) const unsigned int * _Copy_extents)
        {
            this->_Rank = _Rank;

            this->_Src_linear_offset = _Src_linear_offset;
            this->_Src_extents.assign( _Src_extents, _Src_extents + _Rank);
            this->_Src_copy_offset.assign( _Src_copy_offset, _Src_copy_offset + _Rank);

            this->_Dst_linear_offset = _Dst_linear_offset;
            this->_Dst_extents.assign( _Dst_extents, _Dst_extents + _Rank);
            this->_Dst_copy_offset.assign( _Dst_copy_offset, _Dst_copy_offset + _Rank);

            this->_Copy_extents.assign( _Copy_extents, _Copy_extents + _Rank);
        }

        _Array_copy_desc() {}

        unsigned int _Rank{};

        // Shape of source
        unsigned int  _Src_linear_offset{};
        std::vector<unsigned int> _Src_extents;
        std::vector<unsigned int> _Src_copy_offset;

        // Shape of destination
        unsigned int  _Dst_linear_offset{};
        std::vector<unsigned int> _Dst_extents;
        std::vector<unsigned int> _Dst_copy_offset;

        // Shape of copy region
        std::vector<unsigned int> _Copy_extents;
    };

    // Declaration
    _AMPIMP HRESULT __cdecl _Recursive_array_copy(const _Array_copy_desc& _Desc,
                                                  unsigned int _Native_copy_rank,
                                                  std::function<HRESULT(const _Array_copy_desc &_Reduced)> _Native_copy_func);

    _AMPIMP std::pair<accelerator_view, accelerator_view> __cdecl _Get_src_dest_accelerator_view(_In_opt_ const _Buffer_descriptor *_SrcBuffDescPtr,
                                                                                                 _In_opt_ const _Buffer_descriptor *_DestBuffDescPtr);

    // Iterator based copy function
    template<typename _InputInterator, typename _OutputIterator>
    inline _Event _Copy_impl_iter(_InputInterator _SrcFirst, _InputInterator _SrcLast, _OutputIterator _DstFirst)
    {
        std::copy(_SrcFirst, _SrcLast, _DstFirst);
        return _Event();
    }

    // Iterator based copy function
    template <typename InputIterator, typename _Value_type>
    inline _Event _Copy_impl(InputIterator _SrcFirst, _View_shape_ptr _Src_shape, _Inout_ _Buffer * _Dst, _View_shape_ptr _Dst_shape)
    {
        _ASSERTE(_Dst != NULL);
        _ASSERTE(_Src_shape != NULL);
        _ASSERTE(_Dst_shape != NULL);

        if (_Src_shape->_Is_projection()) {
            _Src_shape = _Src_shape->_Get_reduced_shape_for_copy();
        }

        if (_Dst_shape->_Is_projection()) {
            _Dst_shape = _Dst_shape->_Get_reduced_shape_for_copy();
        }

        _ASSERTE(_Src_shape->_Get_rank() == _Dst_shape->_Get_rank());

        _ASSERTE(_View_shape::_Compare_extent_with_elem_size(_Src_shape->_Get_rank(), _Src_shape->_Get_view_extent(),
                                                             sizeof(_Value_type), _Dst_shape->_Get_view_extent(), _Dst->_Get_elem_size()));

        if (_Dst->_Is_host_accessible(_Write_access))
        {
            // The destination buffer is accessible on the host. Map the _Dst buffer
            _Event _Ev = _Dst->_Map_buffer_async(_Write_access);
            _Buffer_ptr _PDestBuf = _Dst;
            return _Ev._Add_continuation(std::function<_Event()>([_SrcFirst, _Src_shape, _PDestBuf, _Dst_shape]() mutable -> _Event {
                return _Copy_impl_iter(_SrcFirst, _Src_shape, reinterpret_cast<_Value_type*>(_PDestBuf->_Get_host_ptr()),
                                       _Create_reinterpreted_shape(_Dst_shape, _PDestBuf->_Get_elem_size(), sizeof(_Value_type)));
            }));
        }
        else
        {
            // The dest buffer is not accessible on host. Let's create a temporary
            // staging buffer on the destination buffer's accelerator_view
            _Buffer_ptr _PTempStagingBuf = _Buffer::_Create_stage_buffer(_Dst->_Get_accelerator_view(), accelerator(accelerator::cpu_accelerator).default_view,
                                                                         _Src_shape->_Get_view_size(), sizeof(_Value_type), true /* _Is_temp */);

            _PTempStagingBuf->_Map_buffer(_Write_access, true /* _Wait */);
            _Value_type *_Dst_ptr = reinterpret_cast<_Value_type*>(_PTempStagingBuf->_Get_host_ptr());
            _Event _Ev = _Copy_impl_iter(_SrcFirst, _Src_shape, _Dst_ptr, _Src_shape);

            // Now copy from the staging buffer to the destination buffer
            _Buffer_ptr _PDestBuf = _Dst;
            return _Ev._Add_continuation(std::function<_Event()>([_PTempStagingBuf, _Src_shape, _PDestBuf, _Dst_shape]() mutable -> _Event {
                return _Copy_impl(_PTempStagingBuf, _Src_shape, _PDestBuf, _Dst_shape);
            }));
        }
    }

    template <typename OutputIterator, typename _Value_type>
    inline _Event _Copy_impl(_In_ _Buffer *_Src, _View_shape_ptr _Src_shape, OutputIterator _DestIter, _View_shape_ptr _Dst_shape)
    {
        _ASSERTE(_Src != NULL);
        _ASSERTE(_Src_shape != NULL);
        _ASSERTE(_Dst_shape != NULL);

        if (_Src_shape->_Is_projection()) {
            _Src_shape = _Src_shape->_Get_reduced_shape_for_copy();
        }

        if (_Dst_shape->_Is_projection()) {
            _Dst_shape = _Dst_shape->_Get_reduced_shape_for_copy();
        }

        _ASSERTE(_Src_shape->_Get_rank() == _Dst_shape->_Get_rank());

        _ASSERTE(_View_shape::_Compare_extent_with_elem_size(_Src_shape->_Get_rank(), _Src_shape->_Get_view_extent(),
                                                             _Src->_Get_elem_size(), _Dst_shape->_Get_view_extent(), sizeof(_Value_type)));

        if (_Src->_Is_host_accessible(_Read_access))
        {
            // The source buffer is accessible on the host. Map the _Src buffer
            _Event _Ev = _Src->_Map_buffer_async(_Read_access);

            _Buffer_ptr _PSrcBuf = _Src;
            return _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _Src_shape, _DestIter, _Dst_shape]() mutable -> _Event {
                return _Copy_impl_iter(reinterpret_cast<_Value_type*>(_PSrcBuf->_Get_host_ptr()),
                                       _Create_reinterpreted_shape(_Src_shape, _PSrcBuf->_Get_elem_size(), sizeof(_Value_type)),
                                       _DestIter, _Dst_shape);
            }));
        }
        else
        {
            // The source buffer is not accessible on host. Lets create a temporary
            // staging buffer on the source buffer's accelerator_view and initiate a copy
            // from the source buffer to the temporary staging buffer
            _Buffer_ptr _PTempStagingBuf = _Buffer::_Create_stage_buffer(_Src->_Get_accelerator_view(), accelerator(accelerator::cpu_accelerator).default_view,
                                                                         _Dst_shape->_Get_view_size(), sizeof(_Value_type), true);

            _Event _Ev = _Src->_Copy_to_async(_PTempStagingBuf, _Src_shape, _Dst_shape);
            return _Ev._Add_continuation(std::function<_Event()>([_PTempStagingBuf, _Dst_shape, _DestIter]() mutable -> _Event {
                return _Copy_impl_iter(reinterpret_cast<_Value_type*>(_PTempStagingBuf->_Get_host_ptr()),
                                       _Dst_shape, _DestIter, _Dst_shape);
            }));
        }
    }

    // Iterator based structured copy function
    template<typename _InputInterator, typename _OutputIterator>
    inline _Event _Copy_impl_iter(_InputInterator _SrcIter, _View_shape_ptr _Src_shape,
                                  _OutputIterator _DstIter, _View_shape_ptr _Dst_shape)
    {
        if (_Src_shape->_Is_projection()) {
            _Src_shape = _Src_shape->_Get_reduced_shape_for_copy();
        }

        if (_Dst_shape->_Is_projection()) {
            _Dst_shape = _Dst_shape->_Get_reduced_shape_for_copy();
        }

        _ASSERTE(_Src_shape->_Get_rank() == _Dst_shape->_Get_rank());
        _ASSERTE(_View_shape::_Compare_extent(_Src_shape->_Get_rank(), _Src_shape->_Get_view_extent(), _Dst_shape->_Get_view_extent()));

        // If both the _Src_shape and _Dst_shape are linear we can be more efficient
        unsigned int _Src_linear_offset, _Src_linear_size, _Dst_linear_offset, _Dst_linear_size;
        if (_Src_shape->_Is_view_linear(_Src_linear_offset, _Src_linear_size) &&
            _Dst_shape->_Is_view_linear(_Dst_linear_offset, _Dst_linear_size))
        {
            _ASSERTE(_Src_linear_size == _Dst_linear_size);

            // These iterators might be not contiguous, therefore we use std::advance
            std::advance(_SrcIter, _Src_linear_offset);
            auto _SrcLast = _SrcIter;
            std::advance(_SrcLast, _Src_linear_size);
            std::advance(_DstIter, _Dst_linear_offset);

            return _Copy_impl_iter(_SrcIter, _SrcLast, _DstIter);
        }

        std::vector<unsigned int> _Src_extent(_Src_shape->_Get_rank());
        std::vector<unsigned int> _Src_offset(_Src_shape->_Get_rank());
        std::vector<unsigned int> _Dst_extent(_Dst_shape->_Get_rank());
        std::vector<unsigned int> _Dst_offset(_Dst_shape->_Get_rank());
        std::vector<unsigned int> _Copy_extent(_Src_shape->_Get_rank());

        for (size_t i = 0; i < _Src_shape->_Get_rank(); ++i) {
            _Src_extent[i] = _Src_shape->_Get_base_extent()[i];
            _Src_offset[i] = _Src_shape->_Get_view_offset()[i];
            _Dst_extent[i] = _Dst_shape->_Get_base_extent()[i];
            _Dst_offset[i] = _Dst_shape->_Get_view_offset()[i];
            _Copy_extent[i] = _Src_shape->_Get_view_extent()[i];
        }

        _Array_copy_desc _Desc(
            _Src_shape->_Get_rank(),
            _Src_shape->_Get_linear_offset(),
            _Src_extent.data(),
            _Src_offset.data(),
            _Dst_shape->_Get_linear_offset(),
            _Dst_extent.data(),
            _Dst_offset.data(),
            _Copy_extent.data());

        // Note: Capturing shape pointers would be incorrect, they are valid for setting up the call.
        // They might be deleted right after this call completes.
        HRESULT hr = _Recursive_array_copy(_Desc, 1, [_SrcIter, _DstIter](const _Array_copy_desc &_Reduced) -> HRESULT {

            auto _SrcFirst = _SrcIter;
            auto _DstFirst = _DstIter;

            std::advance(_DstFirst, _Reduced._Dst_linear_offset + _Reduced._Dst_copy_offset[0]);
            std::advance(_SrcFirst, _Reduced._Src_linear_offset + _Reduced._Src_copy_offset[0]);
            auto _SrcLast = _SrcFirst;
            std::advance(_SrcLast, _Reduced._Copy_extents[0]);

            std::copy(_SrcFirst, _SrcLast, _DstFirst);

            return S_OK;
        });

        if (FAILED(hr)) {
            throw Concurrency::runtime_exception("Failed to copy between buffers", E_FAIL);
        }

        return _Event();
    }

    // A ubiquitous buffer that provides access to the underlying data
    // on any accelerator_view
    class _Ubiquitous_buffer : public _Reference_counter
    {
        friend _Event _Get_access_async(const _View_key _Key, accelerator_view _Av, _Access_mode _Mode, _Buffer_ptr &_Buf_ptr);
        friend _AMPIMP accelerator_view __cdecl _Select_copy_src_accelerator_view(_In_ _View_key _Src_view_key, const accelerator_view &_Dest_accelerator_view);
        friend struct _DPC_call_handle;

    public:

        _AMPIMP static _Ret_ _Ubiquitous_buffer * __cdecl _Create_ubiquitous_buffer(size_t _Num_elems, size_t _Elem_size);

        _AMPIMP static _Ret_ _Ubiquitous_buffer * __cdecl _Create_ubiquitous_buffer(_Buffer_ptr _Master_buffer);

        // Register a new view on top of this _Ubiquitous_buffer
        _AMPIMP void _Register_view(_In_ _View_key _Key, accelerator_view _Cpu_av, _View_shape_ptr _Shape, _In_opt_ const _View_key _Source_view_key = nullptr);

        // Register a copy of an existing view registered with this _Ubiquitous_buffer
        _AMPIMP void _Register_view_copy(_In_ _View_key _New_view_key, _In_ _View_key _Existing_view_key);

        // Unregister a view currently registered with this _Ubiquitous_buffer
        _AMPIMP void _Unregister_view(_In_ _View_key _Key);

        // Obtain a specified mode of access to the specified view on the specified target
        // accelerator_view. This method also serves the purpose of determining the
        // amount of data copy expected to happen as part of this _Get_access request
        // without actually performing the copies or state updates in the _Ubiquitous_buffer. This
        // is used for reporting the implicit data copies that happen when accessing array_views
        // in C++ AMP ETW events
        _AMPIMP _Event _Get_access_async(_In_ _View_key _Key, _Accelerator_view_impl_ptr _Av_view_impl_ptr,
                                         _Access_mode _Mode, _Buffer_ptr &_Buf_ptr,
                                         _Inout_opt_ ULONGLONG *_Sync_size = nullptr);

        // Discard the content underlying this view
        _AMPIMP void _Discard(_In_ _View_key _Key);

        // This method does not synchronize the copies. Should not be used for getting
        // data access but only to get the underlying buffer's properties
        _AMPIMP _Buffer_ptr _Get_master_buffer() const;

        _AMPIMP accelerator_view _Get_master_accelerator_view() const;

        _AMPIMP _View_shape_ptr _Get_view_shape(_In_ _View_key _Key);

        _Ret_ _Accelerator_view_impl* _Get_master_accelerator_view_impl() const
        {
            return _M_master_av;
        }

        size_t _Get_master_buffer_elem_size() const
        {
            return _M_master_buffer_elem_size;
        }

        size_t _Get_master_buffer_num_elems() const
        {
            return _M_master_buffer_num_elems;
        }

        bool _Has_data_source() const
        {
            return _M_has_data_source;
        }

    private:

        // The _Ubiquitous_buffer constructors are private to force construction through the static
        // _Create_ubiquitous_buffer method to ensure the object is allocated in the runtime
        _Ubiquitous_buffer(size_t _Num_elems, size_t _Elem_size);
        _Ubiquitous_buffer(_In_ _Buffer* _Master_buffer);

        // Private destructor to force deletion through _Release
        ~_Ubiquitous_buffer();

        // No default constructor, copy constructor and assignment operator
        _Ubiquitous_buffer();
        _Ubiquitous_buffer(const _Ubiquitous_buffer &rhs);
        _Ubiquitous_buffer &operator=(const _Ubiquitous_buffer &rhs);

        // Helper methods

        // Get access to a buffer on a specified accelerator for a specified pre-registered view.
        // If _Sync_size parameter is not null, then function calculates number of bytes that we
        // need to synchronize to get desired access.
        _AMPIMP _Event _Get_access_async(_In_ _View_key _Key, accelerator_view _Av, _Access_mode _Mode,
                                         _Buffer_ptr &_Buf_ptr, _Inout_opt_ ULONGLONG *_Sync_size = NULL);

        // Commit a view to the main buffer if needed. When the _Sync_size parameter is non-null
        // this method just returns the amount of data to be copied as part of the commit, without
        // actually performing the commit
        _Event _Commit_view_async(_In_ _View_info *_Info, _Inout_opt_ ULONGLONG *_Sync_size = nullptr);

        // Get the _Buffer_ptr corresponding to a specified accelerator_view. When the
        // _Create parameter is true, it creates a new _Buffer if one does not already exist
        // for that accelerator_view
        _Ret_ _Buffer* _Get_buffer(_In_ _Accelerator_view_impl* _Av, bool _Create = true);

        // Sets a new access mode for the specified view
        void _Set_new_access_mode(_Inout_ _View_info *_Info, _Access_mode _New_mode);

        // Unsets the discard flag from the specified view and all other
        // overlapping views
        void _Unset_discard_flag(_Inout_ _View_info *_Info);

        // Determines whether the data underlying the specified view has been discarded
        // based on whether a subsuming view has the discard flag set.
        bool _Should_discard(const _View_info *_Info, _In_opt_ const _View_key _Source_view_key = nullptr) const;

        // Does this view have exclusive data which is not discarded,
        // not on the main accelerator_view and also there is not other view
        // that subsumes this view and is marked dirty
        bool _Has_exclusive_data(const _View_info *_Info) const;

        // Based on the current state of overlapping views in the _Ubiquitous_buffer
        // does the specified view require a data update on the target accelerator_view
        // to fulfil an access request
        bool _Requires_update_on_target_accelerator_view(const _View_info *_Info,
                                                         _Access_mode _Requested_mode,
                                                         _In_ _Accelerator_view_impl* _Target_acclerator_view) const;

        // This method iterates over all views in the specified commit list
        // and flags them as "commit not needed" if that view is subsumed by another view present in the
        // commit list
        static void _Flag_redundant_commits(std::vector<std::pair<_View_info*, bool>> &_Commit_list);

        // This method returns the list of accelerator_views where the specified view already has
        // a valid cached copy of the data and getting read access  would not incur any data movement.
        // The _Can_access_anywhere parameter is an output parameter used to indicate to the
        // caller that the specified view can be accessed on any accelerator_view without incurring
        // any data movement. This is true when there are no modified overlapping views that require
        // synchronization and the specified view has the discard_data flag set.
        // This method is used for determining the source accelerator_view for copy and p_f_e operations
        // involving array_views
        _Accelerator_view_unordered_set _Get_caching_info(_In_ _View_key _Key, _Out_opt_ bool *_Can_access_anywhere = NULL);

        _Accelerator_view_unordered_set _Get_caching_info_impl(_In_ _View_key _Key, _Out_opt_ bool *_Can_access_anywhere);

        _Ret_ _Accelerator_view_impl* _Determine_alternate_target_accelerator_view(_In_ _View_key _Key,
                                                                             _In_ _Accelerator_view_impl* _Original_av,
                                                                             _Access_mode _Mode);

        const _View_info * _Get_view_info_ptr(_In_ const _View_key key) const
        {
            auto const iterator = _M_view_map.find(key);
            return _M_view_map.end() == iterator ? nullptr : iterator->second;
        }

    private:

        // Private data

        // The main accelerator_view for this _Ubiquitous_buffer
        // which is specified at construction time
        _Accelerator_view_impl_ptr _M_master_av;

        // The main _Buffer corresponding to this _Ubiquitous_buffer
        // which is specified at construction time
        _Buffer* _M_master_buffer;

        // The size of each element of the main buffer
        size_t _M_master_buffer_elem_size;

        // The number of elements in the main buffer
        size_t _M_master_buffer_num_elems;

        // Indicates if this ubiquitous buffer has an underlying data source
        bool _M_has_data_source;

        // A map of pre-created _Buffers corresponding to different
        // accelerator_views where the _Ubiquitous_buffer has already been
        // accessed
        std::map<_Accelerator_view_impl_ptr, _Buffer_ptr> _M_buffer_map;

        // A mapping between all registered view keys in this _Ubiquitous_buffer
        // to their corresponding _View_info
        std::unordered_map<_View_key, _View_info*> _M_view_map;

        // Set of distinct views of this buffer. As multiple copies of the same
        // view may have been registered for this _Ubiquitous_buffer, this set
        // maintains the set of distinct views which really matter for the
        // caching protocol. Also, note that some view_info may not have any live registered
        // and hence does not exist in the _M_view_map but may exist here since
        // it has uncommitted data which needs to be considered as part of the cache
        // coherence protocol to prevent modifications underlying this view from being lost
        std::unordered_set<_View_info*> _M_view_info_set;

        // Critical section object to protect the cache directory
        Concurrency::critical_section _M_critical_section;
    };

    // Class defines functions for interoperability with D3D
    class _D3D_interop
    {
    public:
        _AMPIMP static _Ret_ IUnknown * __cdecl _Get_D3D_buffer(_In_ _Buffer *_Buffer_ptr);
        _AMPIMP static _Ret_ IUnknown * __cdecl _Get_D3D_texture(_In_ _Texture *_Texture_ptr);
        _AMPIMP static _Ret_ void * __cdecl _Get_D3D_sampler_data_ptr(_In_ IUnknown *_D3D_sampler);
        _AMPIMP static void __cdecl _Release_D3D_sampler_data_ptr(_In_ void *_Sampler_data_ptr);
        _AMPIMP static _Ret_ IUnknown * __cdecl _Get_D3D_sampler(const Concurrency::accelerator_view &_Av, _In_ _Sampler *_Sampler_ptr);
    };

    inline
    _Event _Get_access_async(const _View_key _Key, accelerator_view _Av, _Access_mode _Mode, _Buffer_ptr &_Buf_ptr)
    {
        return _Key->_Get_buffer_ptr()->_Get_access_async(_Key->_Get_view_key(), _Av, _Mode, _Buf_ptr);
    }

    inline
    _Ret_ _View_shape* _Get_buffer_view_shape(const _Buffer_descriptor& _Descriptor)
    {
        return _Descriptor._Get_buffer_ptr()->_Get_view_shape(_Descriptor._Get_view_key());
    }

    inline
    bool _Is_cpu_accelerator(const accelerator& _Accl)
    {
        return (_Accl.device_path == accelerator::cpu_accelerator);
    }

} // namespace Concurrency::details

} // namespace Concurrency

// =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
//
// Compiler/Runtime Interface
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#define HELPERAPI __cdecl

using namespace Concurrency::details;

extern "C" {

    // This structure is used for storing information about resources required by the kernel.
    enum _Resource_kind
    {
        RESOURCE_BUFFER      = 0,
        RESOURCE_TEXTURE     = 1,
        RESOURCE_SAMPLER     = 2,
    };

    struct _Device_resource_info
    {
        _Resource_kind _M_resource_kind;  // buffer, texture, or sampler

        void * _M_desc;        // Pointer to the _Buffer_descriptor/_Texture_descriptor/_Sampler_descriptor instance
                               // which underlies all device resource

        _Access_mode _M_formal_access_mode;         // scalar: read-only
                                                    // const scalar ref: read-only
                                                    // scalar ref: ReadWrite
                                                    // array: ReadWrite
                                                    // const array: ReadOnly
        size_t _M_actual_arg_num;

        BOOL _Is_buffer() const
        {
            return (_M_resource_kind == RESOURCE_BUFFER);
        }

        BOOL _Is_texture() const
        {
            return (_M_resource_kind == RESOURCE_TEXTURE);
        }

        BOOL _Is_sampler() const
        {
            return (_M_resource_kind == RESOURCE_SAMPLER);
        }

        _Ret_ _Buffer_descriptor * _Get_buffer_desc() const
        {
            _ASSERTE(_Is_buffer());
            return reinterpret_cast<_Buffer_descriptor *>(_M_desc);
        }

        _Ret_ _Texture_descriptor * _Get_texture_desc() const
        {
            _ASSERTE(_Is_texture());
            return reinterpret_cast<_Texture_descriptor *>(_M_desc);
        }

        _Ret_ _Sampler_descriptor * _Get_sampler_desc() const
        {
            _ASSERTE(_Is_sampler());
            return reinterpret_cast<_Sampler_descriptor *>(_M_desc);
        }

        _Ret_ void * _Get_resource_ptr() const
        {
            if (_Is_buffer())
            {
                _Ubiquitous_buffer * _Tmp = _Get_buffer_desc()->_Get_buffer_ptr();
                return reinterpret_cast<void *>(_Tmp);
            }
            else if (_Is_texture())
            {
                _Texture * _Tmp = _Get_texture_desc()->_Get_texture_ptr();
                return reinterpret_cast<void *>(_Tmp);
            }
            else
            {
                _ASSERTE(_Is_sampler());
                _Sampler * _Tmp = _Get_sampler_desc()->_Get_sampler_ptr();
                return reinterpret_cast<void *>(_Tmp);
            }
        }
    };

    // This structure is used for storing information about the const buffers
    struct _Device_const_buffer_info
    {
        void * _M_data;                             // Pointer to the host data to initialize the
                                                    // constant buffer with

        size_t _M_const_buf_size;                   // Size of the const buffer in bytes

        unsigned int _M_is_debug_data;              // Is this debug data which will be
                                                    // initialized by the runtime. 0 (false), 1 (true)
    };
}

namespace Concurrency
{
namespace details
{
    struct _DPC_call_handle
    {
        _Accelerator_view_impl *_M_rv;
        bool _M_is_explicit_target_acclview;

        // Info about the kernel function arguments
        _Device_resource_info * _M_device_resource_info;
        size_t _M_num_resources;
        size_t _M_num_writable_buffers;
        size_t _M_num_samplers;

        // Info about the host buffer created corresponding to the const buffer
        _Device_const_buffer_info * _M_const_buffer_info;
        size_t _M_num_const_buffers;

        bool _M_RW_aliasing;

        // Kernel funcs
        _DPC_shader_blob * _M_shader_blob;

        // Compute domain info
        int _M_is_flat_model;
        unsigned int _M_compute_rank;
        unsigned int * _M_grid_extents;

        // Kernel dispatch info
        unsigned int _M_groupCountX;
        unsigned int _M_groupCountY;
        unsigned int _M_groupCountZ;

        // The shape of the group
        unsigned int _M_groupExtentX;
        unsigned int _M_groupExtentY;
        unsigned int _M_groupExtentZ;

        _DPC_call_handle(const accelerator_view &_Accelerator_view)
        {
            if (!_Accelerator_view.is_auto_selection) {
                _M_rv = _Get_accelerator_view_impl_ptr(_Accelerator_view);
            }
            else {
                _M_rv = NULL;
            }

            _M_is_explicit_target_acclview = false;
            if (_M_rv != NULL) {
                _M_is_explicit_target_acclview = true;
            }

            _M_device_resource_info = NULL;
            _M_num_resources = 0;
            _M_num_writable_buffers = 0;
            _M_num_samplers = 0;

            _M_const_buffer_info = NULL;
            _M_num_const_buffers = 0;

            _M_RW_aliasing = false;

            _M_shader_blob = NULL;

            _M_is_flat_model = 0;
            _M_compute_rank = 0;
            _M_grid_extents = NULL;

            _M_groupCountX = 0;
            _M_groupCountY = 0;
            _M_groupCountZ = 0;

            _M_groupExtentX = 0;
            _M_groupExtentY = 0;
            _M_groupExtentZ = 0;
        }

        ~_DPC_call_handle()
        {
            if (_M_grid_extents) {
                delete [] _M_grid_extents;
            }
        }

        bool _Is_buffer_aliased(_In_ void* const _Buffer_ptr) const
        {
            return ((_M_aliased_buffer_set != nullptr) && (_M_aliased_buffer_set->find(_Buffer_ptr) != _M_aliased_buffer_set->end()));
        }

        bool _Is_buffer_unaccessed(size_t const _Buffer_idx) const
        {
            return ((_M_is_device_buffer_unaccessed != nullptr) && _M_is_device_buffer_unaccessed->operator[](_Buffer_idx));
        }

        void _Set_buffer_unaccessed(size_t _Buffer_idx)
        {
            if (_M_is_device_buffer_unaccessed == nullptr) {
                _M_is_device_buffer_unaccessed = std::unique_ptr<std::vector<bool>>(new std::vector<bool>(_M_num_resources, false));
            }

            _M_is_device_buffer_unaccessed->operator[](_Buffer_idx) = true;
        }

        const int* _Get_redirect_indices() const
        {
            if (!_M_RW_aliasing) {
                return nullptr;
            }

            _ASSERTE(_M_Redirect_indices != nullptr);

            return _M_Redirect_indices->data();
        }

        void _Check_buffer_aliasing();
        void _Update_buffer_rw_property();
        void _Setup_aliasing_redirection_indices();
        void _Select_accelerator_view();
        void _Verify_buffers_against_accelerator_view();

    private:
        std::unique_ptr<std::unordered_set<void*>> _M_aliased_buffer_set;
        std::unique_ptr<std::vector<bool>> _M_is_device_buffer_unaccessed;
        // Info about read-write aliasing
        std::unique_ptr<std::vector<int>> _M_Redirect_indices;
    };

    // This structure is used for passing the scheduling
    // info to the parallel_for_each which is handed back
    // to the compiler-runtime interface methods by the front-end
    struct _Host_Scheduling_info
    {
        // The accelerator view to invoke a parallel_for_each on
        accelerator_view _M_accelerator_view;
    };

} // namespace Concurrency::details


/// <summary>
///     Uninitializes the C++ AMP runtime. It is legal to
///     call this function multiple times during an application's
///     lifetime. Calling any C++ AMP API after calling this function
///     will reinitialize the C++ AMP runtime. Note that it is illegal
///     to use C++ AMP objects across calls to this function and doing
///     so will result in undefined behavior. Also, concurrently calling
///     this function and any other AMP APIs is illegal and would result
///     in undefined behavior.
/// </summary>
_AMPIMP void __cdecl amp_uninitialize();

} // namespace Concurrency

extern "C" {

    // Return a compiler helper handle.
    _AMPIMP _Ret_ _DPC_call_handle * HELPERAPI __dpc_create_call_handle(_In_ _Host_Scheduling_info *_Sch_info) noexcept(false);

    // Destroy the call handle
    _AMPIMP void HELPERAPI __dpc_release_call_handle(_In_ _DPC_call_handle * _Handle) noexcept(false);

    _AMPIMP void HELPERAPI __dpc_set_device_resource_info(_In_ _DPC_call_handle * _Handle, _In_ _Device_resource_info * _DeviceResourceInfo, size_t _NumResources) noexcept(false);

    // Set const buffer info.
    _AMPIMP void HELPERAPI __dpc_set_const_buffer_info(_In_ _DPC_call_handle * _Handle, _In_ _Device_const_buffer_info * _DeviceConstBufferInfo, size_t _NumConstBuffers) noexcept(false);

    // Set the kernel shader info
    _AMPIMP void HELPERAPI __dpc_set_kernel_shader_info(_In_ _DPC_call_handle * _Handle,
                                                        _Inout_ void * _ShaderBlobs) noexcept(false);
    // Set kernel dispatch info
    _AMPIMP void HELPERAPI __dpc_set_kernel_dispatch_info(_In_ _DPC_call_handle * _Handle,
                                                         unsigned int _ComputeRank,
                                                         _In_ int * _Extents,
                                                         unsigned int _GroupRank,
                                                         const unsigned int * _GroupExtents,
                                                         unsigned int & _GroupCountX,
                                                         unsigned int & _GroupCountY,
                                                         unsigned int & _GroupCountZ) noexcept(false);

    // Dispatch the kernel
    _AMPIMP void HELPERAPI __dpc_dispatch_kernel(_In_ _DPC_call_handle * _Handle) noexcept(false);

#ifdef _DEBUG
    // Dispatch the kernel passed as a HLSL source level shader
    // This function is to be used only for testing and debugging purposes
    _AMPIMP void HELPERAPI __dpc_dispatch_kernel_test(_In_ _DPC_call_handle * _Handle, _In_ WCHAR* szFileName, LPCSTR szEntryPoint) noexcept(false);
#endif
}

// =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
//
// C++ AMP ETW Provider
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

namespace Concurrency
{
namespace details
{

// Thread-safe factory method for _Amp_runtime_trace object
_AMPIMP _Ret_ _Amp_runtime_trace* __cdecl _Get_amp_trace();

// Class that gathers C++ AMP diagnostic information and triggers events
class _Amp_runtime_trace
{

// Called by factory to create single instance of _Amp_runtime_trace type
friend BOOL CALLBACK _Init_amp_runtime_trace(PINIT_ONCE _Init_once, PVOID _Param, _Inout_ PVOID *_Context);

public:
    // Destructor for _Amp_runtime_trace, called at program termination
    _AMPIMP ~_Amp_runtime_trace();

    // End event is triggered by multiple other events such us StartComputeEvent to show exactly when given activity completed
    _AMPIMP void _Write_end_event(ULONG _Span_id);

    // Add accelerator configuration information
    // Note: This member function does not have to be exported, it is used by C++ AMP runtime factory
    void _Add_accelerator_config_event(PVOID _Accelerator_id, LPCWSTR _Device_path, LPCWSTR _Device_description);

    // Used by callback function, to write all configuration data when new session is detected
    // Note: This member function does not have to be exported, it is used by C++ AMP runtime factory
    void _Write_all_accelerator_config_events();

    // Started accelerator_view::wait operation
    // Note: This member function does not have to be exported, it is used by C++ AMP runtime factory
    ULONG _Start_accelerator_view_wait_event(PVOID _Accelerator_id, PVOID _Accelerator_view_id);

    // Launched accelerator_view::flush operation
    // Note: This member function does not have to be exported, it is used by C++ AMP runtime factory
    void _Launch_flush_event(PVOID _Accelerator_id, PVOID _Accelerator_view_id);

    // Launched accelerator_view::create_marker operation
    // Note: This member function does not have to be exported, it is used by C++ AMP runtime factory
    ULONG _Launch_marker(PVOID _Accelerator_id, PVOID _Accelerator_view_id);

    // Below are set of helpers that take various types that were available at event injection point and extract all necessary data
    _AMPIMP ULONG _Start_parallel_for_each_event_helper(_In_ _DPC_call_handle *_Handle);

    // This helper wraps functor with wait start and wait end events
    inline concurrency::completion_future _Start_async_op_wait_event_helper(ULONG _Async_op_id, _Event _Ev)
    {
        std::shared_future<void> retFuture;
        concurrency::task_completion_event<void> retTaskCompletionEvent;

        // Create a std::shared_future by creating a deferred task through std::async that waits for the
        // event _Ev to finish. Wrap functor with start and end events
        retFuture = std::async(std::launch::deferred, [=]() mutable {
            try
            {
                if (_Async_op_id == _Amp_runtime_trace::_M_event_disabled)
                {
                    _Ev._Get();
                }
                else
                {
                    auto _Span_id = details::_Get_amp_trace()->_Start_async_op_wait_event(_Async_op_id);
                    _Ev._Get();
                    details::_Get_amp_trace()->_Write_end_event(_Span_id);
                }
            }
            catch(...)
            {
                // If an exception is encountered when executing the asynchronous operation
                // we should set the exception on the retTaskCompletionEvent so that it is
                // appropriately canceled and the exception is propagated to continuations
                retTaskCompletionEvent.set_exception(std::current_exception());
                throw;
            }

            retTaskCompletionEvent.set();
        });

        // Register the async event with the runtime asynchronous events manager
        _Register_async_event(_Ev, retFuture);

        // Lets issue a continuation just to swallow any exceptions that are encountered during the
        // async operation and are never observed by the user or are just observed through the
        // shared_future and not through the task
        concurrency::task<void> retTask(retTaskCompletionEvent);
        retTask.then([](concurrency::task<void> _Task) {
            try {
                _Task.get();
            }
            catch(...) {
            }
        });

        return Concurrency::completion_future(retFuture, retTask);
    }

    _AMPIMP ULONG _Start_array_view_synchronize_event_helper(const _Buffer_descriptor &_Buff_desc);
    _AMPIMP ULONG _Launch_array_view_synchronize_event_helper(const _Buffer_descriptor &_Buff_desc);

    // Helpers for buffers (array, array_view)
    _AMPIMP ULONG _Start_copy_event_helper(const _Buffer_descriptor &_Src, const _Buffer_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Start_copy_event_helper(nullptr_t, const _Buffer_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Start_copy_event_helper(const _Buffer_descriptor &_Src, nullptr_t, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Launch_async_copy_event_helper(const _Buffer_descriptor &_Src, const _Buffer_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Launch_async_copy_event_helper(nullptr_t, const _Buffer_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Launch_async_copy_event_helper(const _Buffer_descriptor &_Src, nullptr_t, ULONGLONG _Num_bytes_for_copy);

    // Helper for textures
    _AMPIMP ULONG _Start_copy_event_helper(const _Texture_descriptor &_Src, nullptr_t, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Start_copy_event_helper(nullptr_t, const _Texture_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Start_copy_event_helper(const _Texture_descriptor &_Src, const _Texture_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Launch_async_copy_event_helper(const _Texture_descriptor &_Src, nullptr_t, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Launch_async_copy_event_helper(nullptr_t, const _Texture_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);
    _AMPIMP ULONG _Launch_async_copy_event_helper(const _Texture_descriptor &_Src, const _Texture_descriptor &_Dest, ULONGLONG _Num_bytes_for_copy);

    void _Enable_provider(bool _Enable = true);

private:
    // Private constructor. This type is created by factory method
    _Amp_runtime_trace(PVOID _Callback_function, _In_ _Trace *_Trace);

    // Disallow copy construction
    _Amp_runtime_trace(const _Amp_runtime_trace&);

    // Disallow assignment operator
    _Amp_runtime_trace& operator=(const _Amp_runtime_trace&);

    // Used internally to write configuration events
    void _Write_accelerator_config_event(const std::tuple<PVOID, LPCWSTR, LPCWSTR> &_ConfigTuple);

    // Event triggered when computation is scheduled
    ULONG _Start_parallel_for_each_event(
        PVOID _Accelerator_id,
        PVOID _Accelerator_view_id,
        BOOL _Is_tiled_explicitly,
        ULONGLONG _Num_of_tiles,
        ULONG _Num_of_threads_per_tile,
        BOOL _Is_aliased,
        ULONG _Num_read_only_resources,
        ULONG _Num_read_write_resources,
        ULONGLONG _Size_of_all_resouces,
        ULONG _Size_of_const_data,
        ULONGLONG _Size_of_data_for_copy);

    // Synchronous copy operation has started
    ULONG _Start_copy_event(
        PVOID _Src_accelerator_id,
        PVOID _Src_accelerator_view_id,
        PVOID _Dst_accelerator_id,
        PVOID _Dst_accelerator_view_id,
        ULONGLONG _Num_bytes_for_copy,
        BOOL _Is_src_staging,
        BOOL _Is_dst_staging);

    // Asynchronous copy operation has been launched
    ULONG _Launch_async_copy_event(
        PVOID _Src_accelerator_id,
        PVOID _Src_accelerator_view_id,
        PVOID _Dst_accelerator_id,
        PVOID _Dst_accelerator_view_id,
        ULONGLONG _Num_bytes_for_copy,
        BOOL _Is_src_staging,
        BOOL _Is_dst_staging);

    // Started waiting for asynchronous operation to complete
    _AMPIMP ULONG _Start_async_op_wait_event(ULONG _Async_op_id);

    // Started array_view::synchronize operation
    ULONG _Start_array_view_synchronize_event(ULONGLONG _Num_bytes_to_synchronize);

    // Async array_view::synchronize operation has been launched
    ULONG _Launch_array_view_synchronize_event(ULONGLONG _Num_bytes_to_synchronize);

    // Helper function that extracts information from buffer descriptor
    std::tuple<PVOID, PVOID, BOOL> _Get_resource_diagnostic_info(const _Buffer_descriptor &_Buff_desc, accelerator_view _Accl_view) const;

    // Helper function that extracts information from texture descriptor
    std::tuple<PVOID, PVOID, BOOL> _Get_resource_diagnostic_info(const _Texture_descriptor &_Tex_desc) const;

    // Generates unique identifiers for span_id and async_op_id
    ULONG _Get_unique_identifier();

    // Critical section object used by callback function to synchronize following situations:
    // a) multiple sessions have started at the same time
    // b) C++ AMP Runtime factory adds new accelerator config event to the collection
    Concurrency::critical_section _M_critical_section;

    // Collection of all configuration events at the time of C++ AMP Runtime initialization
    std::vector<std::tuple<PVOID, LPCWSTR, LPCWSTR>> _M_accelerator_configs;

    // Unique counter for span id and async operation id
    volatile ULONG _M_counter;

    // Type that implements ITrace interface and writes events e.g. ETW events
    _Trace* _M_trace_ptr;

    // Special value that we return to chain events if provider is disabled
    static const ULONG _M_event_disabled = 0;
};

// Helper function to query the number of mipmap levels from texture object
inline unsigned int _Get_mipmap_levels(const _Texture *_Tex)
{
    _ASSERTE(_Tex);
    return _Tex->_Get_mip_levels();
}

} // namespace Concurrency::details
} // namespace Concurrency

namespace concurrency = Concurrency;

#pragma pack(pop)
