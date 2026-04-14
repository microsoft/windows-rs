// __msvc_ostream.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_OSTREAM_HPP
#define __MSVC_OSTREAM_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR
#include <ios>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
#pragma vtordisp(push, 2) // compiler bug workaround

_EXPORT_STD extern "C++" template <class _Elem, class _Traits>
class basic_ostream : virtual public basic_ios<_Elem, _Traits> { // control insertions into a stream buffer
public:
    using _Myios = basic_ios<_Elem, _Traits>;
    using _Mysb  = basic_streambuf<_Elem, _Traits>;
    using _Iter  = ostreambuf_iterator<_Elem, _Traits>;
    using _Nput  = num_put<_Elem, _Iter>;

    explicit __CLR_OR_THIS_CALL basic_ostream(basic_streambuf<_Elem, _Traits>* _Strbuf, bool _Isstd = false) {
        _Myios::init(_Strbuf, _Isstd);
    }

    __CLR_OR_THIS_CALL basic_ostream(_Uninitialized, bool _Addit = true) {
        if (_Addit) {
            this->_Addstd(this); // suppress for basic_iostream
        }
    }

protected:
    __CLR_OR_THIS_CALL basic_ostream(basic_ostream&& _Right) noexcept(false) {
        _Myios::init();
        _Myios::move(_STD move(_Right));
    }

    basic_ostream& __CLR_OR_THIS_CALL operator=(basic_ostream&& _Right) noexcept /* strengthened */ {
        this->swap(_Right);
        return *this;
    }

    void __CLR_OR_THIS_CALL swap(basic_ostream& _Right) noexcept /* strengthened */ {
        if (this != _STD addressof(_Right)) {
            _Myios::swap(_Right);
        }
    }

public:
    __CLR_OR_THIS_CALL basic_ostream(const basic_ostream&)            = delete;
    basic_ostream& __CLR_OR_THIS_CALL operator=(const basic_ostream&) = delete;

    __CLR_OR_THIS_CALL ~basic_ostream() noexcept override {}

    using int_type = typename _Traits::int_type;
    using pos_type = typename _Traits::pos_type;
    using off_type = typename _Traits::off_type;

    class _Sentry_base { // stores thread lock and reference to output stream
    public:
        __CLR_OR_THIS_CALL _Sentry_base(basic_ostream& _Ostr) : _Myostr(_Ostr) { // lock the stream buffer, if there
            const auto _Rdbuf = _Myostr.rdbuf();
            if (_Rdbuf) {
                _Rdbuf->_Lock();
            }
        }

        __CLR_OR_THIS_CALL ~_Sentry_base() noexcept { // destroy after unlocking
            const auto _Rdbuf = _Myostr.rdbuf();
            if (_Rdbuf) {
                _Rdbuf->_Unlock();
            }
        }

        basic_ostream& _Myostr; // the output stream, for _Unlock call at destruction

        _Sentry_base& operator=(const _Sentry_base&) = delete;
    };

    class sentry : public _Sentry_base {
    public:
        explicit __CLR_OR_THIS_CALL sentry(basic_ostream& _Ostr) : _Sentry_base(_Ostr) {
            if (!_Ostr.good()) {
                _Ok = false;
                return;
            }

            const auto _Tied = _Ostr.tie();
            if (!_Tied || _Tied == _STD addressof(_Ostr)) {
                _Ok = true;
                return;
            }

            _Tied->flush();
            _Ok = _Ostr.good(); // store test only after flushing tie
        }

        _STL_DISABLE_DEPRECATED_WARNING
        __CLR_OR_THIS_CALL ~sentry() noexcept {
#if !_HAS_EXCEPTIONS
            const bool _Zero_uncaught_exceptions = true;
#elif _HAS_DEPRECATED_UNCAUGHT_EXCEPTION
            const bool _Zero_uncaught_exceptions = !_STD uncaught_exception(); // TRANSITION, ArchivedOS-12000909
#else // ^^^ _HAS_DEPRECATED_UNCAUGHT_EXCEPTION / !_HAS_DEPRECATED_UNCAUGHT_EXCEPTION vvv
            const bool _Zero_uncaught_exceptions = _STD uncaught_exceptions() == 0;
#endif // ^^^ !_HAS_DEPRECATED_UNCAUGHT_EXCEPTION ^^^

            if (_Zero_uncaught_exceptions) {
                this->_Myostr._Osfx();
            }
        }
        _STL_RESTORE_DEPRECATED_WARNING

