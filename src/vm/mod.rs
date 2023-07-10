
fn copy_within_slice<T: Clone>(v: &mut [T], from: usize, to: usize, len: usize) {
    if from > to {
        let (dst, src) = v.split_at_mut(from);
        dst[to..to + len].clone_from_slice(&src[..len]);
    } else {
        let (src, dst) = v.split_at_mut(to);
        dst[..len].clone_from_slice(&src[from..from + len]);
    }
}

struct Scope {
    data: Box<[u8]>,
    actions: Vec<Action>,
}

pub enum Action {
    Copy {
        from: usize,
        to: usize,
        len: usize,
    },
    Call (usize),
    Match {
        left: usize,
        right: usize,
        len: usize,
        then: usize,
        otherwise: usize,
    },
    Debug {
        from: usize,
        len: usize,
    },
}

pub struct Vm {
    data: Vec<u8>,
}

impl Vm {
    pub fn call(&mut self, id: usize, scopes: &[Scope]) {
        let scope = &scopes[id];
        self.data.extend(scope.data.iter());

        for action in &scope.actions {
            match action {
                Action::Call(id) => self.call(*id, scopes),

                Action::Copy { from, to, len } => {
                    let from = self.data.len() - from - len;
                    let to = self.data.len() - to;
                    copy_within_slice(&mut self.data, from, to, *len);
                },

                Action::Match { left, right, len, then, otherwise } => {
                    let left_slice = &self.data[self.data.len() - left - len .. self.data.len() - left];
                    let right_slice = &self.data[self.data.len() - right - len .. self.data.len() - right];

                    if left_slice == right_slice {
                        self.call(*then, scopes)
                    } else {
                        self.call(*otherwise, scopes)
                    }
                },

                Action::Debug { from, len } => {
                    println![
                        "{:?}",
                        &self.data[self.data.len() - from - len .. self.data.len() - from]
                    ]
                }
            }
        }

        unsafe {
            self.data.set_len(self.data.len() - scope.data.len());
        }

        // for command in &scopes[id] {
        //     match command {
        //         Command::Copy { from, to, len } => {
                    
        //         },

        //         Command::Call (id) => self.call(*id, functions),

        //         // Command::Pop { count } => unsafe {
        //         //     self.data.set_len(self.data.len() - count)
        //         // },

        //         // Command::Push(value) => match value {
        //         //     Value::Ptr { from, count } => {
        //         //         self.data.extend_from_within(self.data.len() - from - count .. self.data.len() - from);
        //         //     },
        //         //     Value::Const(value) => {
        //         //         self.data.extend(value.iter());
        //         //     }
        //         // },

                
        //     }
        // }
    }
}

