use std::marker::PhantomData;

use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, InfoEvaluator,
};

use super::{trace::eval::TraceEval, traits::MachineChip};

mod lookups;
pub use lookups::AllLookupElements;

pub(super) const LOG_CONSTRAINT_DEGREE: u32 = 2;

pub type MachineComponent<C> = FrameworkComponent<MachineEval<C>>;

pub struct MachineEval<C> {
    log_n_rows: u32,
    lookup_elements: AllLookupElements,
    _phantom_data: PhantomData<C>,
}

impl<C> MachineEval<C> {
    pub(crate) fn new(log_n_rows: u32, lookup_elements: AllLookupElements) -> Self {
        Self {
            log_n_rows,
            lookup_elements,
            _phantom_data: PhantomData,
        }
    }
}

impl<C: MachineChip> FrameworkEval for MachineEval<C> {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_rows + LOG_CONSTRAINT_DEGREE
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let trace_eval = TraceEval::new(&mut eval);
        C::add_constraints(&mut eval, &trace_eval, &self.lookup_elements);

        if !self.lookup_elements.is_empty() {
            eval.finalize_logup();
        }
        eval
    }
}

pub(crate) fn machine_component_info<C: MachineChip>() -> InfoEvaluator {
    let eval = MachineEval::<C> {
        log_n_rows: 1,
        lookup_elements: AllLookupElements::dummy(),
        _phantom_data: PhantomData,
    };
    eval.evaluate(InfoEvaluator::empty())
}
