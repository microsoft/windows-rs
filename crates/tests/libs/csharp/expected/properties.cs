namespace Sample
{
    public sealed unsafe class Config : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Config>, WindowsCsharp.IObjectParameter<Sample.Config._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x9bfd9d7f, 0x45e5, 0x525c, 0xa0, 0x76, 0x46, 0x26, 0xb9, 0x79, 0xc0, 0x57);

        internal Config(nint self) : base(self, Iid) {}
        internal Config(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Config WindowsCsharp.IComInterface<Config>.FromAbi(nint self) => new Config(self);
        static Config WindowsCsharp.IComInterface<Config>.FromAgileAbi(nint self) => new Config(self, true);

        public Config() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Config", Iid), Iid) {}

        public int Version
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
        }

        public string Password
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                fixed (char* c = value)
                {
                    WindowsCsharp.Interop.HstringHeader header;
                    nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)c, (uint)(value?.Length ?? 0), &header);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, hstring));
                }
            }
        }

        public string Name
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint hstring;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &hstring));
                return WindowsCsharp.Interop.FromHstring(hstring);
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                fixed (char* c = value)
                {
                    WindowsCsharp.Interop.HstringHeader header;
                    nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)c, (uint)(value?.Length ?? 0), &header);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, hstring));
                }
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

            public int Version
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
            }

            public string Password
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    fixed (char* c = value)
                    {
                        WindowsCsharp.Interop.HstringHeader header;
                        nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)c, (uint)(value?.Length ?? 0), &header);
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, hstring));
                    }
                }
            }

            public string Name
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    nint hstring;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &hstring));
                    return WindowsCsharp.Interop.FromHstring(hstring);
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    fixed (char* c = value)
                    {
                        WindowsCsharp.Interop.HstringHeader header;
                        nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)c, (uint)(value?.Length ?? 0), &header);
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, hstring));
                    }
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
