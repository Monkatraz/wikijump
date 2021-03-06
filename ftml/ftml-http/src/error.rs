/*
 * error.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use tera::Error as TeraError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("remote server returned invalid page response")]
    InvalidResponse,

    #[error("serializing or desezerializing from JSON: {0}")]
    Json(#[from] JsonError),

    #[error("fetching from network: {0}")]
    Request(#[from] ReqwestError),

    #[error("building or rendering template failed: {0}")]
    Template(#[from] TeraError),
}
