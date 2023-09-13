pub mod ast;
pub mod parser;
pub mod checker;

#[salsa::jar(db = Db)]
pub struct Jar(
	ast::TypeId,
	ast::TypeDefinition,
	ast::ProcedureId,
	ast::ProcedureDefinition,
	ast::Diagnostic,
	parser::SourceFile,
	parser::ProgramBlocks,
	parser::tracked_parse_module_item_blocks,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}


// use std::sync::{Arc, Mutex};

#[derive(Default)]
#[salsa::db(Jar)]
pub struct Database {
	storage: salsa::Storage<Self>,

	// The logs are only used for testing and demonstrating reuse:
	// logs: Option<Arc<Mutex<Vec<String>>>>,
}

// impl Database {
// 	/// Enable logging of each salsa event.
// 	#[cfg(test)]
// 	pub fn enable_logging(self) -> Self {
// 		assert!(self.logs.is_none());
// 		Self {
// 			storage: self.storage,
// 			logs: Some(Default::default()),
// 		}
// 	}

// 	#[cfg(test)]
// 	pub fn take_logs(&mut self) -> Vec<String> {
// 		if let Some(logs) = &self.logs {
// 			std::mem::take(&mut *logs.lock().unwrap())
// 		} else {
// 			panic!("logs not enabled");
// 		}
// 	}
// }

impl salsa::Database for Database {
	// fn salsa_event(&self, event: salsa::Event) {
	// 	use salsa::DebugWithDb;
	// 	// Log interesting events, if logging is enabled
	// 	if let Some(logs) = &self.logs {
	// 		// don't log boring events
	// 		if let salsa::EventKind::WillExecute { .. } = event.kind {
	// 			logs.lock()
	// 				.unwrap()
	// 				.push(format!("Event: {:?}", event.debug(self)));
	// 		}
	// 	}
	// }
}

// impl salsa::ParallelDatabase for Database {
// 	fn snapshot(&self) -> salsa::Snapshot<Self> {
// 		salsa::Snapshot::new(Database {
// 			storage: self.storage.snapshot(),
// 			logs: self.logs.clone(),
// 		})
// 	}
// }
