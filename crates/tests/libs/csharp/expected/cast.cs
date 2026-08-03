namespace Sample
{
    public sealed unsafe class Gadget : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Gadget>, WindowsCsharp.IObjectParameter<Sample.Gadget._Parameter>, WindowsCsharp.IObjectParameter<Sample.IExtra._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x29386b07, 0x5461, 0x52a7, 0xae, 0x1b, 0xfe, 0x5f, 0xcc, 0xf9, 0xce, 0x5a);
        private static readonly Guid s_forward0 = new Guid(0xbd45b8c4, 0x2389, 0x5a7f, 0xb2, 0x5f, 0x24, 0xa6, 0xa6, 0x33, 0x8f, 0xaf);

        internal Gadget(nint self) : base(self, Iid) {}
        internal Gadget(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Gadget WindowsCsharp.IComInterface<Gadget>.FromAbi(nint self) => new Gadget(self);
        static Gadget WindowsCsharp.IComInterface<Gadget>.FromAgileAbi(nint self) => new Gadget(self, true);

        public Gadget() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Gadget", Iid), Iid) {}

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
        public int Extra()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint source = lease.Handle;
            nint self;
            Guid iid = s_forward0;
            WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
            try
            {
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &result));
                return result;
            }
            finally
            {
                _ = WindowsCsharp.Com.Release(self);
            }
        }

        public string Label
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint source = lease.Handle;
                nint self;
                Guid iid = s_forward0;
                WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
                try
                {
                    nint hstring;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, &hstring));
                    return WindowsCsharp.Interop.FromHstring(hstring);
                }
                finally
                {
                    _ = WindowsCsharp.Com.Release(self);
                }
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint source = lease.Handle;
                nint self;
                Guid iid = s_forward0;
                WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
                try
                {
                    fixed (char* c = value)
                    {
                        WindowsCsharp.Interop.HstringHeader header;
                        nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)c, (uint)(value?.Length ?? 0), &header);
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[8])(self, hstring));
                    }
                }
                finally
                {
                    _ = WindowsCsharp.Com.Release(self);
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
            public int Extra()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint source = self;
                Guid iid = s_forward0;
                WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
                try
                {
                    int result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &result));
                    return result;
                }
                finally
                {
                    _ = WindowsCsharp.Com.Release(self);
                }
            }

            public string Label
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    nint source = self;
                    Guid iid = s_forward0;
                    WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
                    try
                    {
                        nint hstring;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, &hstring));
                        return WindowsCsharp.Interop.FromHstring(hstring);
                    }
                    finally
                    {
                        _ = WindowsCsharp.Com.Release(self);
                    }
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    nint source = self;
                    Guid iid = s_forward0;
                    WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
                    try
                    {
                        fixed (char* c = value)
                        {
                            WindowsCsharp.Interop.HstringHeader header;
                            nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)c, (uint)(value?.Length ?? 0), &header);
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[8])(self, hstring));
                        }
                    }
                    finally
                    {
                        _ = WindowsCsharp.Com.Release(self);
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

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void BorrowAs(Sample.IExtra.BorrowAction action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Sample.IExtra.Iid);
            action(new Sample.IExtra.Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult BorrowAs<TResult>(Sample.IExtra.BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Sample.IExtra.Iid);
            return action(new Sample.IExtra.Borrowed(lease.Handle));
        }
    }

    public sealed unsafe class IExtra : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IExtra>, WindowsCsharp.IObjectParameter<Sample.IExtra._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0xbd45b8c4, 0x2389, 0x5a7f, 0xb2, 0x5f, 0x24, 0xa6, 0xa6, 0x33, 0x8f, 0xaf);

        internal IExtra(nint self) : base(self, Iid) {}
        internal IExtra(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IExtra WindowsCsharp.IComInterface<IExtra>.FromAbi(nint self) => new IExtra(self);
        static IExtra WindowsCsharp.IComInterface<IExtra>.FromAgileAbi(nint self) => new IExtra(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Extra()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &result));
            return result;
        }

        public string Label
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint hstring;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, &hstring));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[8])(self, hstring));
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

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Extra()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &result));
                return result;
            }

            public string Label
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, &hstring));
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
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[8])(self, hstring));
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
