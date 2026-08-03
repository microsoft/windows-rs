namespace Sample
{
    public enum Shade : int
    {
        Red = 0,
        Green = 1,
        Blue = 2,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Coord
    {
        public float X;
        public float Y;
    }

    public sealed unsafe class Classify : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Classify>, WindowsCsharp.IObjectParameter<Sample.Classify._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0xb51e17a6, 0xfc3e, 0x5918, 0x87, 0x1f, 0x42, 0xfe, 0x28, 0x44, 0x3b, 0x94);

        internal Classify(nint self) : base(self, Iid) {}
        internal Classify(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Classify WindowsCsharp.IComInterface<Classify>.FromAbi(nint self) => new Classify(self);
        static Classify WindowsCsharp.IComInterface<Classify>.FromAgileAbi(nint self) => new Classify(self, true);

        public delegate Sample.Shade Callback(Sample.Shade value);

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
        public static Classify Create(Callback handler) => WindowsCsharp.Com.WrapAgile<Classify>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

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
                *result = (int)callback((Sample.Shade)value);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.Shade Invoke(Sample.Shade value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[3])(self, (int)value, &result));
            return (Sample.Shade)result;
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

    public sealed unsafe class Compute : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Compute>, WindowsCsharp.IObjectParameter<Sample.Compute._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x18a6748f, 0x49f7, 0x547f, 0xae, 0x83, 0xd3, 0xe0, 0x15, 0x2b, 0x41, 0xab);

        internal Compute(nint self) : base(self, Iid) {}
        internal Compute(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Compute WindowsCsharp.IComInterface<Compute>.FromAbi(nint self) => new Compute(self);
        static Compute WindowsCsharp.IComInterface<Compute>.FromAgileAbi(nint self) => new Compute(self, true);

        public delegate int Callback(int x, int y);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, int, int, int*, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Compute Create(Callback handler) => WindowsCsharp.Com.WrapAgile<Compute>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, int x, int y, int* result)
        {
            if (result == null)
            {
                return unchecked((int)0x80004003);
            }
            *result = default;
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                *result = callback(x, y);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Invoke(int x, int y)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[3])(self, x, y, &result));
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

    public sealed unsafe class Transform : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Transform>, WindowsCsharp.IObjectParameter<Sample.Transform._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x9c7fbc06, 0x6934, 0x555b, 0x90, 0xd6, 0xde, 0x21, 0xb8, 0x99, 0xf5, 0x2a);

        internal Transform(nint self) : base(self, Iid) {}
        internal Transform(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Transform WindowsCsharp.IComInterface<Transform>.FromAbi(nint self) => new Transform(self);
        static Transform WindowsCsharp.IComInterface<Transform>.FromAgileAbi(nint self) => new Transform(self, true);

        public delegate Sample.Coord Callback(Sample.Coord p);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, Sample.Coord, Sample.Coord*, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Transform Create(Callback handler) => WindowsCsharp.Com.WrapAgile<Transform>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, Sample.Coord p, Sample.Coord* result)
        {
            if (result == null)
            {
                return unchecked((int)0x80004003);
            }
            *result = default;
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                *result = callback(p);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.Coord Invoke(Sample.Coord p)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Sample.Coord result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.Coord, Sample.Coord*, int>)(*(void***)self)[3])(self, p, &result));
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

    public sealed unsafe class Engine : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Engine>, WindowsCsharp.IObjectParameter<Sample.Engine._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x8c5e149c, 0x7750, 0x5304, 0x93, 0x7c, 0xcc, 0x13, 0x1f, 0xc4, 0x11, 0x62);

        internal Engine(nint self) : base(self, Iid) {}
        internal Engine(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Engine WindowsCsharp.IComInterface<Engine>.FromAbi(nint self) => new Engine(self);
        static Engine WindowsCsharp.IComInterface<Engine>.FromAgileAbi(nint self) => new Engine(self, true);

        public Engine() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Engine", Iid), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Run(Sample.Compute? c)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(c);
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Run<T0>(T0? c) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<Sample.Compute._Parameter>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(c, Sample.Compute.Iid);
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public long AddComputed(Sample.Compute? handler)
        {
            using WindowsCsharp.ComLease sourceLease = Acquire();
            using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
            nint self = sourceLease.Handle;
            long token;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
            return token;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void RemoveComputed(long token)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public WindowsCsharp.EventRevoker Computed(Sample.Compute? handler)
        {
            WindowsCsharp.EventRevoker revoker = new WindowsCsharp.EventRevoker();
            using WindowsCsharp.ComLease sourceLease = Acquire();
            using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
            nint self = sourceLease.Handle;
            long token;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
            _ = WindowsCsharp.Com.AddRef(self);
            try
            {
                revoker.Attach(self, sourceLease.TrustedAgile, token, (delegate* unmanaged<nint, long, int>)(*(void***)self)[8]);
            }
            catch
            {
                _ = ((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token);
                _ = WindowsCsharp.Com.Release(self);
                throw;
            }
            return revoker;
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
            public int Run(Sample.Compute? c)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(c);
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Run<T0>(T0? c) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<Sample.Compute._Parameter>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(c, Sample.Compute.Iid);
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, _olease0.Handle, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public long AddComputed(Sample.Compute? handler)
            {
                using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                long token;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
                return token;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void RemoveComputed(long token)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public WindowsCsharp.EventRevoker Computed(Sample.Compute? handler)
            {
                WindowsCsharp.EventRevoker revoker = new WindowsCsharp.EventRevoker();
                using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                long token;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
                _ = WindowsCsharp.Com.AddRef(self);
                try
                {
                    revoker.Attach(self, false, token, (delegate* unmanaged<nint, long, int>)(*(void***)self)[8]);
                }
                catch
                {
                    _ = ((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token);
                    _ = WindowsCsharp.Com.Release(self);
                    throw;
                }
                return revoker;
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
