namespace Breadth
{
    public sealed unsafe class Handler : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Handler>, WindowsCsharp.IObjectParameter<Breadth.Handler._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x5c34f905, 0xb872, 0x5141, 0xb6, 0x84, 0xff, 0x64, 0x67, 0xea, 0x2a, 0xc5);

        internal Handler(nint self) : base(self, Iid) {}
        internal Handler(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Handler WindowsCsharp.IComInterface<Handler>.FromAbi(nint self) => new Handler(self);
        static Handler WindowsCsharp.IComInterface<Handler>.FromAgileAbi(nint self) => new Handler(self, true);

        public delegate void Callback(Windows.Foundation.IInspectable.Borrowed sender, int value);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, nint, int, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Handler Create(Callback handler) => WindowsCsharp.Com.WrapAgile<Handler>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, nint sender, int value)
        {
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                callback(new Windows.Foundation.IInspectable.Borrowed(sender), value);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Invoke(WindowsCsharp.ComObject? sender, int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(sender);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int, int>)(*(void***)self)[3])(self, _olease0.Handle, value));
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

    public sealed unsafe class Item : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Item>, WindowsCsharp.IObjectParameter<Breadth.Item._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0xc0af6d61, 0xa424, 0x56ae, 0x9c, 0xf1, 0xba, 0x77, 0x5e, 0xf4, 0xc4, 0x01);

        internal Item(nint self) : base(self, Iid) {}
        internal Item(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Item WindowsCsharp.IComInterface<Item>.FromAbi(nint self) => new Item(self);
        static Item WindowsCsharp.IComInterface<Item>.FromAgileAbi(nint self) => new Item(self, true);

        public Item() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Breadth.Item", Iid), Iid) {}

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

    public sealed unsafe class Store : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Store>, WindowsCsharp.IObjectParameter<Breadth.Store._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x09c847e7, 0xc0b9, 0x5baa, 0xa0, 0x71, 0x90, 0x04, 0xff, 0x9f, 0x70, 0xf5);

        internal Store(nint self) : base(self, Iid) {}
        internal Store(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Store WindowsCsharp.IComInterface<Store>.FromAbi(nint self) => new Store(self);
        static Store WindowsCsharp.IComInterface<Store>.FromAgileAbi(nint self) => new Store(self, true);

        public Store() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Breadth.Store", Iid), Iid) {}

        public WindowsCsharp.ComObject? ObjectProperty
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, &value));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.IInspectable>(value);
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                using WindowsCsharp.ComLease valueLease = WindowsCsharp.ComLease.From(value);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, valueLease.Handle));
            }
        }

        public int? ReferenceProperty
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint reference;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &reference));
                return WindowsCsharp.ReferenceBox<int>.Unbox(reference);
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                using WindowsCsharp.ReferenceBox<int> box = new WindowsCsharp.ReferenceBox<int>(value, new Guid(0x548cefbd, 0xbc8a, 0x5fa0, 0x8d, 0xf2, 0x95, 0x74, 0x40, 0xfc, 0x8b, 0xf4));
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, box.Handle));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int? EchoReference(int? value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ReferenceBox<int> _rbox0 = new WindowsCsharp.ReferenceBox<int>(value, new Guid(0x548cefbd, 0xbc8a, 0x5fa0, 0x8d, 0xf2, 0x95, 0x74, 0x40, 0xfc, 0x8b, 0xf4));
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[10])(self, _rbox0.Handle, &result));
            return WindowsCsharp.ReferenceBox<int>.Unbox(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.IAsyncOperation<int>? Operation()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[11])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.IAsyncOperation<int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.IAsyncOperation<string>? StringOperation()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[12])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.IAsyncOperation<string>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.IAsyncOperation<Breadth.Item?>? ItemOperation()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[13])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.IAsyncOperation<Breadth.Item?>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMap<string, int>? Map()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[14])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<string, int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public long AddChanged(Breadth.Handler? handler)
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
        public WindowsCsharp.EventRevoker Changed(Breadth.Handler? handler)
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

            public WindowsCsharp.ComObject? ObjectProperty
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, &value));
                    return WindowsCsharp.Com.Wrap<Windows.Foundation.IInspectable>(value);
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, valueLease.Handle));
                }
            }

            public int? ReferenceProperty
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    nint reference;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &reference));
                    return WindowsCsharp.ReferenceBox<int>.Unbox(reference);
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    using WindowsCsharp.ReferenceBox<int> box = new WindowsCsharp.ReferenceBox<int>(value, new Guid(0x548cefbd, 0xbc8a, 0x5fa0, 0x8d, 0xf2, 0x95, 0x74, 0x40, 0xfc, 0x8b, 0xf4));
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, box.Handle));
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int? EchoReference(int? value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.ReferenceBox<int> _rbox0 = new WindowsCsharp.ReferenceBox<int>(value, new Guid(0x548cefbd, 0xbc8a, 0x5fa0, 0x8d, 0xf2, 0x95, 0x74, 0x40, 0xfc, 0x8b, 0xf4));
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[10])(self, _rbox0.Handle, &result));
                return WindowsCsharp.ReferenceBox<int>.Unbox(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.IAsyncOperation<int>? Operation()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[11])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.IAsyncOperation<int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.IAsyncOperation<string>? StringOperation()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[12])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.IAsyncOperation<string>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.IAsyncOperation<Breadth.Item?>? ItemOperation()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[13])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.IAsyncOperation<Breadth.Item?>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMap<string, int>? Map()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[14])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<string, int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public long AddChanged(Breadth.Handler? handler)
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
            public WindowsCsharp.EventRevoker Changed(Breadth.Handler? handler)
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

    public sealed unsafe class IAsyncOperation<T> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IAsyncOperation<T>>
    {
        private static readonly Guid s_asyncInfo = new Guid(0x00000036, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
        private static readonly int s_referenceType = ComputeReferenceType();
        public static Guid Iid { get; } = ComputeIid();
        private static readonly Guid* s_completedIid = WindowsCsharp.Callback.PinIid(ComputeCompletedIid());

        private static int ComputeReferenceType()
        {
            if (!RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                return -1;
            }
            if (typeof(T) == typeof(Breadth.Item)) return 0;
            if (typeof(T) == typeof(string)) return 1;
            throw new NotSupportedException();
        }

        private static Guid ComputeIid()
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0: return new Guid(0x04653cb8, 0x6178, 0x5f30, 0xb8, 0xf7, 0x13, 0x5b, 0x36, 0x4c, 0x6b, 0x5c);
                    case 1: return new Guid(0x3e1fe603, 0xf897, 0x5263, 0xb3, 0x28, 0x08, 0x06, 0x42, 0x6b, 0x8a, 0x79);
                }
            }
            if (typeof(T) == typeof(int)) return new Guid(0x968b9665, 0x06ed, 0x5774, 0x8f, 0x53, 0x8e, 0xde, 0xab, 0xd5, 0xf7, 0xb5);
            throw new NotSupportedException();
        }

        private static Guid ComputeCompletedIid()
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0: return new Guid(0x6bab735e, 0x7cbe, 0x5aca, 0x9f, 0x08, 0x42, 0xba, 0x13, 0xc3, 0xe8, 0x58);
                    case 1: return new Guid(0xb79a741f, 0x7fb5, 0x50ae, 0x9e, 0x99, 0x91, 0x12, 0x01, 0xec, 0x3d, 0x41);
                }
            }
            if (typeof(T) == typeof(int)) return new Guid(0xd60cae9d, 0x88cb, 0x59f1, 0x85, 0x76, 0x3f, 0xba, 0x44, 0x79, 0x6b, 0xe8);
            throw new NotSupportedException();
        }

        internal IAsyncOperation(nint self) : base(self, Iid) {}
        internal IAsyncOperation(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IAsyncOperation<T> WindowsCsharp.IComInterface<IAsyncOperation<T>>.FromAbi(nint self) => new IAsyncOperation<T>(self);
        static IAsyncOperation<T> WindowsCsharp.IComInterface<IAsyncOperation<T>>.FromAgileAbi(nint self) => new IAsyncOperation<T>(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T GetResults()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return GetResultsAbi(self);
        }

        private static T GetResultsAbi(nint self)
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0:
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &result));
                        Breadth.Item value = WindowsCsharp.Com.Wrap<Breadth.Item>(result)!;
                        return (T)(object)value;
                    }
                    case 1:
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &result));
                        string value = WindowsCsharp.Interop.FromHstring(result)!;
                        return (T)(object)value;
                    }
                }
            }
            if (typeof(T) == typeof(int))
            {
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[8])(self, &result));
                int value = result!;
                return Unsafe.As<int, T>(ref value);
            }
            throw new NotSupportedException();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private bool IsCompleted()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Guid iid = s_asyncInfo;
            nint info;
            WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(self, &iid, &info));
            try
            {
                int status;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)info)[7])(info, &status));
                return status != 0;
            }
            finally
            {
                _ = WindowsCsharp.Com.Release(info);
            }
        }

        private void RegisterContinuation(Action continuation)
        {
            nint handler = WindowsCsharp.Callback.AllocCompleted(s_completedIid, continuation);
            try
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                _ = WindowsCsharp.Com.AddRef(self);
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[6])(self, handler));
                }
                finally
                {
                    _ = WindowsCsharp.Com.Release(self);
                }
            }
            finally
            {
                _ = WindowsCsharp.Com.Release(handler);
            }
        }

        public Awaiter GetAwaiter() => new Awaiter(this);

        public readonly struct Awaiter : ICriticalNotifyCompletion
        {
            private readonly IAsyncOperation<T> _operation;
            internal Awaiter(IAsyncOperation<T> operation) => _operation = operation;
            public bool IsCompleted => _operation.IsCompleted();

            public T GetResult() => _operation.GetResults();

            public void OnCompleted(Action continuation) => UnsafeOnCompleted(continuation);

            public void UnsafeOnCompleted(Action continuation)
            {
                if (continuation is null)
                {
                    throw new ArgumentNullException(nameof(continuation));
                }
                _operation.RegisterContinuation(continuation);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            return WindowsCsharp.Com.As<TInterface>(lease.Handle, lease.TrustedAgile);
        }
    }
}

