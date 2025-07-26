
#[macro_export]
macro_rules! create_table {
    ($struct_name:ident, $($field_name:ident : $field_type:ty),*) => {
        pub struct $struct_name {
            pub next_id: usize,
            pub delete_count: usize,
            pub id: Vec<usize>,
            pub to_delete: Vec<usize>,
            pub mapset: Vec<Option<usize>>,
            pub id_stack: Vec<usize>,
            $(
                pub $field_name: Vec<$field_type>,
            )*
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    next_id: 0,
                    delete_count: 0,
                    id: Vec::with_capacity(100_000),
                    to_delete: Vec::with_capacity(100_000),
                    mapset: Vec::with_capacity(100_000),
                    id_stack: Vec::with_capacity(100_000),
                    $(
                        $field_name: Vec::with_capacity(100_000),
                    )*
                }
            }

            pub fn insert(&mut self, $($field_name: $field_type,)*) -> usize {
                let new_id = self.id_stack.pop().unwrap_or_else(|| {
                    let id = self.next_id;
                    self.next_id += 1;
                    id
                });
                self.id.push(new_id);
                if self.mapset.len() <= new_id {   
                    self.mapset.resize(new_id + 1, None);
                }
                self.mapset[new_id] = Some(self.id.len() - 1);
                $(
                    self.$field_name.push($field_name);
                )*
                new_id
            }

            pub fn delete_by_index(&mut self, index: usize) {
                if !self.to_delete.contains(&index) {
                    self.to_delete.push(index);
                    self.delete_count += 1;
                }
            }

            pub fn delete_by_id(&mut self, id: usize) {
                if let Some(Some(index)) = self.mapset.get(id) {
                    if *index < self.id.len() && self.id[*index] == id {
                         self.delete_by_index(*index);
                    }
                }
            }

            pub fn commit_delete(&mut self) {
                if self.to_delete.is_empty() {
                    return;
                }
                self.to_delete.sort_unstable();
                let mut deleted_count = 0;
                let initial_len = self.id.len();
                for i in (0..initial_len).rev() {
                    if self.to_delete.binary_search(&i).is_ok() {
                        let id_to_delete = self.id[i];
                        self.id_stack.push(id_to_delete);
                        self.mapset[id_to_delete] = None;
                        let last_index = initial_len - deleted_count - 1;
                        if i != last_index {
                            self.id.swap(i, last_index);
                            $( self.$field_name.swap(i, last_index); )*
                            let moved_id = self.id[i];
                            self.mapset[moved_id] = Some(i);
                        }
                        deleted_count += 1;
                    }
                }
                let new_len = initial_len - deleted_count;
                self.id.truncate(new_len);
                $( self.$field_name.truncate(new_len); )*
                self.to_delete.clear();
                self.delete_count = 0;
            }
        }
    };
}

#[macro_export]
macro_rules! query {
    ($table:ident, $id_name:ident, $( $name:ident : $field_name:ident ),*, $code_block:block) => {
        for current in 0..$table.id.len() {
            let $id_name = $table.id[current];
            $(
                let $name = &$table.$field_name[current];
            )*
            $code_block
        }
        $table.commit_delete()
    };
}

#[macro_export]
macro_rules! query_mut {
    ($table:ident, $id_name:ident, $( $name:ident : $field_name:ident ),*, $code_block:block) => {
        for current in 0..$table.id.len() {
            let $id_name = $table.id[current];
            $(
                let $name = &mut $table.$field_name[current];
            )*
            $code_block
        }
        $table.commit_delete()
    };
}