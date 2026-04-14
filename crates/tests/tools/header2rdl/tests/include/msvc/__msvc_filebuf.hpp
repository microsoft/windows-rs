// __msvc_filebuf.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_FILEBUF_HPP
#define __MSVC_FILEBUF_HPP
#include <yvals.h>
#if _STL_COMPILER_PREPROCESSOR

#include <cstdio>
#include <streambuf>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

// TRANSITION, ABI: The _Path_ish functions accepting filesystem::path or experimental::filesystem::path are templates
// which always use the same types as a workaround for user code deriving from iostreams types and
// __declspec(dllexport)ing the derived types. Adding member functions to iostreams broke the ABI of such DLLs.
// Deriving and __declspec(dllexport)ing standard library types is not supported, but in this particular case
// the workaround was inexpensive. The workaround will be removed in the next ABI breaking release of the
// Visual C++ Libraries.
_STD_BEGIN
#if _HAS_CXX17
namespace filesystem {
    _EXPORT_STD class path;
}
#endif // _HAS_CXX17

#ifndef _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM
#ifdef _M_CEE
#define _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM 0
#else // ^^^ defined(_M_CEE) / !defined(_M_CEE) vvv
#define _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM 1
#endif // ^^^ !defined(_M_CEE) ^^^
#endif // _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM

#if _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM
namespace experimental {
    namespace filesystem {
        inline namespace v1 {
            class path;
        }
    } // namespace filesystem
} // namespace experimental
#endif // _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM

// clang-format off
template <class _Ty>
constexpr bool _Is_any_path = _Is_any_of_v<_Ty
#if _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM
    , experimental::filesystem::path
#endif // _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM
#if _HAS_CXX17
    , filesystem::path
#endif // _HAS_CXX17
    >;
// clang-format on

extern "C++" _CRTIMP2_PURE FILE* __CLRCALL_PURE_OR_CDECL _Fiopen(const char*, ios_base::openmode, int);
extern "C++" _CRTIMP2_PURE FILE* __CLRCALL_PURE_OR_CDECL _Fiopen(const wchar_t*, ios_base::openmode, int);

template <class _Elem>
bool _Fgetc(_Elem& _Ch, FILE* _File) { // get an element from a C stream
    return _CSTD fread(&_Ch, sizeof(_Elem), 1, _File) == 1;
}

template <>
inline bool _Fgetc(char& _Byte, FILE* _File) { // get a char element from a C stream
    int _Meta;
    if ((_Meta = _CSTD fgetc(_File)) == EOF) {
        return false;
    } else { // got one, convert to char
        _Byte = static_cast<char>(_Meta);
        return true;
    }
}

template <>
inline bool _Fgetc(wchar_t& _Wchar, FILE* _File) { // get a wchar_t element from a C stream
    wint_t _Meta;
    if ((_Meta = _CSTD fgetwc(_File)) == WEOF) {
        return false;
    } else { // got one, convert to wchar_t
        _Wchar = static_cast<wchar_t>(_Meta);
        return true;
    }
}

#ifdef _CRTBLD
template <>
inline bool _Fgetc(unsigned short& _Wchar, FILE* _File) { // get an unsigned short element from a C stream
    wint_t _Meta;
    if ((_Meta = _CSTD fgetwc(_File)) == WEOF) {
        return false;
    } else { // got one, convert to unsigned short
        _Wchar = static_cast<unsigned short>(_Meta);
        return true;
    }
}
#endif // defined(_CRTBLD)

template <class _Elem>
bool _Fputc(_Elem _Ch, FILE* _File) { // put an element to a C stream
    return _CSTD fwrite(&_Ch, 1, sizeof(_Elem), _File) == sizeof(_Elem);
}

template <>
inline bool _Fputc(char _Byte, FILE* _File) { // put a char element to a C stream
    return _CSTD fputc(_Byte, _File) != EOF;
}

template <>
inline bool _Fputc(wchar_t _Wchar, FILE* _File) { // put a wchar_t element to a C stream
    return _CSTD fputwc(_Wchar, _File) != WEOF;
}

#ifdef _CRTBLD
template <>
inline bool _Fputc(unsigned short _Wchar, FILE* _File) { // put an unsigned short element to a C stream
    return _CSTD fputwc(_Wchar, _File) != WEOF;
}
#endif // defined(_CRTBLD)

