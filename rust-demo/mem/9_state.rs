use std::mem;

enum State {
    Initial,
    Processing,
    Final,
}

// 状态机
struct StateMachine {
    state: State,
    data: Vec<u8>,
}

impl StateMachine {
    fn transition(&mut self, new_state: State) -> State {
        mem::replace(&mut self.state, new_state)
    }

    fn process(&mut self) {
        // 状态转移, 并获取旧状态
        // 这里的 replace 函数会返回旧值
        // 并将新值替换到原来的位置
        // 这里的 match 语句会根据旧状态来做不同的处理
        
        let old_state = self.transition(State::Processing);
        // 处理逻辑
        match old_state {
            State::Initial => {
                println!("初始状态");
            }
            State::Processing => {
                println!("处理中");
            }
            State::Final => {
                println!("已完成");
            }
        }
    }
}

fn main() {
    let mut sm = StateMachine {
        state: State::Initial,
        data: vec![1, 2, 3],
    };
    sm.process();
    sm.process();
    sm.process();
}
