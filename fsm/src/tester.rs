use rand::{Rng, SeedableRng};
use rhai::{Engine, Scope, AST};

use crate::fsm::{FSMError, FSMOutput, StateMachine};

pub struct FSMTester<'a> {
    fsm: StateMachine,
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

fn contract_seed(init: [u8; 32]) -> i64 {
    i64::from_be_bytes(init[24..32].try_into().unwrap())
}

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

impl<'a> FSMTester<'a> {
    pub fn new(fsm: StateMachine, script: String) -> anyhow::Result<Self> {
        let engine = Engine::new();
        let ast = engine.compile(&script)?;
        let mut scope = Scope::new();
        Self::check_script_api(&engine, &ast, &mut scope)?;
        Ok(Self {
            fsm,
            engine,
            ast,
            scope,
        })
    }

    fn check_script_api(engine: &Engine, ast: &AST, scope: &mut Scope<'a>) -> anyhow::Result<()> {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0; 32]);
        let gen_range = move |a: i32, b: i32| rng.gen_range(a..=b);
        let accept_test = engine.call_fn::<String>(scope, ast, "gen_word", (gen_range, true))?; // Generate a test that needs to be accepted.
        let is_accept = engine.call_fn::<bool>(scope, ast, "check_word", (accept_test.clone(),))?;
        if !is_accept {
            anyhow::bail!("gen_word(true) returned {accept_test}, but check_word says False");
        }

        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0; 32]);
        let gen_range = move |a: i32, b: i32| rng.gen_range(a..=b);
        let reject_test = engine.call_fn::<String>(scope, ast, "gen_word", (gen_range, false))?; // Generate a test that needs to be rejected.
        let is_accept = engine.call_fn::<bool>(scope, ast, "check_word", (reject_test.clone(),))?;
        if is_accept {
            anyhow::bail!("gen_word(false) returned {accept_test}, but check_word says True");
        }

        Ok(())
    }

    const TESTS: usize = 100;

    /// Check the FSM against a generated battery of tests.
    ///
    /// If OK, returns how many tests were run.
    /// If fail, returns how many tests succeeded, how many were there in total, and the random seed of the first test that failed.
    pub fn run_testing(&mut self, init_random_seed: i64) -> anyhow::Result<FSMTestingOutput> {
        let mut test_seed_rng = rand_chacha::ChaCha8Rng::from_seed(expand_seed(init_random_seed));
        let mut first_fail_seed = None;
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
            })
        }
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
        let mut test_rng = rand_chacha::ChaCha8Rng::from_seed(expand_seed(seed));
        let gen_range = move |a: i32, b: i32| test_rng.gen_range(a..=b);
        let test_case = self.engine.call_fn::<String>(
            &mut self.scope,
            &self.ast,
            "gen_word",
            (gen_range, seed % 2 == 0),
        )?;
        let true_output = self.engine.call_fn::<bool>(
            &mut self.scope,
            &self.ast,
            "check_word",
            (test_case.clone(),),
        )?;
        let true_output = match true_output {
            true => FSMOutput::Accept,
            false => FSMOutput::Reject,
        };

        let fsm_output = self.fsm.evaluate_unchecked(&test_case);
        match fsm_output {
            Ok(out) => Ok((test_case, Ok((out, true_output)))),
            Err(err) => Ok((test_case, Err(err))),
        }
    }
}

pub enum FSMTestingOutput {
    /// FSM is okay, by agreement of N tests
    Ok(usize),

    /// FSM is not okay: only some tests succeeded, and the test seed of the first failure is given.
    WrongAnswer {
        successes: usize,
        total_tests: usize,
        first_failure_seed: i64,
    },

    /// FSM is invalid
    FSMInvalid(FSMError),
}