template <class _Elem>
bool _Ungetc(const _Elem&, FILE*) { // put back an arbitrary element to a C stream (always fail)
    return false;
}

template <>
inline bool _Ungetc(const char& _Byte, FILE* _File) { // put back a char element to a C stream
    return _CSTD ungetc(static_cast<unsigned char>(_Byte), _File) != EOF;
}

template <>
inline bool _Ungetc(const signed char& _Byte, FILE* _File) { // put back a signed char element to a C stream
    return _CSTD ungetc(static_cast<unsigned char>(_Byte), _File) != EOF;
}

template <>
inline bool _Ungetc(const unsigned char& _Byte, FILE* _File) { // put back an unsigned char element to a C stream
    return _CSTD ungetc(_Byte, _File) != EOF;
}

template <>
inline bool _Ungetc(const wchar_t& _Wchar, FILE* _File) { // put back a wchar_t element to a C stream
    return _CSTD ungetwc(_Wchar, _File) != WEOF;
}

#ifdef _CRTBLD
template <>
inline bool _Ungetc(const unsigned short& _Wchar, FILE* _File) { // put back an unsigned short element to a C stream
    return _CSTD ungetwc(_Wchar, _File) != WEOF;
}
#endif // defined(_CRTBLD)

_EXPORT_STD template <class _Elem, class _Traits>
class basic_filebuf : public basic_streambuf<_Elem, _Traits> { // stream buffer associated with a C stream
public:
    using _Mysb = basic_streambuf<_Elem, _Traits>;
    using _Cvt  = codecvt<_Elem, char, typename _Traits::state_type>;

    basic_filebuf() : _Mysb() {
        _Init(nullptr, _Newfl);
    }

    explicit basic_filebuf(FILE* const _File) : _Mysb() { // extension
        _Init(_File, _Newfl);
    }

    __CLR_OR_THIS_CALL ~basic_filebuf() noexcept override {
        if (_Myfile) {
            _Reset_back(); // revert from _Mychar buffer
        }

        if (_Closef) {
            close();
        }
    }

    using int_type = typename _Traits::int_type;
    using pos_type = typename _Traits::pos_type;
    using off_type = typename _Traits::off_type;

    basic_filebuf(_Uninitialized) noexcept : _Mysb(_Noinit) {}

    basic_filebuf(basic_filebuf&& _Right) {
        _Init(_Right._Myfile, _Newfl); // match buffering styles
        _Init(static_cast<FILE*>(nullptr), _Closefl); // then make *this look closed
        _Assign_rv(_STD move(_Right));
    }

    basic_filebuf& operator=(basic_filebuf&& _Right) {
        _Assign_rv(_STD move(_Right));
        return *this;
    }

    void _Assign_rv(basic_filebuf&& _Right) {
        if (this != _STD addressof(_Right)) {
            close();
            this->swap(_Right);
        }
    }

    void swap(basic_filebuf& _Right) noexcept /* strengthened */ {
        if (this != _STD addressof(_Right)) {
            FILE* _Myfile_sav                       = _Myfile;
            const _Cvt* _Pcvt_sav                   = _Pcvt;
            typename _Traits::state_type _State_sav = _State;
            bool _Wrotesome_sav                     = _Wrotesome;
            bool _Closef_sav                        = _Closef;
            bool _Set_eback_sav                     = _Mysb::eback() == &_Mychar;
            bool _Set_eback_live                    = _Mysb::gptr() == &_Mychar;

            _Elem* _Pfirst0 = _Mysb::pbase();
            _Elem* _Pnext0  = _Mysb::pptr();
            _Elem* _Pend    = _Mysb::epptr();
            _Elem* _Gfirst0 = _Mysb::eback();
            _Elem* _Gnext0  = _Mysb::gptr();
            _Elem* _Gend    = _Mysb::egptr();

            // reinitialize *this
            _Init(_Right._Myfile, _Right._Myfile ? _Openfl : _Newfl);
            _Mysb::setp(_Right.pbase(), _Right.pptr(), _Right.epptr());
            if (_Right.eback() != &_Right._Mychar) {
                _Mysb::setg(_Right.eback(), _Right.gptr(), _Right.egptr());
            } else if (_Right.gptr() != &_Right._Mychar) {
                _Mysb::setg(&_Mychar, &_Mychar + 1, &_Mychar + 1);
            } else {
                _Mysb::setg(&_Mychar, &_Mychar, &_Mychar + 1);
            }

            _Pcvt      = _Right._Pcvt;
            _State     = _Right._State;
            _Wrotesome = _Right._Wrotesome;
            _Closef    = _Right._Closef;

            // reinitialize _Right
            _Right._Init(_Myfile_sav, _Myfile_sav ? _Openfl : _Newfl);
            _Right.setp(_Pfirst0, _Pnext0, _Pend);
            if (!_Set_eback_sav) {
                _Right.setg(_Gfirst0, _Gnext0, _Gend);
            } else if (!_Set_eback_live) {
                _Right.setg(&_Right._Mychar, &_Right._Mychar + 1, &_Right._Mychar + 1);
            } else {
                _Right.setg(&_Right._Mychar, &_Right._Mychar, &_Right._Mychar + 1);
            }

            _Right._Pcvt      = _Pcvt_sav;
            _Right._State     = _State_sav;
            _Right._Wrotesome = _Wrotesome_sav;
            _Right._Closef    = _Closef_sav;

            // swap ancillary data
            _STD swap(_Set_eback, _Right._Set_eback);
            _STD swap(_Set_egptr, _Right._Set_egptr);

            _STD swap(_Mychar, _Right._Mychar);
            _STD swap(_Mysb::_Plocale, _Right._Plocale);
        }
    }

