#![allow(clippy::needless_update)]

use opg::*;

use crate::api::responses;
use crate::models;

pub fn swagger() -> String {
    let api = describe_api! {
        info: {
            title: "Farming",
            version: "1.0.0",
            description: r##"This API allows you to get the information of bridge"##,
        },
        servers: {
            "https://bridge-dao-indexer-test.broxus.com/v1",
            "https://bridge-dao-indexer.broxus.com/v1"
        },
        tags: {
            voters,
            proposals
        },
        paths: {
            ("proposals" / "search" ): {
                POST: {
                    tags: { proposals },
                    summary: "Proposals search",
                    description: "Get proposals data.",
                    body: models::SearchProposalsRequest,
                    200: responses::ProposalsResponse,
                }
            },
            ("proposals" / String / "votes"): {
                POST: {
                    tags: { proposals },
                    summary: "Proposal votes search",
                    description: "Get proposal votes data.",
                    body: models::SearchVotesRequest,
                    200: responses::VotesResponse,
                }
            },
            ("voters" / "votes" ): {
                POST: {
                    tags: { voters },
                    summary: "All votes search",
                    description: "Get votes data.",
                    body: models::SearchVotesRequest,
                    200: responses::VotesResponse,
                }
            },
            ("voters" / String /"votes" ): {
                POST: {
                    tags: { voters },
                    summary: "Voter votes search",
                    description: "Get voter votes data.",
                    body: models::SearchVotesRequest,
                    200: responses::VotesResponse,
                }
            },
            ("voters" / String /"proposals" ): {
                POST: {
                    tags: { voters, proposals },
                    summary: "Voter proposals search",
                    description: "Get voter proposals data.",
                    body: models::SearchProposalsRequest,
                    200: responses::ProposalsVotesResponse,
                }
            },
        }
    };

    serde_yaml::to_string(&api).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_docs() {
        println!("{}", swagger());
    }
}