        explicit __CLR_OR_THIS_CALL operator bool() const {
            return _Ok;
        }

        __CLR_OR_THIS_CALL sentry(const sentry&)            = delete;
        sentry& __CLR_OR_THIS_CALL operator=(const sentry&) = delete;

    private:
        bool _Ok; // true if stream state okay at construction
    };

#pragma push_macro("opfx")
#pragma push_macro("osfx")
#undef opfx
#undef osfx
    // TRANSITION, ABI: non-Standard opfx() is preserved for binary compatibility
    _DEPRECATE_IO_PFX_SFX bool __CLR_OR_THIS_CALL opfx() { // test stream state and flush tie stream as needed
        if (!this->good()) {
            return false;
        }

        const auto _Tied = _Myios::tie();
        if (!_Tied || _Myios::tie() == this) {
            return true;
        }

        _Tied->flush();
        return this->good();
    }

    // TRANSITION, ABI: non-Standard osfx() is preserved for binary compatibility
    _DEPRECATE_IO_PFX_SFX void __CLR_OR_THIS_CALL osfx() noexcept { // perform any wrapup
        _Osfx();
    }
#pragma pop_macro("osfx")
#pragma pop_macro("opfx")

    void __CLR_OR_THIS_CALL _Osfx() noexcept { // perform any wrapup
        _TRY_BEGIN
        if (this->good() && this->flags() & ios_base::unitbuf) {
            if (_Myios::rdbuf()->pubsync() == -1) { // flush stream as needed
                _Myios::setstate(ios_base::badbit);
            }
        }
        _CATCH_ALL
        _CATCH_END
    }

#ifdef _M_CEE_PURE
    basic_ostream& __CLR_OR_THIS_CALL operator<<(basic_ostream&(__clrcall* _Pfn)(basic_ostream&) ) {
        // call basic_ostream manipulator
        return _Pfn(*this);
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(_Myios&(__clrcall* _Pfn)(_Myios&) ) {
        // call basic_ios manipulator
        _Pfn(*this);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(ios_base&(__clrcall* _Pfn)(ios_base&) ) {
        // call ios_base manipulator
        _Pfn(*this);
        return *this;
    }
#endif // defined(_M_CEE_PURE)

    basic_ostream& __CLR_OR_THIS_CALL operator<<(basic_ostream&(__cdecl* _Pfn)(basic_ostream&) ) {
        // call basic_ostream manipulator
        return _Pfn(*this);
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(_Myios&(__cdecl* _Pfn)(_Myios&) ) {
        // call basic_ios manipulator
        _Pfn(*this);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(ios_base&(__cdecl* _Pfn)(ios_base&) ) {
        // call ios_base manipulator
        _Pfn(*this);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(bool _Val) { // insert a boolean
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(short _Val) { // insert a short
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac  = _STD use_facet<_Nput>(this->getloc());
            ios_base::fmtflags _Bfl = this->flags() & ios_base::basefield;

            long _Tmp;
            if (_Bfl == ios_base::oct || _Bfl == ios_base::hex) {
                _Tmp = static_cast<long>(static_cast<unsigned short>(_Val));
            } else {
                _Tmp = static_cast<long>(_Val);
            }

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Tmp).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    // NOTE:
    // If you are not using native wchar_t, the unsigned short inserter
    // is masked by an explicit specialization that treats an unsigned
    // short as a wide character.

    // To read or write unsigned shorts as integers with wchar_t streams,
    // make wchar_t a native type with the command line option /Zc:wchar_t.

    basic_ostream& __CLR_OR_THIS_CALL operator<<(unsigned short _Val) { // insert an unsigned short
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), static_cast<unsigned long>(_Val))
                    .failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(int _Val) { // insert an int
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac  = _STD use_facet<_Nput>(this->getloc());
            ios_base::fmtflags _Bfl = this->flags() & ios_base::basefield;

            long _Tmp;
            if (_Bfl == ios_base::oct || _Bfl == ios_base::hex) {
                _Tmp = static_cast<long>(static_cast<unsigned int>(_Val));
            } else {
                _Tmp = static_cast<long>(_Val);
            }

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Tmp).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(unsigned int _Val) { // insert an unsigned int
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), static_cast<unsigned long>(_Val))
                    .failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(long _Val) { // insert a long
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(unsigned long _Val) { // insert an unsigned long
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(long long _Val) { // insert a long long
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(unsigned long long _Val) { // insert an unsigned long long
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(float _Val) { // insert a float
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), static_cast<double>(_Val)).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(double _Val) { // insert a double
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(long double _Val) { // insert a long double
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL operator<<(const void* _Val) { // insert a void pointer
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (_Ok) { // state okay, use facet to insert
            const _Nput& _Nput_fac = _STD use_facet<_Nput>(this->getloc());

            _TRY_IO_BEGIN
            if (_Nput_fac.put(_Iter(_Myios::rdbuf()), *this, _Myios::fill(), _Val).failed()) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

#if _HAS_CXX23
    template <class = void> // TRANSITION, ABI
    basic_ostream& operator<<(const volatile void* _Val) {
        return *this << const_cast<const void*>(_Val);
    }
#endif // _HAS_CXX23

#if _HAS_CXX17
    template <class = void> // TRANSITION, ABI
    basic_ostream& operator<<(nullptr_t) { // insert a null pointer
        return *this << "nullptr";
    }
#endif // _HAS_CXX17

    basic_ostream& __CLR_OR_THIS_CALL operator<<(_Mysb* _Strbuf) { // insert until end-of-file from a stream buffer
        ios_base::iostate _State = ios_base::goodbit;
        bool _Copied             = false;
        const sentry _Ok(*this);

        if (_Ok && _Strbuf) {
            for (int_type _Meta = _Traits::eof();; _Copied = true) { // extract another character from stream buffer
                _TRY_BEGIN
                _Meta = _Traits::eq_int_type(_Traits::eof(), _Meta) ? _Strbuf->sgetc() : _Strbuf->snextc();
                _CATCH_ALL
                // N4971 [ostream.inserters]/9: "If an exception was thrown
                // while extracting a character, the function sets failbit in the error state,
                // and if failbit is set in exceptions() the caught exception is rethrown."
                _Myios::setstate(ios_base::failbit, _Myios::exceptions() == ios_base::failbit);
                _CATCH_END

                if (_Traits::eq_int_type(_Traits::eof(), _Meta)) {
                    break; // end of file, quit
                }

                _TRY_IO_BEGIN
                if (_Traits::eq_int_type(_Traits::eof(), _Myios::rdbuf()->sputc(_Traits::to_char_type(_Meta)))) {
                    _State |= ios_base::badbit; // insertion failed, quit
                    break;
                }
                _CATCH_IO_END
            }
        }

        this->width(0);
        int _Setstate_with;
        if (_Strbuf) {
            if (_Copied) {
                _Setstate_with = _State;
            } else {
                _Setstate_with = _State | ios_base::failbit;
            }
        } else {
            _Setstate_with = ios_base::badbit;
        }

        _Myios::setstate(_Setstate_with);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL put(_Elem _Ch) { // insert a character
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (!_Ok) {
            _State |= ios_base::badbit;
        } else { // state okay, insert character
            _TRY_IO_BEGIN
            if (_Traits::eq_int_type(_Traits::eof(), _Myios::rdbuf()->sputc(_Ch))) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL write(const _Elem* _Str, streamsize _Count) {
        // insert _Count characters from array _Str
        ios_base::iostate _State = ios_base::goodbit;
        const sentry _Ok(*this);

        if (!_Ok) {
            _State |= ios_base::badbit;
        } else if (0 < _Count) { // state okay, insert characters
            _TRY_IO_BEGIN
            if (_Myios::rdbuf()->sputn(_Str, _Count) != _Count) {
                _State |= ios_base::badbit;
            }
            _CATCH_IO_END
        }

        _Myios::setstate(_State);
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL flush() { // flush output stream
        const auto _Rdbuf = _Myios::rdbuf();
        if (_Rdbuf) { // buffer exists, flush it
            const sentry _Ok(*this);

            if (_Ok) {
                ios_base::iostate _State = ios_base::goodbit;
                _TRY_IO_BEGIN
                if (_Rdbuf->pubsync() == -1) {
                    _State |= ios_base::badbit; // sync failed
                }
                _CATCH_IO_END
                _Myios::setstate(_State);
            }
        }
        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL seekp(pos_type _Pos) { // set output stream position to _Pos
        const sentry _Ok(*this);

        if (!this->fail()) {
            ios_base::iostate _State = ios_base::goodbit;
            _TRY_IO_BEGIN
            if (static_cast<off_type>(_Myios::rdbuf()->pubseekpos(_Pos, ios_base::out)) == -1) {
                _State |= ios_base::failbit; // seek failed
            }
            _CATCH_IO_END
            _Myios::setstate(_State);
        }

        return *this;
    }

    basic_ostream& __CLR_OR_THIS_CALL seekp(off_type _Off, ios_base::seekdir _Way) {
        // change output stream position by _Off, according to _Way
        const sentry _Ok(*this);

        if (!this->fail()) {
            ios_base::iostate _State = ios_base::goodbit;
            _TRY_IO_BEGIN
            if (static_cast<off_type>(_Myios::rdbuf()->pubseekoff(_Off, _Way, ios_base::out)) == -1) {
                _State |= ios_base::failbit; // seek failed
            }
            _CATCH_IO_END
            _Myios::setstate(_State);
        }

        return *this;
    }

    pos_type __CLR_OR_THIS_CALL tellp() {
        const sentry _Ok(*this);

        if (!this->fail()) {
            _TRY_IO_BEGIN
            return _Myios::rdbuf()->pubseekoff(0, ios_base::cur, ios_base::out);
            _CATCH_IO_END
        }

        return pos_type{off_type{-1}};
    }
};

#pragma vtordisp(pop) // compiler bug workaround

#ifndef _NATIVE_WCHAR_T_DEFINED
// NOTE:
// If you are not using native wchar_t, the following explicit
// specialization will mask the member function (above) that treats
// an unsigned short as an integer.

// To read or write unsigned shorts as integers with wchar_t streams,
// make wchar_t a native type with the command line option /Zc:wchar_t.

template <>
inline basic_ostream<unsigned short, char_traits<unsigned short>>& __CLR_OR_THIS_CALL basic_ostream<unsigned short,
    char_traits<unsigned short>>::operator<<(unsigned short _Ch) { // extract a character
    using _Traits = char_traits<unsigned short>;

    ios_base::iostate _State = ios_base::goodbit;
    const sentry _Ok(*this);

    if (_Ok) { // state okay, insert
        streamsize _Pad = this->width() <= 1 ? 0 : this->width() - 1;

        _TRY_IO_BEGIN
        if ((this->flags() & ios_base::adjustfield) != ios_base::left) {
            for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), this->rdbuf()->sputc(this->fill()))) {
                    _State |= ios_base::badbit;
                }
            }
        }

        if (_State == ios_base::goodbit && _Traits::eq_int_type(_Traits::eof(), this->rdbuf()->sputc(_Ch))) {
            _State |= ios_base::badbit;
        }

        for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on right
            if (_Traits::eq_int_type(_Traits::eof(), this->rdbuf()->sputc(this->fill()))) {
                _State |= ios_base::badbit;
            }
        }
        _CATCH_IO_END
    }

    this->width(0);
    _Myios::setstate(_State);
    return *this;
}
#endif // _NATIVE_WCHAR_T_DEFINED

#if defined(_DLL_CPPLIB)

#if !defined(_CRTBLD) || defined(__FORCE_INSTANCE)
template class _CRTIMP2_PURE_IMPORT basic_ostream<char, char_traits<char>>;
template class _CRTIMP2_PURE_IMPORT basic_ostream<wchar_t, char_traits<wchar_t>>;
#endif // !defined(_CRTBLD) || defined(__FORCE_INSTANCE)

#ifdef __FORCE_INSTANCE
template class _CRTIMP2_PURE_IMPORT basic_ostream<unsigned short, char_traits<unsigned short>>;
#endif // defined(__FORCE_INSTANCE)
#endif // defined(_DLL_CPPLIB)

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& operator<<(basic_ostream<_Elem, _Traits>& _Ostr, const char* _Val) { // insert NTBS
    ios_base::iostate _State = ios_base::goodbit;
    streamsize _Count        = static_cast<streamsize>(_CSTD strlen(_Val));
    streamsize _Pad          = _Ostr.width() <= 0 || _Ostr.width() <= _Count ? 0 : _Ostr.width() - _Count;
    const typename basic_ostream<_Elem, _Traits>::sentry _Ok(_Ostr);

    if (!_Ok) {
        _State |= ios_base::badbit;
    } else { // state okay, insert characters
        _TRY_IO_BEGIN
        const ctype<_Elem>& _Ctype_fac = _STD use_facet<ctype<_Elem>>(_Ostr.getloc());
        if ((_Ostr.flags() & ios_base::adjustfield) != ios_base::left) {
            for (; 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        for (; _State == ios_base::goodbit && 0 < _Count; --_Count, ++_Val) {
            if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ctype_fac.widen(*_Val)))) {
                _State |= ios_base::badbit;
            }
        }

        if (_State == ios_base::goodbit) {
            for (; 0 < _Pad; --_Pad) { // pad on right
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        _Ostr.width(0);
        _CATCH_IO_(ios_base, _Ostr)
    }

    _Ostr.setstate(_State);
    return _Ostr;
}

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& operator<<(basic_ostream<_Elem, _Traits>& _Ostr, char _Ch) { // insert a character
    ios_base::iostate _State = ios_base::goodbit;
    const typename basic_ostream<_Elem, _Traits>::sentry _Ok(_Ostr);

    if (_Ok) { // state okay, insert
        const ctype<_Elem>& _Ctype_fac = _STD use_facet<ctype<_Elem>>(_Ostr.getloc());
        streamsize _Pad                = _Ostr.width() <= 1 ? 0 : _Ostr.width() - 1;

        _TRY_IO_BEGIN
        if ((_Ostr.flags() & ios_base::adjustfield) != ios_base::left) {
            for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit;
                }
            }
        }

        if (_State == ios_base::goodbit
            && _Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ctype_fac.widen(_Ch)))) {
            _State |= ios_base::badbit;
        }

        for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on right
            if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                _State |= ios_base::badbit;
            }
        }
        _CATCH_IO_(ios_base, _Ostr)
    }

    _Ostr.width(0);
    _Ostr.setstate(_State);
    return _Ostr;
}

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>& _Ostr, const char* _Val) {
    // insert NTBS into char stream
    using _Elem = char;
    using _Myos = basic_ostream<_Elem, _Traits>;

    ios_base::iostate _State = ios_base::goodbit;
    streamsize _Count        = static_cast<streamsize>(_Traits::length(_Val));
    streamsize _Pad          = _Ostr.width() <= 0 || _Ostr.width() <= _Count ? 0 : _Ostr.width() - _Count;
    const typename _Myos::sentry _Ok(_Ostr);

    if (!_Ok) {
        _State |= ios_base::badbit;
    } else { // state okay, insert
        _TRY_IO_BEGIN
        if ((_Ostr.flags() & ios_base::adjustfield) != ios_base::left) {
            for (; 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        if (_State == ios_base::goodbit && _Ostr.rdbuf()->sputn(_Val, _Count) != _Count) {
            _State |= ios_base::badbit;
        }

        if (_State == ios_base::goodbit) {
            for (; 0 < _Pad; --_Pad) { // pad on right
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        _Ostr.width(0);
        _CATCH_IO_(ios_base, _Ostr)
    }

    _Ostr.setstate(_State);
    return _Ostr;
}

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>& _Ostr, char _Ch) {
    // insert a char into char stream
    using _Elem = char;
    using _Myos = basic_ostream<_Elem, _Traits>;

    ios_base::iostate _State = ios_base::goodbit;
    const typename _Myos::sentry _Ok(_Ostr);

    if (_Ok) { // state okay, insert
        streamsize _Pad = _Ostr.width() <= 1 ? 0 : _Ostr.width() - 1;

        _TRY_IO_BEGIN
        if ((_Ostr.flags() & ios_base::adjustfield) != ios_base::left) {
            for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit;
                }
            }
        }

        if (_State == ios_base::goodbit && _Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ch))) {
            _State |= ios_base::badbit;
        }

        for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on right
            if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                _State |= ios_base::badbit;
            }
        }
        _CATCH_IO_(ios_base, _Ostr)
    }

    _Ostr.width(0);
    _Ostr.setstate(_State);
    return _Ostr;
}

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& operator<<(basic_ostream<_Elem, _Traits>& _Ostr, const _Elem* _Val) { // insert NTCS
    using _Myos = basic_ostream<_Elem, _Traits>;

    ios_base::iostate _State = ios_base::goodbit;
    streamsize _Count        = static_cast<streamsize>(_Traits::length(_Val));
    streamsize _Pad          = _Ostr.width() <= 0 || _Ostr.width() <= _Count ? 0 : _Ostr.width() - _Count;
    const typename _Myos::sentry _Ok(_Ostr);

    if (!_Ok) {
        _State |= ios_base::badbit;
    } else { // state okay, insert
        _TRY_IO_BEGIN
        if ((_Ostr.flags() & ios_base::adjustfield) != ios_base::left) {
            for (; 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        if (_State == ios_base::goodbit && _Ostr.rdbuf()->sputn(_Val, _Count) != _Count) {
            _State |= ios_base::badbit;
        }

        if (_State == ios_base::goodbit) {
            for (; 0 < _Pad; --_Pad) { // pad on right
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        _Ostr.width(0);
        _CATCH_IO_(ios_base, _Ostr)
    }

    _Ostr.setstate(_State);
    return _Ostr;
}

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& operator<<(basic_ostream<_Elem, _Traits>& _Ostr, _Elem _Ch) { // insert a character
    using _Myos = basic_ostream<_Elem, _Traits>;

    ios_base::iostate _State = ios_base::goodbit;
    const typename _Myos::sentry _Ok(_Ostr);

    if (_Ok) { // state okay, insert
        streamsize _Pad = _Ostr.width() <= 1 ? 0 : _Ostr.width() - 1;

        _TRY_IO_BEGIN
        if ((_Ostr.flags() & ios_base::adjustfield) != ios_base::left) {
            for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= ios_base::badbit;
                }
            }
        }

        if (_State == ios_base::goodbit && _Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ch))) {
            _State |= ios_base::badbit;
        }

        for (; _State == ios_base::goodbit && 0 < _Pad; --_Pad) { // pad on right
            if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                _State |= ios_base::badbit;
            }
        }
        _CATCH_IO_(ios_base, _Ostr)
    }

    _Ostr.width(0);
    _Ostr.setstate(_State);
    return _Ostr;
}

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>& _Ostr, const signed char* _Val) {
    // insert a signed char NTBS
    return _Ostr << reinterpret_cast<const char*>(_Val);
}

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>& _Ostr, signed char _Ch) { // insert a signed char
    return _Ostr << static_cast<char>(_Ch);
}

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>& _Ostr, const unsigned char* _Val) {
    // insert an unsigned char NTBS
    return _Ostr << reinterpret_cast<const char*>(_Val);
}

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>& _Ostr, unsigned char _Ch) {
    // insert an unsigned char
    return _Ostr << static_cast<char>(_Ch);
}

