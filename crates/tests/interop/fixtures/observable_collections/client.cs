using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using ObservableCollections;
using Windows.Foundation.Collections;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        ChangeNumbers(widget.Numbers());
        ChangeNames(widget.Names());
        widget.ChangeNumbers(new ObservableVector());
        widget.ChangeNames(new ObservableMap());
    }

    static void ChangeNumbers(IObservableVector<int> numbers)
    {
        Check(numbers.Count == 0);
        var changes = new List<(bool, CollectionChange, uint)>();

        VectorChangedEventHandler<int> handler = (sender, args) =>
        {
            var change = args.CollectionChange;
            changes.Add((ReferenceEquals(sender, numbers), change, change == CollectionChange.Reset ? 0 : args.Index));
        };

        numbers.VectorChanged += handler;
        numbers.Add(1);
        numbers.Add(3);
        numbers.Insert(1, 2);
        numbers[0] = 10;
        numbers.RemoveAt(1);
        numbers.RemoveAt(numbers.Count - 1);
        numbers.Clear();
        numbers.VectorChanged -= handler;
        numbers.Add(4);

        Check(changes.SequenceEqual([
            (true, CollectionChange.ItemInserted, 0u),
            (true, CollectionChange.ItemInserted, 1u),
            (true, CollectionChange.ItemInserted, 1u),
            (true, CollectionChange.ItemChanged, 0u),
            (true, CollectionChange.ItemRemoved, 1u),
            (true, CollectionChange.ItemRemoved, 1u),
            (true, CollectionChange.Reset, 0u),
        ]));

        Check(numbers.SequenceEqual([4]));
    }

    static void ChangeNames(IObservableMap<string, int> names)
    {
        Check(names.Count == 0);
        var changes = new List<(bool, CollectionChange, string)>();

        MapChangedEventHandler<string, int> handler = (sender, args) =>
        {
            var change = args.CollectionChange;
            changes.Add((ReferenceEquals(sender, names), change, change == CollectionChange.Reset ? "" : args.Key));
        };

        names.MapChanged += handler;
        names.Add("one", 1);
        names.Add("two", 2);
        names["one"] = 10;
        Check(names.Remove("two"));
        names.Clear();
        names.MapChanged -= handler;
        names.Add("three", 3);

        Check(changes.SequenceEqual([
            (true, CollectionChange.ItemInserted, "one"),
            (true, CollectionChange.ItemInserted, "two"),
            (true, CollectionChange.ItemChanged, "one"),
            (true, CollectionChange.ItemRemoved, "two"),
            (true, CollectionChange.Reset, ""),
        ]));

        Check(names.Count == 1);
        Check(names["three"] == 3);
    }
}

class VectorChangedEventArgs(CollectionChange change, uint index) : IVectorChangedEventArgs
{
    public CollectionChange CollectionChange => change;
    public uint Index => index;
}

class ObservableVector : Collection<int>, IObservableVector<int>
{
    public event VectorChangedEventHandler<int>? VectorChanged;

    void Raise(CollectionChange change, int index) => VectorChanged?.Invoke(this, new VectorChangedEventArgs(change, (uint)index));

    protected override void InsertItem(int index, int item)
    {
        base.InsertItem(index, item);
        Raise(CollectionChange.ItemInserted, index);
    }

    protected override void SetItem(int index, int item)
    {
        base.SetItem(index, item);
        Raise(CollectionChange.ItemChanged, index);
    }

    protected override void RemoveItem(int index)
    {
        base.RemoveItem(index);
        Raise(CollectionChange.ItemRemoved, index);
    }

    protected override void ClearItems()
    {
        base.ClearItems();
        Raise(CollectionChange.Reset, 0);
    }
}

class MapChangedEventArgs(CollectionChange change, string key) : IMapChangedEventArgs<string>
{
    public CollectionChange CollectionChange => change;
    public string Key => key;
}

class ObservableMap : IObservableMap<string, int>
{
    readonly Dictionary<string, int> map = [];

    public event MapChangedEventHandler<string, int>? MapChanged;

    void Raise(CollectionChange change, string key) => MapChanged?.Invoke(this, new MapChangedEventArgs(change, key));

    public int this[string key]
    {
        get => map[key];
        set
        {
            var change = map.ContainsKey(key) ? CollectionChange.ItemChanged : CollectionChange.ItemInserted;
            map[key] = value;
            Raise(change, key);
        }
    }

    public void Add(string key, int value)
    {
        map.Add(key, value);
        Raise(CollectionChange.ItemInserted, key);
    }

    public bool Remove(string key)
    {
        if (!map.Remove(key))
        {
            return false;
        }

        Raise(CollectionChange.ItemRemoved, key);
        return true;
    }

    public void Clear()
    {
        map.Clear();
        Raise(CollectionChange.Reset, "");
    }

    public ICollection<string> Keys => map.Keys;
    public ICollection<int> Values => map.Values;
    public int Count => map.Count;
    public bool IsReadOnly => false;
    public bool ContainsKey(string key) => map.ContainsKey(key);
    public bool TryGetValue(string key, [MaybeNullWhen(false)] out int value) => map.TryGetValue(key, out value);
    public void Add(KeyValuePair<string, int> item) => Add(item.Key, item.Value);
    public bool Contains(KeyValuePair<string, int> item) => ((ICollection<KeyValuePair<string, int>>)map).Contains(item);
    public void CopyTo(KeyValuePair<string, int>[] array, int index) => ((ICollection<KeyValuePair<string, int>>)map).CopyTo(array, index);
    public bool Remove(KeyValuePair<string, int> item) => Contains(item) && Remove(item.Key);
    public IEnumerator<KeyValuePair<string, int>> GetEnumerator() => map.GetEnumerator();
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
}