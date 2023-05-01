//
// transaction.rs
// Copyright (C) 2023 db3.network Author imotai <codego.me@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//
// the doc of ar transaction can be found from https://docs.arweave.org/developers/server/http-api#transaction-format
//
//
pub struct ArTransaction<'a> {
    format: i32,
    id: &'a str,
    last_tx: &'a str,
    owner: &'a str,
    tags: Vec<(String, String)>,
    target: &'a str,
    quantity: &'a str,
    data_root: &'a str,
    data_size: &'a str,
    data: &'a str,
    reward: &'a str,
    signature: &'a str,
}

impl<'a> ArTransaction<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
