use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use ic_cdk::export::Principal;
use std::cell::RefCell;

type CourseId = u64;

#[derive(Clone, CandidType, Deserialize)]
struct Course {
  id: CourseId,
  title: String,
  description: String,
  instructor: Principal,
}

thread_local! {
  static COURSES: RefCell<Vec<Course>> = RefCell::new(vec![]);
}

#[init]
fn init() {}

#[update]
fn add_course(title: String, description: String) -> CourseId {
  let instructor = ic_cdk::caller();
  COURSES.with(|c| {
    let mut list = c.borrow_mut();
    let id = list.len() as CourseId + 1;
    list.push(Course { id, title, description, instructor });
    id
  })
}

#[query]
fn get_courses() -> Vec<Course> {
  COURSES.with(|c| c.borrow().clone())
}
