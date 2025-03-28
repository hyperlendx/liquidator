pub use liquidator::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CALLBACK_SUCCESS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CALLBACK_SUCCESS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FLASH_MINTER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FLASH_MINTER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IUsdxlFlashMinter",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("USDXL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("USDXL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approvePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hook"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hook"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hyperswapV3Factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hyperswapV3Factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isLiquidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("kittenPairFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kittenPairFactory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IKittenPairFactory",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liqPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("finalToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("finalGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onFlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onFlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLiquidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LiquidatorSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LiquidatorSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3Pa3&\x80a\0\\_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cW\x05\xAEC\x11a\0\x93W\x80c\x9A{\xFFy\x11a\0cW\x80c\x9A{\xFFy\x14a\x02\x81W\x80c\xEA\x93\x9F\xA6\x14a\x02\x94W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xAFW\x80c\xFAF\x1E3\x14a\x02\xC2W__\xFD[\x80cW\x05\xAEC\x14a\x02\x1AW\x80cp\xC2j^\x14a\x02-W\x80c\x827\xE58\x14a\x02HW\x80c\x8D\xA5\xCB[\x14a\x02oW__\xFD[\x80c>\r\x95Z\x11a\0\xCEW\x80c>\r\x95Z\x14a\x01\xA5W\x80cBL&[\x14a\x01\xC0W\x80cDS\xA3t\x14a\x01\xD5W\x80cR\x9A5o\x14a\x01\xE8W__\xFD[\x80c\x08\xBE\xA1'\x14a\0\xFFW\x80c\x16\xF0\x11[\x14a\x017W\x80c#\xE3\x0C\x8B\x14a\x01RW\x80c.C\xC9a\x14a\x01sW[__\xFD[a\x01\x1As\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x1As\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x81V[a\x01ea\x01`6`\x04a*\xF0V[a\x02\xD5V[`@Q\x90\x81R` \x01a\x01.V[a\x01\x86a\x01\x816`\x04a+gV[a\x07\xDDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01.V[a\x01\x1As\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3\x81V[a\x01\xD3a\x01\xCE6`\x04a,\x16V[a\x0E\xFAV[\0[a\x01\xD3a\x01\xE36`\x04a,IV[a\x0F\xC2V[a\x02\na\x01\xF66`\x04a,\x16V[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01.V[a\x01\xD3a\x02(6`\x04a,\x80V[a\x10MV[a\x01\x1As\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x81V[a\x01e\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x81V[_Ta\x01\x1A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD3a\x02\x8F6`\x04a,\xAAV[a\x11\"V[a\x01\x1As\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x81V[a\x01\xD3a\x02\xBD6`\x04a,\x16V[a\x13\xA5V[a\x01\xD3a\x02\xD06`\x04a-\x0FV[a\x14\x18V[_`\x01`\x01`\xA0\x1B\x03\x87\x160\x14a\x03RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FInitiator of onFlashLoan() must `D\x82\x01Ru\x18\x99H\x1B\x1A\\]ZY\x18]\x1B\xDC\x88\x18\xDB\xDB\x9D\x1C\x98X\xDD`R\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x14a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FMsg.sender of onFlashLoan() must`D\x82\x01Rs\x1012\x90:\xB9\xB2<6#60\xB9\xB4&\xB4\xB7:2\xB9`a\x1B`d\x82\x01R`\x84\x01a\x03IV[`\x01`\x01`\xA0\x1B\x03\x86\x16s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x14a\x04>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FFlash loaned token must be USDXL`D\x82\x01R`d\x01a\x03IV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x85\x90s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB1\x91\x90a-^V[\x10\x15a\x04\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid balance for flash loan\0\0`D\x82\x01R`d\x01a\x03IV[a\x05\x07a*'V[a\x05\x13\x83\x85\x01\x85a.>V[`@\x80\x83\x01\x82\x90R\x01Q`\x01`\x01`\xA0\x1B\x03\x16s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x14a\x05\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FDebt asset must be USDXL\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03IV[a\x05\x92\x85\x87a/(V[\x81R`@\x80\x82\x01Q` \x01Q\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x04\x91\x90a-^V[` \x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x82\x01Q\x82\x82\x01Q``\x84\x01Q`\x80\x90\x94\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x06\\\x94\x93\x92_\x90`\x04\x01a/;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06sW__\xFD[PZ\xF1\x15\x80\x15a\x06\x85W=__>=_\xFD[PPPP` \x81\x81\x01Q`@\x80\x84\x01Q\x90\x92\x01Q\x91Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xDBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xFF\x91\x90a-^V[a\x07\t\x91\x90a/oV[` \x82\x01\x81\x90R`@\x82\x01Qa\x07\x1F\x91\x90a\x15\xA2V[\x80Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x04\x82\x01R`$\x81\x01\x91\x90\x91Rs\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAF\x91\x90a/\x82V[P\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x98\x97PPPPPPPPV[_\x80T\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x08\x06WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x08\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a/\x9DV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8A\x90\x8A\x90`\x01`\x01`\xFF\x1B\x03\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x95\x91\x90a-^V[\x11\x15a\x08\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FCollateral asset balance too lar`D\x82\x01Rage`\xF0\x1B`d\x82\x01R`\x84\x01a\x03IV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xFF\x1B\x03\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t]\x91\x90a-^V[\x11\x15a\t\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FDebt asset balance too large\0\0\0\0`D\x82\x01R`d\x01a\x03IV[`@Qi\x06\xB6\x97GFV\xE77v\x17`\xB4\x1B` \x82\x01R`*\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x86`@Q` \x01a\t\xED\x92\x91\x90a/\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x0B\x94W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8C\x94P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nMW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nq\x91\x90a-^V[\x92Pa\x0B\x1A\x89`@Q\x80a\x01\0\x01`@R\x80\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8F`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x18\x11V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x90`\x01`\x01`\xA0\x1B\x03\x8E\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x83\x91\x90a-^V[a\x0B\x8D\x91\x90a/\xFBV[\x92Pa\x0E\xEBV[`@Qh\x06\x87\x97\x06W'7v\x17`\xBC\x1B` \x82\x01R`)\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x86`@Q` \x01a\x0B\xD5\x92\x91\x90a/\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\r\x02W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8C\x94P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CY\x91\x90a-^V[\x92Pa\x0B\x1A\x89`@Q\x80a\x01\0\x01`@R\x80\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8F`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x1AKV[`@Qo:\xB9\xB2<6#60\xB9\xB4&\xB4\xB7:2\xB9`\x81\x1B` \x82\x01R`0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x86`@Q` \x01a\rJ\x92\x91\x90a/\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x0E\xA3W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8B\x94P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a-^V[\x92Pa\x0Eu\x89`@Q\x80a\x01\0\x01`@R\x80\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8F`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x15\x15\x81RPa\x1B\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90cp\xA0\x821\x90`$\x01a\x0BDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid liquidation path\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03IV[PP\x98P\x98\x96PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x0F WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x0F<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a/\x9DV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xBE\x91\x90a/\x82V[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a0!V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x02` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x81\xE0 4At\x97,Y\xF6\xC1\x1A\x8Fl\x90\xB1A\x86b\x14\xE3\xD9\xB5D\xD00\xF0\xB52\xF5\xA1\x0F\x90a\x10A\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a0!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\xB3W`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x10\xAEW=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xAE\x91\x90a/\x82V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fmsg.sender != activeKittenPair\0\0`D\x82\x01R`d\x01a\x03IV[`@\x80Q`\x80\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x84Qa\x01\0\x81\x01\x86R``\x80\x82R\x91\x81\x01\x83\x90R\x94\x85\x01\x82\x90R\x80\x85\x01\x82\x90R\x91\x84\x01\x81\x90R`\xA0\x84\x01\x81\x90R`\xC0\x84\x01\x81\x90R`\xE0\x84\x01R\x81\x01\x91\x90\x91Ra\x11\xE4\x82\x84\x01\x84a.>V[``\x82\x01\x81\x90RQa\x11\xF5\x90a\x1CRV[\x15\x15`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x16\x81R``\x81\x01Q`\xC0\x01Q\x15a\x12\xA0W``\x80\x82\x01Q` \x81\x01Q`@\x80\x83\x01Q\x93\x83\x01Q`\x80\x90\x93\x01Q\x90Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x12r\x94\x93\x91\x92\x90\x91\x90_\x90`\x04\x01a/;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\x89W__\xFD[PZ\xF1\x15\x80\x15a\x12\x9BW=__>=_\xFD[PPPP[``\x81\x01QQa\x12\xAF\x90a\x1C\xA3V[\x15a\x13\x02W``\x81\x01QQa\x12\xC3\x90a\x1C\xDCV[``\x82\x01\x80Q\x91\x90\x91RQ`\xE0\x01Q\x15a\x12\xEFW``\x81\x01Q`\xA0\x81\x01Qa\x12\xEA\x91a\x18\x11V[a\x13\x02V[``\x81\x01Q`\xA0\x81\x01Qa\x13\x02\x91a\x15\xA2V[\x80``\x01Q`\xE0\x01Q\x15a\x13!W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R[\x80Q``\x82\x01Q`\xA0\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13xW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9C\x91\x90a/\x82V[PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a0!V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\x14%\x82\x84\x01\x84a.>V[\x90P___a\x146\x84_\x01Qa\x1D\x13V[\x92P\x92P\x92Pa\x14ds\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x14_\x85\x85\x85a\x1DBV[a\x1D\xACV[\x83`\xC0\x01Q\x15a\x14\xE9W` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x14\xBB\x94\x91\x93\x91\x92_\x90`\x04\x01a/;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\xD2W__\xFD[PZ\xF1\x15\x80\x15a\x14\xE4W=__>=_\xFD[PPPP[__\x89\x13a\x14\xF7W\x87a\x14\xF9V[\x88[\x90Pa\x15\x07\x85_\x01Qa\x1E\0V[\x15a\x15#W\x84Qa\x15\x17\x90a\x1E\x1AV[\x85Ra\x15#\x81\x86a\x1AKV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x96\x91\x90a/\x82V[PPPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R\x81Qa\x15\xDE\x90a\x1CRV[\x15\x15`\xA0\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x85\x01\x81\x90R\x92\x90\x91\x16\x80\x84R`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92R`D\x82\x01Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x80\x91\x90a0GV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x16\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x03IV[`\xA0\x82\x01\x83\x90R` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90\x82\x16\x10`\x80\x80\x85\x01\x91\x90\x91R`@\x80Q\x91\x82\x01\x81R`\x01T\x83\x16\x82Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x93\x82\x01\x93\x90\x93R\x91\x82\x01\x85\x90R\x82Q\x16``\x82\x01Ra\x17K\x90a\x1E7V[`@\x82\x01R`\x01T`\x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02,\r\x9F\x90a\x17xW\x82`@\x01Qa\x17zV[_[\x83`\x80\x01Qa\x17\x89W_a\x17\x8FV[\x83`@\x01Q[0\x86`@Q` \x01a\x17\xA1\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xCF\x94\x93\x92\x91\x90a1,V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xE6W__\xFD[PZ\xF1\x15\x80\x15a\x17\xF8W=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x81Qa\x188\x90a\x1CRV[\x15\x15`@\x84\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x86\x01\x81\x90R\x93\x90\x92\x16\x80\x85R\x91Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x83\x90R`D\x81\x01\x91\x90\x91R\x91\x10\x15\x90s\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xBBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xDF\x91\x90a0GV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x19@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x03IV[`@\x80Q`\x80\x80\x82\x01\x83R`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B` \x80\x85\x01\x91\x90\x91R\x91\x87\x01Q\x93\x83\x01\x93\x90\x93R\x84\x01Q\x90\x91\x16``\x82\x01Ra\x19\x98\x90a\x1E\xC2V[`\xA0\x84\x01R`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x82a\x19\xB9W\x85a\x19\xBBV[_[\x83a\x19\xC6W_a\x19\xC8V[\x86[0\x87`@Q` \x01a\x19\xDA\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x08\x94\x93\x92\x91\x90a1,V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\x1FW__\xFD[PZ\xF1\x15\x80\x15a\x1A1W=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPPV[___a\x1AZ\x84_\x01Qa\x1D\x13V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\x1A\x99s\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x1A\x94\x86\x88\x87a\x1DBV[a CV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xFB\x91\x90a0GV[P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x1B\x16\x8Ba1XV[\x86a\x1B?Wa\x1B:`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a1rV[a\x1BOV[a\x1BOd\x01\0\x02v\xA3`\x01a1\x91V[\x8B`@Q` \x01a\x1B`\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x8F\x95\x94\x93\x92\x91\x90a1\xB0V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xCE\x91\x90a1\xF5V[PPPPPPPPPV[s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x01`\x01`\xA0\x1B\x03\x16c\\\xFF\xE9\xDE0s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x85\x85`@Q` \x01a\x1C$\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xE2\x94\x93\x92\x91\x90a2\x17V[_\x80\x80a\x1C_\x84\x82a!)V[\x92Pa\x1C\x82`\x01a\x1Cq\x81`\x14a/(V[a\x1C{\x91\x90a/oV[\x85\x90a!\x8DV[\x90Pa\x1C\x9Aa\x1C\x93`\x01`\x14a/(V[\x85\x90a!)V[\x91P\x91\x93\x90\x92PV[_a\x1C\xB0`\x01`\x14a/(V[`\x14a\x1C\xBD`\x01\x82a/(V[a\x1C\xC7\x91\x90a/(V[a\x1C\xD1\x91\x90a/(V[\x82Q\x10\x15\x90P\x91\x90PV[``a\x1D\ra\x1C\xED`\x01`\x14a/(V[a\x1C\xF9`\x01`\x14a/(V[\x84Qa\x1D\x05\x91\x90a/oV[\x84\x91\x90a!\xFEV[\x92\x91PPV[_\x80\x80a\x1D \x84\x82a!)V[\x92Pa\x1D-\x84`\x14a#\nV[a\xFF\xFF\x16\x90Pa\x1C\x9Aa\x1C\x93`\x03`\x14a/(V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x1D|W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x1D\xB7\x83\x83a CV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x10\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x03IV[_a\x1E\r`\x03`\x14a/(V[`\x14a\x1C\xBD`\x03\x82a/(V[``a\x1D\ra\x1E+`\x03`\x14a/(V[a\x1C\xF9`\x03`\x14a/(V[\x80Q`@\x80\x83\x01Q``\x84\x01Q\x91Qcx\xA0Q\xAD`\xE1\x1B\x81R_\x93`\x01`\x01`\xA0\x1B\x03\x16\x92c\xF1@\xA3Z\x92a\x1E\x83\x92\x90\x91\x90`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\r\x91\x90a-^V[`@\x80Q`\xE0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c9/7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\\\x91\x90a2IV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01R\x15\x15`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x81R_a\x1F\x95\x84\x83a#\xB4V[` \x85\x01Q\x85Q`\x80\x85\x01Q`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x15\x15`$\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x16\x91\x90a-^V[\x90Pa $\x81a'\x10a/oV[a 0\x83a'\x10a2\xBAV[a :\x91\x90a2\xD1V[\x95\x94PPPPPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a hW__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\x1E\xEDC\xDC\xAA.\xFD\xE0g.\xB5qd\x92\0\xA2\x927\xB7\x95\x8E{\x0F\xBDR\xF7_\xA3[~\xC5,`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a!5\x82`\x14a/(V[\x83Q\x10\x15a!}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x03IV[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_a!\x99\x82`\x01a/(V[\x83Q\x10\x15a!\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoBool_outOfBounds`p\x1B`D\x82\x01R`d\x01a\x03IV[\x82\x82\x01`\x01\x01Q`\xFF\x81\x16a!\xF3W_a!\xF6V[`\x01[\x94\x93PPPPV[``\x81a\"\x0C\x81`\x1Fa/(V[\x10\x15a\"KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x03IV[a\"U\x82\x84a/(V[\x84Q\x10\x15a\"\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x03IV[``\x82\x15\x80\x15a\"\xB7W`@Q\x91P_\x82R` \x82\x01`@Ra#\x01V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\"\xF0W\x80Q\x83R` \x92\x83\x01\x92\x01a\"\xD8V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[_\x81a#\x17\x81`\x03a/(V[\x10\x15a#YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x03IV[a#d\x82`\x03a/(V[\x83Q\x10\x15a#\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x03IV[P\x01`\x03\x01Q\x90V[_\x81`\x80\x01Q\x15a%\xCBW_a$\x04`@Q\x80`\xA0\x01`@R\x80\x85`@\x01Q\x81R` \x01\x85``\x01Q\x81R` \x01\x85`\x80\x01Q\x15\x15\x81R` \x01\x85_\x01Q\x81R` \x01\x85` \x01Q\x81RPa&?V[\x83Q`@\x85\x01Q\x91\x92P\x90a$!\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a$+\x91\x90a2\xD1V[`@\x84\x01R` \x83\x01Q``\x84\x01Qa$L\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a$V\x91\x90a2\xD1V[\x83``\x01\x81\x81RPP__\x84`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x91W\x84``\x01Q\x85`@\x01Qa$\x9CV[\x84`@\x01Q\x85``\x01Q[\x91P\x91P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a$\xECW` \x85\x01Q`@\x87\x01Qa$\xDD\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a$\xE7\x91\x90a2\xD1V[a%\x0FV[\x84Q`@\x87\x01Qa%\x05\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a%\x0F\x91\x90a2\xD1V[`@\x87\x01\x81\x90R_\x90a%\"\x90\x83a/oV[\x90P_\x83a%h`@Q\x80`\xC0\x01`@R\x80\x85\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x80\x01Q\x15\x15\x81R` \x01\x8A_\x01Q\x81R` \x01\x8A` \x01Q\x81RPa'@V[a%r\x91\x90a/oV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a%\xA5W\x87Qa%\xABV[\x87` \x01Q[a%\xB5\x90\x83a2\xBAV[a%\xBF\x91\x90a2\xD1V[\x95PPPPPPa\x1D\rV[__\x83`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a%\xFDW\x83``\x01Q\x84`@\x01Qa&\x08V[\x83`@\x01Q\x84``\x01Q[\x91P\x91P\x84`@\x01Q\x81a&\x1C\x91\x90a/oV[\x82\x86`@\x01Qa&,\x91\x90a2\xBAV[a&6\x91\x90a2\xD1V[\x92PPPa\x1D\rV[_\x81`@\x01Q\x15a'*W``\x82\x01Q\x82Q_\x91\x90a&f\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a&p\x91\x90a2\xD1V[\x90P_\x83`\x80\x01Q\x84` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a&\x90\x91\x90a2\xBAV[a&\x9A\x91\x90a2\xD1V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a&\xB0\x83\x85a2\xBAV[a&\xBA\x91\x90a2\xD1V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a&\xD0\x84\x80a2\xBAV[a&\xDA\x91\x90a2\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0a&\xED\x86\x80a2\xBAV[a&\xF7\x91\x90a2\xD1V[a'\x01\x91\x90a/(V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a'\x16\x82\x84a2\xBAV[a' \x91\x90a2\xD1V[\x96\x95PPPPPPV[` \x82\x01Q\x82Qa\x1D\r\x91\x90a2\xBAV[\x91\x90PV[_\x80[`\xFF\x81\x10\x15a(\xFCW_a'^\x84`@\x01Q\x85_\x01Qa)*V[\x90P\x83` \x01Q\x81\x10\x15a(SW_a'~\x85`@\x01Q\x86_\x01Qa)\xA5V[\x82\x86` \x01Qa'\x8E\x91\x90a/oV[a'\xA0\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a'\xAA\x91\x90a2\xD1V[\x90P\x80_\x03a(8W\x84` \x01Q\x82\x03a'\xC9WPPPP`@\x01Q\x90V[\x84` \x01Qa(\x1E`@Q\x80`\xA0\x01`@R\x80\x88`@\x01Q`\x01a'\xED\x91\x90a/(V[\x81R` \x01\x88_\x01Q\x81R` \x01\x88``\x01Q\x15\x15\x81R` \x01\x88`\x80\x01Q\x81R` \x01\x88`\xA0\x01Q\x81RPa&?V[\x11\x15a(4W`@\x85\x01Qa :\x90`\x01a/(V[P`\x01[\x80\x85`@\x01Qa(H\x91\x90a/(V[`@\x86\x01RPa(\xF3V[_a(e\x85`@\x01Q\x86_\x01Qa)\xA5V[` \x86\x01Qa(t\x90\x84a/oV[a(\x86\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a(\x90\x91\x90a2\xD1V[\x90P\x80_\x03a(\xDCW\x84` \x01Q\x82\x14\x80a(\xC8WP\x84` \x01Qa(\xC6`\x01\x87`@\x01Qa(\xBF\x91\x90a/oV[\x87Qa)*V[\x10[\x15a(\xD8WPPPP`@\x01Q\x90V[P`\x01[\x80\x85`@\x01Qa(\xEC\x91\x90a/oV[`@\x86\x01RP[P`\x01\x01a'CV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04/`\xF3\x1B`D\x82\x01R`d\x01a\x03IV[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a)?\x84\x86a2\xBAV[a)I\x91\x90a2\xD1V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a)_\x85\x80a2\xBAV[a)i\x91\x90a2\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0a)|\x87\x80a2\xBAV[a)\x86\x91\x90a2\xD1V[a)\x90\x91\x90a/(V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a 0\x82\x84a2\xBAV[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a)\xBB\x82\x80a2\xBAV[a)\xC5\x91\x90a2\xD1V[a)\xCF\x91\x90a2\xBAV[a)\xD9\x91\x90a2\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0\x80a)\xED\x85\x80a2\xBAV[a)\xF7\x91\x90a2\xD1V[a*\x02\x86`\x03a2\xBAV[a*\x0C\x91\x90a2\xBAV[a*\x16\x91\x90a2\xD1V[a* \x91\x90a/(V[\x93\x92PPPV[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01a*\x84`@\x80Qa\x01\0\x81\x01\x82R``\x80\x82R_` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a*\x9DW__\xFD[PV[\x805a';\x81a*\x89V[__\x83`\x1F\x84\x01\x12a*\xBBW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xD2W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a*\xE9W__\xFD[\x92P\x92\x90PV[______`\xA0\x87\x89\x03\x12\x15a+\x05W__\xFD[\x865a+\x10\x81a*\x89V[\x95P` \x87\x015a+ \x81a*\x89V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+IW__\xFD[a+U\x89\x82\x8A\x01a*\xABV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[________`\xC0\x89\x8B\x03\x12\x15a+~W__\xFD[\x885a+\x89\x81a*\x89V[\x97P` \x89\x015a+\x99\x81a*\x89V[\x96P`@\x89\x015a+\xA9\x81a*\x89V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xCBW__\xFD[a+\xD7\x8B\x82\x8C\x01a*\xABV[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xF6W__\xFD[a,\x02\x8B\x82\x8C\x01a*\xABV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a,&W__\xFD[\x815a* \x81a*\x89V[\x80\x15\x15\x81\x14a*\x9DW__\xFD[\x805a';\x81a,1V[__`@\x83\x85\x03\x12\x15a,ZW__\xFD[\x825a,e\x81a*\x89V[\x91P` \x83\x015a,u\x81a,1V[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a,\x91W__\xFD[\x825a,\x9C\x81a*\x89V[\x94` \x93\x90\x93\x015\x93PPPV[_____`\x80\x86\x88\x03\x12\x15a,\xBEW__\xFD[\x855a,\xC9\x81a*\x89V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xF2W__\xFD[a,\xFE\x88\x82\x89\x01a*\xABV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15a-\"W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-FW__\xFD[a-R\x87\x82\x88\x01a*\xABV[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a-nW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-\xADWa-\xADa-uV[`@R\x90V[_\x82`\x1F\x83\x01\x12a-\xC2W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xDCWa-\xDCa-uV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.\x0BWa.\x0Ba-uV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a.\"W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a.NW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.dW__\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a.vW__\xFD[a.~a-\x89V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\x94W__\xFD[a.\xA0\x86\x82\x85\x01a-\xB3V[\x82RPa.\xAF` \x83\x01a*\xA0V[` \x82\x01Ra.\xC0`@\x83\x01a*\xA0V[`@\x82\x01Ra.\xD1``\x83\x01a*\xA0V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x80\x83\x015\x90\x82\x01Ra.\xF6`\xC0\x83\x01a,>V[`\xC0\x82\x01Ra/\x07`\xE0\x83\x01a,>V[`\xE0\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1D\rWa\x1D\ra/\x14V[`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16`@\x83\x01R``\x82\x01\x92\x90\x92R\x90\x15\x15`\x80\x82\x01R`\xA0\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1D\rWa\x1D\ra/\x14V[_` \x82\x84\x03\x12\x15a/\x92W__\xFD[\x81Qa* \x81a,1V[` \x80\x82R`/\x90\x82\x01R\x7FOnly owner or liquidator can cal`@\x82\x01Rn6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a0\x1AWa0\x1Aa/\x14V[P\x92\x91PPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a0WW__\xFD[\x81Qa* \x81a*\x89V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Qa\x01\0` \x84\x01Ra0\xADa\x01 \x84\x01\x82a0bV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R``\x84\x01Qa0\xED`\x80\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R`\xC0\x84\x01Qa1\x15`\xE0\x85\x01\x82\x15\x15\x90RV[P`\xE0\x84\x01Q\x80\x15\x15a\x01\0\x85\x01RP\x93\x92PPPV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_a' `\x80\x83\x01\x84a0bV[_`\x01`\xFF\x1B\x82\x01a1lWa1la/\x14V[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1D\rWa\x1D\ra/\x14V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1D\rWa\x1D\ra/\x14V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a1\xEA\x90\x83\x01\x84a0bV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a2\x06W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a' \x90\x83\x01\x84a0bV[_______`\xE0\x88\x8A\x03\x12\x15a2_W__\xFD[\x87Q` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q\x93\x9AP\x91\x98P\x96P\x94Pa2\x88\x81a,1V[`\xA0\x89\x01Q\x90\x93Pa2\x99\x81a*\x89V[`\xC0\x89\x01Q\x90\x92Pa2\xAA\x81a*\x89V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1D\rWa\x1D\ra/\x14V[_\x82a2\xEBWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x8D\x99\xAA\xC4\x18\xDC(\xD1\x0F\xB1\xFD\x86\xFC,\x0C\x08dz\x1F(v\x10^\xE2\x9F\":y\xC2\x11\xA2\x07dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cW\x05\xAEC\x11a\0\x93W\x80c\x9A{\xFFy\x11a\0cW\x80c\x9A{\xFFy\x14a\x02\x81W\x80c\xEA\x93\x9F\xA6\x14a\x02\x94W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xAFW\x80c\xFAF\x1E3\x14a\x02\xC2W__\xFD[\x80cW\x05\xAEC\x14a\x02\x1AW\x80cp\xC2j^\x14a\x02-W\x80c\x827\xE58\x14a\x02HW\x80c\x8D\xA5\xCB[\x14a\x02oW__\xFD[\x80c>\r\x95Z\x11a\0\xCEW\x80c>\r\x95Z\x14a\x01\xA5W\x80cBL&[\x14a\x01\xC0W\x80cDS\xA3t\x14a\x01\xD5W\x80cR\x9A5o\x14a\x01\xE8W__\xFD[\x80c\x08\xBE\xA1'\x14a\0\xFFW\x80c\x16\xF0\x11[\x14a\x017W\x80c#\xE3\x0C\x8B\x14a\x01RW\x80c.C\xC9a\x14a\x01sW[__\xFD[a\x01\x1As\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x1As\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x81V[a\x01ea\x01`6`\x04a*\xF0V[a\x02\xD5V[`@Q\x90\x81R` \x01a\x01.V[a\x01\x86a\x01\x816`\x04a+gV[a\x07\xDDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01.V[a\x01\x1As\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3\x81V[a\x01\xD3a\x01\xCE6`\x04a,\x16V[a\x0E\xFAV[\0[a\x01\xD3a\x01\xE36`\x04a,IV[a\x0F\xC2V[a\x02\na\x01\xF66`\x04a,\x16V[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01.V[a\x01\xD3a\x02(6`\x04a,\x80V[a\x10MV[a\x01\x1As\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x81V[a\x01e\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x81V[_Ta\x01\x1A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD3a\x02\x8F6`\x04a,\xAAV[a\x11\"V[a\x01\x1As\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x81V[a\x01\xD3a\x02\xBD6`\x04a,\x16V[a\x13\xA5V[a\x01\xD3a\x02\xD06`\x04a-\x0FV[a\x14\x18V[_`\x01`\x01`\xA0\x1B\x03\x87\x160\x14a\x03RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FInitiator of onFlashLoan() must `D\x82\x01Ru\x18\x99H\x1B\x1A\\]ZY\x18]\x1B\xDC\x88\x18\xDB\xDB\x9D\x1C\x98X\xDD`R\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x14a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FMsg.sender of onFlashLoan() must`D\x82\x01Rs\x1012\x90:\xB9\xB2<6#60\xB9\xB4&\xB4\xB7:2\xB9`a\x1B`d\x82\x01R`\x84\x01a\x03IV[`\x01`\x01`\xA0\x1B\x03\x86\x16s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x14a\x04>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FFlash loaned token must be USDXL`D\x82\x01R`d\x01a\x03IV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x85\x90s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB1\x91\x90a-^V[\x10\x15a\x04\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid balance for flash loan\0\0`D\x82\x01R`d\x01a\x03IV[a\x05\x07a*'V[a\x05\x13\x83\x85\x01\x85a.>V[`@\x80\x83\x01\x82\x90R\x01Q`\x01`\x01`\xA0\x1B\x03\x16s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x14a\x05\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FDebt asset must be USDXL\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03IV[a\x05\x92\x85\x87a/(V[\x81R`@\x80\x82\x01Q` \x01Q\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x04\x91\x90a-^V[` \x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x82\x01Q\x82\x82\x01Q``\x84\x01Q`\x80\x90\x94\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x06\\\x94\x93\x92_\x90`\x04\x01a/;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06sW__\xFD[PZ\xF1\x15\x80\x15a\x06\x85W=__>=_\xFD[PPPP` \x81\x81\x01Q`@\x80\x84\x01Q\x90\x92\x01Q\x91Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xDBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xFF\x91\x90a-^V[a\x07\t\x91\x90a/oV[` \x82\x01\x81\x90R`@\x82\x01Qa\x07\x1F\x91\x90a\x15\xA2V[\x80Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x04\x82\x01R`$\x81\x01\x91\x90\x91Rs\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAF\x91\x90a/\x82V[P\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x98\x97PPPPPPPPV[_\x80T\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x08\x06WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x08\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a/\x9DV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8A\x90\x8A\x90`\x01`\x01`\xFF\x1B\x03\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x95\x91\x90a-^V[\x11\x15a\x08\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FCollateral asset balance too lar`D\x82\x01Rage`\xF0\x1B`d\x82\x01R`\x84\x01a\x03IV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xFF\x1B\x03\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t]\x91\x90a-^V[\x11\x15a\t\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FDebt asset balance too large\0\0\0\0`D\x82\x01R`d\x01a\x03IV[`@Qi\x06\xB6\x97GFV\xE77v\x17`\xB4\x1B` \x82\x01R`*\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x86`@Q` \x01a\t\xED\x92\x91\x90a/\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x0B\x94W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8C\x94P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nMW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nq\x91\x90a-^V[\x92Pa\x0B\x1A\x89`@Q\x80a\x01\0\x01`@R\x80\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8F`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x18\x11V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x90`\x01`\x01`\xA0\x1B\x03\x8E\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x83\x91\x90a-^V[a\x0B\x8D\x91\x90a/\xFBV[\x92Pa\x0E\xEBV[`@Qh\x06\x87\x97\x06W'7v\x17`\xBC\x1B` \x82\x01R`)\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x86`@Q` \x01a\x0B\xD5\x92\x91\x90a/\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\r\x02W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8C\x94P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CY\x91\x90a-^V[\x92Pa\x0B\x1A\x89`@Q\x80a\x01\0\x01`@R\x80\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8F`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x1AKV[`@Qo:\xB9\xB2<6#60\xB9\xB4&\xB4\xB7:2\xB9`\x81\x1B` \x82\x01R`0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x86`@Q` \x01a\rJ\x92\x91\x90a/\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x0E\xA3W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8B\x94P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a-^V[\x92Pa\x0Eu\x89`@Q\x80a\x01\0\x01`@R\x80\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8F`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x15\x15\x81RPa\x1B\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90cp\xA0\x821\x90`$\x01a\x0BDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid liquidation path\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03IV[PP\x98P\x98\x96PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x0F WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x0F<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a/\x9DV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xBE\x91\x90a/\x82V[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a0!V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x02` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x81\xE0 4At\x97,Y\xF6\xC1\x1A\x8Fl\x90\xB1A\x86b\x14\xE3\xD9\xB5D\xD00\xF0\xB52\xF5\xA1\x0F\x90a\x10A\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a0!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\xB3W`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x10\xAEW=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xAE\x91\x90a/\x82V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fmsg.sender != activeKittenPair\0\0`D\x82\x01R`d\x01a\x03IV[`@\x80Q`\x80\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x84Qa\x01\0\x81\x01\x86R``\x80\x82R\x91\x81\x01\x83\x90R\x94\x85\x01\x82\x90R\x80\x85\x01\x82\x90R\x91\x84\x01\x81\x90R`\xA0\x84\x01\x81\x90R`\xC0\x84\x01\x81\x90R`\xE0\x84\x01R\x81\x01\x91\x90\x91Ra\x11\xE4\x82\x84\x01\x84a.>V[``\x82\x01\x81\x90RQa\x11\xF5\x90a\x1CRV[\x15\x15`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x16\x81R``\x81\x01Q`\xC0\x01Q\x15a\x12\xA0W``\x80\x82\x01Q` \x81\x01Q`@\x80\x83\x01Q\x93\x83\x01Q`\x80\x90\x93\x01Q\x90Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x12r\x94\x93\x91\x92\x90\x91\x90_\x90`\x04\x01a/;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\x89W__\xFD[PZ\xF1\x15\x80\x15a\x12\x9BW=__>=_\xFD[PPPP[``\x81\x01QQa\x12\xAF\x90a\x1C\xA3V[\x15a\x13\x02W``\x81\x01QQa\x12\xC3\x90a\x1C\xDCV[``\x82\x01\x80Q\x91\x90\x91RQ`\xE0\x01Q\x15a\x12\xEFW``\x81\x01Q`\xA0\x81\x01Qa\x12\xEA\x91a\x18\x11V[a\x13\x02V[``\x81\x01Q`\xA0\x81\x01Qa\x13\x02\x91a\x15\xA2V[\x80``\x01Q`\xE0\x01Q\x15a\x13!W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R[\x80Q``\x82\x01Q`\xA0\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13xW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9C\x91\x90a/\x82V[PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03I\x90a0!V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\x14%\x82\x84\x01\x84a.>V[\x90P___a\x146\x84_\x01Qa\x1D\x13V[\x92P\x92P\x92Pa\x14ds\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x14_\x85\x85\x85a\x1DBV[a\x1D\xACV[\x83`\xC0\x01Q\x15a\x14\xE9W` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x14\xBB\x94\x91\x93\x91\x92_\x90`\x04\x01a/;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\xD2W__\xFD[PZ\xF1\x15\x80\x15a\x14\xE4W=__>=_\xFD[PPPP[__\x89\x13a\x14\xF7W\x87a\x14\xF9V[\x88[\x90Pa\x15\x07\x85_\x01Qa\x1E\0V[\x15a\x15#W\x84Qa\x15\x17\x90a\x1E\x1AV[\x85Ra\x15#\x81\x86a\x1AKV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x96\x91\x90a/\x82V[PPPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R\x81Qa\x15\xDE\x90a\x1CRV[\x15\x15`\xA0\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x85\x01\x81\x90R\x92\x90\x91\x16\x80\x84R`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92R`D\x82\x01Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x80\x91\x90a0GV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x16\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x03IV[`\xA0\x82\x01\x83\x90R` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90\x82\x16\x10`\x80\x80\x85\x01\x91\x90\x91R`@\x80Q\x91\x82\x01\x81R`\x01T\x83\x16\x82Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x93\x82\x01\x93\x90\x93R\x91\x82\x01\x85\x90R\x82Q\x16``\x82\x01Ra\x17K\x90a\x1E7V[`@\x82\x01R`\x01T`\x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02,\r\x9F\x90a\x17xW\x82`@\x01Qa\x17zV[_[\x83`\x80\x01Qa\x17\x89W_a\x17\x8FV[\x83`@\x01Q[0\x86`@Q` \x01a\x17\xA1\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xCF\x94\x93\x92\x91\x90a1,V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xE6W__\xFD[PZ\xF1\x15\x80\x15a\x17\xF8W=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x81Qa\x188\x90a\x1CRV[\x15\x15`@\x84\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x86\x01\x81\x90R\x93\x90\x92\x16\x80\x85R\x91Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x83\x90R`D\x81\x01\x91\x90\x91R\x91\x10\x15\x90s\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xBBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xDF\x91\x90a0GV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x19@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x03IV[`@\x80Q`\x80\x80\x82\x01\x83R`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B` \x80\x85\x01\x91\x90\x91R\x91\x87\x01Q\x93\x83\x01\x93\x90\x93R\x84\x01Q\x90\x91\x16``\x82\x01Ra\x19\x98\x90a\x1E\xC2V[`\xA0\x84\x01R`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x82a\x19\xB9W\x85a\x19\xBBV[_[\x83a\x19\xC6W_a\x19\xC8V[\x86[0\x87`@Q` \x01a\x19\xDA\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x08\x94\x93\x92\x91\x90a1,V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\x1FW__\xFD[PZ\xF1\x15\x80\x15a\x1A1W=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPPV[___a\x1AZ\x84_\x01Qa\x1D\x13V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\x1A\x99s\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x1A\x94\x86\x88\x87a\x1DBV[a CV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xFB\x91\x90a0GV[P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x1B\x16\x8Ba1XV[\x86a\x1B?Wa\x1B:`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a1rV[a\x1BOV[a\x1BOd\x01\0\x02v\xA3`\x01a1\x91V[\x8B`@Q` \x01a\x1B`\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x8F\x95\x94\x93\x92\x91\x90a1\xB0V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xCE\x91\x90a1\xF5V[PPPPPPPPPV[s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x01`\x01`\xA0\x1B\x03\x16c\\\xFF\xE9\xDE0s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x85\x85`@Q` \x01a\x1C$\x91\x90a0\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xE2\x94\x93\x92\x91\x90a2\x17V[_\x80\x80a\x1C_\x84\x82a!)V[\x92Pa\x1C\x82`\x01a\x1Cq\x81`\x14a/(V[a\x1C{\x91\x90a/oV[\x85\x90a!\x8DV[\x90Pa\x1C\x9Aa\x1C\x93`\x01`\x14a/(V[\x85\x90a!)V[\x91P\x91\x93\x90\x92PV[_a\x1C\xB0`\x01`\x14a/(V[`\x14a\x1C\xBD`\x01\x82a/(V[a\x1C\xC7\x91\x90a/(V[a\x1C\xD1\x91\x90a/(V[\x82Q\x10\x15\x90P\x91\x90PV[``a\x1D\ra\x1C\xED`\x01`\x14a/(V[a\x1C\xF9`\x01`\x14a/(V[\x84Qa\x1D\x05\x91\x90a/oV[\x84\x91\x90a!\xFEV[\x92\x91PPV[_\x80\x80a\x1D \x84\x82a!)V[\x92Pa\x1D-\x84`\x14a#\nV[a\xFF\xFF\x16\x90Pa\x1C\x9Aa\x1C\x93`\x03`\x14a/(V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x1D|W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x1D\xB7\x83\x83a CV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x10\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x03IV[_a\x1E\r`\x03`\x14a/(V[`\x14a\x1C\xBD`\x03\x82a/(V[``a\x1D\ra\x1E+`\x03`\x14a/(V[a\x1C\xF9`\x03`\x14a/(V[\x80Q`@\x80\x83\x01Q``\x84\x01Q\x91Qcx\xA0Q\xAD`\xE1\x1B\x81R_\x93`\x01`\x01`\xA0\x1B\x03\x16\x92c\xF1@\xA3Z\x92a\x1E\x83\x92\x90\x91\x90`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\r\x91\x90a-^V[`@\x80Q`\xE0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c9/7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\\\x91\x90a2IV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01R\x15\x15`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x81R_a\x1F\x95\x84\x83a#\xB4V[` \x85\x01Q\x85Q`\x80\x85\x01Q`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x15\x15`$\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x16\x91\x90a-^V[\x90Pa $\x81a'\x10a/oV[a 0\x83a'\x10a2\xBAV[a :\x91\x90a2\xD1V[\x95\x94PPPPPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a hW__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\x1E\xEDC\xDC\xAA.\xFD\xE0g.\xB5qd\x92\0\xA2\x927\xB7\x95\x8E{\x0F\xBDR\xF7_\xA3[~\xC5,`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a!5\x82`\x14a/(V[\x83Q\x10\x15a!}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x03IV[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_a!\x99\x82`\x01a/(V[\x83Q\x10\x15a!\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoBool_outOfBounds`p\x1B`D\x82\x01R`d\x01a\x03IV[\x82\x82\x01`\x01\x01Q`\xFF\x81\x16a!\xF3W_a!\xF6V[`\x01[\x94\x93PPPPV[``\x81a\"\x0C\x81`\x1Fa/(V[\x10\x15a\"KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x03IV[a\"U\x82\x84a/(V[\x84Q\x10\x15a\"\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x03IV[``\x82\x15\x80\x15a\"\xB7W`@Q\x91P_\x82R` \x82\x01`@Ra#\x01V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\"\xF0W\x80Q\x83R` \x92\x83\x01\x92\x01a\"\xD8V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[_\x81a#\x17\x81`\x03a/(V[\x10\x15a#YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x03IV[a#d\x82`\x03a/(V[\x83Q\x10\x15a#\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x03IV[P\x01`\x03\x01Q\x90V[_\x81`\x80\x01Q\x15a%\xCBW_a$\x04`@Q\x80`\xA0\x01`@R\x80\x85`@\x01Q\x81R` \x01\x85``\x01Q\x81R` \x01\x85`\x80\x01Q\x15\x15\x81R` \x01\x85_\x01Q\x81R` \x01\x85` \x01Q\x81RPa&?V[\x83Q`@\x85\x01Q\x91\x92P\x90a$!\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a$+\x91\x90a2\xD1V[`@\x84\x01R` \x83\x01Q``\x84\x01Qa$L\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a$V\x91\x90a2\xD1V[\x83``\x01\x81\x81RPP__\x84`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x91W\x84``\x01Q\x85`@\x01Qa$\x9CV[\x84`@\x01Q\x85``\x01Q[\x91P\x91P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a$\xECW` \x85\x01Q`@\x87\x01Qa$\xDD\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a$\xE7\x91\x90a2\xD1V[a%\x0FV[\x84Q`@\x87\x01Qa%\x05\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a%\x0F\x91\x90a2\xD1V[`@\x87\x01\x81\x90R_\x90a%\"\x90\x83a/oV[\x90P_\x83a%h`@Q\x80`\xC0\x01`@R\x80\x85\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x80\x01Q\x15\x15\x81R` \x01\x8A_\x01Q\x81R` \x01\x8A` \x01Q\x81RPa'@V[a%r\x91\x90a/oV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a%\xA5W\x87Qa%\xABV[\x87` \x01Q[a%\xB5\x90\x83a2\xBAV[a%\xBF\x91\x90a2\xD1V[\x95PPPPPPa\x1D\rV[__\x83`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a%\xFDW\x83``\x01Q\x84`@\x01Qa&\x08V[\x83`@\x01Q\x84``\x01Q[\x91P\x91P\x84`@\x01Q\x81a&\x1C\x91\x90a/oV[\x82\x86`@\x01Qa&,\x91\x90a2\xBAV[a&6\x91\x90a2\xD1V[\x92PPPa\x1D\rV[_\x81`@\x01Q\x15a'*W``\x82\x01Q\x82Q_\x91\x90a&f\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a&p\x91\x90a2\xD1V[\x90P_\x83`\x80\x01Q\x84` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a&\x90\x91\x90a2\xBAV[a&\x9A\x91\x90a2\xD1V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a&\xB0\x83\x85a2\xBAV[a&\xBA\x91\x90a2\xD1V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a&\xD0\x84\x80a2\xBAV[a&\xDA\x91\x90a2\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0a&\xED\x86\x80a2\xBAV[a&\xF7\x91\x90a2\xD1V[a'\x01\x91\x90a/(V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a'\x16\x82\x84a2\xBAV[a' \x91\x90a2\xD1V[\x96\x95PPPPPPV[` \x82\x01Q\x82Qa\x1D\r\x91\x90a2\xBAV[\x91\x90PV[_\x80[`\xFF\x81\x10\x15a(\xFCW_a'^\x84`@\x01Q\x85_\x01Qa)*V[\x90P\x83` \x01Q\x81\x10\x15a(SW_a'~\x85`@\x01Q\x86_\x01Qa)\xA5V[\x82\x86` \x01Qa'\x8E\x91\x90a/oV[a'\xA0\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a'\xAA\x91\x90a2\xD1V[\x90P\x80_\x03a(8W\x84` \x01Q\x82\x03a'\xC9WPPPP`@\x01Q\x90V[\x84` \x01Qa(\x1E`@Q\x80`\xA0\x01`@R\x80\x88`@\x01Q`\x01a'\xED\x91\x90a/(V[\x81R` \x01\x88_\x01Q\x81R` \x01\x88``\x01Q\x15\x15\x81R` \x01\x88`\x80\x01Q\x81R` \x01\x88`\xA0\x01Q\x81RPa&?V[\x11\x15a(4W`@\x85\x01Qa :\x90`\x01a/(V[P`\x01[\x80\x85`@\x01Qa(H\x91\x90a/(V[`@\x86\x01RPa(\xF3V[_a(e\x85`@\x01Q\x86_\x01Qa)\xA5V[` \x86\x01Qa(t\x90\x84a/oV[a(\x86\x90g\r\xE0\xB6\xB3\xA7d\0\0a2\xBAV[a(\x90\x91\x90a2\xD1V[\x90P\x80_\x03a(\xDCW\x84` \x01Q\x82\x14\x80a(\xC8WP\x84` \x01Qa(\xC6`\x01\x87`@\x01Qa(\xBF\x91\x90a/oV[\x87Qa)*V[\x10[\x15a(\xD8WPPPP`@\x01Q\x90V[P`\x01[\x80\x85`@\x01Qa(\xEC\x91\x90a/oV[`@\x86\x01RP[P`\x01\x01a'CV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04/`\xF3\x1B`D\x82\x01R`d\x01a\x03IV[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a)?\x84\x86a2\xBAV[a)I\x91\x90a2\xD1V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a)_\x85\x80a2\xBAV[a)i\x91\x90a2\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0a)|\x87\x80a2\xBAV[a)\x86\x91\x90a2\xD1V[a)\x90\x91\x90a/(V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a 0\x82\x84a2\xBAV[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a)\xBB\x82\x80a2\xBAV[a)\xC5\x91\x90a2\xD1V[a)\xCF\x91\x90a2\xBAV[a)\xD9\x91\x90a2\xD1V[g\r\xE0\xB6\xB3\xA7d\0\0\x80a)\xED\x85\x80a2\xBAV[a)\xF7\x91\x90a2\xD1V[a*\x02\x86`\x03a2\xBAV[a*\x0C\x91\x90a2\xBAV[a*\x16\x91\x90a2\xD1V[a* \x91\x90a/(V[\x93\x92PPPV[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01a*\x84`@\x80Qa\x01\0\x81\x01\x82R``\x80\x82R_` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a*\x9DW__\xFD[PV[\x805a';\x81a*\x89V[__\x83`\x1F\x84\x01\x12a*\xBBW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xD2W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a*\xE9W__\xFD[\x92P\x92\x90PV[______`\xA0\x87\x89\x03\x12\x15a+\x05W__\xFD[\x865a+\x10\x81a*\x89V[\x95P` \x87\x015a+ \x81a*\x89V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+IW__\xFD[a+U\x89\x82\x8A\x01a*\xABV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[________`\xC0\x89\x8B\x03\x12\x15a+~W__\xFD[\x885a+\x89\x81a*\x89V[\x97P` \x89\x015a+\x99\x81a*\x89V[\x96P`@\x89\x015a+\xA9\x81a*\x89V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xCBW__\xFD[a+\xD7\x8B\x82\x8C\x01a*\xABV[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xF6W__\xFD[a,\x02\x8B\x82\x8C\x01a*\xABV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a,&W__\xFD[\x815a* \x81a*\x89V[\x80\x15\x15\x81\x14a*\x9DW__\xFD[\x805a';\x81a,1V[__`@\x83\x85\x03\x12\x15a,ZW__\xFD[\x825a,e\x81a*\x89V[\x91P` \x83\x015a,u\x81a,1V[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a,\x91W__\xFD[\x825a,\x9C\x81a*\x89V[\x94` \x93\x90\x93\x015\x93PPPV[_____`\x80\x86\x88\x03\x12\x15a,\xBEW__\xFD[\x855a,\xC9\x81a*\x89V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xF2W__\xFD[a,\xFE\x88\x82\x89\x01a*\xABV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15a-\"W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-FW__\xFD[a-R\x87\x82\x88\x01a*\xABV[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a-nW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-\xADWa-\xADa-uV[`@R\x90V[_\x82`\x1F\x83\x01\x12a-\xC2W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xDCWa-\xDCa-uV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.\x0BWa.\x0Ba-uV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a.\"W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a.NW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.dW__\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a.vW__\xFD[a.~a-\x89V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\x94W__\xFD[a.\xA0\x86\x82\x85\x01a-\xB3V[\x82RPa.\xAF` \x83\x01a*\xA0V[` \x82\x01Ra.\xC0`@\x83\x01a*\xA0V[`@\x82\x01Ra.\xD1``\x83\x01a*\xA0V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x80\x83\x015\x90\x82\x01Ra.\xF6`\xC0\x83\x01a,>V[`\xC0\x82\x01Ra/\x07`\xE0\x83\x01a,>V[`\xE0\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1D\rWa\x1D\ra/\x14V[`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16`@\x83\x01R``\x82\x01\x92\x90\x92R\x90\x15\x15`\x80\x82\x01R`\xA0\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1D\rWa\x1D\ra/\x14V[_` \x82\x84\x03\x12\x15a/\x92W__\xFD[\x81Qa* \x81a,1V[` \x80\x82R`/\x90\x82\x01R\x7FOnly owner or liquidator can cal`@\x82\x01Rn6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a0\x1AWa0\x1Aa/\x14V[P\x92\x91PPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a0WW__\xFD[\x81Qa* \x81a*\x89V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Qa\x01\0` \x84\x01Ra0\xADa\x01 \x84\x01\x82a0bV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R``\x84\x01Qa0\xED`\x80\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R`\xC0\x84\x01Qa1\x15`\xE0\x85\x01\x82\x15\x15\x90RV[P`\xE0\x84\x01Q\x80\x15\x15a\x01\0\x85\x01RP\x93\x92PPPV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_a' `\x80\x83\x01\x84a0bV[_`\x01`\xFF\x1B\x82\x01a1lWa1la/\x14V[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1D\rWa\x1D\ra/\x14V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1D\rWa\x1D\ra/\x14V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a1\xEA\x90\x83\x01\x84a0bV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a2\x06W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a' \x90\x83\x01\x84a0bV[_______`\xE0\x88\x8A\x03\x12\x15a2_W__\xFD[\x87Q` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q\x93\x9AP\x91\x98P\x96P\x94Pa2\x88\x81a,1V[`\xA0\x89\x01Q\x90\x93Pa2\x99\x81a*\x89V[`\xC0\x89\x01Q\x90\x92Pa2\xAA\x81a*\x89V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1D\rWa\x1D\ra/\x14V[_\x82a2\xEBWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x8D\x99\xAA\xC4\x18\xDC(\xD1\x0F\xB1\xFD\x86\xFC,\x0C\x08dz\x1F(v\x10^\xE2\x9F\":y\xC2\x11\xA2\x07dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CALLBACK_SUCCESS` (0x8237e538) function
        pub fn callback_success(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([130, 55, 229, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FLASH_MINTER` (0xea939fa6) function
        pub fn flash_minter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 147, 159, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `USDXL` (0x70c26a5e) function
        pub fn usdxl(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([112, 194, 106, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvePool` (0x424c265b) function
        pub fn approve_pool(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 76, 38, 91], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hook` (0x9a7bff79) function
        pub fn hook(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 123, 255, 121], (p0, p1, p2, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hyperswapV3Factory` (0x3e0d955a) function
        pub fn hyperswap_v3_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([62, 13, 149, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isLiquidator` (0x529a356f) function
        pub fn is_liquidator(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 154, 53, 111], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kittenPairFactory` (0x08bea127) function
        pub fn kitten_pair_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 190, 161, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x2e43c961) function
        pub fn liquidate(
            &self,
            collateral_asset: ::ethers::core::types::Address,
            debt_asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            swap_path: ::ethers::core::types::Bytes,
            liq_path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash(
                    [46, 67, 201, 97],
                    (
                        collateral_asset,
                        debt_asset,
                        user,
                        debt_to_cover,
                        swap_path,
                        liq_path,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onFlashLoan` (0x23e30c8b) function
        pub fn on_flash_loan(
            &self,
            initiator: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            fee: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([35, 227, 12, 139], (initiator, token, amount, fee, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pool` (0x16f0115b) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 240, 17, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recover` (0x5705ae43) function
        pub fn recover(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 5, 174, 67], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLiquidator` (0x4453a374) function
        pub fn set_liquidator(
            &self,
            liquidator: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 83, 163, 116], (liquidator, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LiquidatorSet` event
        pub fn liquidator_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidatorSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LiquidatorSet", abi = "LiquidatorSet(address,bool)")]
    pub struct LiquidatorSetFilter {
        #[ethevent(indexed)]
        pub liquidator: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorEvents {
        LiquidatorSetFilter(LiquidatorSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for LiquidatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LiquidatorSetFilter::decode_log(log) {
                return Ok(LiquidatorEvents::LiquidatorSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LiquidatorEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LiquidatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LiquidatorSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<LiquidatorSetFilter> for LiquidatorEvents {
        fn from(value: LiquidatorSetFilter) -> Self {
            Self::LiquidatorSetFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LiquidatorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `CALLBACK_SUCCESS` function with signature `CALLBACK_SUCCESS()` and selector `0x8237e538`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "CALLBACK_SUCCESS", abi = "CALLBACK_SUCCESS()")]
    pub struct CallbackSuccessCall;
    ///Container type for all input parameters for the `FLASH_MINTER` function with signature `FLASH_MINTER()` and selector `0xea939fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "FLASH_MINTER", abi = "FLASH_MINTER()")]
    pub struct FlashMinterCall;
    ///Container type for all input parameters for the `USDXL` function with signature `USDXL()` and selector `0x70c26a5e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "USDXL", abi = "USDXL()")]
    pub struct UsdxlCall;
    ///Container type for all input parameters for the `approvePool` function with signature `approvePool(address)` and selector `0x424c265b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approvePool", abi = "approvePool(address)")]
    pub struct ApprovePoolCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hook` function with signature `hook(address,uint256,uint256,bytes)` and selector `0x9a7bff79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hook", abi = "hook(address,uint256,uint256,bytes)")]
    pub struct HookCall {
        pub p0: ::ethers::core::types::Address,
        pub p1: ::ethers::core::types::U256,
        pub p2: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hyperswapV3Factory` function with signature `hyperswapV3Factory()` and selector `0x3e0d955a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hyperswapV3Factory", abi = "hyperswapV3Factory()")]
    pub struct HyperswapV3FactoryCall;
    ///Container type for all input parameters for the `isLiquidator` function with signature `isLiquidator(address)` and selector `0x529a356f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isLiquidator", abi = "isLiquidator(address)")]
    pub struct IsLiquidatorCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `kittenPairFactory` function with signature `kittenPairFactory()` and selector `0x08bea127`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "kittenPairFactory", abi = "kittenPairFactory()")]
    pub struct KittenPairFactoryCall;
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,address,address,uint256,bytes,string)` and selector `0x2e43c961`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "liquidate",
        abi = "liquidate(address,address,address,uint256,bytes,string)"
    )]
    pub struct LiquidateCall {
        pub collateral_asset: ::ethers::core::types::Address,
        pub debt_asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub swap_path: ::ethers::core::types::Bytes,
        pub liq_path: ::std::string::String,
    }
    ///Container type for all input parameters for the `onFlashLoan` function with signature `onFlashLoan(address,address,uint256,uint256,bytes)` and selector `0x23e30c8b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onFlashLoan",
        abi = "onFlashLoan(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnFlashLoanCall {
        pub initiator: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pool` function with signature `pool()` and selector `0x16f0115b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pool", abi = "pool()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `recover` function with signature `recover(address,uint256)` and selector `0x5705ae43`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recover", abi = "recover(address,uint256)")]
    pub struct RecoverCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setLiquidator` function with signature `setLiquidator(address,bool)` and selector `0x4453a374`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setLiquidator", abi = "setLiquidator(address,bool)")]
    pub struct SetLiquidatorCall {
        pub liquidator: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorCalls {
        CallbackSuccess(CallbackSuccessCall),
        FlashMinter(FlashMinterCall),
        Usdxl(UsdxlCall),
        ApprovePool(ApprovePoolCall),
        Hook(HookCall),
        HyperswapV3Factory(HyperswapV3FactoryCall),
        IsLiquidator(IsLiquidatorCall),
        KittenPairFactory(KittenPairFactoryCall),
        Liquidate(LiquidateCall),
        OnFlashLoan(OnFlashLoanCall),
        Owner(OwnerCall),
        Pool(PoolCall),
        Recover(RecoverCall),
        SetLiquidator(SetLiquidatorCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CallbackSuccessCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallbackSuccess(decoded));
            }
            if let Ok(decoded) = <FlashMinterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FlashMinter(decoded));
            }
            if let Ok(decoded) = <UsdxlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Usdxl(decoded));
            }
            if let Ok(decoded) = <ApprovePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApprovePool(decoded));
            }
            if let Ok(decoded) = <HookCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Hook(decoded));
            }
            if let Ok(decoded) = <HyperswapV3FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HyperswapV3Factory(decoded));
            }
            if let Ok(decoded) = <IsLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsLiquidator(decoded));
            }
            if let Ok(decoded) = <KittenPairFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KittenPairFactory(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <OnFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnFlashLoan(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) = <RecoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Recover(decoded));
            }
            if let Ok(decoded) = <SetLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetLiquidator(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CallbackSuccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Usdxl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApprovePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Hook(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HyperswapV3Factory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KittenPairFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Recover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallbackSuccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashMinter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Usdxl(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hook(element) => ::core::fmt::Display::fmt(element, f),
                Self::HyperswapV3Factory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsLiquidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::KittenPairFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLiquidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CallbackSuccessCall> for LiquidatorCalls {
        fn from(value: CallbackSuccessCall) -> Self {
            Self::CallbackSuccess(value)
        }
    }
    impl ::core::convert::From<FlashMinterCall> for LiquidatorCalls {
        fn from(value: FlashMinterCall) -> Self {
            Self::FlashMinter(value)
        }
    }
    impl ::core::convert::From<UsdxlCall> for LiquidatorCalls {
        fn from(value: UsdxlCall) -> Self {
            Self::Usdxl(value)
        }
    }
    impl ::core::convert::From<ApprovePoolCall> for LiquidatorCalls {
        fn from(value: ApprovePoolCall) -> Self {
            Self::ApprovePool(value)
        }
    }
    impl ::core::convert::From<HookCall> for LiquidatorCalls {
        fn from(value: HookCall) -> Self {
            Self::Hook(value)
        }
    }
    impl ::core::convert::From<HyperswapV3FactoryCall> for LiquidatorCalls {
        fn from(value: HyperswapV3FactoryCall) -> Self {
            Self::HyperswapV3Factory(value)
        }
    }
    impl ::core::convert::From<IsLiquidatorCall> for LiquidatorCalls {
        fn from(value: IsLiquidatorCall) -> Self {
            Self::IsLiquidator(value)
        }
    }
    impl ::core::convert::From<KittenPairFactoryCall> for LiquidatorCalls {
        fn from(value: KittenPairFactoryCall) -> Self {
            Self::KittenPairFactory(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OnFlashLoanCall> for LiquidatorCalls {
        fn from(value: OnFlashLoanCall) -> Self {
            Self::OnFlashLoan(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<RecoverCall> for LiquidatorCalls {
        fn from(value: RecoverCall) -> Self {
            Self::Recover(value)
        }
    }
    impl ::core::convert::From<SetLiquidatorCall> for LiquidatorCalls {
        fn from(value: SetLiquidatorCall) -> Self {
            Self::SetLiquidator(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for LiquidatorCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `CALLBACK_SUCCESS` function with signature `CALLBACK_SUCCESS()` and selector `0x8237e538`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CallbackSuccessReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FLASH_MINTER` function with signature `FLASH_MINTER()` and selector `0xea939fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FlashMinterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `USDXL` function with signature `USDXL()` and selector `0x70c26a5e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UsdxlReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hyperswapV3Factory` function with signature `hyperswapV3Factory()` and selector `0x3e0d955a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HyperswapV3FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isLiquidator` function with signature `isLiquidator(address)` and selector `0x529a356f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsLiquidatorReturn(pub bool);
    ///Container type for all return fields from the `kittenPairFactory` function with signature `kittenPairFactory()` and selector `0x08bea127`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct KittenPairFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(address,address,address,uint256,bytes,string)` and selector `0x2e43c961`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidateReturn {
        pub final_token: ::ethers::core::types::Address,
        pub final_gain: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `onFlashLoan` function with signature `onFlashLoan(address,address,uint256,uint256,bytes)` and selector `0x23e30c8b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnFlashLoanReturn {
        pub success: [u8; 32],
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pool` function with signature `pool()` and selector `0x16f0115b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolReturn(pub ::ethers::core::types::Address);
}