    basic_filebuf(const basic_filebuf&)            = delete;
    basic_filebuf& operator=(const basic_filebuf&) = delete;

    enum _Initfl { // reasons for a call to _Init
        _Newfl,
        _Openfl,
        _Closefl
    };

    _NODISCARD bool is_open() const noexcept /* strengthened */ {
        return static_cast<bool>(_Myfile);
    }

    basic_filebuf* open(const char* _Filename, ios_base::openmode _Mode, int _Prot = ios_base::_Default_open_prot) {
        // _Prot is an extension
        if (_Myfile) {
            return nullptr;
        }

        const auto _File = _Fiopen(_Filename, _Mode, _Prot);
        if (!_File) {
            return nullptr; // open failed
        }

        _Init(_File, _Openfl);
        _Initcvt(_STD use_facet<_Cvt>(_Mysb::getloc()));
        return this; // open succeeded
    }

    basic_filebuf* open(const string& _Str, ios_base::openmode _Mode, int _Prot = ios_base::_Default_open_prot) {
        // _Prot is an extension
        return open(_Str.c_str(), _Mode, _Prot);
    }

#if _HAS_OLD_IOSTREAMS_MEMBERS
    basic_filebuf* open(const char* _Filename, ios_base::open_mode _Mode) {
        return open(_Filename, static_cast<ios_base::openmode>(_Mode));
    }
#endif // _HAS_OLD_IOSTREAMS_MEMBERS

    basic_filebuf* open(const wchar_t* _Filename, ios_base::openmode _Mode, int _Prot = ios_base::_Default_open_prot) {
        // in standard as const std::filesystem::path::value_type *; _Prot is an extension
        if (_Myfile) {
            return nullptr;
        }

        const auto _File = _Fiopen(_Filename, _Mode, _Prot);
        if (!_File) {
            return nullptr; // open failed
        }

        _Init(_File, _Openfl);
        _Initcvt(_STD use_facet<_Cvt>(_Mysb::getloc()));
        return this; // open succeeded
    }

    basic_filebuf* open(const wstring& _Str, ios_base::openmode _Mode, int _Prot = ios_base::_Default_open_prot) {
        // extension
        return open(_Str.c_str(), _Mode, _Prot);
    }

#if _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM
    template <class _Path_ish = experimental::filesystem::path>
    basic_filebuf* open(
        const _Identity_t<_Path_ish>& _Path, ios_base::openmode _Mode, int _Prot = ios_base::_Default_open_prot) {
        // _Prot is an extension
        return open(_Path.c_str(), _Mode, _Prot);
    }
#endif // _FSTREAM_SUPPORTS_EXPERIMENTAL_FILESYSTEM

#if _HAS_CXX17
    template <int = 0, class _Path_ish = filesystem::path>
    basic_filebuf* open(
        const _Identity_t<_Path_ish>& _Path, ios_base::openmode _Mode, int _Prot = ios_base::_Default_open_prot) {
        // _Prot is an extension
        return open(_Path.c_str(), _Mode, _Prot);
    }
#endif // _HAS_CXX17

#if _HAS_OLD_IOSTREAMS_MEMBERS
    basic_filebuf* open(const wchar_t* _Filename, ios_base::open_mode _Mode) {
        // in standard as const std::filesystem::path::value_type *
        return open(_Filename, static_cast<ios_base::openmode>(_Mode));
    }
#endif // _HAS_OLD_IOSTREAMS_MEMBERS

