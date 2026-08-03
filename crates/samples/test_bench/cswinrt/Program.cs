using Bench;

var widget = new Widget();
ChangedHandler handler = (sender, value) => { };

widget.Changed += handler;
widget.Signal(1);
widget.Changed -= handler;

Console.WriteLine("CsWinRT 3 delegate marshalling succeeded.");
