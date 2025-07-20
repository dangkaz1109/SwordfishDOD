pub mod pool;

pub use pool::Pool;

#[macro_export]
macro_rules! create_table {
    ($struct_name:ident, $($field_name:ident : $field_type:ty),*) => {
        pub struct $struct_name {
            pub tail: usize,
            pub stail: usize,
            pub next_id: usize,
            pub id: $crate::pool::Pool<usize>,
            pub to_delete: $crate::pool::Pool<bool>,
            $(
                pub $field_name: $crate::pool::Pool<$field_type>,
            )*
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    tail: 0,
                    stail: 0,
                    next_id: 0,
                    id: $crate::pool::Pool::new(10_000_000),
                    to_delete: $crate::pool::Pool::new(10_000_000),
                    $(
                        $field_name: $crate::pool::Pool::new(10_000_000),
                    )*
                }
            }

            pub fn insert(&mut self, $($field_name: $field_type,)*) {
                if self.tail < 10_000_000 {
                    self.id.write(self.tail, self.next_id);
                    self.next_id += 1;
                    $(
                        self.$field_name.write(self.tail, $field_name);
                    )*
                    self.tail += 1;
                }
            }

            pub fn delete(&mut self, index: usize) {
                unsafe {
                    *self.to_delete.get(index) = true;
                }
            }

            pub fn commit_delete(&mut self) {
                let mut new_tail = 0;
                for i in 0..self.tail {
                    unsafe {
                        if !*self.to_delete.get(i) {
                            self.id.write(new_tail, *self.id.get(i));
                            $(
                                self.$field_name.write(new_tail, *self.$field_name.get(i));
                            )*
                            new_tail += 1;
                        }
                    }
                }
                self.tail = new_tail;
            } 
            pub fn lenght(&self) -> usize {
                let len = self.tail;
                len
            }
        }
    };
}


macro_rules! get_latest {
    ($table_variable:ident, $($field_name:ident),*) => {{
        if $table_variable.tail > 0 {
            index = $table_variable.tail - 1
        } else {
            panic!("EmptyList")
        }
        ($($table_variable.$field_name.get(index)),*)
    }};
}

macro_rules! get_by_id {
    ($table_variable:ident, &id: ident, $($field_name:ident),*) => {
        let mut have_id = true;
        //TO
    };
}



/// Sequential query macro
#[macro_export]
macro_rules! query {
    ($table_variable:ident, $($name:ident : $field_name:ident),*, $code_block:block) => {
        for i in 0..$table_variable.tail {
            $( let mut $name = unsafe { $table_variable.$field_name.get(i) }; )*
            let current = i;
            unsafe { $code_block }
        }
    };
}



/// Parallel query macro using rayon
#[macro_export]
macro_rules! query_parallel {
    ($table_variable:ident, $($name:ident : $field_name:ident),*, $code_block:block) => {
        (0..$table_variable.tail).into_par_iter().for_each(|i| {
            $( let mut $name = unsafe {$table_variable.$field_name.get(i)}; )*
            let current = i;
            unsafe {
                $code_block
            }
        });
    };
}
