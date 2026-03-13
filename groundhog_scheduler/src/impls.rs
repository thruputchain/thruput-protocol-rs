use crate::DbTip;
use crate::SchedulerGetTipOutcome;
use crate::SchedulerGetTipResult;
use crate::SchedulerNominatedBlock;
use crate::SchedulerNominationOutcome;
use crate::SchedulerNominationResult;

impl From<SchedulerNominationResult>
    for Result<SchedulerNominatedBlock, SchedulerNominationOutcome>
{
    fn from(
        value: SchedulerNominationResult,
    ) -> Result<SchedulerNominatedBlock, SchedulerNominationOutcome> {
        match value {
            SchedulerNominationResult::SchedulerNominationOK {
                nominated_block,
            } => Ok(nominated_block),
            SchedulerNominationResult::SchedulerNominationError => {
                Err(SchedulerNominationOutcome::SchedulerNominationError)
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
