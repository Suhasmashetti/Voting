use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, poll_id: u64, poll_name: String, poll_description: String, start_time: u64, end_time: u64,  ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.poll_name = poll_name;
        poll.poll_id = poll_id;
        poll.poll_description = poll_description;
        poll.poll_voting_start = start_time;
        poll.poll_voting_end = end_time;
        poll.poll_option_index = 0;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>, candidate_name: String, _poll_id: u64) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account;
        candidate.candidate_name = candidate_name;
        ctx.accounts.poll_account.poll_option_index += 1; 

        Ok(())
    }
}

    pub fn vote(ctx: Context<InitializeVote>, _poll_id: u64, _candidate_name: String) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        let poll_account = &mut ctx.accounts.poll_account;
        let present_time = Clock::get()?.unix_timestamp;

        require!(
            present_time >= poll_account.poll_voting_start as i64,
            ErrorCode::VotingNotStarted
        );
        require!(
            present_time < poll_account.poll_voting_end as i64, 
            ErrorCode::VotingEnded
        );

        candidate_account.candidate_votes = candidate_account.candidate_votes.checked_add(1).ok_or(ErrorCode::VotingOverFlow)?;
        Ok(())
    }

#[derive(Accounts)]
#[instruction(poll_id:u64, candidate_name: String)]
pub struct InitializeVote<'info> {
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [b"candidate".as_ref(), poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes().as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [b"candidate".as_ref(), poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes().as_ref()],
        bump
    )]

    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(250)]
    pub poll_description: String,
    pub poll_id: u64,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(30)]
    pub candidate_name: String,
    pub candidate_votes: u64
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
    #[msg("Voting has reached MAX")]
    VotingOverFlow,
}