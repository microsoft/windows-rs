namespace DelegateMarshalling
{
    public sealed unsafe class ObjectCallback : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<ObjectCallback>, WindowsCsharp.IObjectParameter<DelegateMarshalling.ObjectCallback._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x39eb9c40, 0xce9b, 0x5646, 0xb5, 0xf1, 0x2e, 0xa4, 0x5c, 0xd8, 0x6e, 0x3d);

        internal ObjectCallback(nint self) : base(self, Iid) {}
        internal ObjectCallback(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static ObjectCallback WindowsCsharp.IComInterface<ObjectCallback>.FromAbi(nint self) => new ObjectCallback(self);
        static ObjectCallback WindowsCsharp.IComInterface<ObjectCallback>.FromAgileAbi(nint self) => new ObjectCallback(self, true);

        public delegate DelegateMarshalling.IDelegatePeer? Callback(DelegateMarshalling.IDelegatePeer.Borrowed value);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, nint, nint*, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static ObjectCallback Create(Callback handler) => WindowsCsharp.Com.WrapAgile<ObjectCallback>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, nint value, nint* result)
        {
            if (result == null)
            {
                return unchecked((int)0x80004003);
            }
            *result = default;
            nint ownedResult = 0;
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                using WindowsCsharp.ComLease resultLease = WindowsCsharp.ComLease.From(callback(new DelegateMarshalling.IDelegatePeer.Borrowed(value)));
                nint resultValue = resultLease.Handle;
                if (resultValue != 0)
                {
                    _ = WindowsCsharp.Com.AddRef(resultValue);
                    ownedResult = resultValue;
                }
                *result = ownedResult;
                ownedResult = 0;
                return 0;
            }
            catch (Exception error)
            {
                if (ownedResult != 0)
                {
                    _ = WindowsCsharp.Com.Release(ownedResult);
                }
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public DelegateMarshalling.IDelegatePeer? Invoke(DelegateMarshalling.IDelegatePeer? value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(value);
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[3])(self, _olease0.Handle, &result));
            return WindowsCsharp.Com.Wrap<DelegateMarshalling.IDelegatePeer>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public DelegateMarshalling.IDelegatePeer? Invoke<T0>(T0? value) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<DelegateMarshalling.IDelegatePeer._Parameter>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(value, DelegateMarshalling.IDelegatePeer.Iid);
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[3])(self, _olease0.Handle, &result));
            return WindowsCsharp.Com.Wrap<DelegateMarshalling.IDelegatePeer>(result);
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

    public sealed unsafe class ScalarCallback : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<ScalarCallback>, WindowsCsharp.IObjectParameter<DelegateMarshalling.ScalarCallback._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x74176de2, 0xae58, 0x5596, 0xb1, 0x7b, 0x8d, 0xfa, 0xb5, 0x08, 0xaf, 0x1d);

        internal ScalarCallback(nint self) : base(self, Iid) {}
        internal ScalarCallback(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static ScalarCallback WindowsCsharp.IComInterface<ScalarCallback>.FromAbi(nint self) => new ScalarCallback(self);
        static ScalarCallback WindowsCsharp.IComInterface<ScalarCallback>.FromAgileAbi(nint self) => new ScalarCallback(self, true);

        public delegate int Callback(int value);

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
        public static ScalarCallback Create(Callback handler) => WindowsCsharp.Com.WrapAgile<ScalarCallback>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

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
                *result = callback(value);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Invoke(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[3])(self, value, &result));
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

    public sealed unsafe class StringCallback : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<StringCallback>, WindowsCsharp.IObjectParameter<DelegateMarshalling.StringCallback._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x6aa186f8, 0x93c3, 0x5358, 0xb6, 0x73, 0xdd, 0x51, 0x01, 0x39, 0xd7, 0xb7);

        internal StringCallback(nint self) : base(self, Iid) {}
        internal StringCallback(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static StringCallback WindowsCsharp.IComInterface<StringCallback>.FromAbi(nint self) => new StringCallback(self);
        static StringCallback WindowsCsharp.IComInterface<StringCallback>.FromAgileAbi(nint self) => new StringCallback(self, true);

        public delegate string Callback(string value);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, nint, nint*, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static StringCallback Create(Callback handler) => WindowsCsharp.Com.WrapAgile<StringCallback>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, nint value, nint* result)
        {
            if (result == null)
            {
                return unchecked((int)0x80004003);
            }
            *result = default;
            nint ownedResult = 0;
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                ownedResult = WindowsCsharp.Interop.CreateString(callback(WindowsCsharp.Interop.FromHstringBorrowed(value)));
                *result = ownedResult;
                ownedResult = 0;
                return 0;
            }
            catch (Exception error)
            {
                WindowsCsharp.Interop.DeleteHstring(ref ownedResult);
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public string Invoke(string value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            nint result;
            fixed (char* _hbuf0 = value)
            {
                WindowsCsharp.Interop.HstringHeader _hhdr0;
                nint _hstr0 = WindowsCsharp.Interop.CreateStringReference((ushort*)_hbuf0, (uint)(value?.Length ?? 0), &_hhdr0);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[3])(self, _hstr0, &result));
            }
            return WindowsCsharp.Interop.FromHstring(result);
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

    public sealed unsafe class IDelegateHost : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IDelegateHost>, WindowsCsharp.IObjectParameter<DelegateMarshalling.IDelegateHost._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x01d71c62, 0x29d8, 0x5d61, 0xbe, 0x51, 0x2d, 0x8c, 0x49, 0x50, 0x5c, 0xcf);

        internal IDelegateHost(nint self) : base(self, Iid) {}
        internal IDelegateHost(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IDelegateHost WindowsCsharp.IComInterface<IDelegateHost>.FromAbi(nint self) => new IDelegateHost(self);
        static IDelegateHost WindowsCsharp.IComInterface<IDelegateHost>.FromAgileAbi(nint self) => new IDelegateHost(self, true);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UseString(DelegateMarshalling.StringCallback? callback)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[6])(self, _olease0.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UseString<T0>(T0? callback) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<DelegateMarshalling.StringCallback._Parameter>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(callback, DelegateMarshalling.StringCallback.Iid);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[6])(self, _olease0.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UseObject(DelegateMarshalling.ObjectCallback? callback)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, _olease0.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void UseObject<T0>(T0? callback) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<DelegateMarshalling.ObjectCallback._Parameter>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(callback, DelegateMarshalling.ObjectCallback.Iid);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, _olease0.Handle));
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
            public void UseString(DelegateMarshalling.StringCallback? callback)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[6])(self, _olease0.Handle));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void UseString<T0>(T0? callback) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<DelegateMarshalling.StringCallback._Parameter>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(callback, DelegateMarshalling.StringCallback.Iid);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[6])(self, _olease0.Handle));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void UseObject(DelegateMarshalling.ObjectCallback? callback)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, _olease0.Handle));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void UseObject<T0>(T0? callback) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<DelegateMarshalling.ObjectCallback._Parameter>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(callback, DelegateMarshalling.ObjectCallback.Iid);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[7])(self, _olease0.Handle));
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

    public sealed unsafe class IDelegatePeer : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IDelegatePeer>, WindowsCsharp.IObjectParameter<DelegateMarshalling.IDelegatePeer._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x903304db, 0x86cb, 0x5f8f, 0xb7, 0x17, 0xae, 0x8a, 0xd9, 0xe5, 0xd5, 0xc2);

        internal IDelegatePeer(nint self) : base(self, Iid) {}
        internal IDelegatePeer(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IDelegatePeer WindowsCsharp.IComInterface<IDelegatePeer>.FromAbi(nint self) => new IDelegatePeer(self);
        static IDelegatePeer WindowsCsharp.IComInterface<IDelegatePeer>.FromAgileAbi(nint self) => new IDelegatePeer(self, true);

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
