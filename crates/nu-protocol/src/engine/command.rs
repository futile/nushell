use std::path::PathBuf;

use crate::{ast::Call, BlockId, Example, PipelineData, ShellError, Signature};

use super::{EngineState, Stack};

pub trait Command: Send + Sync + CommandClone {
    fn name(&self) -> &str;

    fn signature(&self) -> Signature;

    fn usage(&self) -> &str;

    fn extra_usage(&self) -> &str {
        ""
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError>;

    fn is_binary(&self) -> bool {
        false
    }

    fn examples(&self) -> Vec<Example> {
        Vec::new()
    }

    // This is a built-in command
    fn is_builtin(&self) -> bool {
        true
    }

    // This is a signature for a known external command
    fn is_known_external(&self) -> bool {
        false
    }

    // Is a sub command
    fn is_sub(&self) -> bool {
        self.name().contains(' ')
    }

    // Is a parser keyword (source, def, etc.)
    fn is_parser_keyword(&self) -> bool {
        false
    }

    // Is a plugin command (returns plugin's path, encoding and type of shell
    // if the declaration is a plugin)
    fn is_plugin(&self) -> Option<(&PathBuf, &str, &Option<PathBuf>)> {
        None
    }

    // If command is a block i.e. def blah [] { }, get the block id
    fn get_block_id(&self) -> Option<BlockId> {
        None
    }
}

pub trait CommandClone {
    fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone,
{
    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_box()
    }
}