    basic_filebuf* close() {
        basic_filebuf* _Ans;
        if (_Myfile) { // put any homing sequence and close file
            _Reset_back(); // revert from _Mychar buffer

            _Ans = this;
            if (!_Endwrite()) {
                _Ans = nullptr;
            }

            if (_CSTD fclose(_Myfile) != 0) {
                _Ans = nullptr;
            }
        } else {
            _Ans = nullptr;
        }

        _Init(nullptr, _Closefl);
        return _Ans;
    }

    void __CLR_OR_THIS_CALL _Lock() override { // lock file instead of stream buffer
        if (_Myfile) {
            _CSTD _lock_file(_Myfile);
        }
    }

    void __CLR_OR_THIS_CALL _Unlock() override { // unlock file instead of stream buffer
        if (_Myfile) {
            _CSTD _unlock_file(_Myfile);
        }
    }

#if _HAS_CXX23 && defined(_CPPRTTI)
    template <class _UnicodeConsoleFn, class _FallbackFn, class _Filebuf_type>
    friend ios_base::iostate _Do_on_maybe_unicode_console(ostream&, _UnicodeConsoleFn, _FallbackFn);
#endif // ^^^ _HAS_CXX23 && defined(_CPPRTTI) ^^^

protected:
    int_type __CLR_OR_THIS_CALL overflow(int_type _Meta = _Traits::eof()) override { // put an element to stream
        if (_Traits::eq_int_type(_Traits::eof(), _Meta)) {
            return _Traits::not_eof(_Meta); // EOF, return success code
        }

        if (_Mysb::pptr() && _Mysb::pptr() < _Mysb::epptr()) { // room in buffer, store it
            *_Mysb::_Pninc() = _Traits::to_char_type(_Meta);
            return _Meta;
        }

        if (!_Myfile) {
            return _Traits::eof(); // no open C stream, fail
        }

        _Reset_back(); // revert from _Mychar buffer
        if (!_Pcvt) { // no codecvt facet, put as is
            return _Fputc(_Traits::to_char_type(_Meta), _Myfile) ? _Meta : _Traits::eof();
        }

        // put using codecvt facet
        constexpr size_t _Codecvt_temp_buf = 32;
        char _Str[_Codecvt_temp_buf];
        const _Elem _Ch = _Traits::to_char_type(_Meta);
        const _Elem* _Src;
        char* _Dest;

        // test result of converting one element
        switch (_Pcvt->out(_State, &_Ch, &_Ch + 1, _Src, _Str, _Str + _Codecvt_temp_buf, _Dest)) {
        case codecvt_base::partial:
        case codecvt_base::ok:
            { // converted something, try to put it out
                const auto _Count = static_cast<size_t>(_Dest - _Str);
                if (0 < _Count && _Count != static_cast<size_t>(_CSTD fwrite(_Str, 1, _Count, _Myfile))) {
                    return _Traits::eof(); // write failed
                }

                _Wrotesome = true; // write succeeded
                if (_Src != &_Ch) {
                    return _Meta; // converted whole element
                }

                return _Traits::eof(); // conversion failed
            }

        case codecvt_base::noconv:
            // no conversion, put as is
            return _Fputc(_Ch, _Myfile) ? _Meta : _Traits::eof();

        default:
            return _Traits::eof(); // conversion failed
        }
    }

