////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    forwarder_raw
    (
        callBack
        call_execute_on_dest_context
        call_execute_on_dest_context_readonly
        call_execute_on_dest_context_twice
        call_execute_on_same_context
        callback_args
        callback_args_at_index
        callback_payment_at_index
        callback_payments
        callback_payments_triples
        clear_callback_info
        deploy_contract
        deploy_from_source
        forward_async_call
        forward_async_call_half_payment
        forward_async_retrieve_multi_transfer_funds
        forward_direct_esdt_multi
        forward_direct_esdt_via_transf_exec
        forward_payment
        forward_transf_exec
        forward_transf_exec_egld
        forward_transf_exec_esdt
        forward_transf_exec_twice
        forwarder_async_send_and_retrieve_multi_transfer_funds
        upgrade
        upgrade_from_source
    )
}
