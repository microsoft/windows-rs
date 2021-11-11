#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CredentialPromptType();
    fn IOnlineIdAuthenticator();
    fn IOnlineIdServiceTicket();
    fn IOnlineIdServiceTicketRequest();
    fn IOnlineIdServiceTicketRequestFactory();
    fn IOnlineIdSystemAuthenticatorForUser();
    fn IOnlineIdSystemAuthenticatorStatics();
    fn IOnlineIdSystemIdentity();
    fn IOnlineIdSystemTicketResult();
    fn IUserIdentity();
    fn OnlineIdAuthenticator();
    fn OnlineIdServiceTicket();
    fn OnlineIdServiceTicketRequest();
    fn OnlineIdSystemAuthenticator();
    fn OnlineIdSystemAuthenticatorForUser();
    fn OnlineIdSystemIdentity();
    fn OnlineIdSystemTicketResult();
    fn OnlineIdSystemTicketStatus();
    fn SignOutUserOperation();
    fn UserAuthenticationOperation();
    fn UserIdentity();
}