    int_type __CLR_OR_THIS_CALL pbackfail(int_type _Meta = _Traits::eof()) override {
        // put an element back to stream
        if (_Mysb::gptr() && _Mysb::eback() < _Mysb::gptr()
            && (_Traits::eq_int_type(_Traits::eof(), _Meta)
                || _Traits::eq_int_type(_Traits::to_int_type(_Mysb::gptr()[-1]),
                    _Meta))) { // just back up position
            _Mysb::_Gndec();
            return _Traits::not_eof(_Meta);
        } else if (!_Myfile || _Traits::eq_int_type(_Traits::eof(), _Meta)) {
            return _Traits::eof(); // no open C stream or EOF, fail
        } else if (!_Pcvt && _Ungetc(_Traits::to_char_type(_Meta), _Myfile)) {
            return _Meta; // no facet and unget succeeded, return
        } else if (_Mysb::gptr() != &_Mychar) { // putback to _Mychar
            _Mychar = _Traits::to_char_type(_Meta);
            _Set_back(); // switch to _Mychar buffer
            return _Meta;
        } else {
            return _Traits::eof(); // nowhere to put back
        }
    }

    int_type __CLR_OR_THIS_CALL underflow() override { // get an element from stream, but don't point past it
        int_type _Meta;
        if (_Mysb::gptr() && _Mysb::gptr() < _Mysb::egptr()) {
            return _Traits::to_int_type(*_Mysb::gptr()); // return buffered
        } else if (_Traits::eq_int_type(_Traits::eof(), _Meta = uflow())) {
            return _Meta; // uflow failed, return EOF
        } else { // get a char, don't point past it
            pbackfail(_Meta);
            return _Meta;
        }
    }

    int_type __CLR_OR_THIS_CALL uflow() override { // get an element from stream, point past it
        if (_Mysb::gptr() && _Mysb::gptr() < _Mysb::egptr()) {
            return _Traits::to_int_type(*_Mysb::_Gninc()); // return buffered
        }

        if (!_Myfile) {
            return _Traits::eof(); // no open C stream, fail
        }

        _Reset_back(); // revert from _Mychar buffer
        if (!_Pcvt) { // no codecvt facet, just get it
            _Elem _Ch;
            return _Fgetc(_Ch, _Myfile) ? _Traits::to_int_type(_Ch) : _Traits::eof();
        }

        // build string until codecvt succeeds
        string _Str;

        for (;;) { // get using codecvt facet
            const char* _Src;
            int _Meta = _CSTD fgetc(_Myfile);

            if (_Meta == EOF) {
                return _Traits::eof(); // partial char?
            }

            _Str.push_back(static_cast<char>(_Meta)); // append byte and convert

            _Elem _Ch;
            _Elem* _Dest;

            // test result of converting one element
            switch (_Pcvt->in(_State, _Str.data(), _Str.data() + _Str.size(), _Src, &_Ch, &_Ch + 1, _Dest)) {
            case codecvt_base::partial:
            case codecvt_base::ok:
                if (_Dest != &_Ch) { // got an element, put back excess and deliver it
                    auto _Nleft = _Str.data() + _Str.size() - _Src;
                    while (0 < _Nleft) {
                        _CSTD ungetc(_Src[--_Nleft], _Myfile);
                    }

                    return _Traits::to_int_type(_Ch);
                }

                _Str.erase(0, static_cast<size_t>(_Src - _Str.data())); // partial, discard used input
                break;

            case codecvt_base::noconv:
                // noconv is only possible if _Elem is char, so we can use it directly
                return static_cast<int_type>(_Str.front());

            default:
                return _Traits::eof(); // conversion failed
            }
        }
    }

    streamsize __CLR_OR_THIS_CALL xsgetn(_Elem* _Ptr, streamsize _Count) override {
        // get _Count characters from stream
        if constexpr (sizeof(_Elem) == 1) {
            if (_Count <= 0) {
                return 0;
            }

            if (_Pcvt) { // if we need a nontrivial codecvt transform, do the default expensive thing
                return _Mysb::xsgetn(_Ptr, _Count);
            }

            // assuming this is OK because _Ptr + _Count must be valid
            auto _Count_s           = static_cast<size_t>(_Count);
            const auto _Start_count = _Count;
            const auto _Available   = static_cast<size_t>(_Mysb::_Gnavail());
            if (0 < _Available) { // copy from get area
                const auto _Read_size = (_STD min)(_Count_s, _Available);
                _Traits::copy(_Ptr, _Mysb::gptr(), _Read_size);
                _Ptr += _Read_size;
                _Count_s -= _Read_size;
                _Mysb::gbump(static_cast<int>(_Read_size));
            }

            if (_Myfile) { // open C stream, attempt read
                _Reset_back(); // revert from _Mychar buffer
                // process in 4k - 1 chunks to avoid tripping over fread's clobber-the-end behavior when
                // doing \r\n -> \n translation
                constexpr size_t _Read_size = 4095; // _INTERNAL_BUFSIZ - 1
                while (_Read_size < _Count_s) {
                    const auto _Actual_read = _CSTD fread(_Ptr, sizeof(_Elem), _Read_size, _Myfile);
                    _Ptr += _Actual_read;
                    _Count_s -= _Actual_read;
                    if (_Actual_read != _Read_size) {
                        return static_cast<streamsize>(_Start_count - _Count_s);
                    }
                }

                if (0 < _Count_s) {
                    _Count_s -= _CSTD fread(_Ptr, sizeof(_Elem), _Count_s, _Myfile);
                }
            }

            return static_cast<streamsize>(_Start_count - _Count_s);
        } else { // non-chars always get element-by-element processing
            return _Mysb::xsgetn(_Ptr, _Count);
        }
    }

