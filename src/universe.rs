use fujitsu_mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};
use crate::communicator::Communicator;

pub struct Universe {

}

pub fn initialize() -> anyhow::Result<Universe> {
  let args: Vec<String> = std::env::args().collect();
  let mut argc = args.len() as i32;
  let mut argv: Vec<*mut u8> = Vec::new();
  for arg in &args {
    argv.push(arg.as_ptr() as *mut u8);
  }
  let mut argv_ptr = argv.as_mut_ptr();
  let r = unsafe {
    ffi::MPI_Init(&mut argc as *mut i32, &mut argv_ptr as *mut *mut *mut u8) as u32
  };
  match r {
    MPI_SUCCESS => Ok(Universe {}),
    _ => Err(anyhow::Error::msg(format!("[MPI_Init] Unknown code: {}", r))),
  }
}

impl Universe {
  pub fn world(&self) -> Communicator {
    Communicator::new(unsafe {
      &mut ffi::ompi_mpi_comm_world as *mut ffi::ompi_predefined_communicator_t as MPI_Comm
    })
  }
}

impl Drop for Universe {
  fn drop(&mut self) {
    let r = unsafe {
      ffi::MPI_Finalize() as u32
    };
    if r != MPI_SUCCESS {
      // TODO: Error handling?
    }
  }
}