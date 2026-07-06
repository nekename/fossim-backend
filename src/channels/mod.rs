use crate::Community;

use std::collections::HashMap;
use std::sync::Arc;

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition, TableError};

const CHANNEL_SEQ_COUNTERS: TableDefinition<(&str, &str, &str, &str), u64> =
	TableDefinition::new("channel_seq_counters");

#[derive(Clone)]
pub struct Channels {
	db: Arc<Database>,
}

impl Channels {
	pub fn new() -> Self {
		std::fs::create_dir_all("db").expect("db directory should be able to be created");
		Self {
			db: Arc::new(
				Database::create("db/channels.redb")
					.expect("database should be able to be created"),
			),
		}
	}

	pub fn get_seq_counters(
		&self,
		community: &Community,
		channel_ids: Vec<String>,
	) -> Result<HashMap<String, u64>, redb::Error> {
		let read_txn = self.db.begin_read()?;
		let table = match read_txn.open_table(CHANNEL_SEQ_COUNTERS) {
			Ok(t) => t,
			Err(TableError::TableDoesNotExist(_)) => {
				return Ok(channel_ids.into_iter().map(|id| (id, 0)).collect());
			}
			Err(e) => return Err(e.into()),
		};

		let mut seq_counters = HashMap::new();
		for channel_id in channel_ids {
			let value = table
				.get((
					community.0.as_str(),
					community.1.as_str(),
					community.2.as_str(),
					channel_id.as_str(),
				))
				.map(|x| x.map(|v| v.value()).unwrap_or(0))?;
			seq_counters.insert(channel_id, value);
		}

		Ok(seq_counters)
	}

	pub fn increment_seq_counter(
		&self,
		community: &Community,
		channel_id: &str,
	) -> Result<(), redb::Error> {
		let key = (
			community.0.as_str(),
			community.1.as_str(),
			community.2.as_str(),
			channel_id,
		);

		let write_txn = self.db.begin_write()?;
		{
			let mut table = write_txn.open_table(CHANNEL_SEQ_COUNTERS)?;
			let current_counter = table.get(key)?.map(|v| v.value()).unwrap_or(0);
			table.insert(key, current_counter + 1)?;
		}
		write_txn.commit()?;

		Ok(())
	}
}
