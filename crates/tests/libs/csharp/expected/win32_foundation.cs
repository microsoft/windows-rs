namespace Win32Test
{
    public enum Mode : int
    {
        None = 0,
        Offset = 1,
    }

    [StructLayout(LayoutKind.Sequential)]
    public readonly struct HMODULE : IEquatable<HMODULE>
    {
        public readonly nint Value;

        public HMODULE(nint value)
        {
            Value = value;
        }

        public static implicit operator nint(HMODULE value) => value.Value;
        public static explicit operator HMODULE(nint value) => new(value);

        public static bool operator ==(HMODULE left, HMODULE right) => left.Value == right.Value;
        public static bool operator !=(HMODULE left, HMODULE right) => !(left == right);

        public bool Equals(HMODULE other) => Value == other.Value;
        public override bool Equals(object? obj) => obj is HMODULE other && Equals(other);
        public override int GetHashCode() => Value.GetHashCode();
    }

    [StructLayout(LayoutKind.Sequential)]
    public readonly struct HWND : IEquatable<HWND>
    {
        public readonly nint Value;

        public HWND(nint value)
        {
            Value = value;
        }

        public static implicit operator nint(HWND value) => value.Value;
        public static explicit operator HWND(nint value) => new(value);

        public static bool operator ==(HWND left, HWND right) => left.Value == right.Value;
        public static bool operator !=(HWND left, HWND right) => !(left == right);

        public bool Equals(HWND other) => Value == other.Value;
        public override bool Equals(object? obj) => obj is HWND other && Equals(other);
        public override int GetHashCode() => Value.GetHashCode();
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ArchValue
    {
        public ulong value;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct FloatPair
    {
        public float x;
        public float y;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeState
    {
        public int ready;
        public byte* data;
    }

    [StructLayout(LayoutKind.Explicit)]
    public struct Number
    {
        [FieldOffset(0)]
        public int signed;
        [FieldOffset(0)]
        public uint unsigned;
        [FieldOffset(0)]
        public Win32Test.Point point;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Point
    {
        public int x;
        public int y;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Rect
    {
        public int left;
        public int top;
        public int right;
        public int bottom;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Variant
    {
        public uint tag;
        public Win32Test.Variant.Variant_1 data;

        [StructLayout(LayoutKind.Explicit)]
        public struct Variant_1
        {
            [FieldOffset(0)]
            public int signed;
            [FieldOffset(0)]
            public uint unsigned;
        }
    }

    public static unsafe partial class Apis
    {
        public const uint MAGIC = 305419896u;

        [LibraryImport("test.dll", EntryPoint = "AdjustCount")]
        private static partial int AdjustCountAbi(uint* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool AdjustCount(ref uint value)
        {
            uint _abi0 = value;
            int result = AdjustCountAbi(&_abi0);
            value = _abi0;
            return result != 0;
        }

        [LibraryImport("test.dll", EntryPoint = "AdjustPoint")]
        private static partial int AdjustPointAbi(Win32Test.Point* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static void AdjustPoint(ref Win32Test.Point value)
        {
            Win32Test.Point _abi0 = value;
            WindowsCsharp.Com.Check(AdjustPointAbi(&_abi0));
            value = _abi0;
        }

        [LibraryImport("test.dll", EntryPoint = "CompareBuffers")]
        private static partial int CompareBuffersAbi(byte* left, byte* right, uint count);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool CompareBuffers(byte* left, byte* right, uint count)
        {
            int result = CompareBuffersAbi(left, right, count);
            return result != 0;
        }

        [LibraryImport("test.dll", EntryPoint = "CompareOptionalBuffer")]
        private static partial int CompareOptionalBufferAbi(byte* left, byte* right, uint count);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool CompareOptionalBuffer(byte* left, byte* right, uint count)
        {
            int result = CompareOptionalBufferAbi(left, right, count);
            return result != 0;
        }

        [LibraryImport("ole32.dll", EntryPoint = "CreateStreamOnHGlobal")]
        private static partial int CreateStreamOnHGlobalAbi(nint global, int delete_on_release, nint* stream);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Win32Test.IStream CreateStreamOnHGlobal(nint global, bool delete_on_release)
        {
            nint stream = 0;
            int _comOutHr = CreateStreamOnHGlobalAbi(global, (delete_on_release ? 1 : 0), &stream);
            if (_comOutHr < 0)
            {
                if (stream != 0)
                {
                    _ = WindowsCsharp.Com.Release(stream);
                }
                WindowsCsharp.Com.Check(_comOutHr);
            }
            if (stream == 0)
            {
                WindowsCsharp.Com.Check(unchecked((int)0x80004003));
            }
            return WindowsCsharp.Com.Wrap<Win32Test.IStream>(stream)!;
        }

        [LibraryImport("user32.dll", EntryPoint = "EnumWindows")]
        private static partial int EnumWindowsAbi(delegate* unmanaged[Stdcall]<Win32Test.HWND, nint, int> callback, nint lparam);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool EnumWindows(delegate* unmanaged[Stdcall]<Win32Test.HWND, nint, int> callback, nint lparam)
        {
            int result = EnumWindowsAbi(callback, lparam);
            return result != 0;
        }

        [LibraryImport("test.dll", EntryPoint = "FillOptional")]
        private static partial int FillOptionalAbi(uint* values, uint count);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool FillOptional(Span<uint> values)
        {
            fixed (uint* _abi0 = values)
            {
                int result = FillOptionalAbi((uint*)_abi0, checked((uint)values.Length));
                return result != 0;
            }
        }

        [LibraryImport("test.dll", EntryPoint = "FillValues")]
        private static partial int FillValuesAbi(uint* values, uint count);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool FillValues(Span<uint> values)
        {
            fixed (uint* _abi0 = values)
            {
                int result = FillValuesAbi((uint*)_abi0, checked((uint)values.Length));
                return result != 0;
            }
        }

        [LibraryImport("test.dll", EntryPoint = "GetCount")]
        private static partial int GetCountAbi(uint* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool GetCount(out uint value)
        {
            uint _abi0;
            int result = GetCountAbi(&_abi0);
            value = _abi0;
            return result != 0;
        }

        [LibraryImport("kernel32.dll", EntryPoint = "GetCurrentProcessId")]
        public static partial uint GetCurrentProcessId();

        [LibraryImport("user32.dll", EntryPoint = "GetDesktopWindow")]
        public static partial Win32Test.HWND GetDesktopWindow();

        [LibraryImport("test.dll", EntryPoint = "GetMode")]
        private static partial uint GetModeAbi(int* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static uint GetMode(out Win32Test.Mode value)
        {
            int _abi0;
            uint result = GetModeAbi(&_abi0);
            value = (Win32Test.Mode)_abi0;
            return result;
        }

        [LibraryImport("kernel32.dll", EntryPoint = "GetModuleHandleW")]
        private static partial Win32Test.HMODULE GetModuleHandleWAbi(ushort* module_name);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Win32Test.HMODULE GetModuleHandleW(string? module_name)
        {
            fixed (char* _abi0 = module_name)
            {
                Win32Test.HMODULE result = GetModuleHandleWAbi((ushort*)_abi0);
                return result;
            }
        }

        [LibraryImport("test.dll", EntryPoint = "GetPoint")]
        private static partial int GetPointAbi(Win32Test.Point* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static void GetPoint(out Win32Test.Point value)
        {
            Win32Test.Point _abi0;
            WindowsCsharp.Com.Check(GetPointAbi(&_abi0));
            value = _abi0;
        }

        [LibraryImport("kernel32.dll", EntryPoint = "GetTempPathW")]
        private static partial uint GetTempPathWAbi(uint buffer_length, ushort* buffer);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static uint GetTempPathW(Span<char> buffer)
        {
            fixed (char* _abi1 = buffer)
            {
                uint result = GetTempPathWAbi(checked((uint)buffer.Length), (ushort*)_abi1);
                return result;
            }
        }

        [LibraryImport("user32.dll", EntryPoint = "GetWindowRect")]
        private static partial int GetWindowRectAbi(Win32Test.HWND hwnd, Win32Test.Rect* rect);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool GetWindowRect(Win32Test.HWND hwnd, out Win32Test.Rect rect)
        {
            Win32Test.Rect _abi1;
            int result = GetWindowRectAbi(hwnd, &_abi1);
            rect = _abi1;
            return result != 0;
        }

        [LibraryImport("test.dll", EntryPoint = "InspectState")]
        public static partial uint InspectState(Win32Test.NativeState value);

        [LibraryImport("test.dll", EntryPoint = "IsReady")]
        private static partial int IsReadyAbi();
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsReady()
        {
            int result = IsReadyAbi();
            return result != 0;
        }

        [LibraryImport("test.dll", EntryPoint = "NegativeBuffer")]
        public static partial uint NegativeBuffer(uint count, uint* values);

        [LibraryImport("test.dll", EntryPoint = "NegativeByteBuffer")]
        public static partial uint NegativeByteBuffer(uint count, byte* values);

        [LibraryImport("test.dll", EntryPoint = "NegativeConstantBuffer")]
        public static partial uint NegativeConstantBuffer(uint* values);

        [LibraryImport("test.dll", EntryPoint = "OutOfRangeBuffer")]
        public static partial uint OutOfRangeBuffer(uint count, uint* values);

        [LibraryImport("test.dll", EntryPoint = "OutOfRangeByteBuffer")]
        public static partial uint OutOfRangeByteBuffer(uint count, byte* values);

        [LibraryImport("test.dll", EntryPoint = "PeekValue")]
        private static partial int PeekValueAbi(uint* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool PeekValue(uint* value)
        {
            int result = PeekValueAbi(value);
            return result != 0;
        }

        [LibraryImport("kernel32.dll", EntryPoint = "QueryPerformanceCounter")]
        private static partial int QueryPerformanceCounterAbi(long* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool QueryPerformanceCounter(out long value)
        {
            long _abi0;
            int result = QueryPerformanceCounterAbi(&_abi0);
            value = _abi0;
            return result != 0;
        }

        [LibraryImport("ntdll.dll", EntryPoint = "RtlCompareMemoryUlong")]
        private static partial nuint RtlCompareMemoryUlongAbi(void* source, nuint length, uint pattern);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static nuint RtlCompareMemoryUlong(ReadOnlySpan<byte> source, uint pattern)
        {
            fixed (byte* _abi0 = source)
            {
                nuint result = RtlCompareMemoryUlongAbi((void*)_abi0, checked((nuint)source.Length), pattern);
                return result;
            }
        }

        [LibraryImport("test.dll", EntryPoint = "SelfByteBuffer")]
        public static partial uint SelfByteBuffer(byte* values);

        [LibraryImport("test.dll", EntryPoint = "SelfRelativeBuffer")]
        public static partial uint SelfRelativeBuffer(uint* values);

        [LibraryImport("test.dll", EntryPoint = "SumPoints")]
        private static partial int SumPointsAbi(Win32Test.Point* points, uint count);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static int SumPoints(ReadOnlySpan<Win32Test.Point> points)
        {
            fixed (Win32Test.Point* _abi0 = points)
            {
                int result = SumPointsAbi((Win32Test.Point*)_abi0, checked((uint)points.Length));
                return result;
            }
        }

        [LibraryImport("test.dll", EntryPoint = "Transform")]
        public static partial Win32Test.Point Transform(Win32Test.Point value, Win32Test.Mode mode);

        [LibraryImport("test.dll", EntryPoint = "TransformArch")]
        public static partial Win32Test.ArchValue TransformArch(Win32Test.ArchValue value);

        [LibraryImport("test.dll", EntryPoint = "TransformNumber")]
        public static partial Win32Test.Number TransformNumber(Win32Test.Number value);

        [LibraryImport("test.dll", EntryPoint = "TransformVariant")]
        public static partial Win32Test.Variant TransformVariant(Win32Test.Variant value);

        [LibraryImport("test.dll", EntryPoint = "TryGetOptional")]
        private static partial int TryGetOptionalAbi(uint* value);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool TryGetOptional(uint* value)
        {
            int result = TryGetOptionalAbi(value);
            return result != 0;
        }

        [LibraryImport("test.dll", EntryPoint = "ValidBuffer")]
        private static partial uint ValidBufferAbi(uint count, uint* values);
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static uint ValidBuffer(ReadOnlySpan<uint> values)
        {
            fixed (uint* _abi1 = values)
            {
                uint result = ValidBufferAbi(checked((uint)values.Length), (uint*)_abi1);
                return result;
            }
        }

        [LibraryImport("test.dll", EntryPoint = "ValidConstantBuffer")]
        public static partial uint ValidConstantBuffer(uint* values);
    }

    public sealed unsafe class IBaseDependency : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IBaseDependency>, WindowsCsharp.IObjectParameter<Win32Test.IBaseDependency._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x33445566, 0x7788, 0x99aa, 0xb0, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67);

        internal IBaseDependency(nint self) : base(self, Iid) {}
        internal IBaseDependency(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IBaseDependency WindowsCsharp.IComInterface<IBaseDependency>.FromAbi(nint self) => new IBaseDependency(self);
        static IBaseDependency WindowsCsharp.IComInterface<IBaseDependency>.FromAgileAbi(nint self) => new IBaseDependency(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UsePoint(Win32Test.Point value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Win32Test.Point, int>)(*(void***)self)[3])(self, value));
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
            public void UsePoint(Win32Test.Point value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Win32Test.Point, int>)(*(void***)self)[3])(self, value));
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

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("33445566-7788-99aa-b0ab-cdef01234567")]
    public unsafe partial interface IBaseDependencyAbi
    {
        [PreserveSig]
        int UsePoint(Win32Test.Point value);
    }

    public sealed unsafe class IBufferOps : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IBufferOps>, WindowsCsharp.IObjectParameter<Win32Test.IBufferOps._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x22334455, 0x6677, 0x8899, 0xa0, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67);

        internal IBufferOps(nint self) : base(self, Iid) {}
        internal IBufferOps(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IBufferOps WindowsCsharp.IComInterface<IBufferOps>.FromAbi(nint self) => new IBufferOps(self);
        static IBufferOps WindowsCsharp.IComInterface<IBufferOps>.FromAgileAbi(nint self) => new IBufferOps(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint Sum(ReadOnlySpan<byte> values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            fixed (byte* _abi0 = values)
            {
                uint result = ((delegate* unmanaged<nint, byte*, uint, uint>)(*(void***)self)[3])(self, (byte*)_abi0, checked((uint)values.Length));
                return result;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Fill(Span<uint> values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            fixed (uint* _abi0 = values)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, uint, int>)(*(void***)self)[4])(self, (uint*)_abi0, checked((uint)values.Length)));
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

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint Sum(ReadOnlySpan<byte> values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                fixed (byte* _abi0 = values)
                {
                    uint result = ((delegate* unmanaged<nint, byte*, uint, uint>)(*(void***)self)[3])(self, (byte*)_abi0, checked((uint)values.Length));
                    return result;
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Fill(Span<uint> values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                fixed (uint* _abi0 = values)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, uint, int>)(*(void***)self)[4])(self, (uint*)_abi0, checked((uint)values.Length)));
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

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("22334455-6677-8899-a0ab-cdef01234567")]
    public unsafe partial interface IBufferOpsAbi
    {
        [PreserveSig]
        uint Sum(byte* values, uint count);
        [PreserveSig]
        int Fill(uint* values, uint count);
    }

    public sealed unsafe class ICounter : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<ICounter>, WindowsCsharp.IObjectParameter<Win32Test.ICounter._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x12345678, 0x1234, 0x5678, 0x90, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67);

        internal ICounter(nint self) : base(self, Iid) {}
        internal ICounter(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static ICounter WindowsCsharp.IComInterface<ICounter>.FromAbi(nint self) => new ICounter(self);
        static ICounter WindowsCsharp.IComInterface<ICounter>.FromAgileAbi(nint self) => new ICounter(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int GetValue()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result = ((delegate* unmanaged<nint, int>)(*(void***)self)[3])(self);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Add(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result = ((delegate* unmanaged<nint, int, int>)(*(void***)self)[4])(self, value);
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
            public int GetValue()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result = ((delegate* unmanaged<nint, int>)(*(void***)self)[3])(self);
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Add(int value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result = ((delegate* unmanaged<nint, int, int>)(*(void***)self)[4])(self, value);
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

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("12345678-1234-5678-90ab-cdef01234567")]
    public unsafe partial interface ICounterAbi
    {
        [PreserveSig]
        int GetValue();
        [PreserveSig]
        int Add(int value);
    }

    public sealed unsafe class IDerivedDependency : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IDerivedDependency>, WindowsCsharp.IObjectParameter<Win32Test.IDerivedDependency._Parameter>, WindowsCsharp.IObjectParameter<Win32Test.IBaseDependency._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x44556677, 0x8899, 0xaabb, 0xc0, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67);

        internal IDerivedDependency(nint self) : base(self, Iid) {}
        internal IDerivedDependency(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IDerivedDependency WindowsCsharp.IComInterface<IDerivedDependency>.FromAbi(nint self) => new IDerivedDependency(self);
        static IDerivedDependency WindowsCsharp.IComInterface<IDerivedDependency>.FromAgileAbi(nint self) => new IDerivedDependency(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UsePoint(Win32Test.Point value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Win32Test.Point, int>)(*(void***)self)[3])(self, value));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UseMode(Win32Test.Mode value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[4])(self, (int)value));
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
            public void UsePoint(Win32Test.Point value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Win32Test.Point, int>)(*(void***)self)[3])(self, value));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void UseMode(Win32Test.Mode value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[4])(self, (int)value));
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

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void BorrowAs(Win32Test.IBaseDependency.BorrowAction action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Win32Test.IBaseDependency.Iid);
            action(new Win32Test.IBaseDependency.Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult BorrowAs<TResult>(Win32Test.IBaseDependency.BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Win32Test.IBaseDependency.Iid);
            return action(new Win32Test.IBaseDependency.Borrowed(lease.Handle));
        }
    }

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("44556677-8899-aabb-c0ab-cdef01234567")]
    public unsafe partial interface IDerivedDependencyAbi : Win32Test.IBaseDependencyAbi
    {
        [PreserveSig]
        int UseMode(int value);
    }

    public sealed unsafe class IHandleConsumer : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IHandleConsumer>, WindowsCsharp.IObjectParameter<Win32Test.IHandleConsumer._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x11223344, 0x5566, 0x7788, 0x90, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67);

        internal IHandleConsumer(nint self) : base(self, Iid) {}
        internal IHandleConsumer(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IHandleConsumer WindowsCsharp.IComInterface<IHandleConsumer>.FromAbi(nint self) => new IHandleConsumer(self);
        static IHandleConsumer WindowsCsharp.IComInterface<IHandleConsumer>.FromAgileAbi(nint self) => new IHandleConsumer(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void SetWindow(Win32Test.HWND value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Win32Test.HWND, int>)(*(void***)self)[3])(self, value));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Win32Test.ICounter CreateCounter(int seed)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint counter = 0;
            int _comOutHr = ((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[4])(self, seed, &counter);
            if (_comOutHr < 0)
            {
                if (counter != 0)
                {
                    _ = WindowsCsharp.Com.Release(counter);
                }
                WindowsCsharp.Com.Check(_comOutHr);
            }
            if (counter == 0)
            {
                WindowsCsharp.Com.Check(unchecked((int)0x80004003));
            }
            return WindowsCsharp.Com.Wrap<Win32Test.ICounter>(counter)!;
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
            public void SetWindow(Win32Test.HWND value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Win32Test.HWND, int>)(*(void***)self)[3])(self, value));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Win32Test.ICounter CreateCounter(int seed)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint counter = 0;
                int _comOutHr = ((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[4])(self, seed, &counter);
                if (_comOutHr < 0)
                {
                    if (counter != 0)
                    {
                        _ = WindowsCsharp.Com.Release(counter);
                    }
                    WindowsCsharp.Com.Check(_comOutHr);
                }
                if (counter == 0)
                {
                    WindowsCsharp.Com.Check(unchecked((int)0x80004003));
                }
                return WindowsCsharp.Com.Wrap<Win32Test.ICounter>(counter)!;
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

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("11223344-5566-7788-90ab-cdef01234567")]
    public unsafe partial interface IHandleConsumerAbi
    {
        [PreserveSig]
        int SetWindow(Win32Test.HWND value);
        [PreserveSig]
        int CreateCounter(int seed, nint* counter);
    }

    public sealed unsafe class INativeAbiCases : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<INativeAbiCases>, WindowsCsharp.IObjectParameter<Win32Test.INativeAbiCases._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x10203040, 0x5060, 0x7080, 0x90, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67);

        internal INativeAbiCases(nint self) : base(self, Iid) {}
        internal INativeAbiCases(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static INativeAbiCases WindowsCsharp.IComInterface<INativeAbiCases>.FromAbi(nint self) => new INativeAbiCases(self);
        static INativeAbiCases WindowsCsharp.IComInterface<INativeAbiCases>.FromAgileAbi(nint self) => new INativeAbiCases(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Win32Test.Point GetPoint()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Win32Test.Point result = default;
            ((delegate* unmanaged<nint, Win32Test.Point*, void>)(*(void***)self)[3])(self, &result);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Win32Test.FloatPair GetFloatPair()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Win32Test.FloatPair result = default;
            ((delegate* unmanaged<nint, Win32Test.FloatPair*, void>)(*(void***)self)[4])(self, &result);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Win32Test.ICounter CreateCounter(int seed)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint counter = 0;
            int _comOutHr = ((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[5])(self, seed, &counter);
            if (_comOutHr < 0)
            {
                if (counter != 0)
                {
                    _ = WindowsCsharp.Com.Release(counter);
                }
                WindowsCsharp.Com.Check(_comOutHr);
            }
            if (counter == 0)
            {
                WindowsCsharp.Com.Check(unchecked((int)0x80004003));
            }
            return WindowsCsharp.Com.Wrap<Win32Test.ICounter>(counter)!;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UpdateCounter(nint* counter)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, counter));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void TryCreateCounter(nint* counter)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, counter));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void ReservedCounter(nint* counter)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, counter));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void CreateCounterBuffer(nint* counters, uint count)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, uint, int>)(*(void***)self)[9])(self, counters, count));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void CreateTwoCounters(nint* first, nint* second)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, nint*, int>)(*(void***)self)[10])(self, first, second));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Win32Test.ICounter CreateCounterWithStatus(out uint status)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint counter = 0;
            uint _abi0;
            int _comOutHr = ((delegate* unmanaged<nint, uint*, nint*, int>)(*(void***)self)[11])(self, &_abi0, &counter);
            if (_comOutHr < 0)
            {
                if (counter != 0)
                {
                    _ = WindowsCsharp.Com.Release(counter);
                }
                WindowsCsharp.Com.Check(_comOutHr);
            }
            if (counter == 0)
            {
                WindowsCsharp.Com.Check(unchecked((int)0x80004003));
            }
            status = _abi0;
            return WindowsCsharp.Com.Wrap<Win32Test.ICounter>(counter)!;
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
            public Win32Test.Point GetPoint()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                Win32Test.Point result = default;
                ((delegate* unmanaged<nint, Win32Test.Point*, void>)(*(void***)self)[3])(self, &result);
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Win32Test.FloatPair GetFloatPair()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                Win32Test.FloatPair result = default;
                ((delegate* unmanaged<nint, Win32Test.FloatPair*, void>)(*(void***)self)[4])(self, &result);
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Win32Test.ICounter CreateCounter(int seed)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint counter = 0;
                int _comOutHr = ((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[5])(self, seed, &counter);
                if (_comOutHr < 0)
                {
                    if (counter != 0)
                    {
                        _ = WindowsCsharp.Com.Release(counter);
                    }
                    WindowsCsharp.Com.Check(_comOutHr);
                }
                if (counter == 0)
                {
                    WindowsCsharp.Com.Check(unchecked((int)0x80004003));
                }
                return WindowsCsharp.Com.Wrap<Win32Test.ICounter>(counter)!;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void UpdateCounter(nint* counter)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, counter));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void TryCreateCounter(nint* counter)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, counter));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void ReservedCounter(nint* counter)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, counter));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void CreateCounterBuffer(nint* counters, uint count)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, uint, int>)(*(void***)self)[9])(self, counters, count));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void CreateTwoCounters(nint* first, nint* second)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, nint*, int>)(*(void***)self)[10])(self, first, second));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Win32Test.ICounter CreateCounterWithStatus(out uint status)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint counter = 0;
                uint _abi0;
                int _comOutHr = ((delegate* unmanaged<nint, uint*, nint*, int>)(*(void***)self)[11])(self, &_abi0, &counter);
                if (_comOutHr < 0)
                {
                    if (counter != 0)
                    {
                        _ = WindowsCsharp.Com.Release(counter);
                    }
                    WindowsCsharp.Com.Check(_comOutHr);
                }
                if (counter == 0)
                {
                    WindowsCsharp.Com.Check(unchecked((int)0x80004003));
                }
                status = _abi0;
                return WindowsCsharp.Com.Wrap<Win32Test.ICounter>(counter)!;
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

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("10203040-5060-7080-90ab-cdef01234567")]
    public unsafe partial interface INativeAbiCasesAbi
    {
        [PreserveSig]
        void GetPoint(Win32Test.Point* result__);
        [PreserveSig]
        void GetFloatPair(Win32Test.FloatPair* result__);
        [PreserveSig]
        int CreateCounter(int seed, nint* counter);
        [PreserveSig]
        int UpdateCounter(nint* counter);
        [PreserveSig]
        int TryCreateCounter(nint* counter);
        [PreserveSig]
        int ReservedCounter(nint* counter);
        [PreserveSig]
        int CreateCounterBuffer(nint* counters, uint count);
        [PreserveSig]
        int CreateTwoCounters(nint* first, nint* second);
        [PreserveSig]
        int CreateCounterWithStatus(uint* status, nint* counter);
    }

    public sealed unsafe class ISequentialStream : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<ISequentialStream>, WindowsCsharp.IObjectParameter<Win32Test.ISequentialStream._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x0c733a30, 0x2a1c, 0x11ce, 0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d);

        internal ISequentialStream(nint self) : base(self, Iid) {}
        internal ISequentialStream(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static ISequentialStream WindowsCsharp.IComInterface<ISequentialStream>.FromAbi(nint self) => new ISequentialStream(self);
        static ISequentialStream WindowsCsharp.IComInterface<ISequentialStream>.FromAgileAbi(nint self) => new ISequentialStream(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Read(Span<byte> value, out uint read)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _abi2;
            fixed (byte* _abi0 = value)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[3])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                read = _abi2;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Write(ReadOnlySpan<byte> value, out uint written)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _abi2;
            fixed (byte* _abi0 = value)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[4])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                written = _abi2;
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

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Read(Span<byte> value, out uint read)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _abi2;
                fixed (byte* _abi0 = value)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[3])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                    read = _abi2;
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Write(ReadOnlySpan<byte> value, out uint written)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _abi2;
                fixed (byte* _abi0 = value)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[4])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                    written = _abi2;
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

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("0c733a30-2a1c-11ce-ade5-00aa0044773d")]
    public unsafe partial interface ISequentialStreamAbi
    {
        [PreserveSig]
        int Read(byte* value, uint count, uint* read);
        [PreserveSig]
        int Write(byte* value, uint count, uint* written);
    }

    public sealed unsafe class IStream : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IStream>, WindowsCsharp.IObjectParameter<Win32Test.IStream._Parameter>, WindowsCsharp.IObjectParameter<Win32Test.ISequentialStream._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x0000000c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);

        internal IStream(nint self) : base(self, Iid) {}
        internal IStream(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IStream WindowsCsharp.IComInterface<IStream>.FromAbi(nint self) => new IStream(self);
        static IStream WindowsCsharp.IComInterface<IStream>.FromAgileAbi(nint self) => new IStream(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Read(Span<byte> value, out uint read)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _abi2;
            fixed (byte* _abi0 = value)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[3])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                read = _abi2;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Write(ReadOnlySpan<byte> value, out uint written)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _abi2;
            fixed (byte* _abi0 = value)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[4])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                written = _abi2;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Seek(long offset, uint origin, out ulong position)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            ulong _abi2;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, uint, ulong*, int>)(*(void***)self)[5])(self, offset, origin, &_abi2));
            position = _abi2;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void SetSize(ulong size)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ulong, int>)(*(void***)self)[6])(self, size));
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
            public void Read(Span<byte> value, out uint read)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _abi2;
                fixed (byte* _abi0 = value)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[3])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                    read = _abi2;
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Write(ReadOnlySpan<byte> value, out uint written)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _abi2;
                fixed (byte* _abi0 = value)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, uint, uint*, int>)(*(void***)self)[4])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2));
                    written = _abi2;
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Seek(long offset, uint origin, out ulong position)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                ulong _abi2;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, uint, ulong*, int>)(*(void***)self)[5])(self, offset, origin, &_abi2));
                position = _abi2;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void SetSize(ulong size)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ulong, int>)(*(void***)self)[6])(self, size));
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

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void BorrowAs(Win32Test.ISequentialStream.BorrowAction action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Win32Test.ISequentialStream.Iid);
            action(new Win32Test.ISequentialStream.Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult BorrowAs<TResult>(Win32Test.ISequentialStream.BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Win32Test.ISequentialStream.Iid);
            return action(new Win32Test.ISequentialStream.Borrowed(lease.Handle));
        }
    }

    [System.Runtime.InteropServices.Marshalling.GeneratedComInterface]
    [Guid("0000000c-0000-0000-c000-000000000046")]
    public unsafe partial interface IStreamAbi : Win32Test.ISequentialStreamAbi
    {
        [PreserveSig]
        int Seek(long offset, uint origin, ulong* position);
        [PreserveSig]
        int SetSize(ulong size);
    }

    public sealed unsafe class IUnsupportedDerived : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IUnsupportedDerived>, WindowsCsharp.IObjectParameter<Win32Test.IUnsupportedDerived._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x778899aa, 0xbbcc, 0xddee, 0xf0, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67);

        internal IUnsupportedDerived(nint self) : base(self, Iid) {}
        internal IUnsupportedDerived(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IUnsupportedDerived WindowsCsharp.IComInterface<IUnsupportedDerived>.FromAbi(nint self) => new IUnsupportedDerived(self);
        static IUnsupportedDerived WindowsCsharp.IComInterface<IUnsupportedDerived>.FromAgileAbi(nint self) => new IUnsupportedDerived(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Good(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[4])(self, value));
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
            public void Good(int value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[4])(self, value));
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
