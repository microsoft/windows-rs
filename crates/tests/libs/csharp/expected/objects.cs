namespace Sample
{
    public sealed unsafe class Node : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Node>, WindowsCsharp.IObjectParameter<Sample.Node._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x774dd91e, 0xc091, 0x5433, 0xbe, 0x3e, 0xbd, 0x61, 0xf9, 0x5f, 0xa9, 0x1c);

        internal Node(nint self) : base(self, Iid) {}
        internal Node(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Node WindowsCsharp.IComInterface<Node>.FromAbi(nint self) => new Node(self);
        static Node WindowsCsharp.IComInterface<Node>.FromAgileAbi(nint self) => new Node(self, true);

        public Node() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Node", Iid), Iid) {}

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

        public Sample.IPeer? Peer
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                nint value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &value));
                return WindowsCsharp.Com.Wrap<Sample.IPeer>(value);
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                using WindowsCsharp.ComLease valueLease = WindowsCsharp.ComLease.From(value);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, valueLease.Handle));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.IPeer? Link(Sample.IPeer? other)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(other);
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[10])(self, _olease0.Handle, &result));
            return WindowsCsharp.Com.Wrap<Sample.IPeer>(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.IPeer? Link<T0>(T0? other) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<Sample.IPeer._Parameter>
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(other, Sample.IPeer.Iid);
            nint result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[10])(self, _olease0.Handle, &result));
            return WindowsCsharp.Com.Wrap<Sample.IPeer>(result);
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

            public Sample.IPeer? Peer
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint*, int>)(*(void***)self)[8])(self, &value));
                    return WindowsCsharp.Com.Wrap<Sample.IPeer>(value);
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, int>)(*(void***)self)[9])(self, valueLease.Handle));
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.IPeer? Link(Sample.IPeer? other)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(other);
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[10])(self, _olease0.Handle, &result));
                return WindowsCsharp.Com.Wrap<Sample.IPeer>(result);
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.IPeer? Link<T0>(T0? other) where T0 : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<T0>, WindowsCsharp.IObjectParameter<Sample.IPeer._Parameter>
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                using WindowsCsharp.InterfaceLease _olease0 = WindowsCsharp.InterfaceLease.From(other, Sample.IPeer.Iid);
                nint result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[10])(self, _olease0.Handle, &result));
                return WindowsCsharp.Com.Wrap<Sample.IPeer>(result);
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

    public sealed unsafe class IPeer : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<IPeer>, WindowsCsharp.IObjectParameter<Sample.IPeer._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x9a75f28c, 0x609e, 0x54ee, 0x8c, 0x76, 0xf0, 0x71, 0x9f, 0xc3, 0x6b, 0xc4);

        internal IPeer(nint self) : base(self, Iid) {}
        internal IPeer(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static IPeer WindowsCsharp.IComInterface<IPeer>.FromAbi(nint self) => new IPeer(self);
        static IPeer WindowsCsharp.IComInterface<IPeer>.FromAgileAbi(nint self) => new IPeer(self, true);

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
