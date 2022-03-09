//solana program to perform cpi
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
    _program_id: &Pubkey, 
    account_info: &[AccountInfo], 
    instruction_data: &[u8],
)-> ProgramResult
{
    let account_info_iter = &mut account_info.iter();
    let (number, rest) = instruction_data.split_at(8); //number : number of accounts required to operate an instruction
    let number = number.try_into().map(u64::from_le_bytes).or(Err(ProgramError::MissingRequiredSignature))?;

    msg!("Extracted number:{}",number);
    let mut metas: Vec<AccountMeta> = Vec::with_capacity(std::mem::size_of::<AccountMeta>()*(number as usize));
  
    
    let mut i=0; 
    while i < number
    {
      let account_used = next_account_info(account_info_iter)?;
      if account_used.is_writable == true {
        let tmp_meta: AccountMeta=AccountMeta::new(*account_used.key, account_used.is_signer);
        metas.push(tmp_meta);
      } else {
        let tmp_meta: AccountMeta=AccountMeta::new_readonly(*account_used.key, account_used.is_signer);
        metas.push(tmp_meta);
      }
      
      i=i+1;
  
    }
    msg!("Account Metas include: {}",metas.len());
    let program =next_account_info(account_info_iter)?; 
    msg!("The called progran is :{}",*program.key);
    let instruction=instruction::Instruction::new_with_bytes(*program.key,rest,metas);

    msg!("Instruction Called");

    msg!("Rest: {:?}", rest);
   
    //cpi is 
    invoke(
    &instruction, 
    account_info,)?;
    msg!("Successfully implemented CPI");
        
    Ok(())

}
