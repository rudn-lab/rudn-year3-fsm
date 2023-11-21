use rand::{Rng, SeedableRng};
use rhai::{Engine, Scope, AST};

use crate::fsm::{FSMError, FSMOutput, StateMachine};

pub struct FSMTester<'a> {
    pub fsm: StateMachine,
    engine: Engine,
    ast: AST,
    scope: Scope<'a>,
}

fn expand_seed(init: i64) -> [u8; 32] {
    let mut output = Vec::with_capacity(32);
    output.extend_from_slice(b"Expand into 32-byte key:");
    output.extend_from_slice(&init.to_be_bytes());
    output.try_into().unwrap()
}

// fn contract_seed(init: [u8; 32]) -> i64 {
//     i64::from_be_bytes(init[24..32].try_into().unwrap())
// }

// #[cfg(test)]
// mod test_seed {
//     use rand::{Rng, SeedableRng};
//     use rand_chacha::ChaCha8Rng;

//     use super::{contract_seed, expand_seed};

//     #[test]
//     fn check_seed_round_trip() {
//         // let mut rng = rand::thread_rng();
//         let mut rng = ChaCha8Rng::from_seed(expand_seed(0));
//         for _ in 0..10000 {
//             let seed: i64 = rng.gen();
//             let expanded = expand_seed(seed);
//             let contracted = contract_seed(expanded);
//             assert_eq!(seed, contracted);
//         }
//     }
// }

#[derive(Clone)]
struct RhaiRng {
    inner: rand_chacha::ChaCha8Rng,
}

impl RhaiRng {
    pub fn new(seed: i64) -> Self {
        Self {
            inner: rand_chacha::ChaCha8Rng::from_seed(expand_seed(seed)),
        }
    }

    pub fn gen_range(&mut self, a: i64, b: i64) -> i64 {
        self.inner.gen_range(a..=b)
    }
}

impl<'a> FSMTester<'a> {
    pub fn new(fsm: StateMachine, script: &str) -> anyhow::Result<Self> {
        let mut engine = Engine::new();
        engine.set_max_expr_depths(100, 100);
        let ast = engine.compile(script)?;
        let mut scope = Scope::new();
        Self::check_script_api(&mut engine, &ast, &mut scope)?;
        Ok(Self {
            fsm,
            engine,
            ast,
            scope,
        })
    }

    pub fn semiclone(&self) -> Self {
        let mut engine = Engine::new();
        engine
            .register_type_with_name::<RhaiRng>("RhaiRng")
            .register_fn("gen_range", RhaiRng::gen_range)
            .set_max_expr_depths(100, 100);

        Self {
            fsm: self.fsm.clone(),
            engine,
            ast: self.ast.clone(),
            scope: self.scope.clone(),
        }
    }

    fn check_script_api(
        engine: &mut Engine,
        ast: &AST,
        scope: &mut Scope<'a>,
    ) -> anyhow::Result<()> {
        let rng = RhaiRng::new(0);
        engine
            .register_type_with_name::<RhaiRng>("RhaiRng")
            .register_fn("gen_range", RhaiRng::gen_range);
        scope.push("rng", rng);
        log::debug!("Testing accept case: generating word");
        let accept_test = engine.call_fn::<String>(scope, ast, "gen_word", (true,))?; // Generate a test that needs to be accepted.
        log::debug!("Testing accept case: verifying accept");
        let is_accept = engine.call_fn::<bool>(scope, ast, "check_word", (accept_test.clone(),))?;
        if !is_accept {
            anyhow::bail!("gen_word(true) returned {accept_test}, but check_word says False");
        }

        let rng = RhaiRng::new(0);
        scope.clear();
        scope.push("rng", rng);
        log::debug!("Testing reject case: generating word");
        let reject_test = engine.call_fn::<String>(scope, ast, "gen_word", (false,))?; // Generate a test that needs to be rejected.
        log::debug!("Testing reject case: verifying reject");
        let is_accept = engine.call_fn::<bool>(scope, ast, "check_word", (reject_test.clone(),))?;
        if is_accept {
            anyhow::bail!("gen_word(false) returned {accept_test}, but check_word says True");
        }

        Ok(())
    }

