namespace Sample
{
    public enum Color : int
    {
        Red = 0,
        Green = 1,
        Blue = 2,
    }

    public sealed unsafe class Widget : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Widget>, WindowsCsharp.IObjectParameter<Sample.Widget._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0xaa14fec8, 0x3dbe, 0x54bf, 0xa6, 0x1f, 0x7b, 0x84, 0x06, 0x20, 0xa8, 0xcb);

        internal Widget(nint self) : base(self, Iid) {}
        internal Widget(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Widget WindowsCsharp.IComInterface<Widget>.FromAbi(nint self) => new Widget(self);
        static Widget WindowsCsharp.IComInterface<Widget>.FromAgileAbi(nint self) => new Widget(self, true);

        public Widget() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Widget", Iid), Iid) {}

        public Sample.Color Kind
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                int value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[6])(self, &value));
                return (Sample.Color)value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[7])(self, (int)value));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.Color Blend(Sample.Color a, Sample.Color b)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[8])(self, (int)a, (int)b, &result));
            return (Sample.Color)result;
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

            public Sample.Color Kind
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
                    return (Sample.Color)value;
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[7])(self, (int)value));
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.Color Blend(Sample.Color a, Sample.Color b)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[8])(self, (int)a, (int)b, &result));
                return (Sample.Color)result;
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
