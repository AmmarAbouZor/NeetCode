use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

// Store each user's followees and only their latest 10 tweets.
//
// get_news_feed scans the current user's latest tweets and the latest tweets of
// each followee, keeping only the 10 newest tweets in a min-heap. Since each
// user stores at most 10 tweets, scanning one followee is O(10) = O(1).
//
// Time:
// * post_tweet: O(1)
// * follow/unfollow: O(1) average
// * get_news_feed: O(f * 10 * log(10)) = O(f), where f is the number of followees
//
// Space:
// * O(u + e + 10u) = O(u + e), where u is users and e is follow relationships
// * get_news_feed uses O(10) extra heap/result space

#[derive(Default)]
pub struct Twitter {
    users: HashMap<i32, UserData>,
    counter: usize,
}

struct UserData {
    followees: HashSet<i32>,
    tweets: VecDeque<(usize, i32)>,
}

impl Default for UserData {
    fn default() -> Self {
        let followees = HashSet::new();
        let tweets = VecDeque::with_capacity(10);

        Self { followees, tweets }
    }
}

impl Twitter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let data = self.users.entry(user_id).or_default();
        if data.tweets.len() == 10 {
            data.tweets.pop_front();
        }

        data.tweets.push_back((self.counter, tweet_id));

        self.counter += 1;
    }

    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(11);

        let Some(data) = self.users.get(&user_id) else {
            return Vec::new();
        };

        let iter_tweets = data
            .followees
            .iter()
            .filter_map(|id| self.users.get(id))
            .flat_map(|d| d.tweets.iter())
            .chain(data.tweets.iter());

        for &(timestamp, tweet) in iter_tweets {
            heap.push(Reverse((timestamp, tweet)));
            if heap.len() > 10 {
                heap.pop();
            }
        }

        let mut res = Vec::with_capacity(heap.len());

        while let Some(Reverse((_time, tweet))) = heap.pop() {
            res.push(tweet)
        }

        res.reverse();
        res
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }

        let data = self.users.entry(follower_id).or_default();
        data.followees.insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(data) = self.users.get_mut(&follower_id) {
            data.followees.remove(&followee_id);
        }
    }
}
