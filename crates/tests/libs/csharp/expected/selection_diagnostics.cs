namespace SelectionDiagnostics
{
    public enum Mode : int
    {
        Off = 0,
        On = 1,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Point
    {
        public int X;
        public int Y;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct TextValue
    {
        public string Text;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct TextValueAbi
    {
        public nint Text;

        internal static TextValueAbi FromSurface(TextValue value)
        {
            TextValueAbi result = default;
            try
            {
                result.Text = WindowsCsharp.Interop.CreateString(value.Text);
                return result;
            }
            catch
            {
                result.Dispose();
                throw;
            }
        }

        internal readonly TextValue FromAbi() => new()
        {
            Text = WindowsCsharp.Interop.FromHstringBorrowed(Text),
        };

        internal TextValue ToSurface()
        {
            TextValue result = default;
            try
            {
                result.Text = WindowsCsharp.Interop.TakeHstring(ref Text);
                return result;
            }
            finally
            {
                Dispose();
            }
        }

        internal void Dispose()
        {
            WindowsCsharp.Interop.DeleteHstring(ref Text);
        }
    }

    public sealed unsafe class SupportedCallback : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<SupportedCallback>, WindowsCsharp.IObjectParameter<SelectionDiagnostics.SupportedCallback._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x886b5c65, 0x9057, 0x58d2, 0xa5, 0x28, 0xe0, 0xc9, 0xc1, 0x18, 0xa3, 0x4f);

        internal SupportedCallback(nint self) : base(self, Iid) {}
        internal SupportedCallback(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static SupportedCallback WindowsCsharp.IComInterface<SupportedCallback>.FromAbi(nint self) => new SupportedCallback(self);
        static SupportedCallback WindowsCsharp.IComInterface<SupportedCallback>.FromAgileAbi(nint self) => new SupportedCallback(self, true);

        public delegate int Callback(int value);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, int, int*, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static SupportedCallback Create(Callback handler) => WindowsCsharp.Com.WrapAgile<SupportedCallback>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, int value, int* result)
        {
            if (result == null)
            {
                return unchecked((int)0x80004003);
            }
            *result = default;
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                *result = callback(value);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Invoke(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[3])(self, value, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return WindowsCsharp.Com.As<T>(self, lease.TrustedAgile);
        }

        public delegate void BorrowAction(Borrowed value);
        public delegate TResult BorrowFunc<TResult>(Borrowed value);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Borrow(BorrowAction action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            action(new Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult Borrow<TResult>(BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            return action(new Borrowed(lease.Handle));
        }

        public readonly ref struct Borrowed
        {
            private readonly nint _this;
            internal Borrowed(nint self) => _this = self;
            public bool IsNull => _this == 0;

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                return WindowsCsharp.Com.As<T>(self, false);
            }
        }
    }

    public sealed unsafe class Constructed : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Constructed>, WindowsCsharp.IObjectParameter<SelectionDiagnostics.Constructed._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x9e6713cd, 0x8824, 0x5fec, 0x8d, 0x67, 0x73, 0xe7, 0xf7, 0x3a, 0x69, 0xfc);

        internal Constructed(nint self) : base(self, Iid) {}
        internal Constructed(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Constructed WindowsCsharp.IComInterface<Constructed>.FromAbi(nint self) => new Constructed(self);
        static Constructed WindowsCsharp.IComInterface<Constructed>.FromAgileAbi(nint self) => new Constructed(self, true);

        public int Value
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                int value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[7])(self, value));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return WindowsCsharp.Com.As<T>(self, lease.TrustedAgile);
        }

        public delegate void BorrowAction(Borrowed value);
        public delegate TResult BorrowFunc<TResult>(Borrowed value);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Borrow(BorrowAction action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            action(new Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult Borrow<TResult>(BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            return action(new Borrowed(lease.Handle));
        }

        public readonly ref struct Borrowed
        {
            private readonly nint _this;
            internal Borrowed(nint self) => _this = self;
            public bool IsNull => _this == 0;

            public int Value
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    int value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &value));
                    return value;
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[7])(self, value));
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                return WindowsCsharp.Com.As<T>(self, false);
            }
        }
    }

    public sealed unsafe class Diagnostics : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Diagnostics>, WindowsCsharp.IObjectParameter<SelectionDiagnostics.Diagnostics._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x2a141d2e, 0x41ad, 0x5fc2, 0xa8, 0x50, 0x31, 0x01, 0xab, 0xd2, 0x11, 0x62);

        internal Diagnostics(nint self) : base(self, Iid) {}
        internal Diagnostics(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Diagnostics WindowsCsharp.IComInterface<Diagnostics>.FromAbi(nint self) => new Diagnostics(self);
        static Diagnostics WindowsCsharp.IComInterface<Diagnostics>.FromAgileAbi(nint self) => new Diagnostics(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Good(SelectionDiagnostics.SupportedCallback? callback, SelectionDiagnostics.Point point, SelectionDiagnostics.Mode mode)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, SelectionDiagnostics.Point, int, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, point, (int)mode, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Good<T0>(T0? callback, SelectionDiagnostics.Point point, SelectionDiagnostics.Mode mode) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<SelectionDiagnostics.SupportedCallback._Parameter>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(callback, SelectionDiagnostics.SupportedCallback.Iid);
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, SelectionDiagnostics.Point, int, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, point, (int)mode, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IVector<int>? GoodVector()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[10])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint VectorArray(Windows.Foundation.Collections.IVector<int>?[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint result;
            using WindowsCsharp.ObjectArrayLease _alease0 = WindowsCsharp.ObjectArrayLease.From(values);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[11])(self, (uint)values.Length, _alease0.Values, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return WindowsCsharp.Com.As<T>(self, lease.TrustedAgile);
        }

        public delegate void BorrowAction(Borrowed value);
        public delegate TResult BorrowFunc<TResult>(Borrowed value);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Borrow(BorrowAction action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            action(new Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult Borrow<TResult>(BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            return action(new Borrowed(lease.Handle));
        }

        public readonly ref struct Borrowed
        {
            private readonly nint _this;
            internal Borrowed(nint self) => _this = self;
            public bool IsNull => _this == 0;

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Good(SelectionDiagnostics.SupportedCallback? callback, SelectionDiagnostics.Point point, SelectionDiagnostics.Mode mode)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, SelectionDiagnostics.Point, int, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, point, (int)mode, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Good<T0>(T0? callback, SelectionDiagnostics.Point point, SelectionDiagnostics.Mode mode) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<SelectionDiagnostics.SupportedCallback._Parameter>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(callback, SelectionDiagnostics.SupportedCallback.Iid);
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, SelectionDiagnostics.Point, int, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, point, (int)mode, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IVector<int>? GoodVector()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[10])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint VectorArray(Windows.Foundation.Collections.IVector<int>?[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint result;
                using WindowsCsharp.ObjectArrayLease _alease0 = WindowsCsharp.ObjectArrayLease.From(values);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[11])(self, (uint)values.Length, _alease0.Values, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                return WindowsCsharp.Com.As<T>(self, false);
            }
        }
    }

    public sealed unsafe class StaticDiagnostics : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<StaticDiagnostics>, WindowsCsharp.IObjectParameter<SelectionDiagnostics.StaticDiagnostics._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x77d706e4, 0x278c, 0x5c28, 0x88, 0xf1, 0x79, 0xc5, 0x21, 0x26, 0x54, 0xba);

        internal StaticDiagnostics(nint self) : base(self, Iid) {}
        internal StaticDiagnostics(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static StaticDiagnostics WindowsCsharp.IComInterface<StaticDiagnostics>.FromAbi(nint self) => new StaticDiagnostics(self);
        static StaticDiagnostics WindowsCsharp.IComInterface<StaticDiagnostics>.FromAgileAbi(nint self) => new StaticDiagnostics(self, true);

        public int Value
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                int value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[7])(self, value));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return WindowsCsharp.Com.As<T>(self, lease.TrustedAgile);
        }

        public delegate void BorrowAction(Borrowed value);
        public delegate TResult BorrowFunc<TResult>(Borrowed value);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Borrow(BorrowAction action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            action(new Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult Borrow<TResult>(BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            return action(new Borrowed(lease.Handle));
        }

        public readonly ref struct Borrowed
        {
            private readonly nint _this;
            internal Borrowed(nint self) => _this = self;
            public bool IsNull => _this == 0;

            public int Value
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    int value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &value));
                    return value;
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[7])(self, value));
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                return WindowsCsharp.Com.As<T>(self, false);
            }
        }
    }
}

