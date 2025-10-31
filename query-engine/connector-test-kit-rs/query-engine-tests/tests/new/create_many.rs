use query_engine_tests::*;

#[test_suite(schema(autoinc_id), capabilities(CreateMany, AutoIncrement))]
mod single_col {
    use query_engine_tests::run_query;

    #[connector_test()]
    async fn foo(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, "mutation { createManyTestModel(data: [{},{}]) { count }}"),
          @r###"{"data":{"createManyTestModel":{"count":2}}}"###
        );

        insta::assert_snapshot!(
          run_query!(&runner, "query { findManyTestModel(orderBy: { id: asc }) { id }}"),
          @r###"{"data":{"findManyTestModel":[{"id":1},{"id":2}]}}"###
        );

        Ok(())
    }
}
