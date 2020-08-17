use capra_core::common::dive_segment::DiveSegment;
use capra_core::common::gas::Gas;
use capra_core::deco::deco_algorithm::DecoAlgorithm;
use std::collections::HashMap;

/// A collection of results generated by a dive plan.
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct DiveResult<T: DecoAlgorithm> {
    /// Decompression model after the dive plan
    deco_algorithm: T,
    /// List of all segments that were performed
    total_segments: Vec<(DiveSegment, Gas)>,
    /// Summation of total quantity of gases used
    gas_used: HashMap<Gas, usize>,
}

impl<T: DecoAlgorithm> DiveResult<T> {
    /// Returns a new DiveResult with the given parameters.
    /// # Arguments
    /// * `deco_algorithm` - Decompression model after the dive plan
    /// * `total_segments` - List of all segments that were performed
    /// * `gas_used` - Summation of total quantity of gases used
    pub fn new(
        deco_algorithm: T,
        total_segments: Vec<(DiveSegment, Gas)>,
        gas_used: HashMap<Gas, usize>,
    ) -> Self {
        Self {
            deco_algorithm,
            total_segments,
            gas_used,
        }
    }

    /// Returns the list of all segments that were performed
    pub fn total_segments(&self) -> &Vec<(DiveSegment, Gas)> {
        &self.total_segments
    }

    /// Returns the summation of total quantity of gases used
    pub fn gas_used(&self) -> &HashMap<Gas, usize> {
        &self.gas_used
    }

    /// Returns the list of all segments that were performed
    pub fn deco_algorithm(&self) -> &T {
        &self.deco_algorithm
    }
}