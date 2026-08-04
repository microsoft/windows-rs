namespace Selection
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

    public sealed unsafe class ChangedHandler : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<ChangedHandler>, WindowsCsharp.IObjectParameter<Selection.ChangedHandler._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x95dea58a, 0xeecf, 0x5689, 0x8e, 0x0d, 0xcd, 0x1a, 0xd9, 0x90, 0x63, 0x4b);

        internal ChangedHandler(nint self) : base(self, Iid) {}
        internal ChangedHandler(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static ChangedHandler WindowsCsharp.IComInterface<ChangedHandler>.FromAbi(nint self) => new ChangedHandler(self);
        static ChangedHandler WindowsCsharp.IComInterface<ChangedHandler>.FromAgileAbi(nint self) => new ChangedHandler(self, true);

        public delegate void Callback(int value);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, int, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static ChangedHandler Create(Callback handler) => WindowsCsharp.Com.WrapAgile<ChangedHandler>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, int value)
        {
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                callback(value);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Invoke(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[3])(self, value));
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

    public sealed unsafe class Gadget : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Gadget>, WindowsCsharp.IObjectParameter<Selection.Gadget._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x989fd7f1, 0x0742, 0x5956, 0xbc, 0xd3, 0x0a, 0x81, 0x3c, 0xb1, 0x4d, 0x13);

        internal Gadget(nint self) : base(self, Iid) {}
        internal Gadget(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Gadget WindowsCsharp.IComInterface<Gadget>.FromAbi(nint self) => new Gadget(self);
        static Gadget WindowsCsharp.IComInterface<Gadget>.FromAgileAbi(nint self) => new Gadget(self, true);

        public Gadget() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Selection.Gadget", Iid), Iid) {}

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

    public sealed unsafe class Widget : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Widget>, WindowsCsharp.IObjectParameter<Selection.Widget._Parameter>, WindowsCsharp.IObjectParameter<Selection.IWidgetExtra._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x6a03582e, 0xab02, 0x5cc3, 0xab, 0x1d, 0x65, 0x93, 0xdc, 0x88, 0xc3, 0x9d);
        private static readonly Guid s_forward0 = new Guid(0x932b2384, 0x43b7, 0x5260, 0x87, 0x1f, 0x57, 0x77, 0xbe, 0x0e, 0x9b, 0xaf);

        internal Widget(nint self) : base(self, Iid) {}
        internal Widget(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Widget WindowsCsharp.IComInterface<Widget>.FromAbi(nint self) => new Widget(self);
        static Widget WindowsCsharp.IComInterface<Widget>.FromAgileAbi(nint self) => new Widget(self, true);

        public Widget() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Selection.Widget", Iid), Iid) {}

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

        public Selection.Point Location
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                Selection.Point value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Selection.Point*, int>)(*(void***)self)[8])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Selection.Point, int>)(*(void***)self)[9])(self, value));
            }
        }

        public Selection.Mode State
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                int value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[10])(self, &value));
                return (Selection.Mode)value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[11])(self, (int)value));
            }
        }

        public Selection.Gadget? Peer
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[12])(self, &value));
                return WindowsCsharp.Com.Wrap<Selection.Gadget>(value);
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                using WindowsCsharp.ComLease valueLease = WindowsCsharp.ComLease.From(value);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, valueLease.Handle));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Compute(int input)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[14])(self, input, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public long AddChanged(Selection.ChangedHandler? handler)
        {
            using WindowsCsharp.ComLease sourceLease = Acquire();
            using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
            nint self = sourceLease.Handle;
            long token;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[15])(self, handlerLease.Handle, &token));
            return token;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void RemoveChanged(long token)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[16])(self, token));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public WindowsCsharp.EventRevoker Changed(Selection.ChangedHandler? handler)
        {
            WindowsCsharp.EventRevoker revoker = new WindowsCsharp.EventRevoker();
            using WindowsCsharp.ComLease sourceLease = Acquire();
            using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
            nint self = sourceLease.Handle;
            long token;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[15])(self, handlerLease.Handle, &token));
            _ = WindowsCsharp.Com.AddRef(self);
            try
            {
                revoker.Attach(self, sourceLease.TrustedAgile, token, (delegate* unmanaged<nint, long, int>)(*(void***)self)[16]);
            }
            catch
            {
                _ = ((delegate* unmanaged<nint, long, int>)(*(void***)self)[16])(self, token);
                _ = WindowsCsharp.Com.Release(self);
                throw;
            }
            return revoker;
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

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Transform()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint source = lease.Handle;
            nint self;
            Guid iid = s_forward0;
            WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
            try
            {
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[7])(self, &result));
                return result;
            }
            finally
            {
                _ = WindowsCsharp.Com.Release(self);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Transform(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint source = lease.Handle;
            nint self;
            Guid iid = s_forward0;
            WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(source, &iid, &self));
            try
            {
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[8])(self, value, &result));
                return result;
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

            public Selection.Point Location
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    Selection.Point value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Selection.Point*, int>)(*(void***)self)[8])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Selection.Point, int>)(*(void***)self)[9])(self, value));
                }
            }

            public Selection.Mode State
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[10])(self, &value));
                    return (Selection.Mode)value;
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[11])(self, (int)value));
                }
            }

            public Selection.Gadget? Peer
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    nint value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[12])(self, &value));
                    return WindowsCsharp.Com.Wrap<Selection.Gadget>(value);
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    using WindowsCsharp.ComLease valueLease = WindowsCsharp.ComLease.From(value);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, valueLease.Handle));
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Compute(int input)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[14])(self, input, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public long AddChanged(Selection.ChangedHandler? handler)
            {
                using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                long token;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[15])(self, handlerLease.Handle, &token));
                return token;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void RemoveChanged(long token)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[16])(self, token));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public WindowsCsharp.EventRevoker Changed(Selection.ChangedHandler? handler)
            {
                WindowsCsharp.EventRevoker revoker = new WindowsCsharp.EventRevoker();
                using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                long token;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[15])(self, handlerLease.Handle, &token));
                _ = WindowsCsharp.Com.AddRef(self);
                try
                {
                    revoker.Attach(self, false, token, (delegate* unmanaged<nint, long, int>)(*(void***)self)[16]);
                }
                catch
                {
                    _ = ((delegate* unmanaged<nint, long, int>)(*(void***)self)[16])(self, token);
                    _ = WindowsCsharp.Com.Release(self);
                    throw;
                }
                return revoker;
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

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Transform()
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[7])(self, &result));
                    return result;
                }
                finally
                {
                    _ = WindowsCsharp.Com.Release(self);
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Transform(int value)
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[8])(self, value, &result));
                    return result;
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
        public void BorrowAs(Selection.IWidgetExtra.BorrowAction action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Selection.IWidgetExtra.Iid);
            action(new Selection.IWidgetExtra.Borrowed(lease.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TResult BorrowAs<TResult>(Selection.IWidgetExtra.BorrowFunc<TResult> action)
        {
            using WindowsCsharp.ComLease source = Acquire();
            using WindowsCsharp.InterfaceLease lease = WindowsCsharp.InterfaceLease.From(source.Handle, Selection.IWidgetExtra.Iid);
            return action(new Selection.IWidgetExtra.Borrowed(lease.Handle));
        }
    }

    public sealed unsafe class IStandalone : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IStandalone>, WindowsCsharp.IObjectParameter<Selection.IStandalone._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x78e8c293, 0x1016, 0x5c1f, 0xb1, 0xd6, 0x24, 0xfa, 0xba, 0x1c, 0x08, 0x9c);

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

    public sealed unsafe class IWidgetExtra : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IWidgetExtra>, WindowsCsharp.IObjectParameter<Selection.IWidgetExtra._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x932b2384, 0x43b7, 0x5260, 0x87, 0x1f, 0x57, 0x77, 0xbe, 0x0e, 0x9b, 0xaf);

        internal IWidgetExtra(nint self) : base(self, Iid) {}
        internal IWidgetExtra(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IWidgetExtra WindowsCsharp.IComInterface<IWidgetExtra>.FromAbi(nint self) => new IWidgetExtra(self);
        static IWidgetExtra WindowsCsharp.IComInterface<IWidgetExtra>.FromAgileAbi(nint self) => new IWidgetExtra(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Extra()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Transform()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[7])(self, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Transform(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[8])(self, value, &result));
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

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Transform()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[7])(self, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Transform(int value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[8])(self, value, &result));
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
}