#ifdef __cpp_char8_t // These deleted overloads are specified in P1423.
// don't insert a UTF-8 NTBS
_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, const char8_t*) = delete;
_EXPORT_STD template <class _Traits>
basic_ostream<wchar_t, _Traits>& operator<<(basic_ostream<wchar_t, _Traits>&, const char8_t*) = delete;

// don't insert a UTF-8 code unit
_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, char8_t) = delete;
_EXPORT_STD template <class _Traits>
basic_ostream<wchar_t, _Traits>& operator<<(basic_ostream<wchar_t, _Traits>&, char8_t) = delete;
#endif // defined(__cpp_char8_t)

#if !_HAS_STREAM_INSERTION_OPERATORS_DELETED_IN_CXX20
#ifdef _NATIVE_WCHAR_T_DEFINED
_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, wchar_t) = delete;

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, const wchar_t*) = delete;
#endif // _NATIVE_WCHAR_T_DEFINED

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, char16_t) = delete;
_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, char32_t) = delete;

_EXPORT_STD template <class _Traits>
basic_ostream<wchar_t, _Traits>& operator<<(basic_ostream<wchar_t, _Traits>&, char16_t) = delete;
_EXPORT_STD template <class _Traits>
basic_ostream<wchar_t, _Traits>& operator<<(basic_ostream<wchar_t, _Traits>&, char32_t) = delete;

