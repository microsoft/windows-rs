namespace Activation
{
    [StructLayout(LayoutKind.Sequential)]
    public struct Options
    {
        public string Label;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OptionsAbi
    {
        public nint Label;

        internal static OptionsAbi FromSurface(Options value)
        {
            OptionsAbi result = default;
            try
            {
                result.Label = WindowsCsharp.Interop.CreateString(value.Label);
                return result;
            }
            catch
            {
                result.Dispose();
                throw;
            }
        }

        internal readonly Options FromAbi() => new()
        {
            Label = WindowsCsharp.Interop.FromHstringBorrowed(Label),
        };

        internal Options ToSurface()
        {
            Options result = default;
            try
            {
                result.Label = WindowsCsharp.Interop.TakeHstring(ref Label);
                return result;
            }
            finally
            {
                Dispose();
            }
        }

        internal void Dispose()
        {
            WindowsCsharp.Interop.DeleteHstring(ref Label);
        }
    }

    public sealed unsafe class StartCallback : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<StartCallback>, WindowsCsharp.IObjectParameter<Activation.StartCallback._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x61fbe17a, 0xae5d, 0x59f4, 0xb1, 0x5f, 0x3e, 0x35, 0x9f, 0x16, 0xcf, 0x40);

        internal StartCallback(nint self) : base(self, Iid) {}
        internal StartCallback(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static StartCallback WindowsCsharp.IComInterface<StartCallback>.FromAbi(nint self) => new StartCallback(self);
        static StartCallback WindowsCsharp.IComInterface<StartCallback>.FromAgileAbi(nint self) => new StartCallback(self, true);

        public delegate void Callback();

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static StartCallback Create(Callback handler) => WindowsCsharp.Com.WrapAgile<StartCallback>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self)
        {
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                callback();
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Invoke()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[3])(self));
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

    public sealed unsafe class Control : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Control>, WindowsCsharp.IObjectParameter<Activation.Control._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        public static Guid Iid { get; } = new Guid(0xd5c650a9, 0x026f, 0x5785, 0x9a, 0xcc, 0x93, 0x58, 0xb4, 0x3a, 0xe4, 0x33);
        private static nint s_factory0;
        private static readonly Guid s_factory0_iid = new Guid(0xc8b5a3fe, 0x3fdb, 0x51de, 0xb1, 0x17, 0xe1, 0xbe, 0x24, 0x45, 0xe2, 0x75);

        internal Control(nint self) : base(self, Iid) {}
        internal Control(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Control WindowsCsharp.IComInterface<Control>.FromAbi(nint self) => new Control(self);
        static Control WindowsCsharp.IComInterface<Control>.FromAgileAbi(nint self) => new Control(self, true);

        public Control() : base(FactoryCreate0(), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static nint FactoryCreate0()
        {
            using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_factory0, "Activation.Control", s_factory0_iid);
            nint self = lease.Handle;
            nint _instance;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint, nint*, int>)(*(void***)self)[6])(self, 0, 0, &_instance));
            return _instance;
        }

        public int Tag
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

            public int Tag
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

    public sealed unsafe class Hosted : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Hosted>, WindowsCsharp.IObjectParameter<Activation.Hosted._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x6ba27adf, 0xdd44, 0x59da, 0xaa, 0xd6, 0xab, 0x68, 0x5d, 0x31, 0x46, 0xde);

        internal Hosted(nint self) : base(self, Iid) {}
        internal Hosted(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Hosted WindowsCsharp.IComInterface<Hosted>.FromAbi(nint self) => new Hosted(self);
        static Hosted WindowsCsharp.IComInterface<Hosted>.FromAgileAbi(nint self) => new Hosted(self, true);

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

    public sealed unsafe class Widget : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Widget>, WindowsCsharp.IObjectParameter<Activation.Widget._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x5cc12445, 0xaacf, 0x51b2, 0x97, 0xc5, 0x7f, 0x79, 0x36, 0xc5, 0x03, 0x51);
        private static nint s_factory0;
        private static readonly Guid s_factory0_iid = new Guid(0xd5c218ec, 0xdcb1, 0x528f, 0x97, 0x0a, 0xaa, 0xb9, 0x82, 0x3e, 0xbd, 0xb9);
        private static nint s_static0;
        private static readonly Guid s_static0_iid = new Guid(0xe5554d95, 0x65fb, 0x5af3, 0x99, 0x87, 0x68, 0xe8, 0x77, 0x5e, 0xfa, 0xe2);

        internal Widget(nint self) : base(self, Iid) {}
        internal Widget(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Widget WindowsCsharp.IComInterface<Widget>.FromAbi(nint self) => new Widget(self);
        static Widget WindowsCsharp.IComInterface<Widget>.FromAgileAbi(nint self) => new Widget(self, true);

        public Widget() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Activation.Widget", Iid), Iid) {}

        public Widget(int seed) : base(FactoryCreate0(seed), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static nint FactoryCreate0(int seed)
        {
            using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_factory0, "Activation.Widget", s_factory0_iid);
            nint self = lease.Handle;
            nint _instance;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, nint*, int>)(*(void***)self)[6])(self, seed, &_instance));
            return _instance;
        }

        public Widget(Activation.Options options) : base(FactoryCreate1(options), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static nint FactoryCreate1(Activation.Options options)
        {
            using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_factory0, "Activation.Widget", s_factory0_iid);
            nint self = lease.Handle;
            nint _instance;
            Activation.OptionsAbi _abi0 = default;
            try
            {
                _abi0 = Activation.OptionsAbi.FromSurface(options);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Activation.OptionsAbi, nint*, int>)(*(void***)self)[7])(self, _abi0, &_instance));
            }
            finally
            {
                _abi0.Dispose();
            }
            return _instance;
        }

        public string Label
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
        public int Poke(int delta)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[8])(self, delta, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T As<T>() where T : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            return WindowsCsharp.Com.As<T>(self, lease.TrustedAgile);
        }

        public static int Count
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_static0, "Activation.Widget", s_static0_iid);
                nint self = lease.Handle;
                int value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_static0, "Activation.Widget", s_static0_iid);
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[7])(self, value));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static void Start(Activation.StartCallback? callback)
        {
            using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_static0, "Activation.Widget", s_static0_iid);
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[8])(self, _olease0.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static void Start<T0>(T0? callback) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<Activation.StartCallback._Parameter>
        {
            using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_static0, "Activation.Widget", s_static0_iid);
            nint self = lease.Handle;
            using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(callback, Activation.StartCallback.Iid);
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[8])(self, _olease0.Handle));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Activation.Options Normalize(Activation.Options options)
        {
            using WindowsCsharp.FactoryLease lease = WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_static0, "Activation.Widget", s_static0_iid);
            nint self = lease.Handle;
            Activation.OptionsAbi result = default;
            try
            {
                Activation.OptionsAbi _abi0 = default;
                try
                {
                    _abi0 = Activation.OptionsAbi.FromSurface(options);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Activation.OptionsAbi, Activation.OptionsAbi*, int>)(*(void***)self)[9])(self, _abi0, &result));
                }
                finally
                {
                    _abi0.Dispose();
                }
                return result.ToSurface();
            }
            finally
            {
                result.Dispose();
            }
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
            public int Poke(int delta)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[8])(self, delta, &result));
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
