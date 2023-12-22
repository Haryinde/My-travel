#[macro_use]
extern crate serde;

use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

/// Struct representing a travel plan.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TravelPlan {
    id: u64,
    destination: String,
    start_date: u64,
    end_date: u64,
    transportation: String,
    accommodation: String,

    activities: Vec<String>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct Budget {
    total_budget: f64,
    remaining_budget: f64,
}


// Implement Storable and BoundedStorable for TravelPlan and other Structs
impl Storable for Budget {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        match Decode!(bytes.as_ref(), Self) {
            Ok(decoded) => decoded,
            Err(err) => panic!("Error decoding Budget: {:?}", err),
        }
    }
}

impl BoundedStorable for Budget {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for TravelPlan {
    /// Convert the travel plan to bytes for storage.
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    /// Convert bytes to a travel plan for retrieval.
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        match Decode!(bytes.as_ref(), Self) {
            Ok(decoded) => decoded,
            Err(err) => {
                // Handle deserialization error
                panic!("Error decoding TravelPlan: {:?}", err);
            }
        }
    }
}

impl BoundedStorable for TravelPlan {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static TRAVEL_PLANS: RefCell<StableBTreeMap<u64, TravelPlan, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static BUDGET: RefCell<Budget> = RefCell::new(
        Budget {
            total_budget: 0.0, // Set an initial value
            remaining_budget: 0.0, // Set an initial value
        }
    );
}

/// Struct representing payload for creating a new travel plan.
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TravelPlanPayload {
    destination: String,
    start_date: u64,
    end_date: u64,
    transportation: String,
    accommodation: String,
    activities: Vec<String>,
}

/// Enum representing possible errors in the travel plan operations.
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    DecodeError { msg: String },
}

/// Retrieve a travel plan by ID.
#[ic_cdk::query]
fn get_travel_plan(id: u64) -> Result<TravelPlan, Error> {
    match _get_travel_plan(&id) {
        Some(plan) => Ok(plan),
        None => Err(Error::NotFound {
            msg: format!("Travel plan with id={} not found", id),
        }),
    }
}

/// Add a new travel plan.
#[ic_cdk::update]
fn add_travel_plan(plan: TravelPlanPayload) -> Option<TravelPlan> {
    // Validate that start_date is before end_date
    if plan.start_date >= plan.end_date {
        return None;
    }

    // Validate that essential string fields are not empty
    if plan.destination.is_empty()
        || plan.transportation.is_empty()
        || plan.accommodation.is_empty()
    {
        return None;
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    // Use time to resolve the warning
    let _current_time = time();

    let travel_plan = TravelPlan {
        id,
        destination: plan.destination,
        start_date: plan.start_date,
        end_date: plan.end_date,
        transportation: plan.transportation,
        accommodation: plan.accommodation,
        activities: plan.activities,
    };

    do_insert_travel_plan(&travel_plan);
    Some(travel_plan)
}

/// Update an existing travel plan.
#[ic_cdk::update]
fn update_travel_plan(id: u64, payload: TravelPlanPayload) -> Result<TravelPlan, Error> {
    match TRAVEL_PLANS.with(|service| service.borrow().get(&id)) {
        Some(mut plan) => {
            plan.destination = payload.destination;
            plan.start_date = payload.start_date;
            plan.end_date = payload.end_date;
            plan.transportation = payload.transportation;
            plan.accommodation = payload.accommodation;
            plan.activities = payload.activities;
            do_insert_travel_plan(&plan);
            Ok(plan)
        }
        None => Err(Error::NotFound {
            msg: format!("Travel plan with id={}. plan not found", id),
        }),
    }
}

/// Insert a travel plan into the storage.
fn do_insert_travel_plan(plan: &TravelPlan) {
    TRAVEL_PLANS.with(|service| service.borrow_mut().insert(plan.id, plan.clone()));
}

/// Delete a travel plan by ID.
#[ic_cdk::update]
fn delete_travel_plan(id: u64) -> Result<TravelPlan, Error> {
    match TRAVEL_PLANS.with(|service| service.borrow_mut().remove(&id)) {
        Some(plan) => Ok(plan),
        None => Err(Error::NotFound {
            msg: format!("Travel plan with id={}. plan not found.", id),
        }),
    }
}

/// Retrieve a travel plan by ID (internal function).
fn _get_travel_plan(id: &u64) -> Option<TravelPlan> {
    TRAVEL_PLANS.with(|service| service.borrow().get(id))
}


/// Get the next available ID for a new travel plan.
#[ic_cdk::query]
fn get_next_available_id() -> u64 {
    ID_COUNTER.with(|counter| *counter.borrow().get() + 1)
}


/// Query to get the total number of travel plans.
#[ic_cdk::query]
fn count_travel_plans() -> u64 {
    TRAVEL_PLANS.with(|service| service.borrow().len() as u64)
}

/// Calculate the total duration of a travel plan.
#[ic_cdk::query]
fn calculate_travel_plan_duration(id: u64) -> Option<u64> {
    match TRAVEL_PLANS.with(|service| service.borrow().get(&id)) {
        Some(plan) => Some(plan.end_date - plan.start_date),
        None => None,
    }
}

/// Set the budget for the trip.
#[ic_cdk::update]
fn set_budget(total_budget: f64) -> f64 {
    BUDGET.with(|budget| {
        let mut budget = budget.borrow_mut();
        budget.total_budget = total_budget;
        budget.remaining_budget = total_budget;
        budget.remaining_budget
    })
}

/// Get the remaining budget.
#[ic_cdk::query]
fn get_remaining_budget() -> f64 {
    BUDGET.with(|budget| budget.borrow().remaining_budget)
}

/// Record an expense and update the remaining budget.
#[ic_cdk::update]
fn record_expense(expense_amount: f64) -> Result<f64, Error> {
    if expense_amount < 0.0 {
        return Err(Error::DecodeError {
            msg: "Expense amount must be non-negative".to_string(),
        });
    }

    BUDGET.with(|budget| {
        let mut budget = budget.borrow_mut();
        if expense_amount > budget.remaining_budget {
            return Err(Error::DecodeError {
                msg: "Expense exceeds remaining budget".to_string(),
            });
        }

        budget.remaining_budget -= expense_amount;
        Ok(budget.remaining_budget)
    })
}

// Export Candid for the Travel Itinerary Planner
ic_cdk::export_candid!();