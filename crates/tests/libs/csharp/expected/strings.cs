namespace Sample
{
    public sealed unsafe class Formatter : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Formatter>, WindowsCsharp.IObjectParameter<Sample.Formatter._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0xb674e5ab, 0x64cf, 0x58db, 0xa7, 0x66, 0x6f, 0x59, 0x8e, 0x0c, 0x26, 0x3e);

        internal Formatter(nint self) : base(self, Iid) {}
        internal Formatter(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Formatter WindowsCsharp.IComInterface<Formatter>.FromAbi(nint self) => new Formatter(self);
        static Formatter WindowsCsharp.IComInterface<Formatter>.FromAgileAbi(nint self) => new Formatter(self, true);

        public Formatter() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Formatter", Iid), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public string Echo(string value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            fixed (char* _hbuf0 = value)
            {
                WindowsCsharp.Interop.HstringHeader _hhdr0;
                nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(value?.Length ?? 0), &_hhdr0);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[6])(self, _hstr0, &result));
            }
            return WindowsCsharp.Interop.FromHstring(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public string Concat(string a, string b)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            fixed (char* _hbuf0 = a, _hbuf1 = b)
            {
                WindowsCsharp.Interop.HstringHeader _hhdr0;
                nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(a?.Length ?? 0), &_hhdr0);
                WindowsCsharp.Interop.HstringHeader _hhdr1;
                nint _hstr1 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf1, (uint)(b?.Length ?? 0), &_hhdr1);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint, nint*, int>)(*(void***)self)[7])(self, _hstr0, _hstr1, &result));
            }
            return WindowsCsharp.Interop.FromHstring(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Length(string value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            fixed (char* _hbuf0 = value)
            {
                WindowsCsharp.Interop.HstringHeader _hhdr0;
                nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(value?.Length ?? 0), &_hhdr0);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[8])(self, _hstr0, &result));
            }
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public string Label(int id)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[9])(self, id, &result));
            return WindowsCsharp.Interop.FromHstring(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Store(string value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            fixed (char* _hbuf0 = value)
            {
                WindowsCsharp.Interop.HstringHeader _hhdr0;
                nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(value?.Length ?? 0), &_hhdr0);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[10])(self, _hstr0));
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
            public string Echo(string value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                fixed (char* _hbuf0 = value)
                {
                    WindowsCsharp.Interop.HstringHeader _hhdr0;
                    nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(value?.Length ?? 0), &_hhdr0);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[6])(self, _hstr0, &result));
                }
                return WindowsCsharp.Interop.FromHstring(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public string Concat(string a, string b)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                fixed (char* _hbuf0 = a, _hbuf1 = b)
                {
                    WindowsCsharp.Interop.HstringHeader _hhdr0;
                    nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(a?.Length ?? 0), &_hhdr0);
                    WindowsCsharp.Interop.HstringHeader _hhdr1;
                    nint _hstr1 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf1, (uint)(b?.Length ?? 0), &_hhdr1);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint, nint*, int>)(*(void***)self)[7])(self, _hstr0, _hstr1, &result));
                }
                return WindowsCsharp.Interop.FromHstring(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Length(string value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                fixed (char* _hbuf0 = value)
                {
                    WindowsCsharp.Interop.HstringHeader _hhdr0;
                    nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(value?.Length ?? 0), &_hhdr0);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[8])(self, _hstr0, &result));
                }
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public string Label(int id)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[9])(self, id, &result));
                return WindowsCsharp.Interop.FromHstring(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Store(string value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                fixed (char* _hbuf0 = value)
                {
                    WindowsCsharp.Interop.HstringHeader _hhdr0;
                    nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(value?.Length ?? 0), &_hhdr0);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[10])(self, _hstr0));
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
