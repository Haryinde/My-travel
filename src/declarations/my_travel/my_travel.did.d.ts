import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'DecodeError' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : TravelPlan } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : number } |
  { 'Err' : Error };
export interface TravelPlan {
  'id' : bigint,
  'destination' : string,
  'transportation' : string,
  'activities' : Array<string>,
  'end_date' : bigint,
  'accommodation' : string,
  'start_date' : bigint,
}
export interface TravelPlanPayload {
  'destination' : string,
  'transportation' : string,
  'activities' : Array<string>,
  'end_date' : bigint,
  'accommodation' : string,
  'start_date' : bigint,
}
export interface _SERVICE {
  'add_travel_plan' : ActorMethod<[TravelPlanPayload], [] | [TravelPlan]>,
  'calculate_travel_plan_duration' : ActorMethod<[bigint], [] | [bigint]>,
  'count_travel_plans' : ActorMethod<[], bigint>,
  'delete_travel_plan' : ActorMethod<[bigint], Result>,
  'get_next_available_id' : ActorMethod<[], bigint>,
  'get_remaining_budget' : ActorMethod<[], number>,
  'get_travel_plan' : ActorMethod<[bigint], Result>,
  'record_expense' : ActorMethod<[number], Result_1>,
  'set_budget' : ActorMethod<[number], number>,
  'update_travel_plan' : ActorMethod<[bigint, TravelPlanPayload], Result>,
}
