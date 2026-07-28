use crate::DbTip;
use crate::SchedulerGetTipOutcome;
use crate::SchedulerGetTipResult;
use crate::SchedulerProposalOutcome;
use crate::SchedulerProposalResult;
use crate::SchedulerProposedBlock;

impl From<SchedulerProposalResult>
    for Result<SchedulerProposedBlock, SchedulerProposalOutcome>
{
    fn from(
        value: SchedulerProposalResult,
    ) -> Result<SchedulerProposedBlock, SchedulerProposalOutcome> {
        match value {
            SchedulerProposalResult::SchedulerProposalOK {
                proposed_block,
            } => Ok(proposed_block),
            SchedulerProposalResult::SchedulerProposalError => {
                Err(SchedulerProposalOutcome::SchedulerProposalError)
            }
        }
    }
}

impl From<SchedulerGetTipResult> for Result<DbTip, SchedulerGetTipOutcome> {
    fn from(
        value: SchedulerGetTipResult,
    ) -> Result<DbTip, SchedulerGetTipOutcome> {
        match value {
            SchedulerGetTipResult::SchedulerGetTipOK { db_tip } => Ok(db_tip),
            SchedulerGetTipResult::SchedulerGetTipNoGenesisBlock => {
                Err(SchedulerGetTipOutcome::SchedulerGetTipNoGenesisBlock)
            }
        }
    }
}
