#pragma once
typedef unsigned Count;
typedef unsigned OtherCount;
#ifdef OTHER_NAME
typedef OtherCount (*Reader)(OtherCount value);
struct PublicNames { OtherCount value; };
#else
typedef Count (*Reader)(Count value);
struct PublicNames { Count value; };
#endif
