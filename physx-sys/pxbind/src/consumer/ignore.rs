use std::collections::HashSet;
use std::sync::LazyLock;
use crate::consumer::{Method, Record};

// NOTE: There are other ignore heuristics sprinkled
//  in the codebase. I haven't cleaned those up yet

pub fn should_ignore_record_def(rec_name: &str) -> bool {
	let ignored_records = [
        // circular type
        "PxTempAllocatorChunk",
        // circular type
        "PxSListEntry",
        // function with self ty
        "PxRenderBuffer",
        // generator fucks up inner enum
        "PxSolverConstraintPrepDescBase",
        // generator fucks up inner enum
        "PxContactStreamIterator",
    ];
	
    ignored_records.contains(&rec_name)
//	    || rec_name.starts_with("PxContactPair")
}

pub fn should_ignore_method(
	rec: &Record,
	method: &Method,
) -> bool {
	let Some(rec_name) = rec.name.as_ref() else {
		return false;
	};
	
	let ignored_methods = [
		// all of these methods fuck the generator up, mostly
		// because they generate fucked types like
		// `physx_PxArticulationAxis::Enum_Pod*` (which is obviously wrong)
		// The obvious solution is to fix the consumer, but I can't
		// be fucked to fix this god awful codebase, it could really
		// use a complete rewrite
		("PxArticulationTendonJoint", Some("setCoefficient")),
		("PxArticulationTendonJoint", Some("getCoefficient")),
		("PxArticulationFixedTendon", Some("createTendonJoint")),
		("PxArticulationReducedCoordinate", None),
		("PxArticulationJointReducedCoordinate", None),
		("PxSimulationStatistics", Some("getRbPairStats")),
		// ignored because PxRenderBuffer is an ignored ty
		("PxControllerManager", Some("getRenderBuffer")),
	];
	
	for ig in ignored_methods {
		if ig.0 == rec_name && ig.1.map(|n| n == method.name).unwrap_or(true) {
			return true;
		}
	}
	
	false
}