    streamsize __CLR_OR_THIS_CALL xsputn(const _Elem* _Ptr, streamsize _Count) override {
        // put _Count characters to stream
        if constexpr (sizeof(_Elem) == 1) {
            if (_Pcvt) { // if we need a nontrivial codecvt transform, do the default expensive thing
                return _Mysb::xsputn(_Ptr, _Count);
            }

            const streamsize _Start_count = _Count;
            streamsize _Size              = _Mysb::_Pnavail();
            if (0 < _Count && 0 < _Size) { // copy to write buffer
                if (_Count < _Size) {
                    _Size = _Count;
                }

                _Traits::copy(_Mysb::pptr(), _Ptr, static_cast<size_t>(_Size));
                _Ptr += _Size;
                _Count -= _Size;
                _Mysb::pbump(static_cast<int>(_Size));
            }

            if (0 < _Count && _Myfile) { // open C stream, attempt write
                _Count -= _CSTD fwrite(_Ptr, sizeof(_Elem), static_cast<size_t>(_Count), _Myfile);
            }

            return _Start_count - _Count;
        } else { // non-chars always get element-by-element processing
            return _Mysb::xsputn(_Ptr, _Count);
        }
    }

    pos_type __CLR_OR_THIS_CALL seekoff(off_type _Off, ios_base::seekdir _Way,
        ios_base::openmode = ios_base::in | ios_base::out) override { // change position by _Off
        fpos_t _Fileposition;

        if (_Mysb::gptr() == &_Mychar // something putback
            && _Way == ios_base::cur // a relative seek
            && !_Pcvt) { // not converting
            _Off -= static_cast<off_type>(sizeof(_Elem)); // back up over _Elem bytes
        }

        if (!_Myfile || !_Endwrite()
            || ((_Off != 0 || _Way != ios_base::cur) && _CSTD _fseeki64(_Myfile, _Off, _Way) != 0)
            || _CSTD fgetpos(_Myfile, &_Fileposition) != 0) {
            return pos_type{off_type{-1}}; // report failure
        }

        _Reset_back(); // revert from _Mychar buffer, discarding any putback
        return pos_type{_State, _Fileposition}; // return new position
    }

    pos_type __CLR_OR_THIS_CALL seekpos(pos_type _Pos, ios_base::openmode = ios_base::in | ios_base::out) override {
        // change position to _Pos
        off_type _Off = static_cast<off_type>(_Pos);

        if (!_Myfile || !_Endwrite() || _CSTD fsetpos(_Myfile, &_Off) != 0) {
            return pos_type{off_type{-1}}; // report failure
        }

        _State = _Pos.state();
        _Reset_back(); // revert from _Mychar buffer, discarding any putback
        return pos_type{_State, _Off}; // return new position
    }

    _Mysb* __CLR_OR_THIS_CALL setbuf(_Elem* _Buffer, streamsize _Count) override { // offer _Buffer to C stream
        int _Mode;
        if (!_Buffer && _Count == 0) {
            _Mode = _IONBF;
        } else {
            _Mode = _IOFBF;
        }

        const size_t _Size = static_cast<size_t>(_Count) * sizeof(_Elem);

        if (!_Myfile || _CSTD setvbuf(_Myfile, reinterpret_cast<char*>(_Buffer), _Mode, _Size) != 0) {
            return nullptr; // failed
        }

        // new buffer, reinitialize pointers
        _Init(_Myfile, _Openfl);
        return this;
    }

