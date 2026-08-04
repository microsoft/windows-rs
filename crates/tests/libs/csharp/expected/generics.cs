namespace Sample
{
    public sealed unsafe class Item : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Item>, WindowsCsharp.IObjectParameter<Sample.Item._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0xd789b71a, 0x1209, 0x5587, 0x92, 0x46, 0xc1, 0xbc, 0x7d, 0xc4, 0x21, 0xd7);

        internal Item(nint self) : base(self, Iid) {}
        internal Item(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Item WindowsCsharp.IComInterface<Item>.FromAbi(nint self) => new Item(self);
        static Item WindowsCsharp.IComInterface<Item>.FromAgileAbi(nint self) => new Item(self, true);

        public Item() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Item", Iid), Iid) {}

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

    public sealed unsafe class Store : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Store>, WindowsCsharp.IObjectParameter<Sample.Store._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x620af7f6, 0xb391, 0x579b, 0xae, 0xb2, 0x3d, 0xd7, 0xe4, 0x8a, 0xfb, 0x44);

        internal Store(nint self) : base(self, Iid) {}
        internal Store(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Store WindowsCsharp.IComInterface<Store>.FromAbi(nint self) => new Store(self);
        static Store WindowsCsharp.IComInterface<Store>.FromAgileAbi(nint self) => new Store(self, true);

        public Store() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Store", Iid), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IVector<int>? Items(uint count)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[6])(self, count, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IVector<Sample.Item?>? ObjectItems()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<Sample.Item?>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IVector<Windows.Foundation.IInspectable?>? InspectableItems()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<Windows.Foundation.IInspectable?>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IVector<string>? StringItems()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[9])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<string>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMap<int, int>? Lookup(uint count)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[10])(self, count, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<int, int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMap<string, int>? StringLookup()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[11])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<string, int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMap<int, string>? StringValues()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[12])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<int, string>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMap<int, Sample.Item?>? ObjectValues()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[13])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<int, Sample.Item?>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMap<Sample.Item?, int>? ObjectKeys()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[14])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<Sample.Item?, int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IVectorView<int>? ItemsView(uint count)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[15])(self, count, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVectorView<int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IVectorView<string>? StringItemsView()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[16])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVectorView<string>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMapView<int, int>? LookupView(uint count)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[17])(self, count, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<int, int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMapView<string, int>? StringLookupView()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[18])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<string, int>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMapView<int, string>? StringValuesView()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[19])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<int, string>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMapView<int, Sample.Item?>? ObjectValuesView()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[20])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<int, Sample.Item?>>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Windows.Foundation.Collections.IMapView<Sample.Item?, int>? ObjectKeysView()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[21])(self, &result));
            return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<Sample.Item?, int>>(result);
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
            public Windows.Foundation.Collections.IVector<int>? Items(uint count)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[6])(self, count, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IVector<Sample.Item?>? ObjectItems()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[7])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<Sample.Item?>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IVector<Windows.Foundation.IInspectable?>? InspectableItems()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<Windows.Foundation.IInspectable?>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IVector<string>? StringItems()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[9])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVector<string>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMap<int, int>? Lookup(uint count)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[10])(self, count, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<int, int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMap<string, int>? StringLookup()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[11])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<string, int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMap<int, string>? StringValues()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[12])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<int, string>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMap<int, Sample.Item?>? ObjectValues()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[13])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<int, Sample.Item?>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMap<Sample.Item?, int>? ObjectKeys()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[14])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMap<Sample.Item?, int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IVectorView<int>? ItemsView(uint count)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[15])(self, count, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVectorView<int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IVectorView<string>? StringItemsView()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[16])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IVectorView<string>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMapView<int, int>? LookupView(uint count)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[17])(self, count, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<int, int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMapView<string, int>? StringLookupView()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[18])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<string, int>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMapView<int, string>? StringValuesView()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[19])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<int, string>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMapView<int, Sample.Item?>? ObjectValuesView()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[20])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<int, Sample.Item?>>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Windows.Foundation.Collections.IMapView<Sample.Item?, int>? ObjectKeysView()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[21])(self, &result));
                return WindowsCsharp.Com.Wrap<Windows.Foundation.Collections.IMapView<Sample.Item?, int>>(result);
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

namespace Windows.Foundation.Collections
{
    public sealed unsafe class IVector<T> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IVector<T>>
    {
        private static readonly int s_referenceType = ComputeReferenceType();
        public static Guid Iid { get; } = ComputeIid();

        private static int ComputeReferenceType()
        {
            if (!RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                return -1;
            }
            if (typeof(T) == typeof(Sample.Item)) return 0;
            if (typeof(T) == typeof(Windows.Foundation.IInspectable)) return 1;
            if (typeof(T) == typeof(string)) return 2;
            throw new NotSupportedException();
        }

