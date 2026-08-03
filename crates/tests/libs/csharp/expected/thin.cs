namespace Thin
{
    public sealed unsafe class Widget : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Widget>, WindowsCsharp.IObjectParameter<Thin.Widget._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x45a609ce, 0x810f, 0x537e, 0xb2, 0xa3, 0xcd, 0xd1, 0xbf, 0xd2, 0x30, 0x0e);

        internal Widget(nint self) : base(self, Iid) {}
        internal Widget(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Widget WindowsCsharp.IComInterface<Widget>.FromAbi(nint self) => new Widget(self);
        static Widget WindowsCsharp.IComInterface<Widget>.FromAgileAbi(nint self) => new Widget(self, true);

        public Widget() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Thin.Widget", Iid), Iid) {}

        public int Int32Property
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

        public string StringProperty
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint hstring;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &hstring));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, hstring));
                }
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Add(int a, int b)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            int result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[10])(self, a, b, &result));
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

            public int Int32Property
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

            public string StringProperty
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &hstring));
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
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, hstring));
                    }
                }
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
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[10])(self, a, b, &result));
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
