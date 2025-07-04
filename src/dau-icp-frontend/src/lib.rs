use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, update};
use ic_cdk::export::Principal;
use std::cell::RefCell;

type TokenId = u64;

#[derive(Clone, CandidType, Deserialize)]
struct Certificate {
  token_id: TokenId,
  course_id: u64,
  owner: Principal,
}

thread_local! {
  static CERTS: RefCell<Vec<Certificate>> = RefCell::new(vec![]);
}

#[init]
fn init() {}

#[update]
fn mint_cert(course_id: u64, to: Principal) -> TokenId {
  CERTS.with(|c| {
    let mut list = c.borrow_mut();
    let id = list.len() as TokenId + 1;
    list.push(Certificate { token_id: id, course_id, owner: to });
    id
  })
}
