use rusqlite::Connection;
use rusqlite::OpenFlags;
use serde::Serialize;

pub struct Dba {
    connection: Connection,
}

impl Dba {
    pub fn new(xcresult_path: &str) -> rusqlite::Result<Self> {
        let connection = Connection::open_with_flags(
            format!("{}/database.sqlite3", xcresult_path),
            OpenFlags::SQLITE_OPEN_READ_ONLY,
        )?;
        Ok(Self { connection })
    }

    pub fn get_test_results(&self) -> anyhow::Result<TestResults> {
        let mut stmt = self.connection.prepare(
            "
    SELECT
        tc.name, 
        COUNT(tcr.result) AS failure_count,
        GROUP_CONCAT(ti.compactDescription, '; ') AS failure_reasons,
        GROUP_CONCAT(scl.filePath || ':' || scl.lineNumber, '; ') AS error_locations,
        AVG(tcr.duration) AS average_duration
    FROM TestCases tc
    JOIN TestCaseRuns tcr ON tc.rowid = tcr.testCase_fk
    LEFT JOIN TestIssues ti ON tcr.rowid = ti.testCaseRun_fk
    LEFT JOIN SourceCodeContexts scc ON ti.sourceCodeContext_fk = scc.rowid
    LEFT JOIN SourceCodeLocations scl ON scc.location_fk = scl.rowid
    WHERE tcr.result = 'Failure'
    GROUP BY tc.name
    ORDER BY tc.testSuite_fk, tc.orderInTestSuite;
        ",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(TestRunResult {
                    name: row.get(0)?,
                    failure_count: row.get(1)?,
                    failure_reasons: row.get(2)?,
                    error_locations: row.get(3)?,
                    average_duration: row.get(4)?,
                })
            })?
            .flatten()
            .collect::<Vec<_>>();
        Ok(TestResults { test_results: rows })
    }
}

#[derive(Debug, Serialize)]
pub struct TestResults {
    test_results: Vec<TestRunResult>,
}

#[derive(Debug, Serialize)]
pub struct TestRunResult {
    name: String,
    failure_count: i64,
    failure_reasons: String,
    error_locations: String,
    average_duration: f64,
}
