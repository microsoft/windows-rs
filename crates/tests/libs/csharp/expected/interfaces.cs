namespace Sample
{
    public sealed unsafe class Device : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Device>, WindowsCsharp.IObjectParameter<Sample.Device._Parameter>, WindowsCsharp.IObjectParameter<Sample.IReadable._Parameter>, WindowsCsharp.IObjectParameter<Sample.IWritable._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x1c39901a, 0xb3c8, 0x5e12, 0x83, 0xc8, 0xa9, 0x29, 0x24, 0x48, 0x4f, 0xec);
        private static readonly Guid s_forward0 = new Guid(0x85baa680, 0xdc91, 0x5a25, 0xa8, 0x7b, 0x8d, 0x17, 0x24, 0xf3, 0x84, 0x1b);
        private static readonly Guid s_forward1 = new Guid(0xb46e00a6, 0xd5bc, 0x5781, 0xae, 0xc6, 0x5e, 0xa4, 0xde, 0x3e, 0xa8, 0x2f);

        internal Device(nint self) : base(self, Iid) {}
        internal Device(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Device WindowsCsharp.IComInterface<Device>.FromAbi(nint self) => new Device(self);
        static Device WindowsCsharp.IComInterface<Device>.FromAgileAbi(nint self) => new Device(self, true);

        public Device() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Device", Iid), Iid) {}

        public string Name
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint hstring;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, &hstring));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, hstring));
                }
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Read()
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

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Write(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint source = lease.Handle;
            nint self;
            Guid iid = s_forward1;
            WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[6])(self, value));
            }
            finally
            {
                _ = WindowsCsharp.Com.Release(self);
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, &hstring));
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
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, hstring));
                    }
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Read()
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

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Write(int value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint source = self;
                Guid iid = s_forward1;
                WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[6])(self, value));
                }
                finally
                {
                    _ = WindowsCsharp.Com.Release(self);
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
        public void BorrowAs(Sample.IReadable.BorrowAction action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Sample.IReadable.Iid);
            action(new Sample.IReadable.Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult BorrowAs<TResult>(Sample.IReadable.BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Sample.IReadable.Iid);
            return action(new Sample.IReadable.Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void BorrowAs(Sample.IWritable.BorrowAction action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Sample.IWritable.Iid);
            action(new Sample.IWritable.Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult BorrowAs<TResult>(Sample.IWritable.BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Sample.IWritable.Iid);
            return action(new Sample.IWritable.Borrowed(lease.Handle));
        }
    }

    public sealed unsafe class IReadable : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IReadable>, WindowsCsharp.IObjectParameter<Sample.IReadable._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x85baa680, 0xdc91, 0x5a25, 0xa8, 0x7b, 0x8d, 0x17, 0x24, 0xf3, 0x84, 0x1b);

        internal IReadable(nint self) : base(self, Iid) {}
        internal IReadable(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IReadable WindowsCsharp.IComInterface<IReadable>.FromAbi(nint self) => new IReadable(self);
        static IReadable WindowsCsharp.IComInterface<IReadable>.FromAgileAbi(nint self) => new IReadable(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Read()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &result));
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
            public int Read()
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

    public sealed unsafe class IStandalone : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IStandalone>, WindowsCsharp.IObjectParameter<Sample.IStandalone._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x7f0bb73a, 0x488d, 0x5068, 0xbd, 0xa6, 0x92, 0x33, 0x7b, 0x09, 0x73, 0x92);

        internal IStandalone(nint self) : base(self, Iid) {}
        internal IStandalone(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IStandalone WindowsCsharp.IComInterface<IStandalone>.FromAbi(nint self) => new IStandalone(self);
        static IStandalone WindowsCsharp.IComInterface<IStandalone>.FromAgileAbi(nint self) => new IStandalone(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Ping()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &result));
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
            public int Ping()
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

    public sealed unsafe class IWritable : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IWritable>, WindowsCsharp.IObjectParameter<Sample.IWritable._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0xb46e00a6, 0xd5bc, 0x5781, 0xae, 0xc6, 0x5e, 0xa4, 0xde, 0x3e, 0xa8, 0x2f);

        internal IWritable(nint self) : base(self, Iid) {}
        internal IWritable(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IWritable WindowsCsharp.IComInterface<IWritable>.FromAbi(nint self) => new IWritable(self);
        static IWritable WindowsCsharp.IComInterface<IWritable>.FromAgileAbi(nint self) => new IWritable(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Write(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[6])(self, value));
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
            public void Write(int value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[6])(self, value));
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
