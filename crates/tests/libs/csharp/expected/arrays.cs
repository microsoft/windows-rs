namespace Sample
{
    public enum Mode : int
    {
        None = 0,
        One = 1,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct State
    {
        public bool enabled;
        public Sample.Mode mode;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StateAbi
    {
        public byte enabled;
        public int mode;

        internal static StateAbi FromSurface(State value) => new()
        {
            enabled = (value.enabled ? (byte)1 : (byte)0),
            mode = (int)value.mode,
        };

        internal readonly State ToSurface() => new()
        {
            enabled = enabled != 0,
            mode = (Sample.Mode)mode,
        };
    }

    public sealed unsafe class Arrays : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Arrays>, WindowsCsharp.IObjectParameter<Sample.Arrays._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x7607015b, 0x4d9d, 0x5f94, 0x9f, 0xce, 0x86, 0xbd, 0x9f, 0xa5, 0x84, 0x69);

        internal Arrays(nint self) : base(self, Iid) {}
        internal Arrays(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Arrays WindowsCsharp.IComInterface<Arrays>.FromAbi(nint self) => new Arrays(self);
        static Arrays WindowsCsharp.IComInterface<Arrays>.FromAgileAbi(nint self) => new Arrays(self, true);

        public Arrays() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Arrays", Iid), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Sum(int[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            fixed (int* _aptr0 = values)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int*, int*, int>)(*(void***)self)[6])(self, (uint)values.Length, _aptr0, &result));
            }
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int[] Values()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint resultSize = 0;
            int* result = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[7])(self, &resultSize, &result));
                return WindowsCsharp.Interop.FromArray<int, int>(ref resultSize, ref result);
            }
            finally
            {
                Marshal.FreeCoTaskMem((nint)result);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void GetValues(out int[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _asize0 = 0;
            int* _adata0 = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[8])(self, &_asize0, &_adata0));
                values = WindowsCsharp.Interop.FromArray<int, int>(ref _asize0, ref _adata0);
            }
            finally
            {
                Marshal.FreeCoTaskMem((nint)_adata0);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint CountTrue(bool[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint result;
            fixed (bool* _aptr0 = values)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, byte*, uint*, int>)(*(void***)self)[9])(self, (uint)values.Length, (byte*)_aptr0, &result));
            }
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool[] Booleans()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint resultSize = 0;
            byte* result = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, byte**, int>)(*(void***)self)[10])(self, &resultSize, &result));
                return WindowsCsharp.Interop.FromBooleanArray(ref resultSize, ref result);
            }
            finally
            {
                Marshal.FreeCoTaskMem((nint)result);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void GetBooleans(out bool[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _asize0 = 0;
            byte* _adata0 = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, byte**, int>)(*(void***)self)[11])(self, &_asize0, &_adata0));
                values = WindowsCsharp.Interop.FromBooleanArray(ref _asize0, ref _adata0);
            }
            finally
            {
                Marshal.FreeCoTaskMem((nint)_adata0);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int SumModes(Sample.Mode[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            fixed (Sample.Mode* _aptr0 = values)
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int*, int*, int>)(*(void***)self)[12])(self, (uint)values.Length, (int*)_aptr0, &result));
            }
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.Mode[] Modes()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint resultSize = 0;
            int* result = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[13])(self, &resultSize, &result));
                return WindowsCsharp.Interop.FromArray<Sample.Mode, int>(ref resultSize, ref result);
            }
            finally
            {
                Marshal.FreeCoTaskMem((nint)result);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void GetModes(out Sample.Mode[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _asize0 = 0;
            int* _adata0 = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[14])(self, &_asize0, &_adata0));
                values = WindowsCsharp.Interop.FromArray<Sample.Mode, int>(ref _asize0, ref _adata0);
            }
            finally
            {
                Marshal.FreeCoTaskMem((nint)_adata0);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool InspectState(Sample.State value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            byte result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.StateAbi, byte*, int>)(*(void***)self)[15])(self, Sample.StateAbi.FromSurface(value), &result));
            return result != 0;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.State CurrentState()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Sample.StateAbi result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.StateAbi*, int>)(*(void***)self)[16])(self, &result));
            return result.ToSurface();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint CountStringUnits(string[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint result;
            using WindowsCsharp.StringArrayLease _alease0 = WindowsCsharp.StringArrayLease.From(values);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[17])(self, (uint)values.Length, _alease0.Values, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public string[] Strings()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint resultSize = 0;
            nint* result = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[18])(self, &resultSize, &result));
                return WindowsCsharp.Interop.FromStringArray(ref resultSize, ref result);
            }
            finally
            {
                WindowsCsharp.Interop.FreeStringArray(resultSize, result);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void GetStrings(out string[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _asize0 = 0;
            nint* _adata0 = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[19])(self, &_asize0, &_adata0));
                values = WindowsCsharp.Interop.FromStringArray(ref _asize0, ref _adata0);
            }
            finally
            {
                WindowsCsharp.Interop.FreeStringArray(_asize0, _adata0);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint CountPeers(Sample.IArrayPeer?[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint result;
            using WindowsCsharp.ObjectArrayLease _alease0 = WindowsCsharp.ObjectArrayLease.From(values);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[20])(self, (uint)values.Length, _alease0.Values, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.IArrayPeer?[] Peers()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint resultSize = 0;
            nint* result = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[21])(self, &resultSize, &result));
                return WindowsCsharp.Interop.FromObjectArray<Sample.IArrayPeer>(ref resultSize, ref result);
            }
            finally
            {
                WindowsCsharp.Interop.FreeObjectArray(resultSize, result);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void GetPeers(out Sample.IArrayPeer?[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _asize0 = 0;
            nint* _adata0 = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[22])(self, &_asize0, &_adata0));
                values = WindowsCsharp.Interop.FromObjectArray<Sample.IArrayPeer>(ref _asize0, ref _adata0);
            }
            finally
            {
                WindowsCsharp.Interop.FreeObjectArray(_asize0, _adata0);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint CountInspectables(Windows.Foundation.IInspectable?[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint result;
            using WindowsCsharp.ObjectArrayLease _alease0 = WindowsCsharp.ObjectArrayLease.From(values);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[23])(self, (uint)values.Length, _alease0.Values, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.IInspectable?[] Inspectables()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint resultSize = 0;
            nint* result = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[24])(self, &resultSize, &result));
                return WindowsCsharp.Interop.FromObjectArray<Windows.Foundation.IInspectable>(ref resultSize, ref result);
            }
            finally
            {
                WindowsCsharp.Interop.FreeObjectArray(resultSize, result);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void GetInspectables(out Windows.Foundation.IInspectable?[] values)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            uint _asize0 = 0;
            nint* _adata0 = null;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[25])(self, &_asize0, &_adata0));
                values = WindowsCsharp.Interop.FromObjectArray<Windows.Foundation.IInspectable>(ref _asize0, ref _adata0);
            }
            finally
            {
                WindowsCsharp.Interop.FreeObjectArray(_asize0, _adata0);
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
            public int Sum(int[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                fixed (int* _aptr0 = values)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int*, int*, int>)(*(void***)self)[6])(self, (uint)values.Length, _aptr0, &result));
                }
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int[] Values()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint resultSize = 0;
                int* result = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[7])(self, &resultSize, &result));
                    return WindowsCsharp.Interop.FromArray<int, int>(ref resultSize, ref result);
                }
                finally
                {
                    Marshal.FreeCoTaskMem((nint)result);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void GetValues(out int[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _asize0 = 0;
                int* _adata0 = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[8])(self, &_asize0, &_adata0));
                    values = WindowsCsharp.Interop.FromArray<int, int>(ref _asize0, ref _adata0);
                }
                finally
                {
                    Marshal.FreeCoTaskMem((nint)_adata0);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint CountTrue(bool[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint result;
                fixed (bool* _aptr0 = values)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, byte*, uint*, int>)(*(void***)self)[9])(self, (uint)values.Length, (byte*)_aptr0, &result));
                }
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool[] Booleans()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint resultSize = 0;
                byte* result = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, byte**, int>)(*(void***)self)[10])(self, &resultSize, &result));
                    return WindowsCsharp.Interop.FromBooleanArray(ref resultSize, ref result);
                }
                finally
                {
                    Marshal.FreeCoTaskMem((nint)result);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void GetBooleans(out bool[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _asize0 = 0;
                byte* _adata0 = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, byte**, int>)(*(void***)self)[11])(self, &_asize0, &_adata0));
                    values = WindowsCsharp.Interop.FromBooleanArray(ref _asize0, ref _adata0);
                }
                finally
                {
                    Marshal.FreeCoTaskMem((nint)_adata0);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int SumModes(Sample.Mode[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                fixed (Sample.Mode* _aptr0 = values)
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int*, int*, int>)(*(void***)self)[12])(self, (uint)values.Length, (int*)_aptr0, &result));
                }
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.Mode[] Modes()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint resultSize = 0;
                int* result = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[13])(self, &resultSize, &result));
                    return WindowsCsharp.Interop.FromArray<Sample.Mode, int>(ref resultSize, ref result);
                }
                finally
                {
                    Marshal.FreeCoTaskMem((nint)result);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void GetModes(out Sample.Mode[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _asize0 = 0;
                int* _adata0 = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int**, int>)(*(void***)self)[14])(self, &_asize0, &_adata0));
                    values = WindowsCsharp.Interop.FromArray<Sample.Mode, int>(ref _asize0, ref _adata0);
                }
                finally
                {
                    Marshal.FreeCoTaskMem((nint)_adata0);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool InspectState(Sample.State value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                byte result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.StateAbi, byte*, int>)(*(void***)self)[15])(self, Sample.StateAbi.FromSurface(value), &result));
                return result != 0;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.State CurrentState()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                Sample.StateAbi result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.StateAbi*, int>)(*(void***)self)[16])(self, &result));
                return result.ToSurface();
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint CountStringUnits(string[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint result;
                using WindowsCsharp.StringArrayLease _alease0 = WindowsCsharp.StringArrayLease.From(values);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[17])(self, (uint)values.Length, _alease0.Values, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public string[] Strings()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint resultSize = 0;
                nint* result = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[18])(self, &resultSize, &result));
                    return WindowsCsharp.Interop.FromStringArray(ref resultSize, ref result);
                }
                finally
                {
                    WindowsCsharp.Interop.FreeStringArray(resultSize, result);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void GetStrings(out string[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _asize0 = 0;
                nint* _adata0 = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[19])(self, &_asize0, &_adata0));
                    values = WindowsCsharp.Interop.FromStringArray(ref _asize0, ref _adata0);
                }
                finally
                {
                    WindowsCsharp.Interop.FreeStringArray(_asize0, _adata0);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint CountPeers(Sample.IArrayPeer?[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint result;
                using WindowsCsharp.ObjectArrayLease _alease0 = WindowsCsharp.ObjectArrayLease.From(values);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[20])(self, (uint)values.Length, _alease0.Values, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.IArrayPeer?[] Peers()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint resultSize = 0;
                nint* result = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[21])(self, &resultSize, &result));
                    return WindowsCsharp.Interop.FromObjectArray<Sample.IArrayPeer>(ref resultSize, ref result);
                }
                finally
                {
                    WindowsCsharp.Interop.FreeObjectArray(resultSize, result);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void GetPeers(out Sample.IArrayPeer?[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _asize0 = 0;
                nint* _adata0 = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[22])(self, &_asize0, &_adata0));
                    values = WindowsCsharp.Interop.FromObjectArray<Sample.IArrayPeer>(ref _asize0, ref _adata0);
                }
                finally
                {
                    WindowsCsharp.Interop.FreeObjectArray(_asize0, _adata0);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint CountInspectables(Windows.Foundation.IInspectable?[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint result;
                using WindowsCsharp.ObjectArrayLease _alease0 = WindowsCsharp.ObjectArrayLease.From(values);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, uint*, int>)(*(void***)self)[23])(self, (uint)values.Length, _alease0.Values, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.IInspectable?[] Inspectables()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint resultSize = 0;
                nint* result = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[24])(self, &resultSize, &result));
                    return WindowsCsharp.Interop.FromObjectArray<Windows.Foundation.IInspectable>(ref resultSize, ref result);
                }
                finally
                {
                    WindowsCsharp.Interop.FreeObjectArray(resultSize, result);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void GetInspectables(out Windows.Foundation.IInspectable?[] values)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                uint _asize0 = 0;
                nint* _adata0 = null;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, nint**, int>)(*(void***)self)[25])(self, &_asize0, &_adata0));
                    values = WindowsCsharp.Interop.FromObjectArray<Windows.Foundation.IInspectable>(ref _asize0, ref _adata0);
                }
                finally
                {
                    WindowsCsharp.Interop.FreeObjectArray(_asize0, _adata0);
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

    public sealed unsafe class IArrayPeer : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IArrayPeer>, WindowsCsharp.IObjectParameter<Sample.IArrayPeer._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x5dfb9cb6, 0xcb33, 0x54e3, 0xaf, 0x60, 0xe9, 0xb7, 0xab, 0xdb, 0xde, 0xdc);

        internal IArrayPeer(nint self) : base(self, Iid) {}
        internal IArrayPeer(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IArrayPeer WindowsCsharp.IComInterface<IArrayPeer>.FromAbi(nint self) => new IArrayPeer(self);
        static IArrayPeer WindowsCsharp.IComInterface<IArrayPeer>.FromAgileAbi(nint self) => new IArrayPeer(self, true);

        public int Id
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

            public int Id
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

namespace Windows.Foundation
{
    public sealed unsafe class IInspectable : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IInspectable>
    {
        public static Guid Iid { get; } = new Guid(0xaf86e2e0, 0xb12d, 0x4c6a, 0x9c, 0x5a, 0xd7, 0xaa, 0x65, 0x10, 0x1e, 0x90);

        internal IInspectable(nint self) : base(self, Iid) {}
        internal IInspectable(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IInspectable WindowsCsharp.IComInterface<IInspectable>.FromAbi(nint self) => new IInspectable(self);
        static IInspectable WindowsCsharp.IComInterface<IInspectable>.FromAgileAbi(nint self) => new IInspectable(self, true);

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
}
