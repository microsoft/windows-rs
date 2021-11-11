#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EyesPose();
    fn HandJointKind();
    fn HandMeshObserver();
    fn HandMeshVertex();
    fn HandMeshVertexState();
    fn HandPose();
    fn HeadPose();
    fn IEyesPose();
    fn IEyesPoseStatics();
    fn IHandMeshObserver();
    fn IHandMeshVertexState();
    fn IHandPose();
    fn IHeadPose();
    fn JointPose();
    fn JointPoseAccuracy();
}
