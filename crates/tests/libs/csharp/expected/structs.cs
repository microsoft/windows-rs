namespace Sample
{
    [StructLayout(LayoutKind.Sequential)]
    public struct Description
    {
        public string Title;
        public Sample.Text Detail;
        public bool Visible;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct DescriptionAbi
    {
        public nint Title;
        public Sample.TextAbi Detail;
        public byte Visible;

        internal static DescriptionAbi FromSurface(Description value)
        {
            DescriptionAbi result = default;
            try
            {
                result.Title = WindowsCsharp.Interop.CreateString(value.Title);
                result.Detail = Sample.TextAbi.FromSurface(value.Detail);
                result.Visible = (value.Visible ? (byte)1 : (byte)0);
                return result;
            }
            catch
            {
                result.Dispose();
                throw;
            }
        }

        internal readonly Description FromAbi() => new()
        {
            Title = WindowsCsharp.Interop.FromHstringBorrowed(Title),
            Detail = Detail.FromAbi(),
            Visible = Visible != 0,
        };

        internal Description ToSurface()
        {
            Description result = default;
            try
            {
                result.Title = WindowsCsharp.Interop.TakeHstring(ref Title);
                result.Detail = Detail.ToSurface();
                result.Visible = Visible != 0;
                return result;
            }
            finally
            {
                Dispose();
            }
        }

        internal void Dispose()
        {
            Detail.Dispose();
            WindowsCsharp.Interop.DeleteHstring(ref Title);
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Point
    {
        public float X;
        public float Y;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Rect
    {
        public Sample.Point Origin;
        public float Width;
        public float Height;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Text
    {
        public string Value;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct TextAbi
    {
        public nint Value;

        internal static TextAbi FromSurface(Text value)
        {
            TextAbi result = default;
            try
            {
                result.Value = WindowsCsharp.Interop.CreateString(value.Value);
                return result;
            }
            catch
            {
                result.Dispose();
                throw;
            }
        }

        internal readonly Text FromAbi() => new()
        {
            Value = WindowsCsharp.Interop.FromHstringBorrowed(Value),
        };

        internal Text ToSurface()
        {
            Text result = default;
            try
            {
                result.Value = WindowsCsharp.Interop.TakeHstring(ref Value);
                return result;
            }
            finally
            {
                Dispose();
            }
        }

        internal void Dispose()
        {
            WindowsCsharp.Interop.DeleteHstring(ref Value);
        }
    }

    public sealed unsafe class Shape : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Shape>, WindowsCsharp.IObjectParameter<Sample.Shape._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x5f31e9dc, 0x10f0, 0x51d8, 0xbe, 0x3c, 0xac, 0x8a, 0xeb, 0x19, 0x30, 0x36);

        internal Shape(nint self) : base(self, Iid) {}
        internal Shape(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Shape WindowsCsharp.IComInterface<Shape>.FromAbi(nint self) => new Shape(self);
        static Shape WindowsCsharp.IComInterface<Shape>.FromAgileAbi(nint self) => new Shape(self, true);

        public Shape() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Shape", Iid), Iid) {}

        public Sample.Rect Bounds
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                Sample.Rect value;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.Rect*, int>)(*(void***)self)[6])(self, &value));
                return value;
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.Rect, int>)(*(void***)self)[7])(self, value));
            }
        }

        public Sample.Description Caption
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                Sample.DescriptionAbi value = default;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi*, int>)(*(void***)self)[8])(self, &value));
                    return value.ToSurface();
                }
                finally
                {
                    value.Dispose();
                }
            }
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set
            {
                using WindowsCsharp.ComLease lease = Acquire();
                nint self = lease.Handle;
                Sample.DescriptionAbi abi = Sample.DescriptionAbi.FromSurface(value);
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi, int>)(*(void***)self)[9])(self, abi));
                }
                finally
                {
                    abi.Dispose();
                }
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.Rect Move(Sample.Point delta)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Sample.Rect result;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.Point, Sample.Rect*, int>)(*(void***)self)[10])(self, delta, &result));
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool InspectDescription(Sample.Description value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            byte result;
            Sample.DescriptionAbi _abi0 = default;
            try
            {
                _abi0 = Sample.DescriptionAbi.FromSurface(value);
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi, byte*, int>)(*(void***)self)[11])(self, _abi0, &result));
            }
            finally
            {
                _abi0.Dispose();
            }
            return result != 0;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.Description CurrentDescription()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Sample.DescriptionAbi result = default;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi*, int>)(*(void***)self)[12])(self, &result));
                return result.ToSurface();
            }
            finally
            {
                result.Dispose();
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public Sample.Description FailingDescription()
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            Sample.DescriptionAbi result = default;
            try
            {
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi*, int>)(*(void***)self)[13])(self, &result));
                return result.ToSurface();
            }
            finally
            {
                result.Dispose();
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

            public Sample.Rect Bounds
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    Sample.Rect value;
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.Rect*, int>)(*(void***)self)[6])(self, &value));
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
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.Rect, int>)(*(void***)self)[7])(self, value));
                }
            }

            public Sample.Description Caption
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    Sample.DescriptionAbi value = default;
                    try
                    {
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi*, int>)(*(void***)self)[8])(self, &value));
                        return value.ToSurface();
                    }
                    finally
                    {
                        value.Dispose();
                    }
                }
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                set
                {
                    nint self = _this;
                    if (self == 0)
                    {
                        throw new ObjectDisposedException("borrowed COM interface");
                    }
                    Sample.DescriptionAbi abi = Sample.DescriptionAbi.FromSurface(value);
                    try
                    {
                        WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi, int>)(*(void***)self)[9])(self, abi));
                    }
                    finally
                    {
                        abi.Dispose();
                    }
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.Rect Move(Sample.Point delta)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                Sample.Rect result;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.Point, Sample.Rect*, int>)(*(void***)self)[10])(self, delta, &result));
                return result;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public bool InspectDescription(Sample.Description value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                byte result;
                Sample.DescriptionAbi _abi0 = default;
                try
                {
                    _abi0 = Sample.DescriptionAbi.FromSurface(value);
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi, byte*, int>)(*(void***)self)[11])(self, _abi0, &result));
                }
                finally
                {
                    _abi0.Dispose();
                }
                return result != 0;
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.Description CurrentDescription()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                Sample.DescriptionAbi result = default;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi*, int>)(*(void***)self)[12])(self, &result));
                    return result.ToSurface();
                }
                finally
                {
                    result.Dispose();
                }
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public Sample.Description FailingDescription()
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                Sample.DescriptionAbi result = default;
                try
                {
                    WindowsCsharp.Com.Check(((delegate* unmanaged<nint, Sample.DescriptionAbi*, int>)(*(void***)self)[13])(self, &result));
                    return result.ToSurface();
                }
                finally
                {
                    result.Dispose();
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
