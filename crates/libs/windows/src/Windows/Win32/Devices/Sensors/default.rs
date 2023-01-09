impl ::core::default::Default for ACTIVITY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVITY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVITY_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ACTIVITY_STATE_COUNT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVITY_STATE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVITY_STATE_COUNT").field(&self.0).finish()
    }
}
impl ::core::default::Default for AXIS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AXIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AXIS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ELEVATION_CHANGE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ELEVATION_CHANGE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ELEVATION_CHANGE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HUMAN_PRESENCE_DETECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HUMAN_PRESENCE_DETECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUMAN_PRESENCE_DETECTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HUMAN_PRESENCE_DETECTION_TYPE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUMAN_PRESENCE_DETECTION_TYPE_COUNT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILocationPermissions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILocationPermissions {}
impl ::core::fmt::Debug for ILocationPermissions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILocationPermissions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensor {}
impl ::core::fmt::Debug for ISensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISensorCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorCollection {}
impl ::core::fmt::Debug for ISensorCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISensorDataReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorDataReport {}
impl ::core::fmt::Debug for ISensorDataReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorDataReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISensorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorEvents {}
impl ::core::fmt::Debug for ISensorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISensorManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorManager {}
impl ::core::fmt::Debug for ISensorManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISensorManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorManagerEvents {}
impl ::core::fmt::Debug for ISensorManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorManagerEvents").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOCATION_DESIRED_ACCURACY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOCATION_DESIRED_ACCURACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCATION_DESIRED_ACCURACY").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOCATION_POSITION_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOCATION_POSITION_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCATION_POSITION_SOURCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MAGNETOMETER_ACCURACY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MAGNETOMETER_ACCURACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAGNETOMETER_ACCURACY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MATRIX3X3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MagnetometerAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MagnetometerAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerAccuracy").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEDOMETER_STEP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEDOMETER_STEP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEDOMETER_STEP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEDOMETER_STEP_TYPE_COUNT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEDOMETER_STEP_TYPE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEDOMETER_STEP_TYPE_COUNT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROXIMITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROXIMITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXIMITY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for QUATERNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QUATERNION {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.W == other.W
    }
}
impl ::core::cmp::Eq for QUATERNION {}
impl ::core::fmt::Debug for QUATERNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUATERNION").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("W", &self.W).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for SENSOR_COLLECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SENSOR_CONNECTION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SENSOR_CONNECTION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SENSOR_CONNECTION_TYPES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for SENSOR_PROPERTY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for SENSOR_PROPERTY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AllocatedSizeInBytes == other.AllocatedSizeInBytes && self.Count == other.Count && self.List == other.List
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for SENSOR_PROPERTY_LIST {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for SENSOR_PROPERTY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SENSOR_PROPERTY_LIST").field("AllocatedSizeInBytes", &self.AllocatedSizeInBytes).field("Count", &self.Count).field("List", &self.List).finish()
    }
}
impl ::core::default::Default for SENSOR_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SENSOR_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SENSOR_STATE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for SENSOR_VALUE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIMPLE_DEVICE_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIMPLE_DEVICE_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIMPLE_DEVICE_ORIENTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SensorConnectionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SensorConnectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorConnectionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SensorState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SensorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorState").field(&self.0).finish()
    }
}
impl ::core::default::Default for SimpleDeviceOrientation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SimpleDeviceOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleDeviceOrientation").field(&self.0).finish()
    }
}
impl ::core::default::Default for VEC3D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VEC3D {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::core::cmp::Eq for VEC3D {}
impl ::core::fmt::Debug for VEC3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VEC3D").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
