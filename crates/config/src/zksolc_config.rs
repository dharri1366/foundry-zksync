//! zkSolc Compiler Configuration Module.
//!
//! This module defines structures and builders for configuring the zkSolc compiler.
//! It includes settings for the compiler path, various compiler options, optimization settings,
//! and other parameters that influence how Solidity code is compiled using zkSolc.
//!
//! The main structures in this module are `ZkSolcConfig`, which holds the overall configuration,
//! and `Settings`, which encapsulate specific compiler settings. Additionally, `Optimizer` provides
//! detailed settings for bytecode optimization.
//!
//! This module also provides a builder pattern implementation (`ZkSolcConfigBuilder`) for
//! constructing a `ZkSolcConfig` instance in a flexible and convenient manner.
use foundry_compilers::{
    artifacts::{
        output_selection::OutputSelection, serde_helpers, Libraries, OptimizerDetails,
        SettingsMetadata, Source,
    },
    remappings::Remapping,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const SOLIDITY: &str = "Solidity";
/// Configuration for the zkSolc compiler.
///
/// This struct holds the configuration settings used for the zkSolc compiler,
/// including the path to the compiler binary and various compiler settings.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ZkSolcConfig {
    /// Path to zksolc binary. Can be a URL.
    pub compiler_path: PathBuf,

    /// zkSolc compiler settings
    pub settings: Settings,

    /// contracts to compile
    pub contracts_to_compile: Option<Vec<String>>,

    /// contracts to avoid compiling
    pub avoid_contracts: Option<Vec<String>>,
}

/// Compiler settings for zkSolc.
///
/// This struct holds various settings that influence the behavior of the zkSolc compiler.
/// These settings include file remappings, optimization options, metadata settings,
/// output selection criteria, library addresses, and flags for specific compilation modes.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// A list of remappings to apply to the source files.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remappings: Vec<Remapping>,
    /// The `zksolc` optimization settings.
    pub optimizer: Optimizer,
    /// Metadata settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SettingsMetadata>,
    /// This field can be used to select desired outputs based
    /// on file and contract names.
    #[serde(default)]
    pub output_selection: OutputSelection,
    /// Addresses of the libraries. If not all libraries are given here,
    /// it can result in unlinked objects whose output data is different.
    ///
    /// The top level key is the name of the source file where the library is used.
    /// If remappings are used, this source file should match the global path
    /// after remappings were applied.
    /// If this key is an empty string, that refers to a global level.
    #[serde(default)]
    pub libraries: Libraries,
    /// A flag indicating whether to enable the system contract compilation mode.
    pub is_system: bool,
    /// A flag indicating whether to forcibly switch to the EVM legacy assembly pipeline.
    pub force_evmla: bool,
    /// Path to cache missing library dependencies, used for compiling and deploying libraries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_libraries_path: Option<String>,
    /// Flag to indicate if there are missing libraries, used to enable/disable logs for successful
    /// compilation.
    #[serde(default)]
    pub are_libraries_missing: bool,
    /// List of specific contracts to be compiled.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracts_to_compile: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            remappings: Default::default(),
            optimizer: Default::default(),
            metadata: None,
            output_selection: OutputSelection::default_output_selection(),
            libraries: Default::default(),
            is_system: false,
            force_evmla: false,
            missing_libraries_path: None,
            are_libraries_missing: false,
            contracts_to_compile: Default::default(),
        }
    }
}

/// Settings for the optimizer used in zkSolc compiler.
///
/// This struct configures how the zkSolc compiler optimizes the generated bytecode.
/// It includes settings for enabling the optimizer, choosing the optimization mode,
/// specifying detailed optimization parameters, and handling bytecode size constraints.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Optimizer {
    /// Whether the optimizer is enabled.
    pub enabled: Option<bool>,
    /// The optimization mode string.
    pub mode: Option<String>,
    /// The `solc` optimizer details.
    pub details: Option<OptimizerDetails>,
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(rename = "fallbackToOptimizingForSize")]
    pub fallback_to_optimizing_for_size: Option<bool>,
    /// Whether to disable the system request memoization.
    #[serde(rename = "disableSystemRequestMemoization")]
    pub disable_system_request_memoization: bool,
}
/// A builder for `ZkSolcConfig`.
#[derive(Default)]
pub struct ZkSolcConfigBuilder {
    compiler_path: PathBuf,
    settings: Option<Settings>,
    contracts_to_compile: Option<Vec<String>>,
    avoid_contracts: Option<Vec<String>>,
}

impl ZkSolcConfigBuilder {
    /// Creates a new `ZkSolcConfigBuilder`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the path to the `zksolc` binary.
    pub fn compiler_path(mut self, path: PathBuf) -> Self {
        self.compiler_path = path;
        self
    }
    /// Sets the `Settings` for the `ZkSolcConfig`.
    pub fn settings(mut self, settings: Settings) -> Self {
        self.settings = Some(settings);
        self
    }
    /// Builds the `ZkSolcConfig`.
    pub fn build(self) -> Result<ZkSolcConfig, String> {
        let settings = self.settings.unwrap_or_default();
        Ok(ZkSolcConfig {
            compiler_path: self.compiler_path,
            settings,
            contracts_to_compile: self.contracts_to_compile,
            avoid_contracts: self.avoid_contracts,
        })
    }
}

/// A `ZkStandardJsonCompilerInput` representation used for verify
///
/// This type is an alternative `ZkStandardJsonCompilerInput` but uses non-alphabetic ordering of
/// the `sources` and instead emits the (Path -> Source) path in the same order as the pairs in the
/// `sources` `Vec`. This is used over a map, so we can determine the order in which etherscan will
/// display the verified contracts
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkStandardJsonCompilerInput {
    /// The language used in the source files.
    pub language: String,
    /// A map of source file names to their corresponding source code.
    #[serde(with = "serde_helpers::tuple_vec_map")]
    pub sources: Vec<(PathBuf, Source)>,
    /// The zksolc compiler settings.
    pub settings: Settings,
}
impl ZkStandardJsonCompilerInput {
    /// Creates a new `ZkStandardJsonCompilerInput` instance with the specified parameters.
    pub fn new(sources: Vec<(PathBuf, Source)>, settings: Settings) -> Self {
        Self { language: SOLIDITY.to_string(), sources, settings }
    }
}
