use std::slice;

use crate::general::{c_string_to_option_string, c_structs::c_route_leg::COsrmRouteLeg};
use serde::{Deserialize, Serialize};
use super::{annotation::Annotation, step::Step};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteLeg {
    pub annotation: Option<Annotation>,
    pub duration: f64,
    pub summary: Option<String>,
    pub weight: f64,
    pub distance: f64,
    pub steps: Vec<Step>,
}

impl From<&COsrmRouteLeg> for RouteLeg {
    fn from(leg: &COsrmRouteLeg) -> Self {
        RouteLeg {
            duration: leg.duration,
            summary: c_string_to_option_string(leg.summary),
            weight: leg.weight,
            distance: leg.distance,
            annotation: if leg.annotation != std::ptr::null_mut() {
                let annotation: Annotation = unsafe { (&(*leg.annotation)).into() };

                annotation.into()
            } else {
                None
            },
            steps: if leg.steps != std::ptr::null_mut() {
                unsafe { slice::from_raw_parts(leg.steps, leg.number_of_steps as usize).to_vec() }
                    .iter()
                    .map(|step| step.into())
                    .collect()
            } else {
                Vec::new()
            },
        }
    }
}
