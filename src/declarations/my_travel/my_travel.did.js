export const idlFactory = ({ IDL }) => {
  const TravelPlanPayload = IDL.Record({
    'destination' : IDL.Text,
    'transportation' : IDL.Text,
    'activities' : IDL.Vec(IDL.Text),
    'end_date' : IDL.Nat64,
    'accommodation' : IDL.Text,
    'start_date' : IDL.Nat64,
  });
  const TravelPlan = IDL.Record({
    'id' : IDL.Nat64,
    'destination' : IDL.Text,
    'transportation' : IDL.Text,
    'activities' : IDL.Vec(IDL.Text),
    'end_date' : IDL.Nat64,
    'accommodation' : IDL.Text,
    'start_date' : IDL.Nat64,
  });
  const Error = IDL.Variant({
    'DecodeError' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : TravelPlan, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : Error });
  return IDL.Service({
    'add_travel_plan' : IDL.Func(
        [TravelPlanPayload],
        [IDL.Opt(TravelPlan)],
        [],
      ),
    'calculate_travel_plan_duration' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(IDL.Nat64)],
        ['query'],
      ),
    'count_travel_plans' : IDL.Func([], [IDL.Nat64], ['query']),
    'delete_travel_plan' : IDL.Func([IDL.Nat64], [Result], []),
    'get_next_available_id' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_remaining_budget' : IDL.Func([], [IDL.Float64], ['query']),
    'get_travel_plan' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'record_expense' : IDL.Func([IDL.Float64], [Result_1], []),
    'set_budget' : IDL.Func([IDL.Float64], [IDL.Float64], []),
    'update_travel_plan' : IDL.Func(
        [IDL.Nat64, TravelPlanPayload],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
