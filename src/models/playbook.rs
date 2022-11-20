// Copyright 2022 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "playbooks"]
pub struct Playbook {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub state: String,
}

use self::schema::playbooks;

pub mod schema {
    table! {
        playbooks(id) {
            id -> Unsigned<BigInt>,
            title -> Text,
            description -> Text,
            state -> Text,
        }
    }
}