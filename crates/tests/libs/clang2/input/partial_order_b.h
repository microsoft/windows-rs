struct OtherBroken { unsigned other : 1; };
struct Broken { unsigned another : 1; };
struct Second { OtherBroken* value; Broken* other; };
struct First { Broken* value; OtherBroken* other; };
struct Both { Second* second; First* first; };
