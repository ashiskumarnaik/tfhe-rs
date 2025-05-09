use crate::shortint::parameters::PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS;
use crate::shortint::{ClassicPBSParameters, PBSParameters};
use crate::{set_server_key, ClientKey, ConfigBuilder};

/// GPU setup for tests
///
/// Crates a client key, with the given parameters or default params in None were given
/// and sets the gpu server key for the current thread
pub(crate) fn setup_gpu(params: Option<impl Into<PBSParameters>>) -> ClientKey {
    let config = params
        .map_or_else(ConfigBuilder::default, |p| {
            ConfigBuilder::with_custom_parameters(p.into())
        })
        .build();

    let client_key = ClientKey::generate(config);
    let csks = crate::CompressedServerKey::new(&client_key);
    let server_key = csks.decompress_to_gpu();

    set_server_key(server_key);

    client_key
}

fn setup_default_gpu() -> ClientKey {
    setup_gpu(Option::<ClassicPBSParameters>::None)
}

#[test]
fn test_uint8_quickstart_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_uint8_quickstart(&client_key);
}

#[test]
fn test_uint8_quickstart_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_uint8_quickstart(&client_key);
}

#[test]
fn test_uint32_quickstart_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_uint32_quickstart(&client_key);
}

#[test]
fn test_uint32_quickstart_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_uint32_quickstart(&client_key);
}

#[test]
fn test_uint64_quickstart_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_uint64_quickstart(&client_key);
}

#[test]
fn test_uint64_quickstart_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_uint64_quickstart(&client_key);
}

#[test]
fn test_trivial_uint8_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_uint8_trivial(&client_key);
}

#[test]
fn test_trivial_uint256_small_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_uint256_trivial(&client_key);
}

#[test]
fn test_uint32_bitwise_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_uint32_bitwise(&client_key);
}

#[test]
fn test_uint32_bitwise_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_uint32_bitwise(&client_key);
}

#[test]
fn test_if_then_else_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_if_then_else(&client_key);
}

#[test]
fn test_if_then_else_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_if_then_else(&client_key);
}

#[test]
fn test_sum_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_sum(&client_key);
}

#[test]
fn test_sum_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_sum(&client_key);
}

#[test]
fn test_is_even_is_odd_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_is_even_is_odd(&client_key);
}

#[test]
fn test_is_even_is_odd_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_is_even_is_odd(&client_key);
}

#[test]
fn test_leading_trailing_zeros_ones_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_leading_trailing_zeros_ones(&client_key);
}

#[test]
fn test_leading_trailing_zeros_ones_gpu_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_leading_trailing_zeros_ones(&client_key);
}

#[test]
fn test_ilog2_gpu() {
    let client_key = setup_default_gpu();
    super::test_case_ilog2(&client_key);
}

#[test]
fn test_ilog2_multibit() {
    let client_key = setup_gpu(Some(PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS));
    super::test_case_ilog2(&client_key);
}
