use crate::models::buses::bus_timings::arrival_bus_service::NextBus;
use crate::models::buses::bus_timings::ArrivalBusService;
use crate::models::buses::BusTimings;
use lta::models::bus::bus_arrival;
use lta::models::bus_enums;

pub mod buses {
    tonic::include_proto!("buses");
}

impl From<Vec<bus_arrival::ArrivalBusService>> for BusTimings {
    fn from(arrivals: Vec<bus_arrival::ArrivalBusService>) -> Self {
        BusTimings {
            arrivals: arrivals.into_iter().map(|v| From::from(v)).collect(),
        }
    }
}

impl From<bus_arrival::ArrivalBusService> for ArrivalBusService {
    fn from(arrivals: bus_arrival::ArrivalBusService) -> Self {
        ArrivalBusService {
            service_no: arrivals.service_no,
            operator: from_bus_operator(&arrivals.operator),
            next_bus: arrivals
                .next_bus
                .to_vec()
                .into_iter()
                .filter_map(|v| v.map(|v2| NextBus::from(v2)))
                .collect(),
        }
    }
}

impl From<bus_arrival::NextBus> for NextBus {
    fn from(nb: bus_arrival::NextBus) -> Self {
        NextBus {
            origin_code: nb.origin_code,
            dest_code: nb.dest_code,
            est_arrival: nb.est_arrival.timestamp(),
            lat: nb.lat,
            long: nb.long,
            visit_no: nb.visit_no,
            bus_load: from_bus_load(&nb.load),
            feature: from_bus_feature(&nb.feature),
            bus_type: from_bus_type(&nb.bus_type),
        }
    }
}

fn from_bus_feature(bf: &Option<bus_enums::BusFeature>) -> Option<i32> {
    match bf {
        Some(_) => Some(1),
        _ => None,
    }
}

fn from_bus_load(bl: &bus_enums::BusLoad) -> i32 {
    match bl {
        bus_enums::BusLoad::SeatsAvailable => 0,
        bus_enums::BusLoad::StandingAvailable => 1,
        bus_enums::BusLoad::LimitedStanding => 2,
    }
}

fn from_bus_type(bt: &bus_enums::BusType) -> i32 {
    match bt {
        bus_enums::BusType::SingleDecker => 0,
        bus_enums::BusType::DoubleDecker => 1,
        bus_enums::BusType::Bendy => 2,
    }
}

fn from_bus_operator(bo: &bus_enums::Operator) -> i32 {
    match bo {
        bus_enums::Operator::SBST => 0,
        bus_enums::Operator::SMRT => 1,
        bus_enums::Operator::TTS => 2,
        bus_enums::Operator::GAS => 3,
    }
}
