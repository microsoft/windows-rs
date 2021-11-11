#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GameService();
    fn GameServiceGameOutcome();
    fn GameServicePropertyCollection();
    fn GameServiceScoreKind();
    fn IGameService();
    fn IGameService2();
    fn IGameServicePropertyCollection();
}
