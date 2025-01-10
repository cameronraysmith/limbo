use crate::{generation::plan::InteractionPlan, runner::execution::Execution};

impl InteractionPlan {
    /// Create a smaller interaction plan by deleting a property
    pub(crate) fn shrink_interaction_plan(&self, failing_execution: &Execution) -> InteractionPlan {
        let mut plan = self.clone();
        let failing_property = &self.plan[failing_execution.interaction_index];
        let depending_tables = failing_property.dependencies();

        let before = self.plan.len();

        // Remove all properties after the failing one
        plan.plan.truncate(failing_execution.interaction_index + 1);
        // Remove all properties that do not use the failing tables
        plan.plan
            .retain(|p| p.uses().iter().any(|t| depending_tables.contains(t)));

        let after = plan.plan.len();

        log::info!(
            "Shrinking interaction plan from {} to {} properties",
            before,
            after
        );

        plan
    }
}
