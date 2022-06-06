mod fprofile {
    #[derive(Clone)]
    pub struct User {
      pub first_name: String,
      pub last_name: String,
      pub prefix: String,
      pub suffix: String,
    }
    impl std::fmt::Debug for User {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User").field("first-name", &self.first_name).field("last-name", &self.last_name).field("prefix", &self.prefix).field("suffix", &self.suffix).finish()}
    }
    #[export_name = "gen-users"]
    unsafe extern "C" fn __wit_bindgen_gen_users(arg0: i32, ) -> i32{
      let result = <super::Fprofile as Fprofile>::gen_users(arg0 as u32);
      let ptr0 = RET_AREA.0.as_mut_ptr() as i32;
      let vec6 = result;
      let len6 = vec6.len() as i32;
      let layout6 = core::alloc::Layout::from_size_align_unchecked(vec6.len() * 32, 4);
      let result6 = std::alloc::alloc(layout6);
      if result6.is_null() { std::alloc::handle_alloc_error(layout6); }
      for (i, e) in vec6.into_iter().enumerate() {
        let base = result6 as i32 + (i as i32) * 32;
        {
          let User{ first_name:first_name1, last_name:last_name1, prefix:prefix1, suffix:suffix1, } = e;
          let vec2 = (first_name1.into_bytes()).into_boxed_slice();
          let ptr2 = vec2.as_ptr() as i32;
          let len2 = vec2.len() as i32;
          core::mem::forget(vec2);
          *((base + 4) as *mut i32) = len2;
          *((base + 0) as *mut i32) = ptr2;
          let vec3 = (last_name1.into_bytes()).into_boxed_slice();
          let ptr3 = vec3.as_ptr() as i32;
          let len3 = vec3.len() as i32;
          core::mem::forget(vec3);
          *((base + 12) as *mut i32) = len3;
          *((base + 8) as *mut i32) = ptr3;
          let vec4 = (prefix1.into_bytes()).into_boxed_slice();
          let ptr4 = vec4.as_ptr() as i32;
          let len4 = vec4.len() as i32;
          core::mem::forget(vec4);
          *((base + 20) as *mut i32) = len4;
          *((base + 16) as *mut i32) = ptr4;
          let vec5 = (suffix1.into_bytes()).into_boxed_slice();
          let ptr5 = vec5.as_ptr() as i32;
          let len5 = vec5.len() as i32;
          core::mem::forget(vec5);
          *((base + 28) as *mut i32) = len5;
          *((base + 24) as *mut i32) = ptr5;
         
        }}
        *((ptr0 + 4) as *mut i32) = len6;
        *((ptr0 + 0) as *mut i32) = result6 as i32;
        ptr0
      }
      pub trait Fprofile {
        fn gen_users(pcount: u32,) -> Vec<User>;
      }
     
      #[repr(align(4))]
      struct RetArea([u8; 8]);
      static mut RET_AREA: RetArea = RetArea([0; 8]);
    }
  