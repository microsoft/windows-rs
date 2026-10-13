struct Broken { unsigned first : 1; };
struct OtherBroken { unsigned second : 1; };
struct First { Broken* value; OtherBroken* other; };
struct Second { OtherBroken* value; Broken* other; };
struct Both { First* first; Second* second; };
