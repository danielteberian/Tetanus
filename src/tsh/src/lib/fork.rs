use nix::Result;
use nix::unistd::{fork as nixfork, ForkResult};

pub fn fork() -> Result<ForkResult>
{
	unsafe{ nixfork() }
}