    int __CLR_OR_THIS_CALL sync() override { // synchronize C stream with external file
        if (!_Myfile || _Traits::eq_int_type(_Traits::eof(), overflow()) || 0 <= _CSTD fflush(_Myfile)) {
            return 0;
        }

        return -1;
    }

    void __CLR_OR_THIS_CALL imbue(const locale& _Loc) override {
        // set locale to argument (capture nontrivial codecvt facet)
        _Initcvt(_STD use_facet<_Cvt>(_Loc));
    }

    void _Init(FILE* _File, _Initfl _Which) noexcept { // initialize to C stream _File after {new, open, close}
        using _State_type = typename _Traits::state_type;

        __PURE_APPDOMAIN_GLOBAL static _State_type _Stinit; // initial state

        _Closef    = _Which == _Openfl;
        _Wrotesome = false;

        _Mysb::_Init(); // initialize stream buffer base object

        if (_File && sizeof(_Elem) == 1) { // point inside C stream with [first, first + count) buffer
            _Elem** _Pb = nullptr;
            _Elem** _Pn = nullptr;
            int* _Nr    = nullptr;

            ::_get_stream_buffer_pointers(
                _File, reinterpret_cast<char***>(&_Pb), reinterpret_cast<char***>(&_Pn), &_Nr);
            int* _Nw = _Nr;

            _Mysb::_Init(_Pb, _Pn, _Nr, _Pb, _Pn, _Nw);
        }

        _Myfile = _File;
        _State  = _Stinit;
        _Pcvt   = nullptr; // pointer to codecvt facet
    }

    bool _Endwrite() { // put shift to initial conversion state, as needed
        if (!_Pcvt || !_Wrotesome) {
            return true;
        }

        // may have to put
        if (_Traits::eq_int_type(_Traits::eof(), overflow())) {
            return false;
        }

        constexpr size_t _Codecvt_temp_buf = 32;
        char _Str[_Codecvt_temp_buf];
        char* _Dest;
        switch (_Pcvt->unshift(_State, _Str, _Str + _Codecvt_temp_buf, _Dest)) { // test result of homing conversion
        case codecvt_base::ok:
            _Wrotesome = false; // homed successfully
            _FALLTHROUGH;

        case codecvt_base::partial:
            { // put any generated bytes
                const auto _Count = static_cast<size_t>(_Dest - _Str);
                if (0 < _Count && _Count != static_cast<size_t>(_CSTD fwrite(_Str, 1, _Count, _Myfile))) {
                    return false; // write failed
                }

                return !_Wrotesome;
            }

        case codecvt_base::noconv:
            _Wrotesome = false; // homed successfully
            return true; // nothing else to do

        default:
            return false; // conversion failed
        }
    }

    void _Initcvt(const _Cvt& _Newcvt) noexcept { // initialize codecvt pointer
        if (_Newcvt.always_noconv()) {
            _Pcvt = nullptr; // nothing to do
        } else { // set up for nontrivial codecvt facet
            _Pcvt = _STD addressof(_Newcvt);
            _Mysb::_Init(); // reset any buffering
        }
    }

private:
    const _Cvt* _Pcvt; // pointer to codecvt facet (may be null)
    _Elem _Mychar; // putback character, when _Ungetc fails
    bool _Wrotesome; // true if homing sequence may be needed
    typename _Traits::state_type _State; // current conversion state
    bool _Closef; // true if C stream must be closed
    FILE* _Myfile; // pointer to C stream

    void _Reset_back() noexcept { // restore buffer after putback
        if (_Mysb::eback() == &_Mychar) {
            _Mysb::setg(_Set_eback, _Set_eback, _Set_egptr);
        }
    }

    void _Set_back() noexcept { // set up putback area
        if (_Mysb::eback() != &_Mychar) { // save current get buffer
            _Set_eback = _Mysb::eback();
            _Set_egptr = _Mysb::egptr();
        }
        _Mysb::setg(&_Mychar, &_Mychar, &_Mychar + 1);
    }

    _Elem* _Set_eback; // saves eback() during one-element putback
    _Elem* _Set_egptr; // saves egptr()
};

_EXPORT_STD template <class _Elem, class _Traits>
void swap(basic_filebuf<_Elem, _Traits>& _Left, basic_filebuf<_Elem, _Traits>& _Right) noexcept /* strengthened */ {
    _Left.swap(_Right);
}

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_FILEBUF_HPP