_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, const char16_t*) = delete;
_EXPORT_STD template <class _Traits>
basic_ostream<char, _Traits>& operator<<(basic_ostream<char, _Traits>&, const char32_t*) = delete;

_EXPORT_STD template <class _Traits>
basic_ostream<wchar_t, _Traits>& operator<<(basic_ostream<wchar_t, _Traits>&, const char16_t*) = delete;
_EXPORT_STD template <class _Traits>
basic_ostream<wchar_t, _Traits>& operator<<(basic_ostream<wchar_t, _Traits>&, const char32_t*) = delete;
#endif // !_HAS_STREAM_INSERTION_OPERATORS_DELETED_IN_CXX20

template <class _Ostr, class _Ty, class = void>
struct _Can_stream_out : false_type {};

template <class _Ostr, class _Ty>
struct _Can_stream_out<_Ostr, _Ty, void_t<decltype(_STD declval<_Ostr&>() << _STD declval<const _Ty&>())>> : true_type {
};

_EXPORT_STD template <class _Ostr, class _Ty,
    enable_if_t<conjunction_v<is_convertible<_Ostr*, ios_base*>, _Can_stream_out<_Ostr, _Ty>>, int> = 0>
_Ostr&& operator<<(_Ostr&& _Os, const _Ty& _Val) { // insert to rvalue stream
    _Os << _Val;
    return _STD move(_Os);
}

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& __CLRCALL_OR_CDECL endl(
    basic_ostream<_Elem, _Traits>& _Ostr) { // insert newline and flush stream
    _Ostr.put(_Ostr.widen('\n'));
    _Ostr.flush();
    return _Ostr;
}

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& __CLRCALL_OR_CDECL ends(basic_ostream<_Elem, _Traits>& _Ostr) { // insert null character
    _Ostr.put(_Elem());
    return _Ostr;
}

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& __CLRCALL_OR_CDECL flush(basic_ostream<_Elem, _Traits>& _Ostr) { // flush stream
    _Ostr.flush();
    return _Ostr;
}

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& operator<<(basic_ostream<_Elem, _Traits>& _Ostr, const error_code& _Errcode) {
    // display error code
    return _Ostr << _Errcode.category().name() << ':' << _Errcode.value();
}
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_OSTREAM_HPP
