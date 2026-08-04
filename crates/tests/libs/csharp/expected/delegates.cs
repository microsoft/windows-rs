namespace Sample
{
    public sealed unsafe class ChangedHandler : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<ChangedHandler>, WindowsCsharp.IObjectParameter<Sample.ChangedHandler._Parameter>
    {
        public readonly struct _Parameter {}
        public static Guid Iid { get; } = new Guid(0x46742a21, 0x688c, 0x5457, 0x82, 0xf3, 0xc8, 0xa9, 0xb7, 0x79, 0x64, 0x6e);

        internal ChangedHandler(nint self) : base(self, Iid) {}
        internal ChangedHandler(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static ChangedHandler WindowsCsharp.IComInterface<ChangedHandler>.FromAbi(nint self) => new ChangedHandler(self);
        static ChangedHandler WindowsCsharp.IComInterface<ChangedHandler>.FromAgileAbi(nint self) => new ChangedHandler(self, true);

        public delegate void Callback(int value);

        private static readonly Guid* s_iid = WindowsCsharp.Callback.PinIid(Iid);
        private static readonly nint* s_vtable = BuildVtable();

        private static nint* BuildVtable()
        {
            nint* vtable = (nint*)NativeMemory.Alloc(4, (nuint)sizeof(nint));
            vtable[0] = WindowsCsharp.Callback.QueryInterfacePtr;
            vtable[1] = WindowsCsharp.Callback.AddRefPtr;
            vtable[2] = WindowsCsharp.Callback.ReleasePtr;
            vtable[3] = (nint)(delegate* unmanaged<nint, int, int>)&NativeInvoke;
            return vtable;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static ChangedHandler Create(Callback handler) => WindowsCsharp.Com.WrapAgile<ChangedHandler>(WindowsCsharp.Callback.Alloc((nint)s_vtable, s_iid, handler))!;

        [UnmanagedCallersOnly]
        private static int NativeInvoke(nint self, int value)
        {
            try
            {
                Callback callback = (Callback)WindowsCsharp.Callback.Target(self);
                callback(value);
                return 0;
            }
            catch (Exception error)
            {
                return Marshal.GetHRForException(error);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Invoke(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[3])(self, value));
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

    public sealed unsafe class Source : WindowsCsharp.ComObject, WindowsCsharp.IComInterface<Source>, WindowsCsharp.IObjectParameter<Sample.Source._Parameter>
    {
        public readonly struct _Parameter {}
        private static nint s_module;
        private static nint s_factory;
        public static Guid Iid { get; } = new Guid(0x1d27547e, 0x4ce8, 0x5414, 0x8a, 0x7a, 0x4b, 0x10, 0x43, 0xb6, 0x42, 0xee);

        internal Source(nint self) : base(self, Iid) {}
        internal Source(nint self, bool trustedAgile) : base(self, trustedAgile) {}
        static Source WindowsCsharp.IComInterface<Source>.FromAbi(nint self) => new Source(self);
        static Source WindowsCsharp.IComInterface<Source>.FromAgileAbi(nint self) => new Source(self, true);

        public Source() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, "Sample.Source", Iid), Iid) {}

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Signal(int value)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[6])(self, value));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public long AddChanged(Sample.ChangedHandler? handler)
        {
            using WindowsCsharp.ComLease sourceLease = Acquire();
            using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
            nint self = sourceLease.Handle;
            long token;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
            return token;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void RemoveChanged(long token)
        {
            using WindowsCsharp.ComLease lease = Acquire();
            nint self = lease.Handle;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public WindowsCsharp.EventRevoker Changed(Sample.ChangedHandler? handler)
        {
            WindowsCsharp.EventRevoker revoker = new WindowsCsharp.EventRevoker();
            using WindowsCsharp.ComLease sourceLease = Acquire();
            using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
            nint self = sourceLease.Handle;
            long token;
            WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
            _ = WindowsCsharp.Com.AddRef(self);
            try
            {
                revoker.Attach(self, sourceLease.TrustedAgile, token, (delegate* unmanaged<nint, long, int>)(*(void***)self)[8]);
            }
            catch
            {
                _ = ((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token);
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

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public void Signal(int value)
            {
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, int, int>)(*(void***)self)[6])(self, value));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public long AddChanged(Sample.ChangedHandler? handler)
            {
                using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                long token;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
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
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token));
            }

            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            public WindowsCsharp.EventRevoker Changed(Sample.ChangedHandler? handler)
            {
                WindowsCsharp.EventRevoker revoker = new WindowsCsharp.EventRevoker();
                using WindowsCsharp.ComLease handlerLease = WindowsCsharp.ComLease.From(handler);
                nint self = _this;
                if (self == 0)
                {
                    throw new ObjectDisposedException("borrowed COM interface");
                }
                long token;
                WindowsCsharp.Com.Check(((delegate* unmanaged<nint, nint, long*, int>)(*(void***)self)[7])(self, handlerLease.Handle, &token));
                _ = WindowsCsharp.Com.AddRef(self);
                try
                {
                    revoker.Attach(self, false, token, (delegate* unmanaged<nint, long, int>)(*(void***)self)[8]);
                }
                catch
                {
                    _ = ((delegate* unmanaged<nint, long, int>)(*(void***)self)[8])(self, token);
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
