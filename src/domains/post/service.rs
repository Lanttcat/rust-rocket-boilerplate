use chrono::Utc;
use uuid::Uuid;

use crate::domains::post::domain::Post;
use crate::domains::post::dto::{NewPostRequest, PostVoteRequest, VoteType};
use crate::domains::post::repository as post_repo;

pub fn create_new_post(req: NewPostRequest, member_id: Uuid) {
    let new_post = Post {
        id: Uuid::new_v4(),
        title: req.title.to_string(),
        description: req.description,
        votes_received: vec![],
        anti_votes_received: vec![],
        tags: req.tags,
        created_by: member_id,
        created_at: Utc::now().naive_utc(),
    };

    post_repo::insert(new_post)
}

pub fn post_vote(id: Uuid, req: PostVoteRequest, member_id: Uuid) {
    let post = post_repo::get_by_id(id);
    let is_exist_in_support = post.votes_received.contains(&member_id);
    let is_exist_in_anti = post.anti_votes_received.contains(&member_id);
    let mut result_support: Vec<Uuid> = post.votes_received;
    let mut result_anti: Vec<Uuid> = post.anti_votes_received;

    match req.vote_type {
        VoteType::Up => {
            if is_exist_in_support == false {
                if is_exist_in_anti {
                    result_anti = result_anti
                        .into_iter()
                        .filter(|&x| x.ne(&member_id))
                        .collect();
                }
                result_support.push(member_id);
            }
        }
        VoteType::Down => {
            if is_exist_in_anti == false {
                if is_exist_in_support {
                    result_support = result_support
                        .into_iter()
                        .filter(|&x| x.ne(&member_id))
                        .collect();
                }
                result_anti.push(member_id);
            }
        }
    }

    post_repo::update_post_vote(id, &result_support, &result_anti)
}
