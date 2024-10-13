///Module containing a contract's types and functions.
/**

```solidity
library DataTypes {
    type InterestRateMode is uint8;
    struct EModeCategory { uint16 ltv; uint16 liquidationThreshold; uint16 liquidationBonus; address priceSource; string label; }
    struct ReserveConfigurationMap { uint256 data; }
    struct ReserveData { ReserveConfigurationMap configuration; uint128 liquidityIndex; uint128 currentLiquidityRate; uint128 variableBorrowIndex; uint128 currentVariableBorrowRate; uint128 currentStableBorrowRate; uint40 lastUpdateTimestamp; uint16 id; address aTokenAddress; address stableDebtTokenAddress; address variableDebtTokenAddress; address interestRateStrategyAddress; uint128 accruedToTreasury; uint128 unbacked; uint128 isolationModeTotalDebt; }
    struct UserConfigurationMap { uint256 data; }
}
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod DataTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InterestRateMode(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<InterestRateMode> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl InterestRateMode {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for InterestRateMode {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for InterestRateMode {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct EModeCategory { uint16 ltv; uint16 liquidationThreshold; uint16 liquidationBonus; address priceSource; string label; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct EModeCategory {
        pub ltv: u16,
        pub liquidationThreshold: u16,
        pub liquidationBonus: u16,
        pub priceSource: alloy::sol_types::private::Address,
        pub label: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u16,
            u16,
            u16,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EModeCategory> for UnderlyingRustTuple<'_> {
            fn from(value: EModeCategory) -> Self {
                (
                    value.ltv,
                    value.liquidationThreshold,
                    value.liquidationBonus,
                    value.priceSource,
                    value.label,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EModeCategory {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ltv: tuple.0,
                    liquidationThreshold: tuple.1,
                    liquidationBonus: tuple.2,
                    priceSource: tuple.3,
                    label: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for EModeCategory {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for EModeCategory {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.ltv),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidationThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidationBonus),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.priceSource,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.label,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for EModeCategory {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for EModeCategory {
            const NAME: &'static str = "EModeCategory";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "EModeCategory(uint16 ltv,uint16 liquidationThreshold,uint16 liquidationBonus,address priceSource,string label)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ltv)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidationThreshold,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidationBonus,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.priceSource,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.label,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for EModeCategory {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.ltv)
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidationThreshold,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidationBonus,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.priceSource,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.label,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.ltv, out);
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidationThreshold,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidationBonus,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.priceSource,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.label,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct ReserveConfigurationMap { uint256 data; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ReserveConfigurationMap {
        pub data: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReserveConfigurationMap> for UnderlyingRustTuple<'_> {
            fn from(value: ReserveConfigurationMap) -> Self {
                (value.data,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReserveConfigurationMap {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { data: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ReserveConfigurationMap {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ReserveConfigurationMap {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ReserveConfigurationMap {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ReserveConfigurationMap {
            const NAME: &'static str = "ReserveConfigurationMap";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ReserveConfigurationMap(uint256 data)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.data)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ReserveConfigurationMap {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.data)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct ReserveData { ReserveConfigurationMap configuration; uint128 liquidityIndex; uint128 currentLiquidityRate; uint128 variableBorrowIndex; uint128 currentVariableBorrowRate; uint128 currentStableBorrowRate; uint40 lastUpdateTimestamp; uint16 id; address aTokenAddress; address stableDebtTokenAddress; address variableDebtTokenAddress; address interestRateStrategyAddress; uint128 accruedToTreasury; uint128 unbacked; uint128 isolationModeTotalDebt; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ReserveData {
        pub configuration: <ReserveConfigurationMap as alloy::sol_types::SolType>::RustType,
        pub liquidityIndex: u128,
        pub currentLiquidityRate: u128,
        pub variableBorrowIndex: u128,
        pub currentVariableBorrowRate: u128,
        pub currentStableBorrowRate: u128,
        pub lastUpdateTimestamp: alloy::sol_types::private::primitives::aliases::U40,
        pub id: u16,
        pub aTokenAddress: alloy::sol_types::private::Address,
        pub stableDebtTokenAddress: alloy::sol_types::private::Address,
        pub variableDebtTokenAddress: alloy::sol_types::private::Address,
        pub interestRateStrategyAddress: alloy::sol_types::private::Address,
        pub accruedToTreasury: u128,
        pub unbacked: u128,
        pub isolationModeTotalDebt: u128,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            ReserveConfigurationMap,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<40>,
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <ReserveConfigurationMap as alloy::sol_types::SolType>::RustType,
            u128,
            u128,
            u128,
            u128,
            u128,
            alloy::sol_types::private::primitives::aliases::U40,
            u16,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            u128,
            u128,
            u128,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReserveData> for UnderlyingRustTuple<'_> {
            fn from(value: ReserveData) -> Self {
                (
                    value.configuration,
                    value.liquidityIndex,
                    value.currentLiquidityRate,
                    value.variableBorrowIndex,
                    value.currentVariableBorrowRate,
                    value.currentStableBorrowRate,
                    value.lastUpdateTimestamp,
                    value.id,
                    value.aTokenAddress,
                    value.stableDebtTokenAddress,
                    value.variableDebtTokenAddress,
                    value.interestRateStrategyAddress,
                    value.accruedToTreasury,
                    value.unbacked,
                    value.isolationModeTotalDebt,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReserveData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    configuration: tuple.0,
                    liquidityIndex: tuple.1,
                    currentLiquidityRate: tuple.2,
                    variableBorrowIndex: tuple.3,
                    currentVariableBorrowRate: tuple.4,
                    currentStableBorrowRate: tuple.5,
                    lastUpdateTimestamp: tuple.6,
                    id: tuple.7,
                    aTokenAddress: tuple.8,
                    stableDebtTokenAddress: tuple.9,
                    variableDebtTokenAddress: tuple.10,
                    interestRateStrategyAddress: tuple.11,
                    accruedToTreasury: tuple.12,
                    unbacked: tuple.13,
                    isolationModeTotalDebt: tuple.14,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ReserveData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ReserveData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <ReserveConfigurationMap as alloy_sol_types::SolType>::tokenize(
                        &self.configuration,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityIndex),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentLiquidityRate),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.variableBorrowIndex),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.currentVariableBorrowRate,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.currentStableBorrowRate,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastUpdateTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.aTokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.stableDebtTokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.variableDebtTokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.interestRateStrategyAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.accruedToTreasury),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.unbacked),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.isolationModeTotalDebt,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ReserveData {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ReserveData {
            const NAME: &'static str = "ReserveData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ReserveData(ReserveConfigurationMap configuration,uint128 liquidityIndex,uint128 currentLiquidityRate,uint128 variableBorrowIndex,uint128 currentVariableBorrowRate,uint128 currentStableBorrowRate,uint40 lastUpdateTimestamp,uint16 id,address aTokenAddress,address stableDebtTokenAddress,address variableDebtTokenAddress,address interestRateStrategyAddress,uint128 accruedToTreasury,uint128 unbacked,uint128 isolationModeTotalDebt)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <ReserveConfigurationMap as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ReserveConfigurationMap as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <ReserveConfigurationMap as alloy_sol_types::SolType>::eip712_data_word(
                            &self.configuration,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidityIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currentLiquidityRate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.variableBorrowIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currentVariableBorrowRate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currentStableBorrowRate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.lastUpdateTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aTokenAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stableDebtTokenAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.variableDebtTokenAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.interestRateStrategyAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.accruedToTreasury,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.unbacked)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isolationModeTotalDebt,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ReserveData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <ReserveConfigurationMap as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.configuration,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currentLiquidityRate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.variableBorrowIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currentVariableBorrowRate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currentStableBorrowRate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lastUpdateTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aTokenAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stableDebtTokenAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.variableDebtTokenAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.interestRateStrategyAddress,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.accruedToTreasury,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.unbacked,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isolationModeTotalDebt,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <ReserveConfigurationMap as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.configuration,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currentLiquidityRate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.variableBorrowIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currentVariableBorrowRate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currentStableBorrowRate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    40,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lastUpdateTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.aTokenAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stableDebtTokenAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.variableDebtTokenAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.interestRateStrategyAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.accruedToTreasury,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.unbacked,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isolationModeTotalDebt,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct UserConfigurationMap { uint256 data; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct UserConfigurationMap {
        pub data: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UserConfigurationMap> for UnderlyingRustTuple<'_> {
            fn from(value: UserConfigurationMap) -> Self {
                (value.data,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UserConfigurationMap {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { data: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UserConfigurationMap {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UserConfigurationMap {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for UserConfigurationMap {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for UserConfigurationMap {
            const NAME: &'static str = "UserConfigurationMap";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UserConfigurationMap(uint256 data)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.data)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UserConfigurationMap {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.data)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DataTypes`](self) contract instance.

See the [wrapper's documentation](`DataTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DataTypesInstance<T, P, N> {
        DataTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`DataTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DataTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DataTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DataTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DataTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DataTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`DataTypes`](self) contract instance.

See the [wrapper's documentation](`DataTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> DataTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DataTypesInstance<T, P, N> {
            DataTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DataTypesInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DataTypesInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library DataTypes {
    type InterestRateMode is uint8;
    struct EModeCategory {
        uint16 ltv;
        uint16 liquidationThreshold;
        uint16 liquidationBonus;
        address priceSource;
        string label;
    }
    struct ReserveConfigurationMap {
        uint256 data;
    }
    struct ReserveData {
        ReserveConfigurationMap configuration;
        uint128 liquidityIndex;
        uint128 currentLiquidityRate;
        uint128 variableBorrowIndex;
        uint128 currentVariableBorrowRate;
        uint128 currentStableBorrowRate;
        uint40 lastUpdateTimestamp;
        uint16 id;
        address aTokenAddress;
        address stableDebtTokenAddress;
        address variableDebtTokenAddress;
        address interestRateStrategyAddress;
        uint128 accruedToTreasury;
        uint128 unbacked;
        uint128 isolationModeTotalDebt;
    }
    struct UserConfigurationMap {
        uint256 data;
    }
}

interface IPool {
    event BackUnbacked(address indexed reserve, address indexed backer, uint256 amount, uint256 fee);
    event Borrow(address indexed reserve, address user, address indexed onBehalfOf, uint256 amount, DataTypes.InterestRateMode interestRateMode, uint256 borrowRate, uint16 indexed referralCode);
    event FlashLoan(address indexed target, address initiator, address indexed asset, uint256 amount, DataTypes.InterestRateMode interestRateMode, uint256 premium, uint16 indexed referralCode);
    event IsolationModeTotalDebtUpdated(address indexed asset, uint256 totalDebt);
    event LiquidationCall(address indexed collateralAsset, address indexed debtAsset, address indexed user, uint256 debtToCover, uint256 liquidatedCollateralAmount, address liquidator, bool receiveAToken);
    event MintUnbacked(address indexed reserve, address user, address indexed onBehalfOf, uint256 amount, uint16 indexed referralCode);
    event MintedToTreasury(address indexed reserve, uint256 amountMinted);
    event RebalanceStableBorrowRate(address indexed reserve, address indexed user);
    event Repay(address indexed reserve, address indexed user, address indexed repayer, uint256 amount, bool useATokens);
    event ReserveDataUpdated(address indexed reserve, uint256 liquidityRate, uint256 stableBorrowRate, uint256 variableBorrowRate, uint256 liquidityIndex, uint256 variableBorrowIndex);
    event ReserveUsedAsCollateralDisabled(address indexed reserve, address indexed user);
    event ReserveUsedAsCollateralEnabled(address indexed reserve, address indexed user);
    event Supply(address indexed reserve, address user, address indexed onBehalfOf, uint256 amount, uint16 indexed referralCode);
    event SwapBorrowRateMode(address indexed reserve, address indexed user, DataTypes.InterestRateMode interestRateMode);
    event UserEModeSet(address indexed user, uint8 categoryId);
    event Withdraw(address indexed reserve, address indexed user, address indexed to, uint256 amount);

    function ADDRESSES_PROVIDER() external view returns (address);
    function BRIDGE_PROTOCOL_FEE() external view returns (uint256);
    function FLASHLOAN_PREMIUM_TOTAL() external view returns (uint128);
    function FLASHLOAN_PREMIUM_TO_PROTOCOL() external view returns (uint128);
    function MAX_NUMBER_RESERVES() external view returns (uint16);
    function MAX_STABLE_RATE_BORROW_SIZE_PERCENT() external view returns (uint256);
    function backUnbacked(address asset, uint256 amount, uint256 fee) external returns (uint256);
    function borrow(address asset, uint256 amount, uint256 interestRateMode, uint16 referralCode, address onBehalfOf) external;
    function configureEModeCategory(uint8 id, DataTypes.EModeCategory memory config) external;
    function deposit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
    function dropReserve(address asset) external;
    function finalizeTransfer(address asset, address from, address to, uint256 amount, uint256 balanceFromBefore, uint256 balanceToBefore) external;
    function flashLoan(address receiverAddress, address[] memory assets, uint256[] memory amounts, uint256[] memory interestRateModes, address onBehalfOf, bytes memory params, uint16 referralCode) external;
    function flashLoanSimple(address receiverAddress, address asset, uint256 amount, bytes memory params, uint16 referralCode) external;
    function getConfiguration(address asset) external view returns (DataTypes.ReserveConfigurationMap memory);
    function getEModeCategoryData(uint8 id) external view returns (DataTypes.EModeCategory memory);
    function getReserveAddressById(uint16 id) external view returns (address);
    function getReserveData(address asset) external view returns (DataTypes.ReserveData memory);
    function getReserveNormalizedIncome(address asset) external view returns (uint256);
    function getReserveNormalizedVariableDebt(address asset) external view returns (uint256);
    function getReservesList() external view returns (address[] memory);
    function getUserAccountData(address user) external view returns (uint256 totalCollateralBase, uint256 totalDebtBase, uint256 availableBorrowsBase, uint256 currentLiquidationThreshold, uint256 ltv, uint256 healthFactor);
    function getUserConfiguration(address user) external view returns (DataTypes.UserConfigurationMap memory);
    function getUserEMode(address user) external view returns (uint256);
    function initReserve(address asset, address aTokenAddress, address stableDebtAddress, address variableDebtAddress, address interestRateStrategyAddress) external;
    function liquidationCall(address collateralAsset, address debtAsset, address user, uint256 debtToCover, bool receiveAToken) external;
    function mintToTreasury(address[] memory assets) external;
    function mintUnbacked(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
    function rebalanceStableBorrowRate(address asset, address user) external;
    function repay(address asset, uint256 amount, uint256 interestRateMode, address onBehalfOf) external returns (uint256);
    function repayWithATokens(address asset, uint256 amount, uint256 interestRateMode) external returns (uint256);
    function repayWithPermit(address asset, uint256 amount, uint256 interestRateMode, address onBehalfOf, uint256 deadline, uint8 permitV, bytes32 permitR, bytes32 permitS) external returns (uint256);
    function rescueTokens(address token, address to, uint256 amount) external;
    function resetIsolationModeTotalDebt(address asset) external;
    function setConfiguration(address asset, DataTypes.ReserveConfigurationMap memory configuration) external;
    function setReserveInterestRateStrategyAddress(address asset, address rateStrategyAddress) external;
    function setUserEMode(uint8 categoryId) external;
    function setUserUseReserveAsCollateral(address asset, bool useAsCollateral) external;
    function supply(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
    function supplyWithPermit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode, uint256 deadline, uint8 permitV, bytes32 permitR, bytes32 permitS) external;
    function swapBorrowRateMode(address asset, uint256 interestRateMode) external;
    function updateBridgeProtocolFee(uint256 bridgeProtocolFee) external;
    function updateFlashloanPremiums(uint128 flashLoanPremiumTotal, uint128 flashLoanPremiumToProtocol) external;
    function withdraw(address asset, uint256 amount, address to) external returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "ADDRESSES_PROVIDER",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPoolAddressesProvider"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "BRIDGE_PROTOCOL_FEE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "FLASHLOAN_PREMIUM_TOTAL",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "FLASHLOAN_PREMIUM_TO_PROTOCOL",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MAX_NUMBER_RESERVES",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MAX_STABLE_RATE_BORROW_SIZE_PERCENT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "backUnbacked",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "fee",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "borrow",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "interestRateMode",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "internalType": "uint16"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "configureEModeCategory",
    "inputs": [
      {
        "name": "id",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "config",
        "type": "tuple",
        "internalType": "struct DataTypes.EModeCategory",
        "components": [
          {
            "name": "ltv",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "liquidationThreshold",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "liquidationBonus",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "priceSource",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "label",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "dropReserve",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "finalizeTransfer",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "balanceFromBefore",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "balanceToBefore",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "flashLoan",
    "inputs": [
      {
        "name": "receiverAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "assets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "amounts",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "interestRateModes",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "params",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "flashLoanSimple",
    "inputs": [
      {
        "name": "receiverAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "params",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getConfiguration",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct DataTypes.ReserveConfigurationMap",
        "components": [
          {
            "name": "data",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getEModeCategoryData",
    "inputs": [
      {
        "name": "id",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct DataTypes.EModeCategory",
        "components": [
          {
            "name": "ltv",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "liquidationThreshold",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "liquidationBonus",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "priceSource",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "label",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getReserveAddressById",
    "inputs": [
      {
        "name": "id",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getReserveData",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct DataTypes.ReserveData",
        "components": [
          {
            "name": "configuration",
            "type": "tuple",
            "internalType": "struct DataTypes.ReserveConfigurationMap",
            "components": [
              {
                "name": "data",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "liquidityIndex",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "currentLiquidityRate",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "variableBorrowIndex",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "currentVariableBorrowRate",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "currentStableBorrowRate",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "lastUpdateTimestamp",
            "type": "uint40",
            "internalType": "uint40"
          },
          {
            "name": "id",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "aTokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stableDebtTokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "variableDebtTokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "interestRateStrategyAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "accruedToTreasury",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "unbacked",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "isolationModeTotalDebt",
            "type": "uint128",
            "internalType": "uint128"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getReserveNormalizedIncome",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getReserveNormalizedVariableDebt",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getReservesList",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getUserAccountData",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "totalCollateralBase",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalDebtBase",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "availableBorrowsBase",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "currentLiquidationThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ltv",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "healthFactor",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getUserConfiguration",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct DataTypes.UserConfigurationMap",
        "components": [
          {
            "name": "data",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getUserEMode",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initReserve",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "aTokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stableDebtAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "variableDebtAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "interestRateStrategyAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "liquidationCall",
    "inputs": [
      {
        "name": "collateralAsset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "debtAsset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "debtToCover",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "receiveAToken",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mintToTreasury",
    "inputs": [
      {
        "name": "assets",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mintUnbacked",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rebalanceStableBorrowRate",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "repay",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "interestRateMode",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "repayWithATokens",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "interestRateMode",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "repayWithPermit",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "interestRateMode",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "permitV",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "permitR",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "permitS",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rescueTokens",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resetIsolationModeTotalDebt",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setConfiguration",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "configuration",
        "type": "tuple",
        "internalType": "struct DataTypes.ReserveConfigurationMap",
        "components": [
          {
            "name": "data",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setReserveInterestRateStrategyAddress",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "rateStrategyAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setUserEMode",
    "inputs": [
      {
        "name": "categoryId",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setUserUseReserveAsCollateral",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "useAsCollateral",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supply",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supplyWithPermit",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "internalType": "uint16"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "permitV",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "permitR",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "permitS",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "swapBorrowRateMode",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "interestRateMode",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateBridgeProtocolFee",
    "inputs": [
      {
        "name": "bridgeProtocolFee",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateFlashloanPremiums",
    "inputs": [
      {
        "name": "flashLoanPremiumTotal",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "flashLoanPremiumToProtocol",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdraw",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "BackUnbacked",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "backer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "fee",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Borrow",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "interestRateMode",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum DataTypes.InterestRateMode"
      },
      {
        "name": "borrowRate",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "indexed": true,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FlashLoan",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "initiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "asset",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "interestRateMode",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum DataTypes.InterestRateMode"
      },
      {
        "name": "premium",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "indexed": true,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "IsolationModeTotalDebtUpdated",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "totalDebt",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LiquidationCall",
    "inputs": [
      {
        "name": "collateralAsset",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "debtAsset",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "debtToCover",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "liquidatedCollateralAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "liquidator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "receiveAToken",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MintUnbacked",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "indexed": true,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MintedToTreasury",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amountMinted",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RebalanceStableBorrowRate",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Repay",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "repayer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "useATokens",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ReserveDataUpdated",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "liquidityRate",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "stableBorrowRate",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "variableBorrowRate",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "liquidityIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "variableBorrowIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ReserveUsedAsCollateralDisabled",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ReserveUsedAsCollateralEnabled",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Supply",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "onBehalfOf",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "referralCode",
        "type": "uint16",
        "indexed": true,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SwapBorrowRateMode",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "interestRateMode",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum DataTypes.InterestRateMode"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserEModeSet",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "categoryId",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Withdraw",
    "inputs": [
      {
        "name": "reserve",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod IPool {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /**Event with signature `BackUnbacked(address,address,uint256,uint256)` and selector `0x281596e92b2d974beb7d4f124df30a0b39067b096893e95011ce4bdad798b759`.
```solidity
event BackUnbacked(address indexed reserve, address indexed backer, uint256 amount, uint256 fee);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct BackUnbacked {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub backer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for BackUnbacked {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BackUnbacked(address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                21u8,
                150u8,
                233u8,
                43u8,
                45u8,
                151u8,
                75u8,
                235u8,
                125u8,
                79u8,
                18u8,
                77u8,
                243u8,
                10u8,
                11u8,
                57u8,
                6u8,
                123u8,
                9u8,
                104u8,
                147u8,
                233u8,
                80u8,
                17u8,
                206u8,
                75u8,
                218u8,
                215u8,
                152u8,
                183u8,
                89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    backer: topics.2,
                    amount: data.0,
                    fee: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reserve.clone(), self.backer.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.backer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BackUnbacked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BackUnbacked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BackUnbacked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Borrow(address,address,address,uint256,uint8,uint256,uint16)` and selector `0xb3d084820fb1a9decffb176436bd02558d15fac9b0ddfed8c465bc7359d7dce0`.
```solidity
event Borrow(address indexed reserve, address user, address indexed onBehalfOf, uint256 amount, DataTypes.InterestRateMode interestRateMode, uint256 borrowRate, uint16 indexed referralCode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Borrow {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub onBehalfOf: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub interestRateMode: <DataTypes::InterestRateMode as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub borrowRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub referralCode: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Borrow {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                DataTypes::InterestRateMode,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            const SIGNATURE: &'static str = "Borrow(address,address,address,uint256,uint8,uint256,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                179u8,
                208u8,
                132u8,
                130u8,
                15u8,
                177u8,
                169u8,
                222u8,
                207u8,
                251u8,
                23u8,
                100u8,
                54u8,
                189u8,
                2u8,
                85u8,
                141u8,
                21u8,
                250u8,
                201u8,
                176u8,
                221u8,
                254u8,
                216u8,
                196u8,
                101u8,
                188u8,
                115u8,
                89u8,
                215u8,
                220u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: data.0,
                    onBehalfOf: topics.2,
                    amount: data.1,
                    interestRateMode: data.2,
                    borrowRate: data.3,
                    referralCode: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <DataTypes::InterestRateMode as alloy_sol_types::SolType>::tokenize(
                        &self.interestRateMode,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowRate),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.reserve.clone(),
                    self.onBehalfOf.clone(),
                    self.referralCode.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.onBehalfOf,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.referralCode);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Borrow {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Borrow> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Borrow) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `FlashLoan(address,address,address,uint256,uint8,uint256,uint16)` and selector `0xefefaba5e921573100900a3ad9cf29f222d995fb3b6045797eaea7521bd8d6f0`.
```solidity
event FlashLoan(address indexed target, address initiator, address indexed asset, uint256 amount, DataTypes.InterestRateMode interestRateMode, uint256 premium, uint16 indexed referralCode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct FlashLoan {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub interestRateMode: <DataTypes::InterestRateMode as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub premium: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub referralCode: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for FlashLoan {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                DataTypes::InterestRateMode,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            const SIGNATURE: &'static str = "FlashLoan(address,address,address,uint256,uint8,uint256,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                239u8,
                239u8,
                171u8,
                165u8,
                233u8,
                33u8,
                87u8,
                49u8,
                0u8,
                144u8,
                10u8,
                58u8,
                217u8,
                207u8,
                41u8,
                242u8,
                34u8,
                217u8,
                149u8,
                251u8,
                59u8,
                96u8,
                69u8,
                121u8,
                126u8,
                174u8,
                167u8,
                82u8,
                27u8,
                216u8,
                214u8,
                240u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    target: topics.1,
                    initiator: data.0,
                    asset: topics.2,
                    amount: data.1,
                    interestRateMode: data.2,
                    premium: data.3,
                    referralCode: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initiator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <DataTypes::InterestRateMode as alloy_sol_types::SolType>::tokenize(
                        &self.interestRateMode,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.premium),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.target.clone(),
                    self.asset.clone(),
                    self.referralCode.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.target,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.asset,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.referralCode);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FlashLoan {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FlashLoan> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FlashLoan) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `IsolationModeTotalDebtUpdated(address,uint256)` and selector `0xaef84d3b40895fd58c561f3998000f0583abb992a52fbdc99ace8e8de4d676a5`.
```solidity
event IsolationModeTotalDebtUpdated(address indexed asset, uint256 totalDebt);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct IsolationModeTotalDebtUpdated {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub totalDebt: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for IsolationModeTotalDebtUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "IsolationModeTotalDebtUpdated(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                174u8,
                248u8,
                77u8,
                59u8,
                64u8,
                137u8,
                95u8,
                213u8,
                140u8,
                86u8,
                31u8,
                57u8,
                152u8,
                0u8,
                15u8,
                5u8,
                131u8,
                171u8,
                185u8,
                146u8,
                165u8,
                47u8,
                189u8,
                201u8,
                154u8,
                206u8,
                142u8,
                141u8,
                228u8,
                214u8,
                118u8,
                165u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    asset: topics.1,
                    totalDebt: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalDebt),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.asset.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.asset,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for IsolationModeTotalDebtUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&IsolationModeTotalDebtUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &IsolationModeTotalDebtUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `LiquidationCall(address,address,address,uint256,uint256,address,bool)` and selector `0xe413a321e8681d831f4dbccbca790d2952b56f977908e45be37335533e005286`.
```solidity
event LiquidationCall(address indexed collateralAsset, address indexed debtAsset, address indexed user, uint256 debtToCover, uint256 liquidatedCollateralAmount, address liquidator, bool receiveAToken);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct LiquidationCall {
        #[allow(missing_docs)]
        pub collateralAsset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub debtAsset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub debtToCover: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidatedCollateralAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub receiveAToken: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LiquidationCall {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "LiquidationCall(address,address,address,uint256,uint256,address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                228u8,
                19u8,
                163u8,
                33u8,
                232u8,
                104u8,
                29u8,
                131u8,
                31u8,
                77u8,
                188u8,
                203u8,
                202u8,
                121u8,
                13u8,
                41u8,
                82u8,
                181u8,
                111u8,
                151u8,
                121u8,
                8u8,
                228u8,
                91u8,
                227u8,
                115u8,
                53u8,
                83u8,
                62u8,
                0u8,
                82u8,
                134u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    collateralAsset: topics.1,
                    debtAsset: topics.2,
                    user: topics.3,
                    debtToCover: data.0,
                    liquidatedCollateralAmount: data.1,
                    liquidator: data.2,
                    receiveAToken: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.debtToCover),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.liquidatedCollateralAmount,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.liquidator,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.receiveAToken,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.collateralAsset.clone(),
                    self.debtAsset.clone(),
                    self.user.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.collateralAsset,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.debtAsset,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LiquidationCall {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LiquidationCall> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LiquidationCall) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MintUnbacked(address,address,address,uint256,uint16)` and selector `0xf25af37b3d3ec226063dc9bdc103ece7eb110a50f340fe854bb7bc1b0676d7d0`.
```solidity
event MintUnbacked(address indexed reserve, address user, address indexed onBehalfOf, uint256 amount, uint16 indexed referralCode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct MintUnbacked {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub onBehalfOf: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub referralCode: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MintUnbacked {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            const SIGNATURE: &'static str = "MintUnbacked(address,address,address,uint256,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                242u8,
                90u8,
                243u8,
                123u8,
                61u8,
                62u8,
                194u8,
                38u8,
                6u8,
                61u8,
                201u8,
                189u8,
                193u8,
                3u8,
                236u8,
                231u8,
                235u8,
                17u8,
                10u8,
                80u8,
                243u8,
                64u8,
                254u8,
                133u8,
                75u8,
                183u8,
                188u8,
                27u8,
                6u8,
                118u8,
                215u8,
                208u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: data.0,
                    onBehalfOf: topics.2,
                    amount: data.1,
                    referralCode: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.reserve.clone(),
                    self.onBehalfOf.clone(),
                    self.referralCode.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.onBehalfOf,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.referralCode);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MintUnbacked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MintUnbacked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MintUnbacked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MintedToTreasury(address,uint256)` and selector `0xbfa21aa5d5f9a1f0120a95e7c0749f389863cbdbfff531aa7339077a5bc919de`.
```solidity
event MintedToTreasury(address indexed reserve, uint256 amountMinted);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct MintedToTreasury {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountMinted: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MintedToTreasury {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "MintedToTreasury(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                191u8,
                162u8,
                26u8,
                165u8,
                213u8,
                249u8,
                161u8,
                240u8,
                18u8,
                10u8,
                149u8,
                231u8,
                192u8,
                116u8,
                159u8,
                56u8,
                152u8,
                99u8,
                203u8,
                219u8,
                255u8,
                245u8,
                49u8,
                170u8,
                115u8,
                57u8,
                7u8,
                122u8,
                91u8,
                201u8,
                25u8,
                222u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    amountMinted: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountMinted),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reserve.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MintedToTreasury {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MintedToTreasury> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MintedToTreasury) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RebalanceStableBorrowRate(address,address)` and selector `0x9f439ae0c81e41a04d3fdfe07aed54e6a179fb0db15be7702eb66fa8ef6f5300`.
```solidity
event RebalanceStableBorrowRate(address indexed reserve, address indexed user);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct RebalanceStableBorrowRate {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RebalanceStableBorrowRate {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RebalanceStableBorrowRate(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                159u8,
                67u8,
                154u8,
                224u8,
                200u8,
                30u8,
                65u8,
                160u8,
                77u8,
                63u8,
                223u8,
                224u8,
                122u8,
                237u8,
                84u8,
                230u8,
                161u8,
                121u8,
                251u8,
                13u8,
                177u8,
                91u8,
                231u8,
                112u8,
                46u8,
                182u8,
                111u8,
                168u8,
                239u8,
                111u8,
                83u8,
                0u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reserve.clone(), self.user.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RebalanceStableBorrowRate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RebalanceStableBorrowRate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RebalanceStableBorrowRate,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Repay(address,address,address,uint256,bool)` and selector `0xa534c8dbe71f871f9f3530e97a74601fea17b426cae02e1c5aee42c96c784051`.
```solidity
event Repay(address indexed reserve, address indexed user, address indexed repayer, uint256 amount, bool useATokens);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Repay {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub repayer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub useATokens: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Repay {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Repay(address,address,address,uint256,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                165u8,
                52u8,
                200u8,
                219u8,
                231u8,
                31u8,
                135u8,
                31u8,
                159u8,
                53u8,
                48u8,
                233u8,
                122u8,
                116u8,
                96u8,
                31u8,
                234u8,
                23u8,
                180u8,
                38u8,
                202u8,
                224u8,
                46u8,
                28u8,
                90u8,
                238u8,
                66u8,
                201u8,
                108u8,
                120u8,
                64u8,
                81u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: topics.2,
                    repayer: topics.3,
                    amount: data.0,
                    useATokens: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.useATokens,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.reserve.clone(),
                    self.user.clone(),
                    self.repayer.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.repayer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Repay {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Repay> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Repay) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ReserveDataUpdated(address,uint256,uint256,uint256,uint256,uint256)` and selector `0x804c9b842b2748a22bb64b345453a3de7ca54a6ca45ce00d415894979e22897a`.
```solidity
event ReserveDataUpdated(address indexed reserve, uint256 liquidityRate, uint256 stableBorrowRate, uint256 variableBorrowRate, uint256 liquidityIndex, uint256 variableBorrowIndex);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ReserveDataUpdated {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub liquidityRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub stableBorrowRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub variableBorrowRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidityIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub variableBorrowIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ReserveDataUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ReserveDataUpdated(address,uint256,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                128u8,
                76u8,
                155u8,
                132u8,
                43u8,
                39u8,
                72u8,
                162u8,
                43u8,
                182u8,
                75u8,
                52u8,
                84u8,
                83u8,
                163u8,
                222u8,
                124u8,
                165u8,
                74u8,
                108u8,
                164u8,
                92u8,
                224u8,
                13u8,
                65u8,
                88u8,
                148u8,
                151u8,
                158u8,
                34u8,
                137u8,
                122u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    liquidityRate: data.0,
                    stableBorrowRate: data.1,
                    variableBorrowRate: data.2,
                    liquidityIndex: data.3,
                    variableBorrowIndex: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.stableBorrowRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.variableBorrowRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.variableBorrowIndex),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reserve.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ReserveDataUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ReserveDataUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ReserveDataUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ReserveUsedAsCollateralDisabled(address,address)` and selector `0x44c58d81365b66dd4b1a7f36c25aa97b8c71c361ee4937adc1a00000227db5dd`.
```solidity
event ReserveUsedAsCollateralDisabled(address indexed reserve, address indexed user);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ReserveUsedAsCollateralDisabled {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ReserveUsedAsCollateralDisabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ReserveUsedAsCollateralDisabled(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                68u8,
                197u8,
                141u8,
                129u8,
                54u8,
                91u8,
                102u8,
                221u8,
                75u8,
                26u8,
                127u8,
                54u8,
                194u8,
                90u8,
                169u8,
                123u8,
                140u8,
                113u8,
                195u8,
                97u8,
                238u8,
                73u8,
                55u8,
                173u8,
                193u8,
                160u8,
                0u8,
                0u8,
                34u8,
                125u8,
                181u8,
                221u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reserve.clone(), self.user.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ReserveUsedAsCollateralDisabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ReserveUsedAsCollateralDisabled>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ReserveUsedAsCollateralDisabled,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ReserveUsedAsCollateralEnabled(address,address)` and selector `0x00058a56ea94653cdf4f152d227ace22d4c00ad99e2a43f58cb7d9e3feb295f2`.
```solidity
event ReserveUsedAsCollateralEnabled(address indexed reserve, address indexed user);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ReserveUsedAsCollateralEnabled {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ReserveUsedAsCollateralEnabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ReserveUsedAsCollateralEnabled(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8,
                5u8,
                138u8,
                86u8,
                234u8,
                148u8,
                101u8,
                60u8,
                223u8,
                79u8,
                21u8,
                45u8,
                34u8,
                122u8,
                206u8,
                34u8,
                212u8,
                192u8,
                10u8,
                217u8,
                158u8,
                42u8,
                67u8,
                245u8,
                140u8,
                183u8,
                217u8,
                227u8,
                254u8,
                178u8,
                149u8,
                242u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reserve.clone(), self.user.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ReserveUsedAsCollateralEnabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ReserveUsedAsCollateralEnabled>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ReserveUsedAsCollateralEnabled,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Supply(address,address,address,uint256,uint16)` and selector `0x2b627736bca15cd5381dcf80b0bf11fd197d01a037c52b927a881a10fb73ba61`.
```solidity
event Supply(address indexed reserve, address user, address indexed onBehalfOf, uint256 amount, uint16 indexed referralCode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Supply {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub onBehalfOf: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub referralCode: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Supply {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            const SIGNATURE: &'static str = "Supply(address,address,address,uint256,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                43u8,
                98u8,
                119u8,
                54u8,
                188u8,
                161u8,
                92u8,
                213u8,
                56u8,
                29u8,
                207u8,
                128u8,
                176u8,
                191u8,
                17u8,
                253u8,
                25u8,
                125u8,
                1u8,
                160u8,
                55u8,
                197u8,
                43u8,
                146u8,
                122u8,
                136u8,
                26u8,
                16u8,
                251u8,
                115u8,
                186u8,
                97u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: data.0,
                    onBehalfOf: topics.2,
                    amount: data.1,
                    referralCode: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.reserve.clone(),
                    self.onBehalfOf.clone(),
                    self.referralCode.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.onBehalfOf,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.referralCode);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Supply {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Supply> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Supply) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SwapBorrowRateMode(address,address,uint8)` and selector `0x7962b394d85a534033ba2efcf43cd36de57b7ebeb3de0ca4428965d9b3ddc481`.
```solidity
event SwapBorrowRateMode(address indexed reserve, address indexed user, DataTypes.InterestRateMode interestRateMode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct SwapBorrowRateMode {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub interestRateMode: <DataTypes::InterestRateMode as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SwapBorrowRateMode {
            type DataTuple<'a> = (DataTypes::InterestRateMode,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SwapBorrowRateMode(address,address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                121u8,
                98u8,
                179u8,
                148u8,
                216u8,
                90u8,
                83u8,
                64u8,
                51u8,
                186u8,
                46u8,
                252u8,
                244u8,
                60u8,
                211u8,
                109u8,
                229u8,
                123u8,
                126u8,
                190u8,
                179u8,
                222u8,
                12u8,
                164u8,
                66u8,
                137u8,
                101u8,
                217u8,
                179u8,
                221u8,
                196u8,
                129u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: topics.2,
                    interestRateMode: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <DataTypes::InterestRateMode as alloy_sol_types::SolType>::tokenize(
                        &self.interestRateMode,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reserve.clone(), self.user.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SwapBorrowRateMode {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SwapBorrowRateMode> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SwapBorrowRateMode) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UserEModeSet(address,uint8)` and selector `0xd728da875fc88944cbf17638bcbe4af0eedaef63becd1d1c57cc097eb4608d84`.
```solidity
event UserEModeSet(address indexed user, uint8 categoryId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct UserEModeSet {
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub categoryId: u8,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserEModeSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UserEModeSet(address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                215u8,
                40u8,
                218u8,
                135u8,
                95u8,
                200u8,
                137u8,
                68u8,
                203u8,
                241u8,
                118u8,
                56u8,
                188u8,
                190u8,
                74u8,
                240u8,
                238u8,
                218u8,
                239u8,
                99u8,
                190u8,
                205u8,
                29u8,
                28u8,
                87u8,
                204u8,
                9u8,
                126u8,
                180u8,
                96u8,
                141u8,
                132u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    user: topics.1,
                    categoryId: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.categoryId),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.user.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserEModeSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserEModeSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserEModeSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Withdraw(address,address,address,uint256)` and selector `0x3115d1449a7b732c986cba18244e897a450f61e1bb8d589cd2e69e6c8924f9f7`.
```solidity
event Withdraw(address indexed reserve, address indexed user, address indexed to, uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Withdraw {
        #[allow(missing_docs)]
        pub reserve: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Withdraw {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Withdraw(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                21u8,
                209u8,
                68u8,
                154u8,
                123u8,
                115u8,
                44u8,
                152u8,
                108u8,
                186u8,
                24u8,
                36u8,
                78u8,
                137u8,
                122u8,
                69u8,
                15u8,
                97u8,
                225u8,
                187u8,
                141u8,
                88u8,
                156u8,
                210u8,
                230u8,
                158u8,
                108u8,
                137u8,
                36u8,
                249u8,
                247u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    reserve: topics.1,
                    user: topics.2,
                    to: topics.3,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.reserve.clone(),
                    self.user.clone(),
                    self.to.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reserve,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Withdraw {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Withdraw> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Withdraw) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `ADDRESSES_PROVIDER()` and selector `0x0542975c`.
```solidity
function ADDRESSES_PROVIDER() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ADDRESSES_PROVIDERCall {}
    ///Container type for the return parameters of the [`ADDRESSES_PROVIDER()`](ADDRESSES_PROVIDERCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ADDRESSES_PROVIDERReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ADDRESSES_PROVIDERCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ADDRESSES_PROVIDERCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ADDRESSES_PROVIDERCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ADDRESSES_PROVIDERReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ADDRESSES_PROVIDERReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ADDRESSES_PROVIDERReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ADDRESSES_PROVIDERCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ADDRESSES_PROVIDERReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ADDRESSES_PROVIDER()";
            const SELECTOR: [u8; 4] = [5u8, 66u8, 151u8, 92u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `BRIDGE_PROTOCOL_FEE()` and selector `0x272d9072`.
```solidity
function BRIDGE_PROTOCOL_FEE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct BRIDGE_PROTOCOL_FEECall {}
    ///Container type for the return parameters of the [`BRIDGE_PROTOCOL_FEE()`](BRIDGE_PROTOCOL_FEECall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct BRIDGE_PROTOCOL_FEEReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BRIDGE_PROTOCOL_FEECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: BRIDGE_PROTOCOL_FEECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BRIDGE_PROTOCOL_FEECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BRIDGE_PROTOCOL_FEEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: BRIDGE_PROTOCOL_FEEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BRIDGE_PROTOCOL_FEEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BRIDGE_PROTOCOL_FEECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BRIDGE_PROTOCOL_FEEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BRIDGE_PROTOCOL_FEE()";
            const SELECTOR: [u8; 4] = [39u8, 45u8, 144u8, 114u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `FLASHLOAN_PREMIUM_TOTAL()` and selector `0x074b2e43`.
```solidity
function FLASHLOAN_PREMIUM_TOTAL() external view returns (uint128);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FLASHLOAN_PREMIUM_TOTALCall {}
    ///Container type for the return parameters of the [`FLASHLOAN_PREMIUM_TOTAL()`](FLASHLOAN_PREMIUM_TOTALCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FLASHLOAN_PREMIUM_TOTALReturn {
        pub _0: u128,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FLASHLOAN_PREMIUM_TOTALCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: FLASHLOAN_PREMIUM_TOTALCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FLASHLOAN_PREMIUM_TOTALCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FLASHLOAN_PREMIUM_TOTALReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: FLASHLOAN_PREMIUM_TOTALReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FLASHLOAN_PREMIUM_TOTALReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for FLASHLOAN_PREMIUM_TOTALCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = FLASHLOAN_PREMIUM_TOTALReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FLASHLOAN_PREMIUM_TOTAL()";
            const SELECTOR: [u8; 4] = [7u8, 75u8, 46u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `FLASHLOAN_PREMIUM_TO_PROTOCOL()` and selector `0x6a99c036`.
```solidity
function FLASHLOAN_PREMIUM_TO_PROTOCOL() external view returns (uint128);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FLASHLOAN_PREMIUM_TO_PROTOCOLCall {}
    ///Container type for the return parameters of the [`FLASHLOAN_PREMIUM_TO_PROTOCOL()`](FLASHLOAN_PREMIUM_TO_PROTOCOLCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FLASHLOAN_PREMIUM_TO_PROTOCOLReturn {
        pub _0: u128,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FLASHLOAN_PREMIUM_TO_PROTOCOLCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: FLASHLOAN_PREMIUM_TO_PROTOCOLCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FLASHLOAN_PREMIUM_TO_PROTOCOLCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FLASHLOAN_PREMIUM_TO_PROTOCOLReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: FLASHLOAN_PREMIUM_TO_PROTOCOLReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FLASHLOAN_PREMIUM_TO_PROTOCOLReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for FLASHLOAN_PREMIUM_TO_PROTOCOLCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = FLASHLOAN_PREMIUM_TO_PROTOCOLReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FLASHLOAN_PREMIUM_TO_PROTOCOL()";
            const SELECTOR: [u8; 4] = [106u8, 153u8, 192u8, 54u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `MAX_NUMBER_RESERVES()` and selector `0xf8119d51`.
```solidity
function MAX_NUMBER_RESERVES() external view returns (uint16);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MAX_NUMBER_RESERVESCall {}
    ///Container type for the return parameters of the [`MAX_NUMBER_RESERVES()`](MAX_NUMBER_RESERVESCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MAX_NUMBER_RESERVESReturn {
        pub _0: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_NUMBER_RESERVESCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_NUMBER_RESERVESCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_NUMBER_RESERVESCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_NUMBER_RESERVESReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_NUMBER_RESERVESReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_NUMBER_RESERVESReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_NUMBER_RESERVESCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_NUMBER_RESERVESReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_NUMBER_RESERVES()";
            const SELECTOR: [u8; 4] = [248u8, 17u8, 157u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `MAX_STABLE_RATE_BORROW_SIZE_PERCENT()` and selector `0xe82fec2f`.
```solidity
function MAX_STABLE_RATE_BORROW_SIZE_PERCENT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall {}
    ///Container type for the return parameters of the [`MAX_STABLE_RATE_BORROW_SIZE_PERCENT()`](MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MAX_STABLE_RATE_BORROW_SIZE_PERCENTReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_STABLE_RATE_BORROW_SIZE_PERCENTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_STABLE_RATE_BORROW_SIZE_PERCENTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_STABLE_RATE_BORROW_SIZE_PERCENTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_STABLE_RATE_BORROW_SIZE_PERCENTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_STABLE_RATE_BORROW_SIZE_PERCENT()";
            const SELECTOR: [u8; 4] = [232u8, 47u8, 236u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `backUnbacked(address,uint256,uint256)` and selector `0xd65dc7a1`.
```solidity
function backUnbacked(address asset, uint256 amount, uint256 fee) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct backUnbackedCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub fee: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`backUnbacked(address,uint256,uint256)`](backUnbackedCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct backUnbackedReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<backUnbackedCall> for UnderlyingRustTuple<'_> {
                fn from(value: backUnbackedCall) -> Self {
                    (value.asset, value.amount, value.fee)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for backUnbackedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        fee: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<backUnbackedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: backUnbackedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for backUnbackedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for backUnbackedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = backUnbackedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "backUnbacked(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [214u8, 93u8, 199u8, 161u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `borrow(address,uint256,uint256,uint16,address)` and selector `0xa415bcad`.
```solidity
function borrow(address asset, uint256 amount, uint256 interestRateMode, uint16 referralCode, address onBehalfOf) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct borrowCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
        pub referralCode: u16,
        pub onBehalfOf: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`borrow(address,uint256,uint256,uint16,address)`](borrowCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct borrowReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u16,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<borrowCall> for UnderlyingRustTuple<'_> {
                fn from(value: borrowCall) -> Self {
                    (
                        value.asset,
                        value.amount,
                        value.interestRateMode,
                        value.referralCode,
                        value.onBehalfOf,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for borrowCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        interestRateMode: tuple.2,
                        referralCode: tuple.3,
                        onBehalfOf: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<borrowReturn> for UnderlyingRustTuple<'_> {
                fn from(value: borrowReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for borrowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for borrowCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = borrowReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "borrow(address,uint256,uint256,uint16,address)";
            const SELECTOR: [u8; 4] = [164u8, 21u8, 188u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.interestRateMode),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.referralCode),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `configureEModeCategory(uint8,(uint16,uint16,uint16,address,string))` and selector `0xd579ea7d`.
```solidity
function configureEModeCategory(uint8 id, DataTypes.EModeCategory memory config) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct configureEModeCategoryCall {
        pub id: u8,
        pub config: <DataTypes::EModeCategory as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`configureEModeCategory(uint8,(uint16,uint16,uint16,address,string))`](configureEModeCategoryCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct configureEModeCategoryReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                DataTypes::EModeCategory,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                <DataTypes::EModeCategory as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<configureEModeCategoryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureEModeCategoryCall) -> Self {
                    (value.id, value.config)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureEModeCategoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        config: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<configureEModeCategoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureEModeCategoryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureEModeCategoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configureEModeCategoryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                DataTypes::EModeCategory,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = configureEModeCategoryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configureEModeCategory(uint8,(uint16,uint16,uint16,address,string))";
            const SELECTOR: [u8; 4] = [213u8, 121u8, 234u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <DataTypes::EModeCategory as alloy_sol_types::SolType>::tokenize(
                        &self.config,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `deposit(address,uint256,address,uint16)` and selector `0xe8eda9df`.
```solidity
function deposit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct depositCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub onBehalfOf: alloy::sol_types::private::Address,
        pub referralCode: u16,
    }
    ///Container type for the return parameters of the [`deposit(address,uint256,address,uint16)`](depositCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct depositReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                u16,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositCall) -> Self {
                    (value.asset, value.amount, value.onBehalfOf, value.referralCode)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        onBehalfOf: tuple.2,
                        referralCode: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,uint256,address,uint16)";
            const SELECTOR: [u8; 4] = [232u8, 237u8, 169u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.referralCode),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `dropReserve(address)` and selector `0x63c9b860`.
```solidity
function dropReserve(address asset) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct dropReserveCall {
        pub asset: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`dropReserve(address)`](dropReserveCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct dropReserveReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<dropReserveCall> for UnderlyingRustTuple<'_> {
                fn from(value: dropReserveCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dropReserveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<dropReserveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: dropReserveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dropReserveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for dropReserveCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = dropReserveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "dropReserve(address)";
            const SELECTOR: [u8; 4] = [99u8, 201u8, 184u8, 96u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `finalizeTransfer(address,address,address,uint256,uint256,uint256)` and selector `0xd5ed3933`.
```solidity
function finalizeTransfer(address asset, address from, address to, uint256 amount, uint256 balanceFromBefore, uint256 balanceToBefore) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct finalizeTransferCall {
        pub asset: alloy::sol_types::private::Address,
        pub from: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub balanceFromBefore: alloy::sol_types::private::primitives::aliases::U256,
        pub balanceToBefore: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`finalizeTransfer(address,address,address,uint256,uint256,uint256)`](finalizeTransferCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct finalizeTransferReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<finalizeTransferCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeTransferCall) -> Self {
                    (
                        value.asset,
                        value.from,
                        value.to,
                        value.amount,
                        value.balanceFromBefore,
                        value.balanceToBefore,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeTransferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                        amount: tuple.3,
                        balanceFromBefore: tuple.4,
                        balanceToBefore: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<finalizeTransferReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeTransferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeTransferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizeTransferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizeTransferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizeTransfer(address,address,address,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [213u8, 237u8, 57u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceFromBefore),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceToBefore),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)` and selector `0xab9c4b5d`.
```solidity
function flashLoan(address receiverAddress, address[] memory assets, uint256[] memory amounts, uint256[] memory interestRateModes, address onBehalfOf, bytes memory params, uint16 referralCode) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct flashLoanCall {
        pub receiverAddress: alloy::sol_types::private::Address,
        pub assets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub amounts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub interestRateModes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub onBehalfOf: alloy::sol_types::private::Address,
        pub params: alloy::sol_types::private::Bytes,
        pub referralCode: u16,
    }
    ///Container type for the return parameters of the [`flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)`](flashLoanCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct flashLoanReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                u16,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<flashLoanCall> for UnderlyingRustTuple<'_> {
                fn from(value: flashLoanCall) -> Self {
                    (
                        value.receiverAddress,
                        value.assets,
                        value.amounts,
                        value.interestRateModes,
                        value.onBehalfOf,
                        value.params,
                        value.referralCode,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for flashLoanCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        receiverAddress: tuple.0,
                        assets: tuple.1,
                        amounts: tuple.2,
                        interestRateModes: tuple.3,
                        onBehalfOf: tuple.4,
                        params: tuple.5,
                        referralCode: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<flashLoanReturn> for UnderlyingRustTuple<'_> {
                fn from(value: flashLoanReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for flashLoanReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for flashLoanCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = flashLoanReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)";
            const SELECTOR: [u8; 4] = [171u8, 156u8, 75u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.receiverAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.assets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.amounts),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.interestRateModes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.referralCode),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `flashLoanSimple(address,address,uint256,bytes,uint16)` and selector `0x42b0b77c`.
```solidity
function flashLoanSimple(address receiverAddress, address asset, uint256 amount, bytes memory params, uint16 referralCode) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct flashLoanSimpleCall {
        pub receiverAddress: alloy::sol_types::private::Address,
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub params: alloy::sol_types::private::Bytes,
        pub referralCode: u16,
    }
    ///Container type for the return parameters of the [`flashLoanSimple(address,address,uint256,bytes,uint16)`](flashLoanSimpleCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct flashLoanSimpleReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                u16,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<flashLoanSimpleCall> for UnderlyingRustTuple<'_> {
                fn from(value: flashLoanSimpleCall) -> Self {
                    (
                        value.receiverAddress,
                        value.asset,
                        value.amount,
                        value.params,
                        value.referralCode,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for flashLoanSimpleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        receiverAddress: tuple.0,
                        asset: tuple.1,
                        amount: tuple.2,
                        params: tuple.3,
                        referralCode: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<flashLoanSimpleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: flashLoanSimpleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for flashLoanSimpleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for flashLoanSimpleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = flashLoanSimpleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "flashLoanSimple(address,address,uint256,bytes,uint16)";
            const SELECTOR: [u8; 4] = [66u8, 176u8, 183u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.receiverAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.referralCode),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getConfiguration(address)` and selector `0xc44b11f7`.
```solidity
function getConfiguration(address asset) external view returns (DataTypes.ReserveConfigurationMap memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getConfigurationCall {
        pub asset: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getConfiguration(address)`](getConfigurationCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getConfigurationReturn {
        pub _0: <DataTypes::ReserveConfigurationMap as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getConfigurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConfigurationCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConfigurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (DataTypes::ReserveConfigurationMap,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DataTypes::ReserveConfigurationMap as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getConfigurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConfigurationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConfigurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getConfigurationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getConfigurationReturn;
            type ReturnTuple<'a> = (DataTypes::ReserveConfigurationMap,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getConfiguration(address)";
            const SELECTOR: [u8; 4] = [196u8, 75u8, 17u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getEModeCategoryData(uint8)` and selector `0x6c6f6ae1`.
```solidity
function getEModeCategoryData(uint8 id) external view returns (DataTypes.EModeCategory memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getEModeCategoryDataCall {
        pub id: u8,
    }
    ///Container type for the return parameters of the [`getEModeCategoryData(uint8)`](getEModeCategoryDataCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getEModeCategoryDataReturn {
        pub _0: <DataTypes::EModeCategory as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getEModeCategoryDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEModeCategoryDataCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEModeCategoryDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (DataTypes::EModeCategory,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DataTypes::EModeCategory as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getEModeCategoryDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEModeCategoryDataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEModeCategoryDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEModeCategoryDataCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getEModeCategoryDataReturn;
            type ReturnTuple<'a> = (DataTypes::EModeCategory,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEModeCategoryData(uint8)";
            const SELECTOR: [u8; 4] = [108u8, 111u8, 106u8, 225u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getReserveAddressById(uint16)` and selector `0x52751797`.
```solidity
function getReserveAddressById(uint16 id) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveAddressByIdCall {
        pub id: u16,
    }
    ///Container type for the return parameters of the [`getReserveAddressById(uint16)`](getReserveAddressByIdCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveAddressByIdReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveAddressByIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReserveAddressByIdCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReserveAddressByIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveAddressByIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReserveAddressByIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReserveAddressByIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReserveAddressByIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getReserveAddressByIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReserveAddressById(uint16)";
            const SELECTOR: [u8; 4] = [82u8, 117u8, 23u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getReserveData(address)` and selector `0x35ea6a75`.
```solidity
function getReserveData(address asset) external view returns (DataTypes.ReserveData memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveDataCall {
        pub asset: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getReserveData(address)`](getReserveDataCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveDataReturn {
        pub _0: <DataTypes::ReserveData as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: getReserveDataCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReserveDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (DataTypes::ReserveData,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DataTypes::ReserveData as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReserveDataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReserveDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReserveDataCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getReserveDataReturn;
            type ReturnTuple<'a> = (DataTypes::ReserveData,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReserveData(address)";
            const SELECTOR: [u8; 4] = [53u8, 234u8, 106u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getReserveNormalizedIncome(address)` and selector `0xd15e0053`.
```solidity
function getReserveNormalizedIncome(address asset) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveNormalizedIncomeCall {
        pub asset: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getReserveNormalizedIncome(address)`](getReserveNormalizedIncomeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveNormalizedIncomeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveNormalizedIncomeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReserveNormalizedIncomeCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReserveNormalizedIncomeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveNormalizedIncomeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReserveNormalizedIncomeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReserveNormalizedIncomeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReserveNormalizedIncomeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getReserveNormalizedIncomeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReserveNormalizedIncome(address)";
            const SELECTOR: [u8; 4] = [209u8, 94u8, 0u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getReserveNormalizedVariableDebt(address)` and selector `0x386497fd`.
```solidity
function getReserveNormalizedVariableDebt(address asset) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveNormalizedVariableDebtCall {
        pub asset: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getReserveNormalizedVariableDebt(address)`](getReserveNormalizedVariableDebtCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReserveNormalizedVariableDebtReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveNormalizedVariableDebtCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReserveNormalizedVariableDebtCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReserveNormalizedVariableDebtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReserveNormalizedVariableDebtReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReserveNormalizedVariableDebtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReserveNormalizedVariableDebtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReserveNormalizedVariableDebtCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getReserveNormalizedVariableDebtReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReserveNormalizedVariableDebt(address)";
            const SELECTOR: [u8; 4] = [56u8, 100u8, 151u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getReservesList()` and selector `0xd1946dbc`.
```solidity
function getReservesList() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReservesListCall {}
    ///Container type for the return parameters of the [`getReservesList()`](getReservesListCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getReservesListReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReservesListCall> for UnderlyingRustTuple<'_> {
                fn from(value: getReservesListCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReservesListCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReservesListReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getReservesListReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getReservesListReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReservesListCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getReservesListReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReservesList()";
            const SELECTOR: [u8; 4] = [209u8, 148u8, 109u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getUserAccountData(address)` and selector `0xbf92857c`.
```solidity
function getUserAccountData(address user) external view returns (uint256 totalCollateralBase, uint256 totalDebtBase, uint256 availableBorrowsBase, uint256 currentLiquidationThreshold, uint256 ltv, uint256 healthFactor);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getUserAccountDataCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getUserAccountData(address)`](getUserAccountDataCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getUserAccountDataReturn {
        pub totalCollateralBase: alloy::sol_types::private::primitives::aliases::U256,
        pub totalDebtBase: alloy::sol_types::private::primitives::aliases::U256,
        pub availableBorrowsBase: alloy::sol_types::private::primitives::aliases::U256,
        pub currentLiquidationThreshold: alloy::sol_types::private::primitives::aliases::U256,
        pub ltv: alloy::sol_types::private::primitives::aliases::U256,
        pub healthFactor: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserAccountDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserAccountDataCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserAccountDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserAccountDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserAccountDataReturn) -> Self {
                    (
                        value.totalCollateralBase,
                        value.totalDebtBase,
                        value.availableBorrowsBase,
                        value.currentLiquidationThreshold,
                        value.ltv,
                        value.healthFactor,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserAccountDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        totalCollateralBase: tuple.0,
                        totalDebtBase: tuple.1,
                        availableBorrowsBase: tuple.2,
                        currentLiquidationThreshold: tuple.3,
                        ltv: tuple.4,
                        healthFactor: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserAccountDataCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUserAccountDataReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserAccountData(address)";
            const SELECTOR: [u8; 4] = [191u8, 146u8, 133u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getUserConfiguration(address)` and selector `0x4417a583`.
```solidity
function getUserConfiguration(address user) external view returns (DataTypes.UserConfigurationMap memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getUserConfigurationCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getUserConfiguration(address)`](getUserConfigurationCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getUserConfigurationReturn {
        pub _0: <DataTypes::UserConfigurationMap as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserConfigurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserConfigurationCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserConfigurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (DataTypes::UserConfigurationMap,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DataTypes::UserConfigurationMap as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserConfigurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserConfigurationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserConfigurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserConfigurationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUserConfigurationReturn;
            type ReturnTuple<'a> = (DataTypes::UserConfigurationMap,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserConfiguration(address)";
            const SELECTOR: [u8; 4] = [68u8, 23u8, 165u8, 131u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getUserEMode(address)` and selector `0xeddf1b79`.
```solidity
function getUserEMode(address user) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getUserEModeCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getUserEMode(address)`](getUserEModeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getUserEModeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserEModeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getUserEModeCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUserEModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserEModeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getUserEModeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUserEModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserEModeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUserEModeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserEMode(address)";
            const SELECTOR: [u8; 4] = [237u8, 223u8, 27u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initReserve(address,address,address,address,address)` and selector `0x7a708e92`.
```solidity
function initReserve(address asset, address aTokenAddress, address stableDebtAddress, address variableDebtAddress, address interestRateStrategyAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initReserveCall {
        pub asset: alloy::sol_types::private::Address,
        pub aTokenAddress: alloy::sol_types::private::Address,
        pub stableDebtAddress: alloy::sol_types::private::Address,
        pub variableDebtAddress: alloy::sol_types::private::Address,
        pub interestRateStrategyAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initReserve(address,address,address,address,address)`](initReserveCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initReserveReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initReserveCall> for UnderlyingRustTuple<'_> {
                fn from(value: initReserveCall) -> Self {
                    (
                        value.asset,
                        value.aTokenAddress,
                        value.stableDebtAddress,
                        value.variableDebtAddress,
                        value.interestRateStrategyAddress,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initReserveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        aTokenAddress: tuple.1,
                        stableDebtAddress: tuple.2,
                        variableDebtAddress: tuple.3,
                        interestRateStrategyAddress: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initReserveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initReserveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initReserveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initReserveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initReserveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initReserve(address,address,address,address,address)";
            const SELECTOR: [u8; 4] = [122u8, 112u8, 142u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.aTokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.stableDebtAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.variableDebtAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.interestRateStrategyAddress,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `liquidationCall(address,address,address,uint256,bool)` and selector `0x00a718a9`.
```solidity
function liquidationCall(address collateralAsset, address debtAsset, address user, uint256 debtToCover, bool receiveAToken) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct liquidationCallCall {
        pub collateralAsset: alloy::sol_types::private::Address,
        pub debtAsset: alloy::sol_types::private::Address,
        pub user: alloy::sol_types::private::Address,
        pub debtToCover: alloy::sol_types::private::primitives::aliases::U256,
        pub receiveAToken: bool,
    }
    ///Container type for the return parameters of the [`liquidationCall(address,address,address,uint256,bool)`](liquidationCallCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct liquidationCallReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<liquidationCallCall> for UnderlyingRustTuple<'_> {
                fn from(value: liquidationCallCall) -> Self {
                    (
                        value.collateralAsset,
                        value.debtAsset,
                        value.user,
                        value.debtToCover,
                        value.receiveAToken,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for liquidationCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        collateralAsset: tuple.0,
                        debtAsset: tuple.1,
                        user: tuple.2,
                        debtToCover: tuple.3,
                        receiveAToken: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<liquidationCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: liquidationCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for liquidationCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for liquidationCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = liquidationCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "liquidationCall(address,address,address,uint256,bool)";
            const SELECTOR: [u8; 4] = [0u8, 167u8, 24u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.collateralAsset,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.debtAsset,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.debtToCover),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.receiveAToken,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `mintToTreasury(address[])` and selector `0x9cd19996`.
```solidity
function mintToTreasury(address[] memory assets) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintToTreasuryCall {
        pub assets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`mintToTreasury(address[])`](mintToTreasuryCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintToTreasuryReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintToTreasuryCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintToTreasuryCall) -> Self {
                    (value.assets,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintToTreasuryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { assets: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintToTreasuryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mintToTreasuryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mintToTreasuryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintToTreasuryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintToTreasuryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mintToTreasury(address[])";
            const SELECTOR: [u8; 4] = [156u8, 209u8, 153u8, 150u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.assets),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `mintUnbacked(address,uint256,address,uint16)` and selector `0x69a933a5`.
```solidity
function mintUnbacked(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintUnbackedCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub onBehalfOf: alloy::sol_types::private::Address,
        pub referralCode: u16,
    }
    ///Container type for the return parameters of the [`mintUnbacked(address,uint256,address,uint16)`](mintUnbackedCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintUnbackedReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                u16,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintUnbackedCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintUnbackedCall) -> Self {
                    (value.asset, value.amount, value.onBehalfOf, value.referralCode)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintUnbackedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        onBehalfOf: tuple.2,
                        referralCode: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintUnbackedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mintUnbackedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintUnbackedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintUnbackedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintUnbackedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mintUnbacked(address,uint256,address,uint16)";
            const SELECTOR: [u8; 4] = [105u8, 169u8, 51u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.referralCode),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `rebalanceStableBorrowRate(address,address)` and selector `0xcd112382`.
```solidity
function rebalanceStableBorrowRate(address asset, address user) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct rebalanceStableBorrowRateCall {
        pub asset: alloy::sol_types::private::Address,
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`rebalanceStableBorrowRate(address,address)`](rebalanceStableBorrowRateCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct rebalanceStableBorrowRateReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rebalanceStableBorrowRateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rebalanceStableBorrowRateCall) -> Self {
                    (value.asset, value.user)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rebalanceStableBorrowRateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        user: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rebalanceStableBorrowRateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rebalanceStableBorrowRateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rebalanceStableBorrowRateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rebalanceStableBorrowRateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rebalanceStableBorrowRateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rebalanceStableBorrowRate(address,address)";
            const SELECTOR: [u8; 4] = [205u8, 17u8, 35u8, 130u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.user,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `repay(address,uint256,uint256,address)` and selector `0x573ade81`.
```solidity
function repay(address asset, uint256 amount, uint256 interestRateMode, address onBehalfOf) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct repayCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
        pub onBehalfOf: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`repay(address,uint256,uint256,address)`](repayCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct repayReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<repayCall> for UnderlyingRustTuple<'_> {
                fn from(value: repayCall) -> Self {
                    (value.asset, value.amount, value.interestRateMode, value.onBehalfOf)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for repayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        interestRateMode: tuple.2,
                        onBehalfOf: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<repayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: repayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for repayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for repayCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = repayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "repay(address,uint256,uint256,address)";
            const SELECTOR: [u8; 4] = [87u8, 58u8, 222u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.interestRateMode),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `repayWithATokens(address,uint256,uint256)` and selector `0x2dad97d4`.
```solidity
function repayWithATokens(address asset, uint256 amount, uint256 interestRateMode) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct repayWithATokensCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`repayWithATokens(address,uint256,uint256)`](repayWithATokensCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct repayWithATokensReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<repayWithATokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: repayWithATokensCall) -> Self {
                    (value.asset, value.amount, value.interestRateMode)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for repayWithATokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        interestRateMode: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<repayWithATokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: repayWithATokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for repayWithATokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for repayWithATokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = repayWithATokensReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "repayWithATokens(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [45u8, 173u8, 151u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.interestRateMode),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `0xee3e210b`.
```solidity
function repayWithPermit(address asset, uint256 amount, uint256 interestRateMode, address onBehalfOf, uint256 deadline, uint8 permitV, bytes32 permitR, bytes32 permitS) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct repayWithPermitCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
        pub onBehalfOf: alloy::sol_types::private::Address,
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        pub permitV: u8,
        pub permitR: alloy::sol_types::private::FixedBytes<32>,
        pub permitS: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)`](repayWithPermitCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct repayWithPermitReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<repayWithPermitCall> for UnderlyingRustTuple<'_> {
                fn from(value: repayWithPermitCall) -> Self {
                    (
                        value.asset,
                        value.amount,
                        value.interestRateMode,
                        value.onBehalfOf,
                        value.deadline,
                        value.permitV,
                        value.permitR,
                        value.permitS,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for repayWithPermitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        interestRateMode: tuple.2,
                        onBehalfOf: tuple.3,
                        deadline: tuple.4,
                        permitV: tuple.5,
                        permitR: tuple.6,
                        permitS: tuple.7,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<repayWithPermitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: repayWithPermitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for repayWithPermitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for repayWithPermitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = repayWithPermitReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [238u8, 62u8, 33u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.interestRateMode),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.permitV),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.permitR),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.permitS),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `rescueTokens(address,address,uint256)` and selector `0xcea9d26f`.
```solidity
function rescueTokens(address token, address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct rescueTokensCall {
        pub token: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`rescueTokens(address,address,uint256)`](rescueTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct rescueTokensReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rescueTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: rescueTokensCall) -> Self {
                    (value.token, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rescueTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        to: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rescueTokensReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rescueTokensReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rescueTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rescueTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rescueTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rescueTokens(address,address,uint256)";
            const SELECTOR: [u8; 4] = [206u8, 169u8, 210u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `resetIsolationModeTotalDebt(address)` and selector `0xe43e88a1`.
```solidity
function resetIsolationModeTotalDebt(address asset) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct resetIsolationModeTotalDebtCall {
        pub asset: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`resetIsolationModeTotalDebt(address)`](resetIsolationModeTotalDebtCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct resetIsolationModeTotalDebtReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resetIsolationModeTotalDebtCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetIsolationModeTotalDebtCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetIsolationModeTotalDebtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resetIsolationModeTotalDebtReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetIsolationModeTotalDebtReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetIsolationModeTotalDebtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resetIsolationModeTotalDebtCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resetIsolationModeTotalDebtReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resetIsolationModeTotalDebt(address)";
            const SELECTOR: [u8; 4] = [228u8, 62u8, 136u8, 161u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setConfiguration(address,(uint256))` and selector `0xf51e435b`.
```solidity
function setConfiguration(address asset, DataTypes.ReserveConfigurationMap memory configuration) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setConfigurationCall {
        pub asset: alloy::sol_types::private::Address,
        pub configuration: <DataTypes::ReserveConfigurationMap as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setConfiguration(address,(uint256))`](setConfigurationCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setConfigurationReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                DataTypes::ReserveConfigurationMap,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <DataTypes::ReserveConfigurationMap as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setConfigurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setConfigurationCall) -> Self {
                    (value.asset, value.configuration)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setConfigurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        configuration: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setConfigurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setConfigurationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setConfigurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setConfigurationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                DataTypes::ReserveConfigurationMap,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setConfigurationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setConfiguration(address,(uint256))";
            const SELECTOR: [u8; 4] = [245u8, 30u8, 67u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <DataTypes::ReserveConfigurationMap as alloy_sol_types::SolType>::tokenize(
                        &self.configuration,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setReserveInterestRateStrategyAddress(address,address)` and selector `0x1d2118f9`.
```solidity
function setReserveInterestRateStrategyAddress(address asset, address rateStrategyAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setReserveInterestRateStrategyAddressCall {
        pub asset: alloy::sol_types::private::Address,
        pub rateStrategyAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setReserveInterestRateStrategyAddress(address,address)`](setReserveInterestRateStrategyAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setReserveInterestRateStrategyAddressReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setReserveInterestRateStrategyAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setReserveInterestRateStrategyAddressCall) -> Self {
                    (value.asset, value.rateStrategyAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setReserveInterestRateStrategyAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        rateStrategyAddress: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setReserveInterestRateStrategyAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setReserveInterestRateStrategyAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setReserveInterestRateStrategyAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setReserveInterestRateStrategyAddressCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setReserveInterestRateStrategyAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setReserveInterestRateStrategyAddress(address,address)";
            const SELECTOR: [u8; 4] = [29u8, 33u8, 24u8, 249u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rateStrategyAddress,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setUserEMode(uint8)` and selector `0x28530a47`.
```solidity
function setUserEMode(uint8 categoryId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setUserEModeCall {
        pub categoryId: u8,
    }
    ///Container type for the return parameters of the [`setUserEMode(uint8)`](setUserEModeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setUserEModeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUserEModeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUserEModeCall) -> Self {
                    (value.categoryId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUserEModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { categoryId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUserEModeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUserEModeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUserEModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUserEModeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUserEModeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUserEMode(uint8)";
            const SELECTOR: [u8; 4] = [40u8, 83u8, 10u8, 71u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.categoryId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setUserUseReserveAsCollateral(address,bool)` and selector `0x5a3b74b9`.
```solidity
function setUserUseReserveAsCollateral(address asset, bool useAsCollateral) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setUserUseReserveAsCollateralCall {
        pub asset: alloy::sol_types::private::Address,
        pub useAsCollateral: bool,
    }
    ///Container type for the return parameters of the [`setUserUseReserveAsCollateral(address,bool)`](setUserUseReserveAsCollateralCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setUserUseReserveAsCollateralReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUserUseReserveAsCollateralCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUserUseReserveAsCollateralCall) -> Self {
                    (value.asset, value.useAsCollateral)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUserUseReserveAsCollateralCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        useAsCollateral: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUserUseReserveAsCollateralReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUserUseReserveAsCollateralReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUserUseReserveAsCollateralReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUserUseReserveAsCollateralCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUserUseReserveAsCollateralReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUserUseReserveAsCollateral(address,bool)";
            const SELECTOR: [u8; 4] = [90u8, 59u8, 116u8, 185u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.useAsCollateral,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `supply(address,uint256,address,uint16)` and selector `0x617ba037`.
```solidity
function supply(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supplyCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub onBehalfOf: alloy::sol_types::private::Address,
        pub referralCode: u16,
    }
    ///Container type for the return parameters of the [`supply(address,uint256,address,uint16)`](supplyCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supplyReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                u16,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: supplyCall) -> Self {
                    (value.asset, value.amount, value.onBehalfOf, value.referralCode)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        onBehalfOf: tuple.2,
                        referralCode: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: supplyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supplyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supplyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supply(address,uint256,address,uint16)";
            const SELECTOR: [u8; 4] = [97u8, 123u8, 160u8, 55u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.referralCode),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `supplyWithPermit(address,uint256,address,uint16,uint256,uint8,bytes32,bytes32)` and selector `0x02c205f0`.
```solidity
function supplyWithPermit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode, uint256 deadline, uint8 permitV, bytes32 permitR, bytes32 permitS) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supplyWithPermitCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub onBehalfOf: alloy::sol_types::private::Address,
        pub referralCode: u16,
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        pub permitV: u8,
        pub permitR: alloy::sol_types::private::FixedBytes<32>,
        pub permitS: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`supplyWithPermit(address,uint256,address,uint16,uint256,uint8,bytes32,bytes32)`](supplyWithPermitCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supplyWithPermitReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                u16,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supplyWithPermitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supplyWithPermitCall) -> Self {
                    (
                        value.asset,
                        value.amount,
                        value.onBehalfOf,
                        value.referralCode,
                        value.deadline,
                        value.permitV,
                        value.permitR,
                        value.permitS,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supplyWithPermitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        onBehalfOf: tuple.2,
                        referralCode: tuple.3,
                        deadline: tuple.4,
                        permitV: tuple.5,
                        permitR: tuple.6,
                        permitS: tuple.7,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supplyWithPermitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supplyWithPermitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supplyWithPermitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supplyWithPermitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supplyWithPermitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supplyWithPermit(address,uint256,address,uint16,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [2u8, 194u8, 5u8, 240u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.onBehalfOf,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.referralCode),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.permitV),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.permitR),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.permitS),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swapBorrowRateMode(address,uint256)` and selector `0x94ba89a2`.
```solidity
function swapBorrowRateMode(address asset, uint256 interestRateMode) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapBorrowRateModeCall {
        pub asset: alloy::sol_types::private::Address,
        pub interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`swapBorrowRateMode(address,uint256)`](swapBorrowRateModeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapBorrowRateModeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapBorrowRateModeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapBorrowRateModeCall) -> Self {
                    (value.asset, value.interestRateMode)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapBorrowRateModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        interestRateMode: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapBorrowRateModeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapBorrowRateModeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapBorrowRateModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapBorrowRateModeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapBorrowRateModeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapBorrowRateMode(address,uint256)";
            const SELECTOR: [u8; 4] = [148u8, 186u8, 137u8, 162u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.interestRateMode),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateBridgeProtocolFee(uint256)` and selector `0x3036b439`.
```solidity
function updateBridgeProtocolFee(uint256 bridgeProtocolFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateBridgeProtocolFeeCall {
        pub bridgeProtocolFee: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateBridgeProtocolFee(uint256)`](updateBridgeProtocolFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateBridgeProtocolFeeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateBridgeProtocolFeeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateBridgeProtocolFeeCall) -> Self {
                    (value.bridgeProtocolFee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateBridgeProtocolFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridgeProtocolFee: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateBridgeProtocolFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateBridgeProtocolFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateBridgeProtocolFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateBridgeProtocolFeeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateBridgeProtocolFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateBridgeProtocolFee(uint256)";
            const SELECTOR: [u8; 4] = [48u8, 54u8, 180u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.bridgeProtocolFee),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateFlashloanPremiums(uint128,uint128)` and selector `0xbcb6e522`.
```solidity
function updateFlashloanPremiums(uint128 flashLoanPremiumTotal, uint128 flashLoanPremiumToProtocol) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateFlashloanPremiumsCall {
        pub flashLoanPremiumTotal: u128,
        pub flashLoanPremiumToProtocol: u128,
    }
    ///Container type for the return parameters of the [`updateFlashloanPremiums(uint128,uint128)`](updateFlashloanPremiumsCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateFlashloanPremiumsReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Uint<128>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128, u128);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateFlashloanPremiumsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateFlashloanPremiumsCall) -> Self {
                    (value.flashLoanPremiumTotal, value.flashLoanPremiumToProtocol)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateFlashloanPremiumsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        flashLoanPremiumTotal: tuple.0,
                        flashLoanPremiumToProtocol: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateFlashloanPremiumsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateFlashloanPremiumsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateFlashloanPremiumsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateFlashloanPremiumsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Uint<128>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateFlashloanPremiumsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateFlashloanPremiums(uint128,uint128)";
            const SELECTOR: [u8; 4] = [188u8, 182u8, 229u8, 34u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.flashLoanPremiumTotal,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.flashLoanPremiumToProtocol,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `withdraw(address,uint256,address)` and selector `0x69328dec`.
```solidity
function withdraw(address asset, uint256 amount, address to) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct withdrawCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdraw(address,uint256,address)`](withdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct withdrawReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawCall) -> Self {
                    (value.asset, value.amount, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw(address,uint256,address)";
            const SELECTOR: [u8; 4] = [105u8, 50u8, 141u8, 236u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`IPool`](self) function calls.
    pub enum IPoolCalls {
        ADDRESSES_PROVIDER(ADDRESSES_PROVIDERCall),
        BRIDGE_PROTOCOL_FEE(BRIDGE_PROTOCOL_FEECall),
        FLASHLOAN_PREMIUM_TOTAL(FLASHLOAN_PREMIUM_TOTALCall),
        FLASHLOAN_PREMIUM_TO_PROTOCOL(FLASHLOAN_PREMIUM_TO_PROTOCOLCall),
        MAX_NUMBER_RESERVES(MAX_NUMBER_RESERVESCall),
        MAX_STABLE_RATE_BORROW_SIZE_PERCENT(MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall),
        backUnbacked(backUnbackedCall),
        borrow(borrowCall),
        configureEModeCategory(configureEModeCategoryCall),
        deposit(depositCall),
        dropReserve(dropReserveCall),
        finalizeTransfer(finalizeTransferCall),
        flashLoan(flashLoanCall),
        flashLoanSimple(flashLoanSimpleCall),
        getConfiguration(getConfigurationCall),
        getEModeCategoryData(getEModeCategoryDataCall),
        getReserveAddressById(getReserveAddressByIdCall),
        getReserveData(getReserveDataCall),
        getReserveNormalizedIncome(getReserveNormalizedIncomeCall),
        getReserveNormalizedVariableDebt(getReserveNormalizedVariableDebtCall),
        getReservesList(getReservesListCall),
        getUserAccountData(getUserAccountDataCall),
        getUserConfiguration(getUserConfigurationCall),
        getUserEMode(getUserEModeCall),
        initReserve(initReserveCall),
        liquidationCall(liquidationCallCall),
        mintToTreasury(mintToTreasuryCall),
        mintUnbacked(mintUnbackedCall),
        rebalanceStableBorrowRate(rebalanceStableBorrowRateCall),
        repay(repayCall),
        repayWithATokens(repayWithATokensCall),
        repayWithPermit(repayWithPermitCall),
        rescueTokens(rescueTokensCall),
        resetIsolationModeTotalDebt(resetIsolationModeTotalDebtCall),
        setConfiguration(setConfigurationCall),
        setReserveInterestRateStrategyAddress(setReserveInterestRateStrategyAddressCall),
        setUserEMode(setUserEModeCall),
        setUserUseReserveAsCollateral(setUserUseReserveAsCollateralCall),
        supply(supplyCall),
        supplyWithPermit(supplyWithPermitCall),
        swapBorrowRateMode(swapBorrowRateModeCall),
        updateBridgeProtocolFee(updateBridgeProtocolFeeCall),
        updateFlashloanPremiums(updateFlashloanPremiumsCall),
        withdraw(withdrawCall),
    }
    #[automatically_derived]
    impl IPoolCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 167u8, 24u8, 169u8],
            [2u8, 194u8, 5u8, 240u8],
            [5u8, 66u8, 151u8, 92u8],
            [7u8, 75u8, 46u8, 67u8],
            [29u8, 33u8, 24u8, 249u8],
            [39u8, 45u8, 144u8, 114u8],
            [40u8, 83u8, 10u8, 71u8],
            [45u8, 173u8, 151u8, 212u8],
            [48u8, 54u8, 180u8, 57u8],
            [53u8, 234u8, 106u8, 117u8],
            [56u8, 100u8, 151u8, 253u8],
            [66u8, 176u8, 183u8, 124u8],
            [68u8, 23u8, 165u8, 131u8],
            [82u8, 117u8, 23u8, 151u8],
            [87u8, 58u8, 222u8, 129u8],
            [90u8, 59u8, 116u8, 185u8],
            [97u8, 123u8, 160u8, 55u8],
            [99u8, 201u8, 184u8, 96u8],
            [105u8, 50u8, 141u8, 236u8],
            [105u8, 169u8, 51u8, 165u8],
            [106u8, 153u8, 192u8, 54u8],
            [108u8, 111u8, 106u8, 225u8],
            [122u8, 112u8, 142u8, 146u8],
            [148u8, 186u8, 137u8, 162u8],
            [156u8, 209u8, 153u8, 150u8],
            [164u8, 21u8, 188u8, 173u8],
            [171u8, 156u8, 75u8, 93u8],
            [188u8, 182u8, 229u8, 34u8],
            [191u8, 146u8, 133u8, 124u8],
            [196u8, 75u8, 17u8, 247u8],
            [205u8, 17u8, 35u8, 130u8],
            [206u8, 169u8, 210u8, 111u8],
            [209u8, 94u8, 0u8, 83u8],
            [209u8, 148u8, 109u8, 188u8],
            [213u8, 121u8, 234u8, 125u8],
            [213u8, 237u8, 57u8, 51u8],
            [214u8, 93u8, 199u8, 161u8],
            [228u8, 62u8, 136u8, 161u8],
            [232u8, 47u8, 236u8, 47u8],
            [232u8, 237u8, 169u8, 223u8],
            [237u8, 223u8, 27u8, 121u8],
            [238u8, 62u8, 33u8, 11u8],
            [245u8, 30u8, 67u8, 91u8],
            [248u8, 17u8, 157u8, 81u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IPoolCalls {
        const NAME: &'static str = "IPoolCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 44usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ADDRESSES_PROVIDER(_) => {
                    <ADDRESSES_PROVIDERCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::BRIDGE_PROTOCOL_FEE(_) => {
                    <BRIDGE_PROTOCOL_FEECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::FLASHLOAN_PREMIUM_TOTAL(_) => {
                    <FLASHLOAN_PREMIUM_TOTALCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::FLASHLOAN_PREMIUM_TO_PROTOCOL(_) => {
                    <FLASHLOAN_PREMIUM_TO_PROTOCOLCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_NUMBER_RESERVES(_) => {
                    <MAX_NUMBER_RESERVESCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_STABLE_RATE_BORROW_SIZE_PERCENT(_) => {
                    <MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::backUnbacked(_) => {
                    <backUnbackedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::borrow(_) => <borrowCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::configureEModeCategory(_) => {
                    <configureEModeCategoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit(_) => <depositCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::dropReserve(_) => {
                    <dropReserveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizeTransfer(_) => {
                    <finalizeTransferCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::flashLoan(_) => {
                    <flashLoanCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::flashLoanSimple(_) => {
                    <flashLoanSimpleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getConfiguration(_) => {
                    <getConfigurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEModeCategoryData(_) => {
                    <getEModeCategoryDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReserveAddressById(_) => {
                    <getReserveAddressByIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReserveData(_) => {
                    <getReserveDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReserveNormalizedIncome(_) => {
                    <getReserveNormalizedIncomeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReserveNormalizedVariableDebt(_) => {
                    <getReserveNormalizedVariableDebtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReservesList(_) => {
                    <getReservesListCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserAccountData(_) => {
                    <getUserAccountDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserConfiguration(_) => {
                    <getUserConfigurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserEMode(_) => {
                    <getUserEModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initReserve(_) => {
                    <initReserveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::liquidationCall(_) => {
                    <liquidationCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mintToTreasury(_) => {
                    <mintToTreasuryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mintUnbacked(_) => {
                    <mintUnbackedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rebalanceStableBorrowRate(_) => {
                    <rebalanceStableBorrowRateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::repay(_) => <repayCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::repayWithATokens(_) => {
                    <repayWithATokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::repayWithPermit(_) => {
                    <repayWithPermitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rescueTokens(_) => {
                    <rescueTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resetIsolationModeTotalDebt(_) => {
                    <resetIsolationModeTotalDebtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setConfiguration(_) => {
                    <setConfigurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setReserveInterestRateStrategyAddress(_) => {
                    <setReserveInterestRateStrategyAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUserEMode(_) => {
                    <setUserEModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUserUseReserveAsCollateral(_) => {
                    <setUserUseReserveAsCollateralCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supply(_) => <supplyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::supplyWithPermit(_) => {
                    <supplyWithPermitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapBorrowRateMode(_) => {
                    <swapBorrowRateModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateBridgeProtocolFee(_) => {
                    <updateBridgeProtocolFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateFlashloanPremiums(_) => {
                    <updateFlashloanPremiumsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdraw(_) => <withdrawCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IPoolCalls>] = &[
                {
                    fn liquidationCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <liquidationCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::liquidationCall)
                    }
                    liquidationCall
                },
                {
                    fn supplyWithPermit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <supplyWithPermitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::supplyWithPermit)
                    }
                    supplyWithPermit
                },
                {
                    fn ADDRESSES_PROVIDER(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <ADDRESSES_PROVIDERCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::ADDRESSES_PROVIDER)
                    }
                    ADDRESSES_PROVIDER
                },
                {
                    fn FLASHLOAN_PREMIUM_TOTAL(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <FLASHLOAN_PREMIUM_TOTALCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::FLASHLOAN_PREMIUM_TOTAL)
                    }
                    FLASHLOAN_PREMIUM_TOTAL
                },
                {
                    fn setReserveInterestRateStrategyAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <setReserveInterestRateStrategyAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::setReserveInterestRateStrategyAddress)
                    }
                    setReserveInterestRateStrategyAddress
                },
                {
                    fn BRIDGE_PROTOCOL_FEE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <BRIDGE_PROTOCOL_FEECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::BRIDGE_PROTOCOL_FEE)
                    }
                    BRIDGE_PROTOCOL_FEE
                },
                {
                    fn setUserEMode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <setUserEModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::setUserEMode)
                    }
                    setUserEMode
                },
                {
                    fn repayWithATokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <repayWithATokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::repayWithATokens)
                    }
                    repayWithATokens
                },
                {
                    fn updateBridgeProtocolFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <updateBridgeProtocolFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::updateBridgeProtocolFee)
                    }
                    updateBridgeProtocolFee
                },
                {
                    fn getReserveData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getReserveDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getReserveData)
                    }
                    getReserveData
                },
                {
                    fn getReserveNormalizedVariableDebt(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getReserveNormalizedVariableDebtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getReserveNormalizedVariableDebt)
                    }
                    getReserveNormalizedVariableDebt
                },
                {
                    fn flashLoanSimple(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <flashLoanSimpleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::flashLoanSimple)
                    }
                    flashLoanSimple
                },
                {
                    fn getUserConfiguration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getUserConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getUserConfiguration)
                    }
                    getUserConfiguration
                },
                {
                    fn getReserveAddressById(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getReserveAddressByIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getReserveAddressById)
                    }
                    getReserveAddressById
                },
                {
                    fn repay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <repayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::repay)
                    }
                    repay
                },
                {
                    fn setUserUseReserveAsCollateral(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <setUserUseReserveAsCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::setUserUseReserveAsCollateral)
                    }
                    setUserUseReserveAsCollateral
                },
                {
                    fn supply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <supplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::supply)
                    }
                    supply
                },
                {
                    fn dropReserve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <dropReserveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::dropReserve)
                    }
                    dropReserve
                },
                {
                    fn withdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <withdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::withdraw)
                    }
                    withdraw
                },
                {
                    fn mintUnbacked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <mintUnbackedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::mintUnbacked)
                    }
                    mintUnbacked
                },
                {
                    fn FLASHLOAN_PREMIUM_TO_PROTOCOL(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <FLASHLOAN_PREMIUM_TO_PROTOCOLCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::FLASHLOAN_PREMIUM_TO_PROTOCOL)
                    }
                    FLASHLOAN_PREMIUM_TO_PROTOCOL
                },
                {
                    fn getEModeCategoryData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getEModeCategoryDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getEModeCategoryData)
                    }
                    getEModeCategoryData
                },
                {
                    fn initReserve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <initReserveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::initReserve)
                    }
                    initReserve
                },
                {
                    fn swapBorrowRateMode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <swapBorrowRateModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::swapBorrowRateMode)
                    }
                    swapBorrowRateMode
                },
                {
                    fn mintToTreasury(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <mintToTreasuryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::mintToTreasury)
                    }
                    mintToTreasury
                },
                {
                    fn borrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <borrowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::borrow)
                    }
                    borrow
                },
                {
                    fn flashLoan(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <flashLoanCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::flashLoan)
                    }
                    flashLoan
                },
                {
                    fn updateFlashloanPremiums(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <updateFlashloanPremiumsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::updateFlashloanPremiums)
                    }
                    updateFlashloanPremiums
                },
                {
                    fn getUserAccountData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getUserAccountDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getUserAccountData)
                    }
                    getUserAccountData
                },
                {
                    fn getConfiguration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getConfiguration)
                    }
                    getConfiguration
                },
                {
                    fn rebalanceStableBorrowRate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <rebalanceStableBorrowRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::rebalanceStableBorrowRate)
                    }
                    rebalanceStableBorrowRate
                },
                {
                    fn rescueTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <rescueTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::rescueTokens)
                    }
                    rescueTokens
                },
                {
                    fn getReserveNormalizedIncome(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getReserveNormalizedIncomeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getReserveNormalizedIncome)
                    }
                    getReserveNormalizedIncome
                },
                {
                    fn getReservesList(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getReservesListCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getReservesList)
                    }
                    getReservesList
                },
                {
                    fn configureEModeCategory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <configureEModeCategoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::configureEModeCategory)
                    }
                    configureEModeCategory
                },
                {
                    fn finalizeTransfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <finalizeTransferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::finalizeTransfer)
                    }
                    finalizeTransfer
                },
                {
                    fn backUnbacked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <backUnbackedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::backUnbacked)
                    }
                    backUnbacked
                },
                {
                    fn resetIsolationModeTotalDebt(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <resetIsolationModeTotalDebtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::resetIsolationModeTotalDebt)
                    }
                    resetIsolationModeTotalDebt
                },
                {
                    fn MAX_STABLE_RATE_BORROW_SIZE_PERCENT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::MAX_STABLE_RATE_BORROW_SIZE_PERCENT)
                    }
                    MAX_STABLE_RATE_BORROW_SIZE_PERCENT
                },
                {
                    fn deposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <depositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::deposit)
                    }
                    deposit
                },
                {
                    fn getUserEMode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <getUserEModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::getUserEMode)
                    }
                    getUserEMode
                },
                {
                    fn repayWithPermit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <repayWithPermitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::repayWithPermit)
                    }
                    repayWithPermit
                },
                {
                    fn setConfiguration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <setConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::setConfiguration)
                    }
                    setConfiguration
                },
                {
                    fn MAX_NUMBER_RESERVES(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IPoolCalls> {
                        <MAX_NUMBER_RESERVESCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IPoolCalls::MAX_NUMBER_RESERVES)
                    }
                    MAX_NUMBER_RESERVES
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::ADDRESSES_PROVIDER(inner) => {
                    <ADDRESSES_PROVIDERCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BRIDGE_PROTOCOL_FEE(inner) => {
                    <BRIDGE_PROTOCOL_FEECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FLASHLOAN_PREMIUM_TOTAL(inner) => {
                    <FLASHLOAN_PREMIUM_TOTALCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FLASHLOAN_PREMIUM_TO_PROTOCOL(inner) => {
                    <FLASHLOAN_PREMIUM_TO_PROTOCOLCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_NUMBER_RESERVES(inner) => {
                    <MAX_NUMBER_RESERVESCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_STABLE_RATE_BORROW_SIZE_PERCENT(inner) => {
                    <MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::backUnbacked(inner) => {
                    <backUnbackedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::borrow(inner) => {
                    <borrowCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::configureEModeCategory(inner) => {
                    <configureEModeCategoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit(inner) => {
                    <depositCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::dropReserve(inner) => {
                    <dropReserveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizeTransfer(inner) => {
                    <finalizeTransferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::flashLoan(inner) => {
                    <flashLoanCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::flashLoanSimple(inner) => {
                    <flashLoanSimpleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getConfiguration(inner) => {
                    <getConfigurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getEModeCategoryData(inner) => {
                    <getEModeCategoryDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReserveAddressById(inner) => {
                    <getReserveAddressByIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReserveData(inner) => {
                    <getReserveDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReserveNormalizedIncome(inner) => {
                    <getReserveNormalizedIncomeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReserveNormalizedVariableDebt(inner) => {
                    <getReserveNormalizedVariableDebtCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReservesList(inner) => {
                    <getReservesListCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserAccountData(inner) => {
                    <getUserAccountDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserConfiguration(inner) => {
                    <getUserConfigurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserEMode(inner) => {
                    <getUserEModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initReserve(inner) => {
                    <initReserveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::liquidationCall(inner) => {
                    <liquidationCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mintToTreasury(inner) => {
                    <mintToTreasuryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mintUnbacked(inner) => {
                    <mintUnbackedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rebalanceStableBorrowRate(inner) => {
                    <rebalanceStableBorrowRateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::repay(inner) => {
                    <repayCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::repayWithATokens(inner) => {
                    <repayWithATokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::repayWithPermit(inner) => {
                    <repayWithPermitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rescueTokens(inner) => {
                    <rescueTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resetIsolationModeTotalDebt(inner) => {
                    <resetIsolationModeTotalDebtCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setConfiguration(inner) => {
                    <setConfigurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setReserveInterestRateStrategyAddress(inner) => {
                    <setReserveInterestRateStrategyAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setUserEMode(inner) => {
                    <setUserEModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setUserUseReserveAsCollateral(inner) => {
                    <setUserUseReserveAsCollateralCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supply(inner) => {
                    <supplyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supplyWithPermit(inner) => {
                    <supplyWithPermitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapBorrowRateMode(inner) => {
                    <swapBorrowRateModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateBridgeProtocolFee(inner) => {
                    <updateBridgeProtocolFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateFlashloanPremiums(inner) => {
                    <updateFlashloanPremiumsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ADDRESSES_PROVIDER(inner) => {
                    <ADDRESSES_PROVIDERCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BRIDGE_PROTOCOL_FEE(inner) => {
                    <BRIDGE_PROTOCOL_FEECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FLASHLOAN_PREMIUM_TOTAL(inner) => {
                    <FLASHLOAN_PREMIUM_TOTALCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FLASHLOAN_PREMIUM_TO_PROTOCOL(inner) => {
                    <FLASHLOAN_PREMIUM_TO_PROTOCOLCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_NUMBER_RESERVES(inner) => {
                    <MAX_NUMBER_RESERVESCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_STABLE_RATE_BORROW_SIZE_PERCENT(inner) => {
                    <MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::backUnbacked(inner) => {
                    <backUnbackedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::borrow(inner) => {
                    <borrowCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::configureEModeCategory(inner) => {
                    <configureEModeCategoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit(inner) => {
                    <depositCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::dropReserve(inner) => {
                    <dropReserveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizeTransfer(inner) => {
                    <finalizeTransferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::flashLoan(inner) => {
                    <flashLoanCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::flashLoanSimple(inner) => {
                    <flashLoanSimpleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getConfiguration(inner) => {
                    <getConfigurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getEModeCategoryData(inner) => {
                    <getEModeCategoryDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReserveAddressById(inner) => {
                    <getReserveAddressByIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReserveData(inner) => {
                    <getReserveDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReserveNormalizedIncome(inner) => {
                    <getReserveNormalizedIncomeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReserveNormalizedVariableDebt(inner) => {
                    <getReserveNormalizedVariableDebtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReservesList(inner) => {
                    <getReservesListCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserAccountData(inner) => {
                    <getUserAccountDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserConfiguration(inner) => {
                    <getUserConfigurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserEMode(inner) => {
                    <getUserEModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initReserve(inner) => {
                    <initReserveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::liquidationCall(inner) => {
                    <liquidationCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mintToTreasury(inner) => {
                    <mintToTreasuryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mintUnbacked(inner) => {
                    <mintUnbackedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rebalanceStableBorrowRate(inner) => {
                    <rebalanceStableBorrowRateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::repay(inner) => {
                    <repayCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::repayWithATokens(inner) => {
                    <repayWithATokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::repayWithPermit(inner) => {
                    <repayWithPermitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rescueTokens(inner) => {
                    <rescueTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resetIsolationModeTotalDebt(inner) => {
                    <resetIsolationModeTotalDebtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setConfiguration(inner) => {
                    <setConfigurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setReserveInterestRateStrategyAddress(inner) => {
                    <setReserveInterestRateStrategyAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setUserEMode(inner) => {
                    <setUserEModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setUserUseReserveAsCollateral(inner) => {
                    <setUserUseReserveAsCollateralCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supply(inner) => {
                    <supplyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::supplyWithPermit(inner) => {
                    <supplyWithPermitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapBorrowRateMode(inner) => {
                    <swapBorrowRateModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateBridgeProtocolFee(inner) => {
                    <updateBridgeProtocolFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateFlashloanPremiums(inner) => {
                    <updateFlashloanPremiumsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IPool`](self) events.
    pub enum IPoolEvents {
        BackUnbacked(BackUnbacked),
        Borrow(Borrow),
        FlashLoan(FlashLoan),
        IsolationModeTotalDebtUpdated(IsolationModeTotalDebtUpdated),
        LiquidationCall(LiquidationCall),
        MintUnbacked(MintUnbacked),
        MintedToTreasury(MintedToTreasury),
        RebalanceStableBorrowRate(RebalanceStableBorrowRate),
        Repay(Repay),
        ReserveDataUpdated(ReserveDataUpdated),
        ReserveUsedAsCollateralDisabled(ReserveUsedAsCollateralDisabled),
        ReserveUsedAsCollateralEnabled(ReserveUsedAsCollateralEnabled),
        Supply(Supply),
        SwapBorrowRateMode(SwapBorrowRateMode),
        UserEModeSet(UserEModeSet),
        Withdraw(Withdraw),
    }
    #[automatically_derived]
    impl IPoolEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8,
                5u8,
                138u8,
                86u8,
                234u8,
                148u8,
                101u8,
                60u8,
                223u8,
                79u8,
                21u8,
                45u8,
                34u8,
                122u8,
                206u8,
                34u8,
                212u8,
                192u8,
                10u8,
                217u8,
                158u8,
                42u8,
                67u8,
                245u8,
                140u8,
                183u8,
                217u8,
                227u8,
                254u8,
                178u8,
                149u8,
                242u8,
            ],
            [
                40u8,
                21u8,
                150u8,
                233u8,
                43u8,
                45u8,
                151u8,
                75u8,
                235u8,
                125u8,
                79u8,
                18u8,
                77u8,
                243u8,
                10u8,
                11u8,
                57u8,
                6u8,
                123u8,
                9u8,
                104u8,
                147u8,
                233u8,
                80u8,
                17u8,
                206u8,
                75u8,
                218u8,
                215u8,
                152u8,
                183u8,
                89u8,
            ],
            [
                43u8,
                98u8,
                119u8,
                54u8,
                188u8,
                161u8,
                92u8,
                213u8,
                56u8,
                29u8,
                207u8,
                128u8,
                176u8,
                191u8,
                17u8,
                253u8,
                25u8,
                125u8,
                1u8,
                160u8,
                55u8,
                197u8,
                43u8,
                146u8,
                122u8,
                136u8,
                26u8,
                16u8,
                251u8,
                115u8,
                186u8,
                97u8,
            ],
            [
                49u8,
                21u8,
                209u8,
                68u8,
                154u8,
                123u8,
                115u8,
                44u8,
                152u8,
                108u8,
                186u8,
                24u8,
                36u8,
                78u8,
                137u8,
                122u8,
                69u8,
                15u8,
                97u8,
                225u8,
                187u8,
                141u8,
                88u8,
                156u8,
                210u8,
                230u8,
                158u8,
                108u8,
                137u8,
                36u8,
                249u8,
                247u8,
            ],
            [
                68u8,
                197u8,
                141u8,
                129u8,
                54u8,
                91u8,
                102u8,
                221u8,
                75u8,
                26u8,
                127u8,
                54u8,
                194u8,
                90u8,
                169u8,
                123u8,
                140u8,
                113u8,
                195u8,
                97u8,
                238u8,
                73u8,
                55u8,
                173u8,
                193u8,
                160u8,
                0u8,
                0u8,
                34u8,
                125u8,
                181u8,
                221u8,
            ],
            [
                121u8,
                98u8,
                179u8,
                148u8,
                216u8,
                90u8,
                83u8,
                64u8,
                51u8,
                186u8,
                46u8,
                252u8,
                244u8,
                60u8,
                211u8,
                109u8,
                229u8,
                123u8,
                126u8,
                190u8,
                179u8,
                222u8,
                12u8,
                164u8,
                66u8,
                137u8,
                101u8,
                217u8,
                179u8,
                221u8,
                196u8,
                129u8,
            ],
            [
                128u8,
                76u8,
                155u8,
                132u8,
                43u8,
                39u8,
                72u8,
                162u8,
                43u8,
                182u8,
                75u8,
                52u8,
                84u8,
                83u8,
                163u8,
                222u8,
                124u8,
                165u8,
                74u8,
                108u8,
                164u8,
                92u8,
                224u8,
                13u8,
                65u8,
                88u8,
                148u8,
                151u8,
                158u8,
                34u8,
                137u8,
                122u8,
            ],
            [
                159u8,
                67u8,
                154u8,
                224u8,
                200u8,
                30u8,
                65u8,
                160u8,
                77u8,
                63u8,
                223u8,
                224u8,
                122u8,
                237u8,
                84u8,
                230u8,
                161u8,
                121u8,
                251u8,
                13u8,
                177u8,
                91u8,
                231u8,
                112u8,
                46u8,
                182u8,
                111u8,
                168u8,
                239u8,
                111u8,
                83u8,
                0u8,
            ],
            [
                165u8,
                52u8,
                200u8,
                219u8,
                231u8,
                31u8,
                135u8,
                31u8,
                159u8,
                53u8,
                48u8,
                233u8,
                122u8,
                116u8,
                96u8,
                31u8,
                234u8,
                23u8,
                180u8,
                38u8,
                202u8,
                224u8,
                46u8,
                28u8,
                90u8,
                238u8,
                66u8,
                201u8,
                108u8,
                120u8,
                64u8,
                81u8,
            ],
            [
                174u8,
                248u8,
                77u8,
                59u8,
                64u8,
                137u8,
                95u8,
                213u8,
                140u8,
                86u8,
                31u8,
                57u8,
                152u8,
                0u8,
                15u8,
                5u8,
                131u8,
                171u8,
                185u8,
                146u8,
                165u8,
                47u8,
                189u8,
                201u8,
                154u8,
                206u8,
                142u8,
                141u8,
                228u8,
                214u8,
                118u8,
                165u8,
            ],
            [
                179u8,
                208u8,
                132u8,
                130u8,
                15u8,
                177u8,
                169u8,
                222u8,
                207u8,
                251u8,
                23u8,
                100u8,
                54u8,
                189u8,
                2u8,
                85u8,
                141u8,
                21u8,
                250u8,
                201u8,
                176u8,
                221u8,
                254u8,
                216u8,
                196u8,
                101u8,
                188u8,
                115u8,
                89u8,
                215u8,
                220u8,
                224u8,
            ],
            [
                191u8,
                162u8,
                26u8,
                165u8,
                213u8,
                249u8,
                161u8,
                240u8,
                18u8,
                10u8,
                149u8,
                231u8,
                192u8,
                116u8,
                159u8,
                56u8,
                152u8,
                99u8,
                203u8,
                219u8,
                255u8,
                245u8,
                49u8,
                170u8,
                115u8,
                57u8,
                7u8,
                122u8,
                91u8,
                201u8,
                25u8,
                222u8,
            ],
            [
                215u8,
                40u8,
                218u8,
                135u8,
                95u8,
                200u8,
                137u8,
                68u8,
                203u8,
                241u8,
                118u8,
                56u8,
                188u8,
                190u8,
                74u8,
                240u8,
                238u8,
                218u8,
                239u8,
                99u8,
                190u8,
                205u8,
                29u8,
                28u8,
                87u8,
                204u8,
                9u8,
                126u8,
                180u8,
                96u8,
                141u8,
                132u8,
            ],
            [
                228u8,
                19u8,
                163u8,
                33u8,
                232u8,
                104u8,
                29u8,
                131u8,
                31u8,
                77u8,
                188u8,
                203u8,
                202u8,
                121u8,
                13u8,
                41u8,
                82u8,
                181u8,
                111u8,
                151u8,
                121u8,
                8u8,
                228u8,
                91u8,
                227u8,
                115u8,
                53u8,
                83u8,
                62u8,
                0u8,
                82u8,
                134u8,
            ],
            [
                239u8,
                239u8,
                171u8,
                165u8,
                233u8,
                33u8,
                87u8,
                49u8,
                0u8,
                144u8,
                10u8,
                58u8,
                217u8,
                207u8,
                41u8,
                242u8,
                34u8,
                217u8,
                149u8,
                251u8,
                59u8,
                96u8,
                69u8,
                121u8,
                126u8,
                174u8,
                167u8,
                82u8,
                27u8,
                216u8,
                214u8,
                240u8,
            ],
            [
                242u8,
                90u8,
                243u8,
                123u8,
                61u8,
                62u8,
                194u8,
                38u8,
                6u8,
                61u8,
                201u8,
                189u8,
                193u8,
                3u8,
                236u8,
                231u8,
                235u8,
                17u8,
                10u8,
                80u8,
                243u8,
                64u8,
                254u8,
                133u8,
                75u8,
                183u8,
                188u8,
                27u8,
                6u8,
                118u8,
                215u8,
                208u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IPoolEvents {
        const NAME: &'static str = "IPoolEvents";
        const COUNT: usize = 16usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<BackUnbacked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BackUnbacked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BackUnbacked)
                }
                Some(<Borrow as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Borrow as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Borrow)
                }
                Some(<FlashLoan as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <FlashLoan as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::FlashLoan)
                }
                Some(
                    <IsolationModeTotalDebtUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <IsolationModeTotalDebtUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::IsolationModeTotalDebtUpdated)
                }
                Some(<LiquidationCall as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LiquidationCall as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::LiquidationCall)
                }
                Some(<MintUnbacked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MintUnbacked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MintUnbacked)
                }
                Some(<MintedToTreasury as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MintedToTreasury as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MintedToTreasury)
                }
                Some(
                    <RebalanceStableBorrowRate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RebalanceStableBorrowRate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RebalanceStableBorrowRate)
                }
                Some(<Repay as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Repay as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Repay)
                }
                Some(
                    <ReserveDataUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ReserveDataUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ReserveDataUpdated)
                }
                Some(
                    <ReserveUsedAsCollateralDisabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ReserveUsedAsCollateralDisabled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ReserveUsedAsCollateralDisabled)
                }
                Some(
                    <ReserveUsedAsCollateralEnabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ReserveUsedAsCollateralEnabled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ReserveUsedAsCollateralEnabled)
                }
                Some(<Supply as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Supply as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Supply)
                }
                Some(
                    <SwapBorrowRateMode as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SwapBorrowRateMode as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SwapBorrowRateMode)
                }
                Some(<UserEModeSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UserEModeSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserEModeSet)
                }
                Some(<Withdraw as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Withdraw as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Withdraw)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for IPoolEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BackUnbacked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Borrow(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FlashLoan(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::IsolationModeTotalDebtUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LiquidationCall(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MintUnbacked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MintedToTreasury(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RebalanceStableBorrowRate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Repay(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ReserveDataUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ReserveUsedAsCollateralDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ReserveUsedAsCollateralEnabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Supply(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SwapBorrowRateMode(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserEModeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Withdraw(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BackUnbacked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Borrow(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FlashLoan(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::IsolationModeTotalDebtUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LiquidationCall(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MintUnbacked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MintedToTreasury(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RebalanceStableBorrowRate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Repay(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ReserveDataUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ReserveUsedAsCollateralDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ReserveUsedAsCollateralEnabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Supply(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SwapBorrowRateMode(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserEModeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Withdraw(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IPool`](self) contract instance.

See the [wrapper's documentation](`IPoolInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IPoolInstance<T, P, N> {
        IPoolInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IPoolInstance<T, P, N>>,
    > {
        IPoolInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        IPoolInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IPool`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IPool`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IPoolInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IPoolInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IPoolInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IPool`](self) contract instance.

See the [wrapper's documentation](`IPoolInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<IPoolInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IPoolInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IPoolInstance<T, P, N> {
            IPoolInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`ADDRESSES_PROVIDER`] function.
        pub fn ADDRESSES_PROVIDER(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, ADDRESSES_PROVIDERCall, N> {
            self.call_builder(&ADDRESSES_PROVIDERCall {})
        }
        ///Creates a new call builder for the [`BRIDGE_PROTOCOL_FEE`] function.
        pub fn BRIDGE_PROTOCOL_FEE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BRIDGE_PROTOCOL_FEECall, N> {
            self.call_builder(&BRIDGE_PROTOCOL_FEECall {})
        }
        ///Creates a new call builder for the [`FLASHLOAN_PREMIUM_TOTAL`] function.
        pub fn FLASHLOAN_PREMIUM_TOTAL(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, FLASHLOAN_PREMIUM_TOTALCall, N> {
            self.call_builder(&FLASHLOAN_PREMIUM_TOTALCall {})
        }
        ///Creates a new call builder for the [`FLASHLOAN_PREMIUM_TO_PROTOCOL`] function.
        pub fn FLASHLOAN_PREMIUM_TO_PROTOCOL(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            FLASHLOAN_PREMIUM_TO_PROTOCOLCall,
            N,
        > {
            self.call_builder(
                &FLASHLOAN_PREMIUM_TO_PROTOCOLCall {
                },
            )
        }
        ///Creates a new call builder for the [`MAX_NUMBER_RESERVES`] function.
        pub fn MAX_NUMBER_RESERVES(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_NUMBER_RESERVESCall, N> {
            self.call_builder(&MAX_NUMBER_RESERVESCall {})
        }
        ///Creates a new call builder for the [`MAX_STABLE_RATE_BORROW_SIZE_PERCENT`] function.
        pub fn MAX_STABLE_RATE_BORROW_SIZE_PERCENT(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall,
            N,
        > {
            self.call_builder(
                &MAX_STABLE_RATE_BORROW_SIZE_PERCENTCall {
                },
            )
        }
        ///Creates a new call builder for the [`backUnbacked`] function.
        pub fn backUnbacked(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            fee: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, backUnbackedCall, N> {
            self.call_builder(
                &backUnbackedCall {
                    asset,
                    amount,
                    fee,
                },
            )
        }
        ///Creates a new call builder for the [`borrow`] function.
        pub fn borrow(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
            referralCode: u16,
            onBehalfOf: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, borrowCall, N> {
            self.call_builder(
                &borrowCall {
                    asset,
                    amount,
                    interestRateMode,
                    referralCode,
                    onBehalfOf,
                },
            )
        }
        ///Creates a new call builder for the [`configureEModeCategory`] function.
        pub fn configureEModeCategory(
            &self,
            id: u8,
            config: <DataTypes::EModeCategory as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, configureEModeCategoryCall, N> {
            self.call_builder(
                &configureEModeCategoryCall {
                    id,
                    config,
                },
            )
        }
        ///Creates a new call builder for the [`deposit`] function.
        pub fn deposit(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            onBehalfOf: alloy::sol_types::private::Address,
            referralCode: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositCall, N> {
            self.call_builder(
                &depositCall {
                    asset,
                    amount,
                    onBehalfOf,
                    referralCode,
                },
            )
        }
        ///Creates a new call builder for the [`dropReserve`] function.
        pub fn dropReserve(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, dropReserveCall, N> {
            self.call_builder(&dropReserveCall { asset })
        }
        ///Creates a new call builder for the [`finalizeTransfer`] function.
        pub fn finalizeTransfer(
            &self,
            asset: alloy::sol_types::private::Address,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            balanceFromBefore: alloy::sol_types::private::primitives::aliases::U256,
            balanceToBefore: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, finalizeTransferCall, N> {
            self.call_builder(
                &finalizeTransferCall {
                    asset,
                    from,
                    to,
                    amount,
                    balanceFromBefore,
                    balanceToBefore,
                },
            )
        }
        ///Creates a new call builder for the [`flashLoan`] function.
        pub fn flashLoan(
            &self,
            receiverAddress: alloy::sol_types::private::Address,
            assets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            amounts: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            interestRateModes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            onBehalfOf: alloy::sol_types::private::Address,
            params: alloy::sol_types::private::Bytes,
            referralCode: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, flashLoanCall, N> {
            self.call_builder(
                &flashLoanCall {
                    receiverAddress,
                    assets,
                    amounts,
                    interestRateModes,
                    onBehalfOf,
                    params,
                    referralCode,
                },
            )
        }
        ///Creates a new call builder for the [`flashLoanSimple`] function.
        pub fn flashLoanSimple(
            &self,
            receiverAddress: alloy::sol_types::private::Address,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            params: alloy::sol_types::private::Bytes,
            referralCode: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, flashLoanSimpleCall, N> {
            self.call_builder(
                &flashLoanSimpleCall {
                    receiverAddress,
                    asset,
                    amount,
                    params,
                    referralCode,
                },
            )
        }
        ///Creates a new call builder for the [`getConfiguration`] function.
        pub fn getConfiguration(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getConfigurationCall, N> {
            self.call_builder(&getConfigurationCall { asset })
        }
        ///Creates a new call builder for the [`getEModeCategoryData`] function.
        pub fn getEModeCategoryData(
            &self,
            id: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getEModeCategoryDataCall, N> {
            self.call_builder(&getEModeCategoryDataCall { id })
        }
        ///Creates a new call builder for the [`getReserveAddressById`] function.
        pub fn getReserveAddressById(
            &self,
            id: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, getReserveAddressByIdCall, N> {
            self.call_builder(&getReserveAddressByIdCall { id })
        }
        ///Creates a new call builder for the [`getReserveData`] function.
        pub fn getReserveData(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getReserveDataCall, N> {
            self.call_builder(&getReserveDataCall { asset })
        }
        ///Creates a new call builder for the [`getReserveNormalizedIncome`] function.
        pub fn getReserveNormalizedIncome(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getReserveNormalizedIncomeCall, N> {
            self.call_builder(
                &getReserveNormalizedIncomeCall {
                    asset,
                },
            )
        }
        ///Creates a new call builder for the [`getReserveNormalizedVariableDebt`] function.
        pub fn getReserveNormalizedVariableDebt(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getReserveNormalizedVariableDebtCall,
            N,
        > {
            self.call_builder(
                &getReserveNormalizedVariableDebtCall {
                    asset,
                },
            )
        }
        ///Creates a new call builder for the [`getReservesList`] function.
        pub fn getReservesList(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getReservesListCall, N> {
            self.call_builder(&getReservesListCall {})
        }
        ///Creates a new call builder for the [`getUserAccountData`] function.
        pub fn getUserAccountData(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUserAccountDataCall, N> {
            self.call_builder(&getUserAccountDataCall { user })
        }
        ///Creates a new call builder for the [`getUserConfiguration`] function.
        pub fn getUserConfiguration(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUserConfigurationCall, N> {
            self.call_builder(&getUserConfigurationCall { user })
        }
        ///Creates a new call builder for the [`getUserEMode`] function.
        pub fn getUserEMode(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUserEModeCall, N> {
            self.call_builder(&getUserEModeCall { user })
        }
        ///Creates a new call builder for the [`initReserve`] function.
        pub fn initReserve(
            &self,
            asset: alloy::sol_types::private::Address,
            aTokenAddress: alloy::sol_types::private::Address,
            stableDebtAddress: alloy::sol_types::private::Address,
            variableDebtAddress: alloy::sol_types::private::Address,
            interestRateStrategyAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initReserveCall, N> {
            self.call_builder(
                &initReserveCall {
                    asset,
                    aTokenAddress,
                    stableDebtAddress,
                    variableDebtAddress,
                    interestRateStrategyAddress,
                },
            )
        }
        ///Creates a new call builder for the [`liquidationCall`] function.
        pub fn liquidationCall(
            &self,
            collateralAsset: alloy::sol_types::private::Address,
            debtAsset: alloy::sol_types::private::Address,
            user: alloy::sol_types::private::Address,
            debtToCover: alloy::sol_types::private::primitives::aliases::U256,
            receiveAToken: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, liquidationCallCall, N> {
            self.call_builder(
                &liquidationCallCall {
                    collateralAsset,
                    debtAsset,
                    user,
                    debtToCover,
                    receiveAToken,
                },
            )
        }
        ///Creates a new call builder for the [`mintToTreasury`] function.
        pub fn mintToTreasury(
            &self,
            assets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintToTreasuryCall, N> {
            self.call_builder(&mintToTreasuryCall { assets })
        }
        ///Creates a new call builder for the [`mintUnbacked`] function.
        pub fn mintUnbacked(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            onBehalfOf: alloy::sol_types::private::Address,
            referralCode: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintUnbackedCall, N> {
            self.call_builder(
                &mintUnbackedCall {
                    asset,
                    amount,
                    onBehalfOf,
                    referralCode,
                },
            )
        }
        ///Creates a new call builder for the [`rebalanceStableBorrowRate`] function.
        pub fn rebalanceStableBorrowRate(
            &self,
            asset: alloy::sol_types::private::Address,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, rebalanceStableBorrowRateCall, N> {
            self.call_builder(
                &rebalanceStableBorrowRateCall {
                    asset,
                    user,
                },
            )
        }
        ///Creates a new call builder for the [`repay`] function.
        pub fn repay(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
            onBehalfOf: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, repayCall, N> {
            self.call_builder(
                &repayCall {
                    asset,
                    amount,
                    interestRateMode,
                    onBehalfOf,
                },
            )
        }
        ///Creates a new call builder for the [`repayWithATokens`] function.
        pub fn repayWithATokens(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, repayWithATokensCall, N> {
            self.call_builder(
                &repayWithATokensCall {
                    asset,
                    amount,
                    interestRateMode,
                },
            )
        }
        ///Creates a new call builder for the [`repayWithPermit`] function.
        pub fn repayWithPermit(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
            onBehalfOf: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
            permitV: u8,
            permitR: alloy::sol_types::private::FixedBytes<32>,
            permitS: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, repayWithPermitCall, N> {
            self.call_builder(
                &repayWithPermitCall {
                    asset,
                    amount,
                    interestRateMode,
                    onBehalfOf,
                    deadline,
                    permitV,
                    permitR,
                    permitS,
                },
            )
        }
        ///Creates a new call builder for the [`rescueTokens`] function.
        pub fn rescueTokens(
            &self,
            token: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, rescueTokensCall, N> {
            self.call_builder(
                &rescueTokensCall {
                    token,
                    to,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`resetIsolationModeTotalDebt`] function.
        pub fn resetIsolationModeTotalDebt(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, resetIsolationModeTotalDebtCall, N> {
            self.call_builder(
                &resetIsolationModeTotalDebtCall {
                    asset,
                },
            )
        }
        ///Creates a new call builder for the [`setConfiguration`] function.
        pub fn setConfiguration(
            &self,
            asset: alloy::sol_types::private::Address,
            configuration: <DataTypes::ReserveConfigurationMap as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setConfigurationCall, N> {
            self.call_builder(
                &setConfigurationCall {
                    asset,
                    configuration,
                },
            )
        }
        ///Creates a new call builder for the [`setReserveInterestRateStrategyAddress`] function.
        pub fn setReserveInterestRateStrategyAddress(
            &self,
            asset: alloy::sol_types::private::Address,
            rateStrategyAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            setReserveInterestRateStrategyAddressCall,
            N,
        > {
            self.call_builder(
                &setReserveInterestRateStrategyAddressCall {
                    asset,
                    rateStrategyAddress,
                },
            )
        }
        ///Creates a new call builder for the [`setUserEMode`] function.
        pub fn setUserEMode(
            &self,
            categoryId: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, setUserEModeCall, N> {
            self.call_builder(&setUserEModeCall { categoryId })
        }
        ///Creates a new call builder for the [`setUserUseReserveAsCollateral`] function.
        pub fn setUserUseReserveAsCollateral(
            &self,
            asset: alloy::sol_types::private::Address,
            useAsCollateral: bool,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            setUserUseReserveAsCollateralCall,
            N,
        > {
            self.call_builder(
                &setUserUseReserveAsCollateralCall {
                    asset,
                    useAsCollateral,
                },
            )
        }
        ///Creates a new call builder for the [`supply`] function.
        pub fn supply(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            onBehalfOf: alloy::sol_types::private::Address,
            referralCode: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, supplyCall, N> {
            self.call_builder(
                &supplyCall {
                    asset,
                    amount,
                    onBehalfOf,
                    referralCode,
                },
            )
        }
        ///Creates a new call builder for the [`supplyWithPermit`] function.
        pub fn supplyWithPermit(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            onBehalfOf: alloy::sol_types::private::Address,
            referralCode: u16,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
            permitV: u8,
            permitR: alloy::sol_types::private::FixedBytes<32>,
            permitS: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supplyWithPermitCall, N> {
            self.call_builder(
                &supplyWithPermitCall {
                    asset,
                    amount,
                    onBehalfOf,
                    referralCode,
                    deadline,
                    permitV,
                    permitR,
                    permitS,
                },
            )
        }
        ///Creates a new call builder for the [`swapBorrowRateMode`] function.
        pub fn swapBorrowRateMode(
            &self,
            asset: alloy::sol_types::private::Address,
            interestRateMode: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapBorrowRateModeCall, N> {
            self.call_builder(
                &swapBorrowRateModeCall {
                    asset,
                    interestRateMode,
                },
            )
        }
        ///Creates a new call builder for the [`updateBridgeProtocolFee`] function.
        pub fn updateBridgeProtocolFee(
            &self,
            bridgeProtocolFee: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateBridgeProtocolFeeCall, N> {
            self.call_builder(
                &updateBridgeProtocolFeeCall {
                    bridgeProtocolFee,
                },
            )
        }
        ///Creates a new call builder for the [`updateFlashloanPremiums`] function.
        pub fn updateFlashloanPremiums(
            &self,
            flashLoanPremiumTotal: u128,
            flashLoanPremiumToProtocol: u128,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateFlashloanPremiumsCall, N> {
            self.call_builder(
                &updateFlashloanPremiumsCall {
                    flashLoanPremiumTotal,
                    flashLoanPremiumToProtocol,
                },
            )
        }
        ///Creates a new call builder for the [`withdraw`] function.
        pub fn withdraw(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawCall, N> {
            self.call_builder(&withdrawCall { asset, amount, to })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`BackUnbacked`] event.
        pub fn BackUnbacked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BackUnbacked, N> {
            self.event_filter::<BackUnbacked>()
        }
        ///Creates a new event filter for the [`Borrow`] event.
        pub fn Borrow_filter(&self) -> alloy_contract::Event<T, &P, Borrow, N> {
            self.event_filter::<Borrow>()
        }
        ///Creates a new event filter for the [`FlashLoan`] event.
        pub fn FlashLoan_filter(&self) -> alloy_contract::Event<T, &P, FlashLoan, N> {
            self.event_filter::<FlashLoan>()
        }
        ///Creates a new event filter for the [`IsolationModeTotalDebtUpdated`] event.
        pub fn IsolationModeTotalDebtUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, IsolationModeTotalDebtUpdated, N> {
            self.event_filter::<IsolationModeTotalDebtUpdated>()
        }
        ///Creates a new event filter for the [`LiquidationCall`] event.
        pub fn LiquidationCall_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, LiquidationCall, N> {
            self.event_filter::<LiquidationCall>()
        }
        ///Creates a new event filter for the [`MintUnbacked`] event.
        pub fn MintUnbacked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MintUnbacked, N> {
            self.event_filter::<MintUnbacked>()
        }
        ///Creates a new event filter for the [`MintedToTreasury`] event.
        pub fn MintedToTreasury_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MintedToTreasury, N> {
            self.event_filter::<MintedToTreasury>()
        }
        ///Creates a new event filter for the [`RebalanceStableBorrowRate`] event.
        pub fn RebalanceStableBorrowRate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RebalanceStableBorrowRate, N> {
            self.event_filter::<RebalanceStableBorrowRate>()
        }
        ///Creates a new event filter for the [`Repay`] event.
        pub fn Repay_filter(&self) -> alloy_contract::Event<T, &P, Repay, N> {
            self.event_filter::<Repay>()
        }
        ///Creates a new event filter for the [`ReserveDataUpdated`] event.
        pub fn ReserveDataUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ReserveDataUpdated, N> {
            self.event_filter::<ReserveDataUpdated>()
        }
        ///Creates a new event filter for the [`ReserveUsedAsCollateralDisabled`] event.
        pub fn ReserveUsedAsCollateralDisabled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ReserveUsedAsCollateralDisabled, N> {
            self.event_filter::<ReserveUsedAsCollateralDisabled>()
        }
        ///Creates a new event filter for the [`ReserveUsedAsCollateralEnabled`] event.
        pub fn ReserveUsedAsCollateralEnabled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ReserveUsedAsCollateralEnabled, N> {
            self.event_filter::<ReserveUsedAsCollateralEnabled>()
        }
        ///Creates a new event filter for the [`Supply`] event.
        pub fn Supply_filter(&self) -> alloy_contract::Event<T, &P, Supply, N> {
            self.event_filter::<Supply>()
        }
        ///Creates a new event filter for the [`SwapBorrowRateMode`] event.
        pub fn SwapBorrowRateMode_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SwapBorrowRateMode, N> {
            self.event_filter::<SwapBorrowRateMode>()
        }
        ///Creates a new event filter for the [`UserEModeSet`] event.
        pub fn UserEModeSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserEModeSet, N> {
            self.event_filter::<UserEModeSet>()
        }
        ///Creates a new event filter for the [`Withdraw`] event.
        pub fn Withdraw_filter(&self) -> alloy_contract::Event<T, &P, Withdraw, N> {
            self.event_filter::<Withdraw>()
        }
    }
}
