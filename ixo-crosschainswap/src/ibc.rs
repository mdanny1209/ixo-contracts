
pub fn do_ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    // The channel this packet is being relayed along on this chain.
    let channel = msg.packet.dest.channel_id;
    let msg: IbcExecuteMsg = from_binary(&msg.packet.data)?;
    match msg {
        IbcExecuteMsg::Increment {} => execute_increment(deps, channel),
    }
}


fn execute_increment(deps: DepsMut, channel: String) -> Result<IbcReceiveResponse, ContractError> {
    let count = try_increment(deps, channel)?;
    Ok(IbcReceiveResponse::new()
        .add_attribute("method", "execute_increment")
        .add_attribute("count", count.to_string())
        .set_ack(make_ack_success()))
}