table! {
    proposals (id) {
        id -> Integer,
        proposal_category -> Text,
        proposal_id -> Text,
        proposal_title -> Text,
        proposal_summary -> Text,
        proposal_problem -> Text,
        proposal_solution -> Text,
        proposal_funds -> Integer,
        proposal_url -> Text,
        proposal_files_url -> Text,
        proposer_name -> Text,
        proposer_contact -> Text,
        proposer_url -> Text,
        chain_proposal_id -> Text,
        chain_voteplan_id -> Text,
        chain_proposal_index -> Integer,
        chain_vote_start_time -> Integer,
        chain_vote_end_time -> Integer,
        chain_committee_end_time -> Integer,
        chain_vote_options -> Text,
    }
}
