use solana_program::{
  account_info::{AccountInfo,next_account_info},
  entrypoint,
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  program_error::ProgramError,
  msg,
  instruction::AccountMeta,
  instruction,
  program::invoke,
};
entrypoint!(cpi_use);

 pub fn cpi_use(
    program_id: &Pubkey, 
    account_info: &[AccountInfo], 
    instruction_data: &[u8],
)-> ProgramResult
{
    let account_info_iter = &mut account_info.iter();
    let (number, rest) = instruction_data.split_at(8);
    let number = number.try_into().map(u64::from_le_bytes).or(Err(ProgramError::MissingRequiredSignature))?;
    let mut metas: Vec<AccountMeta> = Vec::with_capacity(std::mem::size_of::<AccountMeta>()*(number as usize));
    let mut i=0;
    while i < number
    {
      let account_used = next_account_info(account_info_iter)?; 
      let tmp_meta: AccountMeta=AccountMeta::new(*account_used.key, account_used.is_signer);
      metas.push(tmp_meta);
      i=i+1;

    }

    let instruction=instruction::Instruction::new_with_bytes(*program_id, rest,metas);
    //cpi is 
      invoke(
      &instruction, 
      account_info)?;
      msg!("Successfully implemented CPI");
        
    Ok(())

}