namespace Windows.Foundation.Collections
{
    public sealed unsafe class IVector<T> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IVector<T>> where T : unmanaged
    {
        public static Guid Iid { get; } = ComputeIid();

        private static Guid ComputeIid()
        {
            if (typeof(T) == typeof(int)) return new Guid(0xb939af5b, 0xb45d, 0x5489, 0x91, 0x49, 0x61, 0x44, 0x2c, 0x19, 0x05, 0xfe);
            throw new NotSupportedException();
        }

        internal IVector(nint self) : base(self, Iid) {}
        internal IVector(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IVector<T> WindowsCsharp.IComInterface<IVector<T>>.FromAbi(nint self) => new IVector<T>(self);
        static IVector<T> WindowsCsharp.IComInterface<IVector<T>>.FromAgileAbi(nint self) => new IVector<T>(self, true);

        public uint Count
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                uint value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[7])(self, &value));
                return value;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T GetAt(uint index)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            T result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, T*, int>)(*(void***)self)[6])(self, index, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint GetMany(uint startIndex, Span<T> items)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint actual;
            fixed (T* p = items)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, T*, uint*, int>)(*(void***)self)[16])(self, startIndex, (uint)items.Length, p, &actual));
            }
            return actual;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Append(T value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, T, int>)(*(void***)self)[13])(self, value));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void RemoveAtEnd()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[14])(self));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Clear()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[15])(self));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Enumerator GetEnumerator() => new Enumerator(this);

        public struct Enumerator
        {
            private const int BufferLength = 64;
            private readonly IVector<T> _vector;
            private Buffer _buffer;
            private uint _start;
            private int _index;
            private int _length;
            private T _current;

            internal Enumerator(IVector<T> vector)
            {
                _vector = vector;
                _buffer = default;
                _start = 0;
                _index = 0;
                _length = 0;
                _current = default!;
            }

            public readonly T Current => _current;

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool MoveNext()
            {
                if (_index >= _length)
                {
                    _index = 0;
                    _length = (int)_vector.GetMany(_start, MemoryMarshal.CreateSpan(ref Unsafe.As<Buffer, T>(ref _buffer), BufferLength));
                    _start += (uint)_length;
                    if (_length == 0)
                    {
                        return false;
                    }
                }
                _current = Unsafe.Add(ref Unsafe.As<Buffer, T>(ref _buffer), _index);
                _index++;
                return true;
            }

            [InlineArray(BufferLength)]
            private struct Buffer
            {
                private T _element0;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            return WindowsCsharp.Com.As<TInterface>(lease.Handle, lease.TrustedAgile);
        }

        public delegate void BorrowAction(Borrowed value);
        public delegate TResult BorrowFunc<TResult>(Borrowed value);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Borrow(BorrowAction action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            action(new Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult Borrow<TResult>(BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            return action(new Borrowed(lease.Handle));
        }

        public readonly ref struct Borrowed
        {
            private readonly nint _this;
            internal Borrowed(nint self) => _this = self;

            public uint Count
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    uint value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)_this)[7])(_this, &value));
                    return value;
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public T GetAt(uint index)
            {
                T result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, T*, int>)(*(void***)_this)[6])(_this, index, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint GetMany(uint startIndex, Span<T> items)
            {
                uint actual;
                fixed (T* p = items)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, T*, uint*, int>)(*(void***)_this)[16])(_this, startIndex, (uint)items.Length, p, &actual));
                }
                return actual;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Append(T value) => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, T, int>)(*(void***)_this)[13])(_this, value));

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void RemoveAtEnd() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[14])(_this));

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Clear() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[15])(_this));

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface> => WindowsCsharp.Com.As<TInterface>(_this, false);
        }
    }
}
