struct Broken { unsigned& first; };
struct OtherBroken { unsigned& second; };
struct First { Broken* value; OtherBroken* other; };
struct Second { OtherBroken* value; Broken* other; };
struct Both { First* first; Second* second; };