        private static Guid ComputeIid()
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0: return new Guid(0x8086796a, 0x141b, 0x5fdd, 0xb6, 0xe5, 0xaf, 0x16, 0x4b, 0x9d, 0x32, 0x34);
                    case 1: return new Guid(0xb32bdca4, 0x5e52, 0x5b27, 0xbc, 0x5d, 0xd6, 0x6a, 0x1a, 0x26, 0x8c, 0x2a);
                    case 2: return new Guid(0x98b9acc1, 0x4b56, 0x532e, 0xac, 0x73, 0x03, 0xd5, 0x29, 0x1c, 0xca, 0x90);
                }
            }
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
            return GetAtAbi(self, index);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint GetMany(uint startIndex, Span<T> items)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return GetManyAbi(self, startIndex, items);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Append(T value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            AppendAbi(lease.Handle, value);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal void AppendObject(WindowsCsharp.ComObject? value, Guid iid)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            using WindowsCsharp.InterfaceLease itemLease = WindowsCsharp.InterfaceLease.From(value, iid);
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, itemLease.Handle));
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

        private static T GetAtAbi(nint self, uint index)
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0:
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[6])(self, index, &result));
                        Sample.Item value = WindowsCsharp.Com.Wrap<Sample.Item>(result)!;
                        return (T)(object)value;
                    }
                    case 1:
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[6])(self, index, &result));
                        Windows.Foundation.IInspectable value = WindowsCsharp.Com.Wrap<Windows.Foundation.IInspectable>(result)!;
                        return (T)(object)value;
                    }
                    case 2:
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[6])(self, index, &result));
                        string value = WindowsCsharp.Interop.FromHstring(result)!;
                        return (T)(object)value;
                    }
                }
            }
            if (typeof(T) == typeof(int))
            {
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int*, int>)(*(void***)self)[6])(self, index, &result));
                int value = result;
                return Unsafe.As<int, T>(ref value);
            }
            throw new NotSupportedException();
        }

        private static uint GetManyAbi(nint self, uint startIndex, Span<T> items)
        {
            if (items.IsEmpty)
            {
                return 0;
            }
            if (typeof(T) == typeof(int))
            {
                uint actual;
                ref T first = ref MemoryMarshal.GetReference(items);
                fixed (int* p = &Unsafe.As<T, int>(ref first))
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, int*, uint*, int>)(*(void***)self)[16])(self, startIndex, (uint)items.Length, p, &actual));
                }
                return actual;
            }
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0:
                    {
                        const uint Capacity = 64;
                        nint* values = stackalloc nint[(int)Capacity];
                        uint total = 0;
                        while (total < (uint)items.Length)
                        {
                            uint requested = Math.Min(Capacity, (uint)items.Length - total);
                            for (uint i = 0; i < requested; i++)
                            {
                                values[i] = 0;
                            }
                            uint actual;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, nint*, uint*, int>)(*(void***)self)[16])(self, startIndex + total, requested, values, &actual));
                            try
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    nint abi = values[i];
                                    values[i] = 0;
                                    Sample.Item? value = WindowsCsharp.Com.Wrap<Sample.Item>(abi);
                                    items[(int)(total + i)] = (T)(object?)value!;
                                }
                            }
                            finally
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    if (values[i] != 0)
                                    {
                                        _ = WindowsCsharp.Com.Release(values[i]);
                                    }
                                }
                            }
                            total += actual;
                            if (actual < requested)
                            {
                                break;
                            }
                        }
                        return total;
                    }
                    case 1:
                    {
                        const uint Capacity = 64;
                        nint* values = stackalloc nint[(int)Capacity];
                        uint total = 0;
                        while (total < (uint)items.Length)
                        {
                            uint requested = Math.Min(Capacity, (uint)items.Length - total);
                            for (uint i = 0; i < requested; i++)
                            {
                                values[i] = 0;
                            }
                            uint actual;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, nint*, uint*, int>)(*(void***)self)[16])(self, startIndex + total, requested, values, &actual));
                            try
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    nint abi = values[i];
                                    values[i] = 0;
                                    Windows.Foundation.IInspectable? value = WindowsCsharp.Com.Wrap<Windows.Foundation.IInspectable>(abi);
                                    items[(int)(total + i)] = (T)(object?)value!;
                                }
                            }
                            finally
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    if (values[i] != 0)
                                    {
                                        _ = WindowsCsharp.Com.Release(values[i]);
                                    }
                                }
                            }
                            total += actual;
                            if (actual < requested)
                            {
                                break;
                            }
                        }
                        return total;
                    }
                    case 2:
                    {
                        const uint Capacity = 64;
                        nint* values = stackalloc nint[(int)Capacity];
                        uint total = 0;
                        while (total < (uint)items.Length)
                        {
                            uint requested = Math.Min(Capacity, (uint)items.Length - total);
                            for (uint i = 0; i < requested; i++)
                            {
                                values[i] = 0;
                            }
                            uint actual;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, nint*, uint*, int>)(*(void***)self)[16])(self, startIndex + total, requested, values, &actual));
                            try
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    nint abi = values[i];
                                    values[i] = 0;
                                    string value = WindowsCsharp.Interop.FromHstring(abi);
                                    items[(int)(total + i)] = (T)(object)value;
                                }
                            }
                            finally
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    if (values[i] != 0)
                                    {
                                        _ = WindowsCsharp.Interop.WindowsDeleteString(values[i]);
                                    }
                                }
                            }
                            total += actual;
                            if (actual < requested)
                            {
                                break;
                            }
                        }
                        return total;
                    }
                }
            }
            uint size;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[7])(self, &size));
            if (startIndex >= size)
            {
                return 0;
            }
            uint count = Math.Min((uint)items.Length, size - startIndex);
            for (uint i = 0; i < count; i++)
            {
                items[(int)i] = GetAtAbi(self, startIndex + i);
            }
            return count;
        }

        private static void AppendAbi(nint self, T value)
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0:
                    {
                        object? boxed = value;
                        if (boxed is null)
                        {
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, 0));
                            return;
                        }
                        Sample.Item item = (Sample.Item)boxed;
                        using WindowsCsharp.ComLease itemLease = item.Acquire();
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, itemLease.Handle));
                        return;
                    }
                    case 1:
                    {
                        object? boxed = value;
                        if (boxed is null)
                        {
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, 0));
                            return;
                        }
                        Windows.Foundation.IInspectable item = (Windows.Foundation.IInspectable)boxed;
                        using WindowsCsharp.ComLease itemLease = item.Acquire();
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, itemLease.Handle));
                        return;
                    }
                    case 2:
                    {
                        string? item = (string?)(object?)value;
                        fixed (char* buffer = item)
                        {
                            WindowsCsharp.Interop.HstringHeader header;
                            nint hstring = WindowsCsharp.Interop.CreateStringReference((ushort*)buffer, (uint)(item?.Length ?? 0), &header);
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[13])(self, hstring));
                        }
                        return;
                    }
                }
            }
            if (typeof(T) == typeof(int))
            {
                int item = Unsafe.As<T, int>(ref value);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[13])(self, item));
                return;
            }
            throw new NotSupportedException();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Enumerator GetEnumerator() => new Enumerator(this);

        public struct Enumerator : IDisposable
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

            public void Dispose()
            {
                for (int i = _index; i < _length; i++)
                {
                    ref T value = ref Unsafe.Add(ref Unsafe.As<Buffer, T>(ref _buffer), i);
                    if (value is WindowsCsharp.ComObject item)
                    {
                        item.Dispose();
                    }
                    value = default!;
                }
                _index = _length;
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
                return GetAtAbi(_this, index);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint GetMany(uint startIndex, Span<T> items)
            {
                return GetManyAbi(_this, startIndex, items);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Append(T value) => AppendAbi(_this, value);

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void RemoveAtEnd() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[14])(_this));

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Clear() => WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)_this)[15])(_this));

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface> => WindowsCsharp.Com.As<TInterface>(_this, false);
        }
    }

    public static class IVectorObjectExtensions
    {
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static void Append<TValue>(this IVector<Sample.Item?> vector, TValue? value) where TValue : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TValue>, WindowsCsharp.IObjectParameter<Sample.Item._Parameter>
        {
            vector.AppendObject(value, Sample.Item.Iid);
        }

    }

    public sealed unsafe class IVectorView<T> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IVectorView<T>>
    {
        private static readonly int s_referenceType = ComputeReferenceType();
        public static Guid Iid { get; } = ComputeIid();

        private static int ComputeReferenceType()
        {
            if (!RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                return -1;
            }
            if (typeof(T) == typeof(string)) return 0;
            throw new NotSupportedException();
        }

        private static Guid ComputeIid()
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0: return new Guid(0x2f13c006, 0xa03a, 0x5f69, 0xb0, 0x90, 0x75, 0xa4, 0x3e, 0x33, 0x42, 0x3e);
                }
            }
            if (typeof(T) == typeof(int)) return new Guid(0x8d720cdf, 0x3934, 0x5d3f, 0x9a, 0x55, 0x40, 0xe8, 0x06, 0x3b, 0x08, 0x6a);
            throw new NotSupportedException();
        }

        internal IVectorView(nint self) : base(self, Iid) {}
        internal IVectorView(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IVectorView<T> WindowsCsharp.IComInterface<IVectorView<T>>.FromAbi(nint self) => new IVectorView<T>(self);
        static IVectorView<T> WindowsCsharp.IComInterface<IVectorView<T>>.FromAgileAbi(nint self) => new IVectorView<T>(self, true);

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
            return GetAtAbi(self, index);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint GetMany(uint startIndex, Span<T> items)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return GetManyAbi(self, startIndex, items);
        }

        private static T GetAtAbi(nint self, uint index)
        {
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0:
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, nint*, int>)(*(void***)self)[6])(self, index, &result));
                        string value = WindowsCsharp.Interop.FromHstring(result)!;
                        return (T)(object)value;
                    }
                }
            }
            if (typeof(T) == typeof(int))
            {
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int*, int>)(*(void***)self)[6])(self, index, &result));
                int value = result;
                return Unsafe.As<int, T>(ref value);
            }
            throw new NotSupportedException();
        }

        private static uint GetManyAbi(nint self, uint startIndex, Span<T> items)
        {
            if (items.IsEmpty)
            {
                return 0;
            }
            if (typeof(T) == typeof(int))
            {
                uint actual;
                ref T first = ref MemoryMarshal.GetReference(items);
                fixed (int* p = &Unsafe.As<T, int>(ref first))
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, int*, uint*, int>)(*(void***)self)[9])(self, startIndex, (uint)items.Length, p, &actual));
                }
                return actual;
            }
            if (RuntimeHelpers.IsReferenceOrContainsReferences<T>())
            {
                switch (s_referenceType)
                {
                    case 0:
                    {
                        const uint Capacity = 64;
                        nint* values = stackalloc nint[(int)Capacity];
                        uint total = 0;
                        while (total < (uint)items.Length)
                        {
                            uint requested = Math.Min(Capacity, (uint)items.Length - total);
                            for (uint i = 0; i < requested; i++)
                            {
                                values[i] = 0;
                            }
                            uint actual;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, uint, nint*, uint*, int>)(*(void***)self)[9])(self, startIndex + total, requested, values, &actual));
                            try
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    nint abi = values[i];
                                    values[i] = 0;
                                    string value = WindowsCsharp.Interop.FromHstring(abi);
                                    items[(int)(total + i)] = (T)(object)value;
                                }
                            }
                            finally
                            {
                                for (uint i = 0; i < actual; i++)
                                {
                                    if (values[i] != 0)
                                    {
                                        _ = WindowsCsharp.Interop.WindowsDeleteString(values[i]);
                                    }
                                }
                            }
                            total += actual;
                            if (actual < requested)
                            {
                                break;
                            }
                        }
                        return total;
                    }
                }
            }
            uint size;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[7])(self, &size));
            if (startIndex >= size)
            {
                return 0;
            }
            uint count = Math.Min((uint)items.Length, size - startIndex);
            for (uint i = 0; i < count; i++)
            {
                items[(int)i] = GetAtAbi(self, startIndex + i);
            }
            return count;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Enumerator GetEnumerator() => new Enumerator(this);

        public struct Enumerator
        {
            private const int BufferLength = 64;
            private readonly IVectorView<T> _vector;
            private Buffer _buffer;
            private uint _start;
            private int _index;
            private int _length;
            private T _current;

            internal Enumerator(IVectorView<T> vector)
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
                return GetAtAbi(_this, index);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public uint GetMany(uint startIndex, Span<T> items)
            {
                return GetManyAbi(_this, startIndex, items);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface> => WindowsCsharp.Com.As<TInterface>(_this, false);
        }
    }

    public sealed unsafe class IMap<K, V> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IMap<K, V>>
    {
        public static Guid Iid { get; } = ComputeIid();

        private static Guid ComputeIid()
        {
            if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int)) return new Guid(0x732c4374, 0x5ddd, 0x553d, 0xa8, 0x82, 0x12, 0xeb, 0xcd, 0xbf, 0x06, 0x09);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item)) return new Guid(0x5029bda8, 0x6878, 0x5814, 0x9c, 0x71, 0xff, 0x8c, 0x98, 0x39, 0x66, 0x88);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(int)) return new Guid(0x19da7f0f, 0xdb46, 0x5b15, 0x8e, 0x00, 0x27, 0xcb, 0xa1, 0xf7, 0xb4, 0x1d);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(string)) return new Guid(0x3835d7ac, 0xfcf8, 0x5c8b, 0x9b, 0x5c, 0x2b, 0x1b, 0xc4, 0x2c, 0xec, 0x74);
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0xae681871, 0xdd82, 0x5299, 0x93, 0xea, 0x02, 0x75, 0xe4, 0xe0, 0x73, 0xe7);
            throw new NotSupportedException();
        }

        private static Guid ComputeIterableIid()
        {
            if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int)) return new Guid(0x7279ce08, 0xdfae, 0x5883, 0xb7, 0x24, 0xbf, 0xe8, 0x0a, 0xe7, 0x38, 0x2f);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item)) return new Guid(0xb3a149ab, 0x0cd8, 0x5978, 0xba, 0x33, 0xf4, 0xab, 0x40, 0x29, 0x8a, 0x7c);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(int)) return new Guid(0xd3827986, 0x1127, 0x5e0a, 0x99, 0xeb, 0xfb, 0x27, 0x67, 0x49, 0x68, 0x3f);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(string)) return new Guid(0x8e5a70be, 0x6911, 0x5869, 0x97, 0x49, 0x4f, 0x8d, 0x50, 0xf2, 0x7f, 0x85);
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0x2aa69c56, 0xc3a4, 0x58f9, 0xb1, 0x4c, 0x46, 0x5b, 0xca, 0xf8, 0xc7, 0xba);
            throw new NotSupportedException();
        }

        private static Guid ComputeIteratorIid()
        {
            if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int)) return new Guid(0xfedc13b6, 0xbfcf, 0x53d1, 0xa0, 0xd2, 0x1c, 0xa1, 0x8e, 0xb2, 0x0e, 0x27);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item)) return new Guid(0x493a2fd4, 0x22ae, 0x5e1d, 0x87, 0x10, 0xd5, 0x33, 0x39, 0x71, 0x1c, 0xfe);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(int)) return new Guid(0xed5f93a3, 0x7002, 0x5ac5, 0xb1, 0x0f, 0x76, 0x38, 0xac, 0x11, 0xaf, 0x0e);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(string)) return new Guid(0x41aa9c36, 0x1926, 0x5006, 0x83, 0x66, 0x01, 0xd7, 0xe3, 0xca, 0x77, 0x7b);
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
            if (typeof(K) == typeof(Sample.Item))
            {
                object? boxedKey = key;
                if (boxedKey is null)
                {
                    if (typeof(V) == typeof(int))
                    {
                        int result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, 0, &result));
                        int value = result;
                        return Unsafe.As<int, V>(ref value);
                    }
                    throw new NotSupportedException();
                }
                Sample.Item objectKey = (Sample.Item)boxedKey;
                using WindowsCsharp.ComLease keyLease = objectKey.Acquire();
                if (typeof(V) == typeof(int))
                {
                    int result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, keyLease.Handle, &result));
                    int value = result;
                    return Unsafe.As<int, V>(ref value);
                }
                throw new NotSupportedException();
            }
            if (typeof(K) == typeof(int))
            {
                int abiKey = Unsafe.As<K, int>(ref key);
                if (typeof(V) == typeof(Sample.Item))
                {
                    nint result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[6])(self, abiKey, &result));
                    Sample.Item value = WindowsCsharp.Com.Wrap<Sample.Item>(result);
                    return (V)(object?)value!;
                }
                if (typeof(V) == typeof(int))
                {
                    int result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[6])(self, abiKey, &result));
                    int value = result;
                    return Unsafe.As<int, V>(ref value);
                }
                if (typeof(V) == typeof(string))
                {
                    nint result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[6])(self, abiKey, &result));
                    string value = WindowsCsharp.Interop.FromHstring(result);
                    return (V)(object)value;
                }
                throw new NotSupportedException();
            }
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
            if (typeof(K) == typeof(Sample.Item))
            {
                object? boxedKey = key;
                using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                byte result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***)self)[8])(self, keyLease.Handle, &result));
                return result != 0;
            }
            if (typeof(K) == typeof(int))
            {
                int abiKey = Unsafe.As<K, int>(ref key);
                byte result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, byte*, int>)(*(void***)self)[8])(self, abiKey, &result));
                return result != 0;
            }
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
            if (typeof(K) == typeof(Sample.Item))
            {
                object? boxedKey = key;
                using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                if (typeof(V) == typeof(int))
                {
                    int abiValue = Unsafe.As<V, int>(ref value);
                    byte replaced;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int, byte*, int>)(*(void***)self)[10])(self, keyLease.Handle, abiValue, &replaced));
                    return replaced != 0;
                }
                throw new NotSupportedException();
            }
            if (typeof(K) == typeof(int))
            {
                int abiKey = Unsafe.As<K, int>(ref key);
                if (typeof(V) == typeof(Sample.Item))
                {
                    object? boxedValue = value;
                    if (boxedValue is null)
                    {
                        byte replaced;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint, byte*, int>)(*(void***)self)[10])(self, abiKey, 0, &replaced));
                        return replaced != 0;
                    }
                    Sample.Item objectValue = (Sample.Item)boxedValue;
                    using WindowsCsharp.ComLease valueLease = objectValue.Acquire();
                    byte replacedValue;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint, byte*, int>)(*(void***)self)[10])(self, abiKey, valueLease.Handle, &replacedValue));
                    return replacedValue != 0;
                }
                if (typeof(V) == typeof(int))
                {
                    int abiValue = Unsafe.As<V, int>(ref value);
                    byte replaced;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, byte*, int>)(*(void***)self)[10])(self, abiKey, abiValue, &replaced));
                    return replaced != 0;
                }
                if (typeof(V) == typeof(string))
                {
                    string? textValue = (string?)(object?)value;
                    fixed (char* valueChars = textValue)
                    {
                        WindowsCsharp.Interop.HstringHeader valueHeader;
                        nint abiValue = WindowsCsharp.Interop.CreateStringReference((ushort*)valueChars, (uint)(textValue?.Length ?? 0), &valueHeader);
                        byte replaced;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint, byte*, int>)(*(void***)self)[10])(self, abiKey, abiValue, &replaced));
                        return replaced != 0;
                    }
                }
                throw new NotSupportedException();
            }
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
            if (typeof(K) == typeof(Sample.Item))
            {
                object? boxedKey = key;
                using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[11])(self, keyLease.Handle));
                return;
            }
            if (typeof(K) == typeof(int))
            {
                int abiKey = Unsafe.As<K, int>(ref key);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[11])(self, abiKey));
                return;
            }
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
                        if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int))
                        {
                            nint abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            Sample.Item keyValue = WindowsCsharp.Com.Wrap<Sample.Item>(abiKey);
                            K key = (K)(object?)keyValue!;
                            try
                            {
                                int abiValue;
                                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[7])(pair, &abiValue));
                                int valueResult = abiValue;
                                V value = Unsafe.As<int, V>(ref valueResult);
                                return new Entry(key, value);
                            }
                            catch
                            {
                                object? keyObject = key;
                                if (keyObject is WindowsCsharp.ComObject owner)
                                {
                                    owner.Dispose();
                                }
                                throw;
                            }
                        }
                        if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item))
                        {
                            int abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            int keyValue = abiKey;
                            K key = Unsafe.As<int, K>(ref keyValue);
                            nint abiValue;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)pair)[7])(pair, &abiValue));
                            Sample.Item valueResult = WindowsCsharp.Com.Wrap<Sample.Item>(abiValue);
                            V value = (V)(object?)valueResult!;
                            return new Entry(key, value);
                        }
                        if (typeof(K) == typeof(int) && typeof(V) == typeof(int))
                        {
                            int abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            int keyValue = abiKey;
                            K key = Unsafe.As<int, K>(ref keyValue);
                            int abiValue;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[7])(pair, &abiValue));
                            int valueResult = abiValue;
                            V value = Unsafe.As<int, V>(ref valueResult);
                            return new Entry(key, value);
                        }
                        if (typeof(K) == typeof(int) && typeof(V) == typeof(string))
                        {
                            int abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            int keyValue = abiKey;
                            K key = Unsafe.As<int, K>(ref keyValue);
                            nint abiValue;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)pair)[7])(pair, &abiValue));
                            string valueResult = WindowsCsharp.Interop.FromHstring(abiValue);
                            V value = (V)(object)valueResult;
                            return new Entry(key, value);
                        }
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
                if (typeof(K) == typeof(Sample.Item))
                {
                    object? boxedKey = key;
                    if (boxedKey is null)
                    {
                        if (typeof(V) == typeof(int))
                        {
                            int result;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)_this)[6])(_this, 0, &result));
                            int value = result;
                            return Unsafe.As<int, V>(ref value);
                        }
                        throw new NotSupportedException();
                    }
                    Sample.Item objectKey = (Sample.Item)boxedKey;
                    using WindowsCsharp.ComLease keyLease = objectKey.Acquire();
                    if (typeof(V) == typeof(int))
                    {
                        int result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)_this)[6])(_this, keyLease.Handle, &result));
                        int value = result;
                        return Unsafe.As<int, V>(ref value);
                    }
                    throw new NotSupportedException();
                }
                if (typeof(K) == typeof(int))
                {
                    int abiKey = Unsafe.As<K, int>(ref key);
                    if (typeof(V) == typeof(Sample.Item))
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)_this)[6])(_this, abiKey, &result));
                        Sample.Item value = WindowsCsharp.Com.Wrap<Sample.Item>(result);
                        return (V)(object?)value!;
                    }
                    if (typeof(V) == typeof(int))
                    {
                        int result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)_this)[6])(_this, abiKey, &result));
                        int value = result;
                        return Unsafe.As<int, V>(ref value);
                    }
                    if (typeof(V) == typeof(string))
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)_this)[6])(_this, abiKey, &result));
                        string value = WindowsCsharp.Interop.FromHstring(result);
                        return (V)(object)value;
                    }
                    throw new NotSupportedException();
                }
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
                if (typeof(K) == typeof(Sample.Item))
                {
                    object? boxedKey = key;
                    using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                    byte result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***)_this)[8])(_this, keyLease.Handle, &result));
                    return result != 0;
                }
                if (typeof(K) == typeof(int))
                {
                    int abiKey = Unsafe.As<K, int>(ref key);
                    byte result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, byte*, int>)(*(void***)_this)[8])(_this, abiKey, &result));
                    return result != 0;
                }
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
                if (typeof(K) == typeof(Sample.Item))
                {
                    object? boxedKey = key;
                    using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                    if (typeof(V) == typeof(int))
                    {
                        int abiValue = Unsafe.As<V, int>(ref value);
                        byte replaced;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int, byte*, int>)(*(void***)_this)[10])(_this, keyLease.Handle, abiValue, &replaced));
                        return replaced != 0;
                    }
                    throw new NotSupportedException();
                }
                if (typeof(K) == typeof(int))
                {
                    int abiKey = Unsafe.As<K, int>(ref key);
                    if (typeof(V) == typeof(Sample.Item))
                    {
                        object? boxedValue = value;
                        if (boxedValue is null)
                        {
                            byte replaced;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint, byte*, int>)(*(void***)_this)[10])(_this, abiKey, 0, &replaced));
                            return replaced != 0;
                        }
                        Sample.Item objectValue = (Sample.Item)boxedValue;
                        using WindowsCsharp.ComLease valueLease = objectValue.Acquire();
                        byte replacedValue;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint, byte*, int>)(*(void***)_this)[10])(_this, abiKey, valueLease.Handle, &replacedValue));
                        return replacedValue != 0;
                    }
                    if (typeof(V) == typeof(int))
                    {
                        int abiValue = Unsafe.As<V, int>(ref value);
                        byte replaced;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, byte*, int>)(*(void***)_this)[10])(_this, abiKey, abiValue, &replaced));
                        return replaced != 0;
                    }
                    if (typeof(V) == typeof(string))
                    {
                        string? textValue = (string?)(object?)value;
                        fixed (char* valueChars = textValue)
                        {
                            WindowsCsharp.Interop.HstringHeader valueHeader;
                            nint abiValue = WindowsCsharp.Interop.CreateStringReference((ushort*)valueChars, (uint)(textValue?.Length ?? 0), &valueHeader);
                            byte replaced;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint, byte*, int>)(*(void***)_this)[10])(_this, abiKey, abiValue, &replaced));
                            return replaced != 0;
                        }
                    }
                    throw new NotSupportedException();
                }
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
                if (typeof(K) == typeof(Sample.Item))
                {
                    object? boxedKey = key;
                    using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)_this)[11])(_this, keyLease.Handle));
                    return;
                }
                if (typeof(K) == typeof(int))
                {
                    int abiKey = Unsafe.As<K, int>(ref key);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)_this)[11])(_this, abiKey));
                    return;
                }
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

    public sealed unsafe class IMapView<K, V> : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IMapView<K, V>>
    {
        public static Guid Iid { get; } = ComputeIid();

        private static Guid ComputeIid()
        {
            if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int)) return new Guid(0x15eb3f7b, 0x948c, 0x5706, 0x9e, 0xbc, 0x4a, 0xd9, 0x50, 0x72, 0xaf, 0x23);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item)) return new Guid(0x3e2bab07, 0xd8b6, 0x55c9, 0xbf, 0x9f, 0xd3, 0x40, 0xb4, 0xcb, 0x0d, 0x33);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(int)) return new Guid(0x14815e90, 0x9809, 0x56af, 0x9d, 0xc0, 0x74, 0x59, 0xa8, 0xf7, 0x28, 0x41);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(string)) return new Guid(0x3cb34a71, 0x4741, 0x59c3, 0xa2, 0xd6, 0xfa, 0x7a, 0x00, 0xc0, 0xe9, 0x0c);
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0x06c17849, 0xdfc8, 0x501a, 0xbf, 0x47, 0x16, 0x15, 0x2f, 0xa2, 0x1d, 0x4b);
            throw new NotSupportedException();
        }

        private static Guid ComputeIterableIid()
        {
            if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int)) return new Guid(0x7279ce08, 0xdfae, 0x5883, 0xb7, 0x24, 0xbf, 0xe8, 0x0a, 0xe7, 0x38, 0x2f);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item)) return new Guid(0xb3a149ab, 0x0cd8, 0x5978, 0xba, 0x33, 0xf4, 0xab, 0x40, 0x29, 0x8a, 0x7c);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(int)) return new Guid(0xd3827986, 0x1127, 0x5e0a, 0x99, 0xeb, 0xfb, 0x27, 0x67, 0x49, 0x68, 0x3f);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(string)) return new Guid(0x8e5a70be, 0x6911, 0x5869, 0x97, 0x49, 0x4f, 0x8d, 0x50, 0xf2, 0x7f, 0x85);
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0x2aa69c56, 0xc3a4, 0x58f9, 0xb1, 0x4c, 0x46, 0x5b, 0xca, 0xf8, 0xc7, 0xba);
            throw new NotSupportedException();
        }

        private static Guid ComputeIteratorIid()
        {
            if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int)) return new Guid(0xfedc13b6, 0xbfcf, 0x53d1, 0xa0, 0xd2, 0x1c, 0xa1, 0x8e, 0xb2, 0x0e, 0x27);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item)) return new Guid(0x493a2fd4, 0x22ae, 0x5e1d, 0x87, 0x10, 0xd5, 0x33, 0x39, 0x71, 0x1c, 0xfe);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(int)) return new Guid(0xed5f93a3, 0x7002, 0x5ac5, 0xb1, 0x0f, 0x76, 0x38, 0xac, 0x11, 0xaf, 0x0e);
            if (typeof(K) == typeof(int) && typeof(V) == typeof(string)) return new Guid(0x41aa9c36, 0x1926, 0x5006, 0x83, 0x66, 0x01, 0xd7, 0xe3, 0xca, 0x77, 0x7b);
            if (typeof(K) == typeof(string) && typeof(V) == typeof(int)) return new Guid(0x96c8b304, 0x4108, 0x5f67, 0x8b, 0x2f, 0x21, 0x39, 0x75, 0xf0, 0x85, 0xb2);
            throw new NotSupportedException();
        }

        internal IMapView(nint self) : base(self, Iid) {}
        internal IMapView(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IMapView<K, V> WindowsCsharp.IComInterface<IMapView<K, V>>.FromAbi(nint self) => new IMapView<K, V>(self);
        static IMapView<K, V> WindowsCsharp.IComInterface<IMapView<K, V>>.FromAgileAbi(nint self) => new IMapView<K, V>(self, true);

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
            if (typeof(K) == typeof(Sample.Item))
            {
                object? boxedKey = key;
                if (boxedKey is null)
                {
                    if (typeof(V) == typeof(int))
                    {
                        int result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, 0, &result));
                        int value = result;
                        return Unsafe.As<int, V>(ref value);
                    }
                    throw new NotSupportedException();
                }
                Sample.Item objectKey = (Sample.Item)boxedKey;
                using WindowsCsharp.ComLease keyLease = objectKey.Acquire();
                if (typeof(V) == typeof(int))
                {
                    int result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)self)[6])(self, keyLease.Handle, &result));
                    int value = result;
                    return Unsafe.As<int, V>(ref value);
                }
                throw new NotSupportedException();
            }
            if (typeof(K) == typeof(int))
            {
                int abiKey = Unsafe.As<K, int>(ref key);
                if (typeof(V) == typeof(Sample.Item))
                {
                    nint result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[6])(self, abiKey, &result));
                    Sample.Item value = WindowsCsharp.Com.Wrap<Sample.Item>(result);
                    return (V)(object?)value!;
                }
                if (typeof(V) == typeof(int))
                {
                    int result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[6])(self, abiKey, &result));
                    int value = result;
                    return Unsafe.As<int, V>(ref value);
                }
                if (typeof(V) == typeof(string))
                {
                    nint result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[6])(self, abiKey, &result));
                    string value = WindowsCsharp.Interop.FromHstring(result);
                    return (V)(object)value;
                }
                throw new NotSupportedException();
            }
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
            if (typeof(K) == typeof(Sample.Item))
            {
                object? boxedKey = key;
                using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                byte result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***)self)[8])(self, keyLease.Handle, &result));
                return result != 0;
            }
            if (typeof(K) == typeof(int))
            {
                int abiKey = Unsafe.As<K, int>(ref key);
                byte result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, byte*, int>)(*(void***)self)[8])(self, abiKey, &result));
                return result != 0;
            }
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
                        if (typeof(K) == typeof(Sample.Item) && typeof(V) == typeof(int))
                        {
                            nint abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            Sample.Item keyValue = WindowsCsharp.Com.Wrap<Sample.Item>(abiKey);
                            K key = (K)(object?)keyValue!;
                            try
                            {
                                int abiValue;
                                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[7])(pair, &abiValue));
                                int valueResult = abiValue;
                                V value = Unsafe.As<int, V>(ref valueResult);
                                return new Entry(key, value);
                            }
                            catch
                            {
                                object? keyObject = key;
                                if (keyObject is WindowsCsharp.ComObject owner)
                                {
                                    owner.Dispose();
                                }
                                throw;
                            }
                        }
                        if (typeof(K) == typeof(int) && typeof(V) == typeof(Sample.Item))
                        {
                            int abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            int keyValue = abiKey;
                            K key = Unsafe.As<int, K>(ref keyValue);
                            nint abiValue;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)pair)[7])(pair, &abiValue));
                            Sample.Item valueResult = WindowsCsharp.Com.Wrap<Sample.Item>(abiValue);
                            V value = (V)(object?)valueResult!;
                            return new Entry(key, value);
                        }
                        if (typeof(K) == typeof(int) && typeof(V) == typeof(int))
                        {
                            int abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            int keyValue = abiKey;
                            K key = Unsafe.As<int, K>(ref keyValue);
                            int abiValue;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[7])(pair, &abiValue));
                            int valueResult = abiValue;
                            V value = Unsafe.As<int, V>(ref valueResult);
                            return new Entry(key, value);
                        }
                        if (typeof(K) == typeof(int) && typeof(V) == typeof(string))
                        {
                            int abiKey;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)pair)[6])(pair, &abiKey));
                            int keyValue = abiKey;
                            K key = Unsafe.As<int, K>(ref keyValue);
                            nint abiValue;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)pair)[7])(pair, &abiValue));
                            string valueResult = WindowsCsharp.Interop.FromHstring(abiValue);
                            V value = (V)(object)valueResult;
                            return new Entry(key, value);
                        }
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
                if (typeof(K) == typeof(Sample.Item))
                {
                    object? boxedKey = key;
                    if (boxedKey is null)
                    {
                        if (typeof(V) == typeof(int))
                        {
                            int result;
                            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)_this)[6])(_this, 0, &result));
                            int value = result;
                            return Unsafe.As<int, V>(ref value);
                        }
                        throw new NotSupportedException();
                    }
                    Sample.Item objectKey = (Sample.Item)boxedKey;
                    using WindowsCsharp.ComLease keyLease = objectKey.Acquire();
                    if (typeof(V) == typeof(int))
                    {
                        int result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int*, int>)(*(void***)_this)[6])(_this, keyLease.Handle, &result));
                        int value = result;
                        return Unsafe.As<int, V>(ref value);
                    }
                    throw new NotSupportedException();
                }
                if (typeof(K) == typeof(int))
                {
                    int abiKey = Unsafe.As<K, int>(ref key);
                    if (typeof(V) == typeof(Sample.Item))
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)_this)[6])(_this, abiKey, &result));
                        Sample.Item value = WindowsCsharp.Com.Wrap<Sample.Item>(result);
                        return (V)(object?)value!;
                    }
                    if (typeof(V) == typeof(int))
                    {
                        int result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)_this)[6])(_this, abiKey, &result));
                        int value = result;
                        return Unsafe.As<int, V>(ref value);
                    }
                    if (typeof(V) == typeof(string))
                    {
                        nint result;
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)_this)[6])(_this, abiKey, &result));
                        string value = WindowsCsharp.Interop.FromHstring(result);
                        return (V)(object)value;
                    }
                    throw new NotSupportedException();
                }
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
                if (typeof(K) == typeof(Sample.Item))
                {
                    object? boxedKey = key;
                    using WindowsCsharp.ComLease keyLease = WindowsCsharp.ComLease.From(boxedKey as WindowsCsharp.ComObject);
                    byte result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, byte*, int>)(*(void***)_this)[8])(_this, keyLease.Handle, &result));
                    return result != 0;
                }
                if (typeof(K) == typeof(int))
                {
                    int abiKey = Unsafe.As<K, int>(ref key);
                    byte result;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, byte*, int>)(*(void***)_this)[8])(_this, abiKey, &result));
                    return result != 0;
                }
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
            public TInterface As<TInterface>() where TInterface : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<TInterface> => WindowsCsharp.Com.As<TInterface>(_this, false);
        }
    }
}
