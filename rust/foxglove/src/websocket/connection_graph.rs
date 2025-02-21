use std::collections::{HashMap, HashSet};

use serde::Serialize;

/// A HashMap where the keys are the topic or service name and the value is a set of string ids.
pub type MapOfSets = HashMap<String, HashSet<String>>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewPublishedTopic<'a> {
    name: &'a str,
    publisher_ids: &'a HashSet<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewSubscribedTopic<'a> {
    name: &'a str,
    subscriber_ids: &'a HashSet<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewAdvertisedService<'a> {
    name: &'a str,
    provider_ids: &'a HashSet<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "op")]
#[serde(rename_all = "camelCase")]
struct ConnectionGraphDiff<'a> {
    published_topics: Vec<NewPublishedTopic<'a>>,
    subscribed_topics: Vec<NewSubscribedTopic<'a>>,
    advertised_services: Vec<NewAdvertisedService<'a>>,
    removed_topics: HashSet<String>,
    removed_services: Vec<String>,
}

impl ConnectionGraphDiff<'_> {
    fn new() -> Self {
        Self {
            published_topics: Vec::new(),
            subscribed_topics: Vec::new(),
            advertised_services: Vec::new(),
            removed_topics: HashSet::new(),
            removed_services: Vec::new(),
        }
    }

    fn to_json(&self) -> String {
        // This shouldn't fail, see serde docs
        serde_json::to_string(self).unwrap()
    }
}

/// The connection graph data. Requires capability [`Capability::ConnectionGraph`].
/// See https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#connection-graph-update
#[derive(Debug, Default)]
pub struct ConnectionGraph {
    /// A map of active topic names to the set of string publisher ids.
    pub published_topics: MapOfSets,
    /// A map of active topic names to the set of string subscriber ids.
    pub subscribed_topics: MapOfSets,
    /// A map of active service names to the set of string provider ids.
    pub advertised_services: MapOfSets,
}

impl ConnectionGraph {
    /// Create a new, empty connection graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace self with updated, computing the difference and returning it as JSON
    /// See: https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#connection-graph-update
    pub(crate) fn update(&mut self, updated: ConnectionGraph) -> String {
        let mut diff = ConnectionGraphDiff::new();

        // Get new or changed published topics
        for (name, publisher_ids) in updated.published_topics.iter() {
            if let Some(self_publisher_ids) = self.published_topics.get(name) {
                if self_publisher_ids == publisher_ids {
                    // No change
                    continue;
                }
            }

            diff.published_topics.push(NewPublishedTopic {
                name,
                publisher_ids,
            });
        }

        // Get new or changed subscribed topics
        for (name, subscriber_ids) in updated.subscribed_topics.iter() {
            if let Some(self_subscriber_ids) = self.subscribed_topics.get(name) {
                if self_subscriber_ids == subscriber_ids {
                    // No change
                    continue;
                }
            }

            diff.subscribed_topics.push(NewSubscribedTopic {
                name,
                subscriber_ids,
            });
        }

        // Get new or changed advertised services
        for (name, provider_ids) in updated.advertised_services.iter() {
            if let Some(self_provider_ids) = self.advertised_services.get(name) {
                if self_provider_ids == provider_ids {
                    // No change
                    continue;
                }
            }

            diff.advertised_services
                .push(NewAdvertisedService { name, provider_ids });
        }

        // Get removed advertised services
        for name in std::mem::take(&mut self.advertised_services).into_keys() {
            if !updated.advertised_services.contains_key(&name) {
                diff.removed_services.push(name);
            }
        }

        // Get the topics from both published_topics and subscribed_topics that are no longer in either
        for name in std::mem::take(&mut self.published_topics)
            .into_keys()
            .chain(std::mem::take(&mut self.subscribed_topics).into_keys())
        {
            if updated.published_topics.contains_key(&name) {
                continue;
            }
            if updated.subscribed_topics.contains_key(&name) {
                continue;
            }
            diff.removed_topics.insert(name);
        }

        let json_diff = diff.to_json();
        *self = updated;
        json_diff
    }
}
