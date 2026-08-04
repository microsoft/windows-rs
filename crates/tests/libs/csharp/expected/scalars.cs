namespace Sample
{
    public sealed unsafe class Numbers : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Numbers>, WindowsCsharp.IObjectParameter<Sample.Numbers._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x4aac3b7c, 0x3a1a, 0x51df, 0x8f, 0x42, 0xfc, 0x77, 0xc5, 0x5c, 0xda, 0x36);

        internal Numbers(nint self) : base(self, Iid) {}
        internal Numbers(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Numbers WindowsCsharp.IComInterface<Numbers>.FromAbi(nint self) => new Numbers(self);
        static Numbers WindowsCsharp.IComInterface<Numbers>.FromAgileAbi(nint self) => new Numbers(self, true);

        public Numbers() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Numbers", Iid), Iid) {}

        public sbyte Int8
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                sbyte value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, sbyte*, int>)(*(void***)self)[6])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, sbyte, int>)(*(void***)self)[7])(self, value));
            }
        }

        public byte UInt8
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                byte value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[8])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte, int>)(*(void***)self)[9])(self, value));
            }
        }

        public short Int16
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                short value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, short*, int>)(*(void***)self)[10])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, short, int>)(*(void***)self)[11])(self, value));
            }
        }

        public ushort UInt16
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                ushort value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ushort*, int>)(*(void***)self)[12])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ushort, int>)(*(void***)self)[13])(self, value));
            }
        }

        public int Int32
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                int value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[14])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[15])(self, value));
            }
        }

        public uint UInt32
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                uint value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[16])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int>)(*(void***)self)[17])(self, value));
            }
        }

        public long Int64
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                long value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long*, int>)(*(void***)self)[18])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[19])(self, value));
            }
        }

        public ulong UInt64
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                ulong value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ulong*, int>)(*(void***)self)[20])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ulong, int>)(*(void***)self)[21])(self, value));
            }
        }

        public float Single
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                float value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, float*, int>)(*(void***)self)[22])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, float, int>)(*(void***)self)[23])(self, value));
            }
        }

        public double Double
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                double value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, double*, int>)(*(void***)self)[24])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, double, int>)(*(void***)self)[25])(self, value));
            }
        }

        public bool Boolean
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                byte value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[26])(self, &value));
                return value != 0;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte, int>)(*(void***)self)[27])(self, (value ? (byte)1 : (byte)0)));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public long Sum(int a, long b)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            long result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, long, long*, int>)(*(void***)self)[28])(self, a, b, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool Not(bool value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            byte result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte, byte*, int>)(*(void***)self)[29])(self, (value ? (byte)1 : (byte)0), &result));
            return result != 0;
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

            public sbyte Int8
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    sbyte value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, sbyte*, int>)(*(void***)self)[6])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, sbyte, int>)(*(void***)self)[7])(self, value));
                }
            }

            public byte UInt8
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    byte value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[8])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte, int>)(*(void***)self)[9])(self, value));
                }
            }

            public short Int16
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    short value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, short*, int>)(*(void***)self)[10])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, short, int>)(*(void***)self)[11])(self, value));
                }
            }

            public ushort UInt16
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    ushort value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ushort*, int>)(*(void***)self)[12])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ushort, int>)(*(void***)self)[13])(self, value));
                }
            }

            public int Int32
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int*, int>)(*(void***)self)[14])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[15])(self, value));
                }
            }

            public uint UInt32
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    uint value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint*, int>)(*(void***)self)[16])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, uint, int>)(*(void***)self)[17])(self, value));
                }
            }

            public long Int64
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    long value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long*, int>)(*(void***)self)[18])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[19])(self, value));
                }
            }

            public ulong UInt64
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    ulong value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ulong*, int>)(*(void***)self)[20])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, ulong, int>)(*(void***)self)[21])(self, value));
                }
            }

            public float Single
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    float value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, float*, int>)(*(void***)self)[22])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, float, int>)(*(void***)self)[23])(self, value));
                }
            }

            public double Double
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    double value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, double*, int>)(*(void***)self)[24])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, double, int>)(*(void***)self)[25])(self, value));
                }
            }

            public bool Boolean
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    byte value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte*, int>)(*(void***)self)[26])(self, &value));
                    return value != 0;
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte, int>)(*(void***)self)[27])(self, (value ? (byte)1 : (byte)0)));
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public long Sum(int a, long b)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                long result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, long, long*, int>)(*(void***)self)[28])(self, a, b, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool Not(bool value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                byte result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, byte, byte*, int>)(*(void***)self)[29])(self, (value ? (byte)1 : (byte)0), &result));
                return result != 0;
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