    #[cfg(target_family = "wasm")]
    const TESTS: usize = 250;

    #[cfg(not(target_family = "wasm"))]
    const TESTS: usize = 2500;

    /// Check the FSM against a generated battery of tests.
    ///
    /// If OK, returns how many tests were run.
    /// If fail, returns how many tests succeeded, how many were there in total, and the random seed of the first test that failed.
    pub fn run_testing(&mut self, init_random_seed: i64) -> anyhow::Result<FSMTestingOutput> {
        let mut test_seed_rng = rand_chacha::ChaCha8Rng::from_seed(expand_seed(init_random_seed));
        let mut first_fail_seed = None;
        let mut first_fail_seed_true_outcome = None;
        let mut successes = 0;

        // If the FSM is obviously invalid, bail.
        if let Some(err) = self.fsm.check_error() {
            return Ok(FSMTestingOutput::FSMInvalid(err));
        }

        for _ in 0..Self::TESTS {
            let test_seed = test_seed_rng.gen();
            let test_outcome = self.test_once(test_seed)?;
            match test_outcome.1 {
                Err(error) => {
                    // FSM is invalid, but we couldn't detect it immediately
                    log::error!("FSM validity error that was not detected immediately: {error}");
                    log::error!("FSM: {:?}", self.fsm);
                    return Ok(FSMTestingOutput::FSMInvalid(error));
                }
                Ok((user_answer, true_answer)) => {
                    if user_answer == true_answer {
                        successes += 1;
                    } else {
                        if first_fail_seed.is_none() {
                            first_fail_seed = Some(test_seed);
                            first_fail_seed_true_outcome = Some(true_answer)
                        }
                    }
                }
            }
        }

        if first_fail_seed.is_none() {
            Ok(FSMTestingOutput::Ok(successes))
        } else {
            Ok(FSMTestingOutput::WrongAnswer {
                successes,
                total_tests: Self::TESTS,
                first_failure_seed: first_fail_seed.unwrap(),
                first_failure_expected_result: first_fail_seed_true_outcome.unwrap(),
            })
        }
    }

    pub fn check_word(&mut self, word: String) -> anyhow::Result<FSMOutput> {
        let true_output: bool =
            self.engine
                .call_fn::<bool>(&mut self.scope, &self.ast, "check_word", (word,))?;
        Ok(match true_output {
            true => FSMOutput::Accept,
            false => FSMOutput::Reject,
        })
    }

    pub fn make_test_case(
        &mut self,
        seed: i64,
        goal_output: bool,
    ) -> Result<(String, FSMOutput), anyhow::Error> {
        let test_rng = RhaiRng::new(seed);
        self.scope.clear();
        self.scope.push("rng", test_rng);

        let test_case = self.engine.call_fn::<String>(
            &mut self.scope,
            &self.ast,
            "gen_word",
            (goal_output,),
        )?;
        let true_output = self.check_word(test_case.clone())?;
        Ok((test_case, true_output))
    }

    /// Check the FSM against a single generated test.
    /// Return a tuple of (FSM output, true output) on success.
    /// If the FSM evaluation fails, the inner Result contains the error, and the errored word is returned;
    /// if the script fails, the outer Result contains the error.
    ///
    /// Uses evaluate_unchecked -- make sure to check the FSM first, to avoid long loops.
    pub fn test_once(
        &mut self,
        seed: i64,
    ) -> Result<(String, Result<(FSMOutput, FSMOutput), FSMError>), anyhow::Error> {
        let (test_case, true_output) = self.make_test_case(seed, seed % 2 == 0)?;

        let fsm_output = self.fsm.evaluate_unchecked(&test_case);
        match fsm_output {
            Ok(out) => Ok((test_case, Ok((out, true_output)))),
            Err(err) => Ok((test_case, Err(err))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FSMTestingOutput {
    /// FSM is okay, by agreement of N tests
    Ok(usize),

    /// FSM is not okay: only some tests succeeded, and the test seed of the first failure is given.
    WrongAnswer {
        successes: usize,
        total_tests: usize,
        first_failure_seed: i64,
        first_failure_expected_result: FSMOutput,
    },

    /// FSM is invalid
    FSMInvalid(FSMError),
}
