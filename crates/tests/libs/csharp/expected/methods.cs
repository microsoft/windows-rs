namespace Sample
{
    public sealed unsafe class Calculator : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Calculator>, WindowsCsharp.IObjectParameter<Sample.Calculator._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x5aebab85, 0xac27, 0x5066, 0x8e, 0xa8, 0x2c, 0x19, 0xdc, 0x0d, 0x50, 0x79);

        internal Calculator(nint self) : base(self, Iid) {}
        internal Calculator(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Calculator WindowsCsharp.IComInterface<Calculator>.FromAbi(nint self) => new Calculator(self);
        static Calculator WindowsCsharp.IComInterface<Calculator>.FromAgileAbi(nint self) => new Calculator(self, true);

        public Calculator() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Calculator", Iid), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Reset()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[6])(self));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Value()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[7])(self, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Add(int a, int b)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[8])(self, a, b, &result));
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
            public void Reset()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int>)(*(void***)self)[6])(self));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public int Value()
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
            public int Add(int a, int b)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                int result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[8])(self, a, b, &result));
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