namespace Windows.Foundation.Collections
{
    public sealed unsafe class IMap<K, V> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IMap<K, V>> where V : unmanaged
    {
        public static Guid Iid { get; } = ComputeIid();

        private static Guid ComputeIid()
        {
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0xae681871, 0xdd82, 0x5299, 0x93, 0xea, 0x02, 0x75, 0xe4, 0xe0, 0x73, 0xe7);
            throw new NotSupportedException();
        }

        private static Guid ComputeIterableIid()
        {
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0x2aa69c56, 0xc3a4, 0x58f9, 0xb1, 0x4c, 0x46, 0x5b, 0xca, 0xf8, 0xc7, 0xba);
            throw new NotSupportedException();
        }

        private static Guid ComputeIteratorIid()
        {
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0x96c8b304, 0x4108, 0x5f67, 0x8b, 0x2f, 0x21, 0x39, 0x75, 0xf0, 0x85, 0xb2);
            throw new NotSupportedException();
        }

        internal IMap(nint self) : base(self, Iid) {}
        internal IMap(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IMap<K, V> WindowsCsharp.IComInterface<IMap<K, V>>.FromAbi(nint self) => new IMap<K, V>(self);
        static IMap<K, V> WindowsCsharp.IComInterface<IMap<K, V>>.FromAgileAbi(nint self) => new IMap<K, V>(self, true);

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
        public V Lookup(K key)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            if (typeof(K) == typeof(string))
            {
                string? text = Unsafe.As<K, string?>(ref key);
                fixed (char* chars = text)
                {
                    WindowsCsharp.Interop.HstringHeader header;
                    nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                    if (typeof(V) == typeof(int))
                    {
                        int result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, hstring, &result));
                        int value = result;
                        return Unsafe.As<int, V>(ref value);
                    }
                    throw new NotSupportedException();
                }
            }
            throw new NotSupportedException();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool HasKey(K key)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            if (typeof(K) == typeof(string))
            {
                string? text = Unsafe.As<K, string?>(ref key);
                fixed (char* chars = text)
                {
                    WindowsCsharp.Interop.HstringHeader header;
                    nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                    byte result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***)self)[8])(self, hstring, &result));
                    return result != 0;
                }
            }
            throw new NotSupportedException();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool Insert(K key, V value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            if (typeof(K) == typeof(string))
            {
                string? text = Unsafe.As<K, string?>(ref key);
                fixed (char* chars = text)
                {
                    WindowsCsharp.Interop.HstringHeader header;
                    nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                    if (typeof(V) == typeof(int))
                    {
                        int abiValue = Unsafe.As<V, int>(ref value);
                        byte replaced;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int, byte*, int>)(*(void***)self)[10])(self, hstring, abiValue, &replaced));
                        return replaced != 0;
                    }
                    throw new NotSupportedException();
                }
            }
            throw new NotSupportedException();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Remove(K key)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            if (typeof(K) == typeof(string))
            {
                string? text = Unsafe.As<K, string?>(ref key);
                fixed (char* chars = text)
                {
                    WindowsCsharp.Interop.HstringHeader header;
                    nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[11])(self, hstring));
                    return;
                }
            }
            throw new NotSupportedException();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Clear()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[12])(self));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Enumerator GetEnumerator()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Guid iid = ComputeIterableIid();
            nint iterable;
            WindowsCsharp.Com.Check(WindowsCsharp.Com.QueryInterface(self, &iid, &iterable));
            try
            {
                nint iterator;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)iterable)[6])(iterable, &iterator));
                return new Enumerator(iterator);
            }
            finally
            {
                _ = WindowsCsharp.Com.Release(iterable);
            }
        }

        public sealed class Enumerator : WindowsCsharp.ComObject
        {
            private bool _started;
            internal Enumerator(nint self) : base(self, ComputeIteratorIid()) {}

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool MoveNext()
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                if (!_started)
                {
                    _started = true;
                    byte current;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[7])(self, &current));
                    return current != 0;
                }
                byte moved;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[8])(self, &moved));
                return moved != 0;
            }

            public Entry Current
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    using WindowsCsharp.ComLease lease = Acquire();
                    nint self = lease.Handle;
                    nint pair;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[6])(self, &pair));
                    try
                    {
                        if (typeof(K) == typeof(string) && typeof(V) == typeof(int))
                        {
                            nint abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            string keyValue = WindowsCsharp.Interop.FromHstring(abiKey);
                            K key = (K)(object)keyValue;
                            int abiValue;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[7])(pair, &abiValue));
                            int valueResult = abiValue;
                            V value = Unsafe.As<int, V>(ref valueResult);
                            return new Entry(key, value);
                        }
                        throw new NotSupportedException();
                    }
                    finally
                    {
                        _ = WindowsCsharp.Com.Release(pair);
                    }
                }
            }
        }

        public readonly struct Entry : IDisposable
        {
            public K Key { get; }
            public V Value { get; }

            internal Entry(K key, V value)
            {
                Key = key;
                Value = value;
            }

            public void Dispose()
            {
                object? keyObject = Key;
                if (keyObject is WindowsCsharp.ComObject key)
                {
                    key.Dispose();
                }
                object? valueObject = Value;
                if (valueObject is WindowsCsharp.ComObject value)
                {
                    value.Dispose();
                }
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
            public V Lookup(K key)
            {
                if (typeof(K) == typeof(string))
                {
                    string? text = Unsafe.As<K, string?>(ref key);
                    fixed (char* chars = text)
                    {
                        WindowsCsharp.Interop.HstringHeader header;
                        nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                        if (typeof(V) == typeof(int))
                        {
                            int result;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)_this)[6])(_this, hstring, &result));
                            int value = result;
                            return Unsafe.As<int, V>(ref value);
                        }
                        throw new NotSupportedException();
                    }
                }
                throw new NotSupportedException();
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool HasKey(K key)
            {
                if (typeof(K) == typeof(string))
                {
                    string? text = Unsafe.As<K, string?>(ref key);
                    fixed (char* chars = text)
                    {
                        WindowsCsharp.Interop.HstringHeader header;
                        nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                        byte result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***)_this)[8])(_this, hstring, &result));
                        return result != 0;
                    }
                }
                throw new NotSupportedException();
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool Insert(K key, V value)
            {
                if (typeof(K) == typeof(string))
                {
                    string? text = Unsafe.As<K, string?>(ref key);
                    fixed (char* chars = text)
                    {
                        WindowsCsharp.Interop.HstringHeader header;
                        nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                        if (typeof(V) == typeof(int))
                        {
                            int abiValue = Unsafe.As<V, int>(ref value);
                            byte replaced;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int, byte*, int>)(*(void***)_this)[10])(_this, hstring, abiValue, &replaced));
                            return replaced != 0;
                        }
                        throw new NotSupportedException();
                    }
                }
                throw new NotSupportedException();
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Remove(K key)
            {
                if (typeof(K) == typeof(string))
                {
                    string? text = Unsafe.As<K, string?>(ref key);
                    fixed (char* chars = text)
                    {
                        WindowsCsharp.Interop.HstringHeader header;
                        nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)chars, (uint)(text?.Length ?? 0), &header);
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)_this)[11])(_this, hstring));
                        return;
                    }
                }
                throw new NotSupportedException();
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Clear() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[12])(_this));

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface> => WindowsCsharp.Com.As<TInterface>(_this, false);
        }
    }
}
