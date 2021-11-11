#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn WSL_DISTRIBUTION_FLAGS();
    fn WslConfigureDistribution();
    fn WslGetDistributionConfiguration();
    fn WslIsDistributionRegistered();
    fn WslLaunch();
    fn WslLaunchInteractive();
    fn WslRegisterDistribution();
    fn WslUnregisterDistribution();
}
