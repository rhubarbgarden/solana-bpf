impl Processor {
  pub fn process{...}
  
  fn process_init_escrow(
      accounts: &[AccountInfo],
      amount: u64,
      program_id: &Pubkey,
  ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let initializer = next_account_info(account_info_iter)?;

      if !initializer.is_signer {
          return Err(ProgramError::MissingRequiredSignature);
      }

      Ok(())
  }
}

use crate::instruction::EscrowInstruction;

pub struct Processor;
impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
      let instruction = EscrowInstruction::unpack(instruction_data)?;

      match instruction {
          EscrowInstruction::InitEscrow { amount } => {
              msg!("Instruction: InitEscrow");
              Self::process_init_escrow(accounts, amount, program_id)
          }
      }
  }
}